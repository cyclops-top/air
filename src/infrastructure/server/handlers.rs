use axum::{
    body::Body,
    extract::{Request, State},
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse, Json, Response},
};
use std::sync::Arc;
use std::ops::Range;
use percent_encoding::percent_decode_str;
use crate::application::file_service::FileService;
use crate::domain::models::{AppState, LogAction};
use crate::domain::traits::UiRenderer;
use crate::fs_utils;

pub struct ServerState {
    pub file_service: Arc<FileService>,
    pub ui_renderer: Arc<dyn UiRenderer>,
    pub app_state: Arc<AppState>,
}

pub async fn handle_request(
    State(state): State<Arc<ServerState>>,
    headers: HeaderMap,
    req: Request,
) -> Response {
    let uri_path = req.uri().path().to_string();
    let decoded_path = match percent_decode_str(&uri_path).decode_utf8() {
        Ok(p) => p.to_string(),
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    let abs_path = match fs_utils::sanitize_path(&state.app_state.root_path, &decoded_path) {
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

    let metadata = match std::fs::metadata(&abs_path) {
        Ok(m) => m,
        Err(_) => return StatusCode::NOT_FOUND.into_response(),
    };

    if metadata.is_dir() {
        if !uri_path.ends_with('/') {
            return axum::response::Redirect::permanent(&format!("{}/", uri_path)).into_response();
        }

        match state.file_service.get_listing(&decoded_path, &abs_path).await {
            Ok(listing) => {
                let accept = headers.get(header::ACCEPT).and_then(|v| v.to_str().ok()).unwrap_or("");
                let mut res = if accept.contains("application/json") {
                    Json(listing).into_response()
                } else {
                    Html(state.ui_renderer.render_html(&listing)).into_response()
                };
                res.extensions_mut().insert(LogAction::OpenDir);
                res
            }
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    } else {
        let file_size = metadata.len() as usize;
        let range_header = headers.get(header::RANGE).and_then(|v| v.to_str().ok());
        let range = range_header.and_then(|h| parse_range(h, file_size));

        match state.file_service.get_file_content(&abs_path, range.clone()).await {
            Ok((bytes, hash)) => {
                let etag = format!("\"{}\"", hash);
                if let Some(if_none_match) = headers.get(header::IF_NONE_MATCH).and_then(|v| v.to_str().ok()) {
                    if if_none_match == etag {
                        return StatusCode::NOT_MODIFIED.into_response();
                    }
                }

                let mut res = if let Some(r) = range {
                    let mut response = (StatusCode::PARTIAL_CONTENT, Body::from(bytes)).into_response();
                    response.headers_mut().insert(
                        header::CONTENT_RANGE,
                        header::HeaderValue::from_str(&format!("bytes {}-{}/{}", r.start, r.end - 1, file_size)).unwrap(),
                    );
                    response
                } else {
                    (StatusCode::OK, Body::from(bytes)).into_response()
                };

                let mime = mime_guess::from_path(&abs_path).first_or_octet_stream();
                res.headers_mut().insert(header::CONTENT_TYPE, mime.as_ref().parse().unwrap());
                res.headers_mut().insert(header::ACCEPT_RANGES, "bytes".parse().unwrap());
                res.headers_mut().insert(header::ETAG, etag.parse().unwrap());
                res.extensions_mut().insert(LogAction::DownloadFile);
                res
            }
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }
}

fn parse_range(range_header: &str, file_size: usize) -> Option<Range<usize>> {
    if !range_header.starts_with("bytes=") { return None; }
    let range_str = &range_header[6..];
    let parts: Vec<&str> = range_str.split('-').collect();
    if parts.len() != 2 { return None; }
    let start = parts[0].parse::<usize>().ok();
    let end = parts[1].parse::<usize>().ok();
    match (start, end) {
        (Some(s), Some(e)) => if s <= e && e < file_size { Some(s..e + 1) } else { None },
        (Some(s), None) => if s < file_size { Some(s..file_size) } else { None },
        (None, Some(e)) => if e > 0 { let s = file_size.saturating_sub(e); Some(s..file_size) } else { None },
        _ => None,
    }
}