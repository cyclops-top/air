use crate::{fs_utils, view};
use axum::{
    body::Body,
    extract::{Request, State},
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse, Json, Response},
};
use bytes::Bytes;
use memmap2::Mmap;
use percent_encoding::percent_decode_str;
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Weak};

use std::collections::VecDeque;
use std::sync::Mutex;

#[derive(Clone, Copy, Debug)]
pub enum LogAction {
    OpenDir,
    DownloadFile,
    Favicon,
}

pub struct LogEntry {
    pub time: String,
    pub ip: String,
    pub action: LogAction,
    pub duration: std::time::Duration,
    pub path: String,
    pub is_success: bool,
    pub range: Option<String>,
}

pub struct Stats {
    pub total_files: std::sync::atomic::AtomicU64,
    pub total_bytes: std::sync::atomic::AtomicU64,
    pub logs: Mutex<VecDeque<LogEntry>>,
    pub start_time: std::time::Instant,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            total_files: std::sync::atomic::AtomicU64::new(0),
            total_bytes: std::sync::atomic::AtomicU64::new(0),
            logs: Mutex::new(VecDeque::new()),
            start_time: std::time::Instant::now(),
        }
    }
}

pub struct DigestEntry {
    pub hash: String, // Base64 encoded SHA-256
    pub mtime: std::time::SystemTime,
    pub size: u64,
}

pub struct MappedFile {
    pub mmap: Mmap,
    pub path: PathBuf,
    cache: Arc<MmapCache>,
}

impl Drop for MappedFile {
    fn drop(&mut self) {
        self.cache.remove(&self.path);
    }
}

pub struct MmapCache {
    mappings: dashmap::DashMap<PathBuf, Weak<MappedFile>>,
}

impl MmapCache {
    pub fn new() -> Self {
        Self {
            mappings: dashmap::DashMap::new(),
        }
    }

    pub fn get_or_create(self: &Arc<Self>, path: &Path) -> std::io::Result<Arc<MappedFile>> {
        if let Some(weak) = self.mappings.get(path) {
            if let Some(arc) = weak.upgrade() {
                return Ok(arc);
            }
        }

        // Create new mapping
        let file = std::fs::File::open(path)?;
        let mmap = unsafe { Mmap::map(&file)? };

        let mapped_file = Arc::new(MappedFile {
            mmap,
            path: path.to_path_buf(),
            cache: self.clone(),
        });

        self.mappings
            .insert(path.to_path_buf(), Arc::downgrade(&mapped_file));
        Ok(mapped_file)
    }

    fn remove(&self, path: &Path) {
        // We only remove if the Weak reference is dead or points to something else
        // (though with PathBuf as key, it should be unique).
        // To be safe, we check if it can still be upgraded.
        if let Some(weak) = self.mappings.get(path) {
            if weak.upgrade().is_none() {
                drop(weak);
                self.mappings.remove(path);
            }
        }
    }
}

pub struct AppState {
    pub root_path: PathBuf,
    pub stats: Arc<Stats>,
    pub enable_https: bool,
    pub digest_cache: dashmap::DashMap<PathBuf, DigestEntry>,
    pub mmap_cache: Arc<MmapCache>,
    pub lan_ip: String,
    pub port: u16,
}

