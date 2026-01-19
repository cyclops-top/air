pub mod handlers;

use std::sync::Arc;
use std::net::SocketAddr;
use std::path::PathBuf;
use axum::{routing::get, Router, extract::Request, middleware::Next, response::{Response, IntoResponse}};
use crate::application::file_service::FileService;
use crate::domain::traits::UiRenderer;
use crate::domain::models::{AppState, LogAction, LogEntry};
use self::handlers::{handle_request, ServerState};
use crate::cert;

pub async fn start_server(
    port_opt: Option<u16>,
    _root_path: PathBuf,
    enable_https: bool,
    _lan_ip: std::net::IpAddr,
    file_service: Arc<FileService>,
    ui_renderer: Arc<dyn UiRenderer>,
    app_state: Arc<AppState>,
) -> anyhow::Result<u16> {
    let state = Arc::new(ServerState {
        file_service,
        ui_renderer,
        app_state: app_state.clone(),
    });

    let router = Router::new()
        .fallback(handle_request)
        .route("/favicon.ico", get(favicon))
        .layer(axum::middleware::from_fn_with_state(app_state.clone(), stats_middleware))
        .with_state(state);

    let start_port = port_opt.unwrap_or(10000);
    let mut current_port = start_port;
    
    loop {
        let addr = SocketAddr::from(([0, 0, 0, 0], current_port));
        if enable_https {
            let config = cert::get_config(_lan_ip)?;
            let server = axum_server::bind_rustls(addr, config).serve(router.clone().into_make_service());
            tokio::spawn(async move { let _ = server.await; });
            return Ok(current_port);
        } else {
            let listener = match tokio::net::TcpListener::bind(addr).await {
                Ok(l) => l,
                Err(_) if port_opt.is_none() && current_port < 65535 => {
                    current_port += 1;
                    continue;
                }
                Err(e) => return Err(e.into()),
            };
            tokio::spawn(async move { let _ = axum::serve(listener, router.into_make_service()).await; });
            return Ok(current_port);
        }
    }
}

const FAVICON_SVG: &[u8] = include_bytes!("../../../docs/favicon.svg");
async fn favicon() -> impl IntoResponse {
    let mut res = ([(axum::http::header::CONTENT_TYPE, "image/svg+xml")], FAVICON_SVG).into_response();
    res.extensions_mut().insert(LogAction::Favicon);
    res
}

async fn stats_middleware(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    req: Request,
    next: Next,
) -> Response {
    let start = std::time::Instant::now();
    let ip = req.headers().get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_else(|| "unknown")
        .to_string();
    let path = req.uri().path().to_string();
    let range = req.headers().get(axum::http::header::RANGE).and_then(|v| v.to_str().ok()).map(|s| s.to_string());

    let response = next.run(req).await;
    let duration = start.elapsed();
    let is_success = response.status().is_success() || response.status() == axum::http::StatusCode::NOT_MODIFIED;
    
    if let Some(action) = response.extensions().get::<LogAction>().cloned() {
        let entry = LogEntry {
            time: chrono::Local::now().format("%H:%M:%S").to_string(),
            ip,
            action,
            duration,
            path,
            is_success,
            range,
        };
        let mut logs = state.stats.logs.lock().unwrap();
        logs.push_back(entry);
        if logs.len() > 100 { logs.pop_front(); }
    }
    response
}