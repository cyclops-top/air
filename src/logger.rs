use crate::handlers::{AppState, LogAction, LogEntry};
use axum::{
    extract::{ConnectInfo, Request, State},
    middleware::Next,
    response::Response,
};
use chrono::Local;
use std::{net::SocketAddr, sync::Arc, time::Instant};

pub async fn log_request(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request,
    next: Next,
) -> Response {
    let start = Instant::now();
    let method = req.method().clone();
    let uri = req.uri().clone();
    let raw_path = uri.path().to_string();

    // Extract Range header before moving req
    let range = req.headers()
        .get(axum::http::header::RANGE)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.replace("bytes=", ""));

    let response = next.run(req).await;

    // Only log GET requests
    if method != axum::http::Method::GET {
        return response;
    }

    let duration = start.elapsed();
    let status = response.status();
    let action = response.extensions().get::<LogAction>();

    // Skip Favicon logging as requested
    if let Some(LogAction::Favicon) = action {
        return response;
    }

    // Decode URL path for readability
    let decoded_path = percent_encoding::percent_decode_str(&raw_path)
        .decode_utf8_lossy()
        .to_string();

    let time = Local::now().format("%H:%M:%S").to_string();
    let ip = addr.ip().to_string();
    let is_success = status.is_success();

    if let Some(&action) = action {
        let entry = LogEntry {
            time,
            ip,
            action,
            duration,
            path: decoded_path,
            is_success,
            range,
        };

        // Push to stats queue
        if let Ok(mut logs) = state.stats.logs.lock() {
            logs.push_back(entry);
            // Limit log history to say 1000 entries
            if logs.len() > 1000 {
                logs.pop_front();
            }
        }
    }

    response
}