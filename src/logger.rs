use axum::{
    extract::{ConnectInfo, Request, State},
    middleware::Next,
    response::Response,
};
use chrono::Local;
use std::{net::SocketAddr, sync::Arc, time::Instant};
use crate::handlers::AppState;

pub async fn log_request(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request,
    next: Next,
) -> Response {
    let start = Instant::now();
    let method = req.method().clone();
    let uri = req.uri().clone();
    let path = uri.path().to_string();

    let response = next.run(req).await;

    let duration = start.elapsed();
    let status = response.status();

    let time = Local::now().format("%H:%M:%S");

    let status_str = if let Some(reason) = status.canonical_reason() {
        format!("{} ({})", status.as_u16(), reason)
    } else {
        status.as_u16().to_string()
    };

    let log_line = format!(
        "[{}] {} {} {} {} - {:?}",
        time,
        addr.ip(),
        method,
        path,
        status_str,
        duration
    );

    // Push to stats queue
    if let Ok(mut logs) = state.stats.logs.lock() {
        logs.push_back(log_line);
        // Limit log history to say 1000 entries
        if logs.len() > 1000 {
            logs.pop_front();
        }
    }

    response
}