pub async fn handle_request(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    req: Request,
) -> Response {
    // We need to clone the path logic to avoid borrowing req while we need to move it later.
    let uri_path = req.uri().path().to_string();

    // Decode path
    let decoded_path = match percent_decode_str(&uri_path).decode_utf8() {
        Ok(p) => p.to_string(),
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    // Sanitize
    let abs_path = match fs_utils::sanitize_path(&state.root_path, &decoded_path) {
        Ok(p) => p,
        Err(e) => {
            if let Some(io_err) = e.downcast_ref::<std::io::Error>() {
                if io_err.kind() == std::io::ErrorKind::NotFound {
                    return StatusCode::NOT_FOUND.into_response();
                }
            }
            return StatusCode::FORBIDDEN.into_response();
        }
    };

    // Check metadata
    let metadata = match std::fs::metadata(&abs_path) {
        Ok(m) => m,
        Err(_) => return StatusCode::NOT_FOUND.into_response(),
    };

    if metadata.is_dir() {
        // Enforce trailing slash for directories
        if !uri_path.ends_with('/') {
            let mut new_uri = uri_path;
            new_uri.push('/');
            if let Some(query) = req.uri().query() {
                new_uri.push('?');
                new_uri.push_str(query);
            }
            return axum::response::Redirect::permanent(&new_uri).into_response();
        }

        // List directory
        let mut res = list_directory(state.clone(), &abs_path, &decoded_path, headers).await;
        res.extensions_mut().insert(LogAction::OpenDir);
        return res;
    } else {
        // 1. Calculate or Retrieve Digest
        let mtime = metadata
            .modified()
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH);
        let size = metadata.len();

        let hash = if let Some(entry) = state.digest_cache.get(&abs_path) {
            if entry.mtime == mtime && entry.size == size {
                entry.hash.clone()
            } else {
                drop(entry);
                let h = fs_utils::calculate_sha256(&abs_path).await.unwrap_or_default();
                state.digest_cache.insert(
                    abs_path.clone(),
                    DigestEntry {
                        hash: h.clone(),
                        mtime,
                        size,
                    },
                );
                h
            }
        } else {
            let h = fs_utils::calculate_sha256(&abs_path).await.unwrap_or_default();
            state.digest_cache.insert(
                abs_path.clone(),
                DigestEntry {
                    hash: h.clone(),
                    mtime,
                    size,
                },
            );
            h
        };

        // 2. ETag validation
        let etag = format!("\"{}\"", hash);
        if let Some(if_none_match) = headers.get(header::IF_NONE_MATCH).and_then(|v| v.to_str().ok()) {
            if if_none_match == etag {
                return StatusCode::NOT_MODIFIED.into_response();
            }
        }

        // Update stats
        state
            .stats
            .total_files
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        state
            .stats
            .total_bytes
            .fetch_add(metadata.len(), std::sync::atomic::Ordering::Relaxed);

        // Serve file using mmap
        let mapped = match state.mmap_cache.get_or_create(&abs_path) {
            Ok(m) => m,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to map file: {}", e),
                )
                    .into_response()
            }
        };

        let file_size = mapped.mmap.len();
        let mime = mime_guess::from_path(&abs_path).first_or_octet_stream();

        let mut res = if let Some(range_header) = headers.get(header::RANGE).and_then(|v| v.to_str().ok()) {
            if let Some(range) = parse_range(range_header, file_size) {
                let bytes = Bytes::copy_from_slice(&mapped.mmap[range.clone()]);
                // We use copy_from_slice here because Bytes from Mmap slice is tricky without unsafe
                // or specific crates. To truly be zero-copy and shared, we'd want to wrap Arc<MappedFile>
                // in something that implements Buf or Stream.
                // For now, let's use a simpler approach that still uses the shared mmap.
                
                let mut response = (StatusCode::PARTIAL_CONTENT, Body::from(bytes)).into_response();
                response.headers_mut().insert(
                    header::CONTENT_RANGE,
                    header::HeaderValue::from_str(&format!(
                        "bytes {}-{}/{}",
                        range.start,
                        range.end - 1,
                        file_size
                    ))
                    .unwrap(),
                );
                response
            } else {
                StatusCode::RANGE_NOT_SATISFIABLE.into_response()
            }
        } else {
            let bytes = Bytes::copy_from_slice(&mapped.mmap[..]);
            (StatusCode::OK, Body::from(bytes)).into_response()
        };

        res.headers_mut().insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_str(mime.as_ref()).unwrap(),
        );
        res.headers_mut().insert(
            header::ACCEPT_RANGES,
            header::HeaderValue::from_static("bytes"),
        );

        // 3. Inject Digest and ETag Headers
        if !hash.is_empty() {
            if let Ok(val) = header::HeaderValue::from_str(&format!("SHA-256={}", hash)) {
                res.headers_mut()
                    .insert(header::HeaderName::from_static("digest"), val);
            }
            if let Ok(val) = header::HeaderValue::from_str(&etag) {
                res.headers_mut().insert(header::ETAG, val);
            }
        }

        res.extensions_mut().insert(LogAction::DownloadFile);
        return res;
    }
}

const FAVICON_SVG: &[u8] = include_bytes!("../docs/favicon.svg");

pub async fn favicon() -> impl IntoResponse {
    let mut res = ([(header::CONTENT_TYPE, "image/svg+xml")], FAVICON_SVG).into_response();
    res.extensions_mut().insert(LogAction::Favicon);
    res
}

fn parse_range(range_header: &str, file_size: usize) -> Option<Range<usize>> {
    if !range_header.starts_with("bytes=") {
        return None;
    }

    let range_str = &range_header[6..];
    let parts: Vec<&str> = range_str.split('-').collect();
    if parts.len() != 2 {
        return None;
    }

    let start = parts[0].parse::<usize>().ok();
    let end = parts[1].parse::<usize>().ok();

    match (start, end) {
        (Some(s), Some(e)) => {
            if s <= e && e < file_size {
                Some(s..e + 1)
            } else {
                None
            }
        }
        (Some(s), None) => {
            if s < file_size {
                Some(s..file_size)
            } else {
                None
            }
        }
        (None, Some(e)) => {
            if e > 0 {
                let s = file_size.saturating_sub(e);
                Some(s..file_size)
            } else {
                None
            }
        }
        _ => None,
    }
}

