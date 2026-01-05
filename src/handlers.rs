use axum::{
    extract::{Request, State},
    http::{header, StatusCode, HeaderMap},
    response::{Html, IntoResponse, Response, Json},
};
use std::path::PathBuf;
use std::sync::Arc;
use crate::{fs_utils, view};
use percent_encoding::percent_decode_str;
use tower::ServiceExt;
use tower_http::services::ServeFile;

#[derive(Clone)]
pub struct AppState {
    pub root_path: PathBuf,
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
        Err(_) => return StatusCode::FORBIDDEN.into_response(),
    };

    // Check metadata
    let metadata = match std::fs::metadata(&abs_path) {
        Ok(m) => m,
        Err(_) => return StatusCode::NOT_FOUND.into_response(),
    };

    if metadata.is_dir() {
        // List directory
        return list_directory(&abs_path, &decoded_path, headers).await;
    } else {
        // Serve file
        // ServeFile handles Range requests automatically.
        match ServeFile::new(abs_path).oneshot(req).await {
            Ok(res) => res.into_response(),
            Err(err) => {
                // Log error if needed, for now return 500
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to serve file: {}", err)).into_response()
            }
        }
    }
}

async fn list_directory(
    abs_path: &std::path::Path, 
    request_path: &str,
    headers: HeaderMap
) -> Response {
    let mut items = Vec::new();
    
    // Read dir
    let read_dir = match std::fs::read_dir(abs_path) {
        Ok(rd) => rd,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    for entry in read_dir {
        if let Ok(entry) = entry {
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
    let accept = headers.get(header::ACCEPT).and_then(|v| v.to_str().ok()).unwrap_or("");
    
    if accept.contains("application/json") {
        Json(listing).into_response()
    } else {
        Html(view::render_html(&listing)).into_response()
    }
}
