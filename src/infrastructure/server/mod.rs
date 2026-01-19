pub mod handlers;

use std::sync::Arc;
use std::net::SocketAddr;
use std::path::PathBuf;
use axum::{routing::get, Router, extract::{Request, State, FromRef}, middleware::Next, response::{Response, IntoResponse}};
use crate::application::file_service::FileService;
use crate::domain::traits::UiRenderer;
use crate::domain::models::{AppState, LogAction, LogEntry};
use crate::infrastructure::api::{ApiState, handle_api_list, handle_api_list_root};
use self::handlers::{handle_request, ServerState};
use crate::cert;

/// 组合状态机，解决 Axum 路由单一 State 限制
#[derive(Clone)]
pub struct CombinedState {
    pub server: Arc<ServerState>,
    pub api: Arc<ApiState>,
}

// 实现 FromRef 使得具体处理器依然能直接提取各自所需的子状态
impl FromRef<CombinedState> for Arc<ServerState> {
    fn from_ref(state: &CombinedState) -> Self { state.server.clone() }
}
impl FromRef<CombinedState> for Arc<ApiState> {
    fn from_ref(state: &CombinedState) -> Self { state.api.clone() }
}

pub async fn start_server(
    port_opt: Option<u16>,
    _root_path: PathBuf,
    enable_https: bool,
    lan_ip: std::net::IpAddr,
    file_service: Arc<FileService>,
    ui_renderer: Arc<dyn UiRenderer>,
    app_state: Arc<AppState>,
) -> anyhow::Result<u16> {
    let server_state = Arc::new(ServerState {
        file_service: file_service.clone(),
        ui_renderer,
        app_state: app_state.clone(),
    });

    let api_state = Arc::new(ApiState {
        file_service,
        app_state: app_state.clone(),
    });

    let combined_state = CombinedState {
        server: server_state,
        api: api_state,
    };

    let router = Router::new()
        .route("/favicon.ico", get(favicon))
        .route("/api/list/", get(handle_api_list_root))
        .route("/api/list/{*path}", get(handle_api_list))
        .fallback(handle_request)
        .layer(axum::middleware::from_fn_with_state(app_state.clone(), stats_middleware))
        .with_state(combined_state);

    // 端口选择策略：
    // 1. 如果用户指定了端口，直接使用该端口。
    // 2. 如果没指定，依次尝试 9567 -> 9568。
    // 3. 如果都被占用，使用随机端口 (0)。
    let ports_to_try = if let Some(p) = port_opt {
        vec![p]
    } else {
        vec![9567, 9568, 0]
    };

    for &port in &ports_to_try {
        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        
        // 尝试绑定以检测端口可用性
        match tokio::net::TcpListener::bind(addr).await {
            Ok(listener) => {
                let final_port = listener.local_addr()?.port();
                let router_clone = router.clone();
                let lan_ip_clone = lan_ip;

                tokio::spawn(async move {
                    if enable_https {
                        // 释放原有的 TCP 监听器，因为 axum_server 需要自己管理 socket
                        drop(listener);
                        if let Ok(config) = cert::get_config(lan_ip_clone) {
                            let _ = axum_server::bind_rustls(addr, config)
                                .serve(router_clone.into_make_service())
                                .await;
                        }
                    } else {
                        let _ = axum::serve(listener, router_clone.into_make_service()).await;
                    }
                });
                return Ok(final_port);
            }
            Err(_) => {
                if port_opt.is_some() {
                    return Err(anyhow::anyhow!("Port {} is already in use", port));
                }
                continue; // 尝试下一个端口
            }
        }
    }

    Err(anyhow::anyhow!("No available ports found"))
}

const FAVICON_SVG: &[u8] = include_bytes!("../../../docs/favicon.svg");
async fn favicon() -> impl IntoResponse {
    let mut res = ([(axum::http::header::CONTENT_TYPE, "image/svg+xml")], FAVICON_SVG).into_response();
    res.extensions_mut().insert(LogAction::Favicon);
    res
}

async fn stats_middleware(
    State(state): State<Arc<AppState>>,
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