async fn list_directory(
    state: Arc<AppState>,
    abs_path: &std::path::Path,
    request_path: &str,
    headers: HeaderMap,
) -> Response {
    let mut items = Vec::new();

    // Read dir
    let read_dir = match std::fs::read_dir(abs_path) {
        Ok(rd) => rd,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    for entry in read_dir.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();

        // Hidden file filter
        if name.starts_with('.') {
            continue;
        }

        let meta = entry.metadata().ok();
        let size = meta.as_ref().map(|m| m.len()).unwrap_or(0);
        let is_dir = meta.as_ref().map(|m| m.is_dir()).unwrap_or(false);

        // Modification time
        // Simple string format
        let mod_time = if let Some(m) = meta {
            if let Ok(t) = m.modified() {
                // Convert SystemTime to string using chrono
                let dt: chrono::DateTime<chrono::Local> = t.into();
                dt.to_rfc3339()
            } else {
                "".to_string()
            }
        } else {
            "".to_string()
        };

        items.push(view::FileEntry {
            name,
            is_dir,
            size,
            mod_time,
        });
    }

    // Sort: Directories first, then files
    items.sort_by(|a, b| {
        if a.is_dir && !b.is_dir {
            std::cmp::Ordering::Less
        } else if !a.is_dir && b.is_dir {
            std::cmp::Ordering::Greater
        } else {
            a.name.cmp(&b.name)
        }
    });

    let listing = view::DirectoryListing {
        current_path: request_path.to_string(),
        items,
        lan_ip: state.lan_ip.clone(),
        port: state.port,
    };

    // Check Accept header
    let accept = headers
        .get(header::ACCEPT)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if accept.contains("application/json") {
        Json(listing).into_response()
    } else {
        Html(view::render_html(&listing)).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;
    use axum::http::header;

    #[tokio::test]
    async fn test_favicon() {
        let response = favicon().await.into_response();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE).unwrap(),
            "image/svg+xml"
        );

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(body, &FAVICON_SVG[..]);
    }

    #[test]
    fn test_digest_cache_invalidation_logic() {
        let cache = dashmap::DashMap::new();
        let path = PathBuf::from("test.txt");
        let mtime1 = std::time::SystemTime::now();
        let size1 = 100;
        let hash1 = "hash1".to_string();

        // Initial insert
        cache.insert(
            path.clone(),
            DigestEntry {
                hash: hash1.clone(),
                mtime: mtime1,
                size: size1,
            },
        );

        // 1. Same mtime and size -> Cache Hit
        {
            let entry = cache.get(&path).unwrap();
            assert_eq!(entry.mtime, mtime1);
            assert_eq!(entry.size, size1);
            assert_eq!(entry.hash, hash1);
        }

        // 2. Different mtime -> Cache Miss (simulated by logic in handle_request)
        let mtime2 = mtime1 + std::time::Duration::from_secs(1);
        let size2 = 100;
        
        let hash_to_use = if let Some(entry) = cache.get(&path) {
            if entry.mtime == mtime2 && entry.size == size2 {
                entry.hash.clone()
            } else {
                "hash2".to_string() // Recalculated
            }
        } else {
            "hash2".to_string()
        };
        assert_eq!(hash_to_use, "hash2");

        // 3. Different size -> Cache Miss
        let mtime3 = mtime1;
        let size3 = 200;
        let hash_to_use = if let Some(entry) = cache.get(&path) {
            if entry.mtime == mtime3 && entry.size == size3 {
                entry.hash.clone()
            } else {
                "hash3".to_string() // Recalculated
            }
        } else {
            "hash3".to_string()
        };
        assert_eq!(hash_to_use, "hash3");
    }

    #[tokio::test]
    async fn test_mmap_cache_sharing() {
        let cache = Arc::new(MmapCache::new());
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test_mmap.txt");
        std::fs::write(&file_path, "mmap test content").unwrap();

        // 1. Create first mapping
        let mmap1 = cache.get_or_create(&file_path).unwrap();
        assert_eq!(&mmap1.mmap[..], b"mmap test content");

        // 2. Get second mapping (should be shared)
        let mmap2 = cache.get_or_create(&file_path).unwrap();
        assert!(Arc::ptr_eq(&mmap1, &mmap2));

        // 3. Drop all mappings
        let weak = Arc::downgrade(&mmap1);
        drop(mmap1);
        drop(mmap2);

        // 4. Verify cache eventually clears (when last reference dropped)
        assert!(weak.upgrade().is_none());
        
        // Cache entry should be removable/removed on next access or via explicit cleanup
        // Our 'remove' logic is triggered on Drop of MappedFile.
        assert_eq!(cache.mappings.len(), 0);
    }
}