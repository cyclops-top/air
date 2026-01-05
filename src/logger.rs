use axum::{
    extract::{ConnectInfo, Request},
    middleware::Next,
    response::Response,
};
use chrono::Local;
use std::{net::SocketAddr, time::Instant};

pub async fn log_request(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request,
    next: Next,
) -> Response {
    let start = Instant::now();
    let method = req.method().clone();
    let uri = req.uri().clone();
    let path = uri.path();

    let response = next.run(req).await;

    let duration = start.elapsed();
    let status = response.status();

    // Format: [Time] [IP] [Method] [Path] [Status] [Duration]
    // Time example: 14:20:01
    let time = Local::now().format("%H:%M:%S");

    // Simplify status to just code + reason if possible, or just code. 
    // Spec example: 200 (OK) or 206 (Partial Content). 
    // axum/http status default printing might not include reason string easily without a lookup, 
    // but typical Debug/Display might be close. 
    // Let's match the spec example "200 (OK)" roughly or just "200". 
    // The spec says: `206 (Partial Content)`
    let status_str = if let Some(reason) = status.canonical_reason() {
        format!("{} ({})", status.as_u16(), reason)
    } else {
        status.as_u16().to_string()
    };

    println!(
        "[{}] {} {} {} {} - {:?}",
        time,
        addr.ip(),
        method,
        path,
        status_str,
        duration
    );

    response
}
