use axum::{middleware, Router, routing::get};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::TcpListener;

use crate::{handlers, logger};

pub async fn start(port: u16, root: PathBuf) -> anyhow::Result<Arc<handlers::AppState>> {
    let stats = Arc::new(handlers::Stats::default());
    let state = Arc::new(handlers::AppState {
        root_path: root,
        stats: stats.clone(),
    });

    let app = Router::new()
        .route("/favicon.ico", get(handlers::favicon))
        .fallback(get(handlers::handle_request))
        .layer(middleware::from_fn_with_state(state.clone(), logger::log_request))
        .with_state(state.clone());

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await?;

    tokio::spawn(async move {
        let _ = axum::serve(
            listener,
            app.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await;
    });

        Ok(state)

    }

    