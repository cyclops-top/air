use crate::{fs_utils, view};
use axum::{
    extract::{Request, State},
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse, Json, Response},
};
use percent_encoding::percent_decode_str;
use std::path::PathBuf;
use std::sync::Arc;
use tower::ServiceExt;
use tower_http::services::ServeFile;

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

pub struct AppState {
    pub root_path: PathBuf,
    pub stats: Arc<Stats>,
    pub enable_https: bool,
    pub digest_cache: dashmap::DashMap<PathBuf, DigestEntry>,
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
        let mut res = list_directory(&abs_path, &decoded_path, headers).await;
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

        // Serve file
        // ServeFile handles Range requests automatically.
        let mut res = match ServeFile::new(abs_path).oneshot(req).await {
            Ok(res) => res.into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to serve file: {}", err),
            )
                .into_response(),
        };

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

async fn list_directory(
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
}