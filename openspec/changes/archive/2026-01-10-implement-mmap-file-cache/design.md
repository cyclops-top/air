# Design: Memory-Mapped File Sharing Cache

## 1. Overview
The goal is to replace standard file I/O with memory-mapped I/O for file serving. We will implement a cache that stores active memory mappings and shares them across requests.

## 2. Components

### 2.1 MmapCache
A thread-safe registry of active memory mappings.
```rust
pub struct MmapCache {
    // Map path to a weak reference of the mapping
    mappings: DashMap<PathBuf, Weak<MappedFile>>,
}
```

### 2.2 MappedFile
A wrapper around `memmap2::Mmap` that handles self-removal from the cache when dropped.
```rust
pub struct MappedFile {
    pub mmap: memmap2::Mmap,
    path: PathBuf,
    cache: Arc<MmapCache>,
}

impl Drop for MappedFile {
    fn drop(&mut self) {
        // Only remove from cache if no one else has recreated it
        // and this was the last reference.
        self.cache.remove_if_match(&self.path, self);
    }
}
```
*Note: `remove_if_match` logic needs to be careful about race conditions.*

### 2.3 Custom File Handler
A new handler function (or modification to `handle_request`) that:
1. Acquires an `Arc<MappedFile>` from `MmapCache`.
2. Identifies the requested range.
3. Creates a `Body` that wraps a slice of the `Mmap`.
4. Ensures the `Arc<MappedFile>` is kept alive for the duration of the response.

## 3. Reference Counting and Cleanup
We use `Arc` for sharing and `Weak` in the cache to avoid keeping the mapping alive if no requests are using it.
When a request comes:
1. Check `MmapCache` for `Weak<MappedFile>`.
2. If `Weak::upgrade()` succeeds, we have a shared mapping.
3. If not, create a new `Mmap`, wrap in `Arc<MappedFile>`, and store `Arc::downgrade()` in the cache.

## 4. Zero-Copy in Axum
To achieve zero-copy, we can use `axum::body::Body` created from `Bytes`.
We can wrap the `Arc<MappedFile>` in a custom struct that implements `Buf` or just use `Bytes` if we can safely wrap the `Mmap` slice.
Actually, `Bytes` can be created from an `Arc<Vec<u8>>` or similar. For `Mmap`, we might need a custom `Body` implementation or use a library that provides `Bytes` from `Mmap`.
If we use `axum::body::Body::from(bytes::Bytes)`, we need to ensure the underlying memory stays valid.
One way:
```rust
let data: &'static [u8] = unsafe { std::mem::transmute(&mmap[range]) };
let body = Body::from(Bytes::from_static(data)); // DANGEROUS without care
```
Better:
```rust
// Use a Body that holds the Arc<MappedFile>
struct MmapBody {
    data: Arc<MappedFile>,
    range: std::ops::Range<usize>,
}
// Implement Stream for MmapBody
```
Or use `bytes::Bytes` with a custom destructor if possible, but Axum/Bytes doesn't make this trivial in 1.0 without some tricks.
Actually, `http_body_util::Full<Bytes>` is common.
We can use `Bytes::from_owner` if we use a crate like `bytes-owner` or just implement a simple `Stream`.

## 5. Range Request Handling
Since we are replacing `ServeFile`, we must manually handle:
- `Range` header parsing (e.g., `bytes=0-1023`).
- `206 Partial Content` vs `200 OK`.
- `Content-Range` header.
- `Content-Type` detection (using `mime_guess` or similar, or just `tower_http`'s logic).
- `Accept-Ranges: bytes`.

## 6. Concurrency and Locking
`DashMap` handles most concurrency. To avoid "thundering herd" (multiple requests creating the same mmap simultaneously), we can use a `dashmap` entry lock or a separate `Mutex` map for file paths being initialized.
