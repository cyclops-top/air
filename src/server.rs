use axum::{middleware, Router, routing::get};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::TcpListener;

use crate::{handlers, logger};

pub async fn start(port: u16, root: PathBuf) -> anyhow::Result<()> {
    let state = Arc::new(handlers::AppState {
        root_path: root,
    });

    // We use a fallback to catch all paths, including root if we want.
    // Or explicit routes.
    // Using fallback service is often easiest for static file server type behavior.
    
    let app = Router::new()
        //.route("/", get(handlers::handle_request))
        //.route("/*path", get(handlers::handle_request)) 
        // fallback matches everything not matched by other routes
        .fallback(get(handlers::handle_request))
        .layer(middleware::from_fn(logger::log_request))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await?;

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}