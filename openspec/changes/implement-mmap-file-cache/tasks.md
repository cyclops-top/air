# Tasks: Implement Memory-Mapped File Sharing Cache

- [x] **Dependencies Update**
    - Add `memmap2` to `Cargo.toml`.
- [x] **Core Implementation**
    - Implement `MmapCache` struct in `src/handlers.rs` or a new module.
    - Implement `MappedFile` wrapper with `Drop` logic for self-removal from cache.
    - Add `MmapCache` to `AppState`.
- [x] **Handler Integration**
    - Modify `handle_request` in `src/handlers.rs` to use `MmapCache`.
    - Implement manual `Range` request parsing and response generation.
    - Implement `MmapBody` or a mechanism to stream `mmap` data zero-copy in Axum.
- [x] **Refinement**
    - Ensure `Content-Type` and `ETag` headers are correctly handled with the new implementation.
    - Add logging for mmap creation/release (optional, but good for debugging).
- [x] **Verification**
    - [x] Verify large file download works correctly.
    - [x] Verify `Range` requests (e.g., `curl -H "Range: bytes=0-100"`) work correctly.
    - [x] Verify multiple concurrent requests share the same mapping (can be checked via logs).
    - [x] Verify `ETag` and `304 Not Modified` still work as expected.
