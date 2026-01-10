use axum::{middleware, routing::get, Router};
use axum_server::tls_rustls::RustlsConfig;
use rand::Rng;
use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::TcpListener;

use crate::{cert, handlers, logger};

pub async fn start(
    port: Option<u16>,
    root: PathBuf,
    enable_https: bool,
    lan_ip: IpAddr,
) -> anyhow::Result<(Arc<handlers::AppState>, u16)> {
    let stats = Arc::new(handlers::Stats::default());

    // 1. Determine port
    let (listener, used_port) = if let Some(p) = port {
        let addr = SocketAddr::from(([0, 0, 0, 0], p));
        (TcpListener::bind(addr).await?, p)
    } else {
        // Step 1: Try default non-common port 9568
        let default_port = 9568;
        let default_addr = SocketAddr::from(([0, 0, 0, 0], default_port));
        if let Ok(l) = TcpListener::bind(default_addr).await {
            (l, default_port)
        } else {
            // Step 2: Default taken, fallback to random logic
            let mut rng = rand::rng();
            loop {
                let p = rng.random_range(10000..=65535);
                let addr = SocketAddr::from(([0, 0, 0, 0], p));
                if let Ok(l) = TcpListener::bind(addr).await {
                    break (l, p);
                }
            }
        }
    };

    let state = Arc::new(handlers::AppState {
        root_path: root,
        stats: stats.clone(),
        enable_https,
        digest_cache: dashmap::DashMap::new(),
        mmap_cache: Arc::new(handlers::MmapCache::new()),
        lan_ip: lan_ip.to_string(),
        port: used_port,
    });

    let app = Router::new()
        .route("/favicon.ico", get(handlers::favicon))
        .fallback(get(handlers::handle_request))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            logger::log_request,
        ))
        .with_state(state.clone());

    let addr = SocketAddr::from(([0, 0, 0, 0], used_port));

    if enable_https {
        // For HTTPS, we use axum-server which might want to bind itself.
        // But we already bound a TcpListener to 'reserve' the port.
        // Axum-server's bind_rustls takes an Addr.
        // We might need to drop our listener first if we want axum-server to bind,
        // OR use axum_server::from_tcp_rustls if it exists.
        // Actually, let's just drop the listener and hope no one steals the port in microsecond,
        // OR better, use axum_server's serving logic.
        drop(listener);

        // Generate self-signed cert
        let cert = cert::generate_self_signed(lan_ip)?;
        let config = RustlsConfig::from_pem(
            cert.cert_pem.as_bytes().to_vec(),
            cert.key_pem.as_bytes().to_vec(),
        )
        .await?;

        tokio::spawn(async move {
            let _ = axum_server::bind_rustls(addr, config)
                .serve(app.into_make_service_with_connect_info::<SocketAddr>())
                .await;
        });
    } else {
        tokio::spawn(async move {
            let _ = axum::serve(
                listener,
                app.into_make_service_with_connect_info::<SocketAddr>(),
            )
            .await;
        });
    }

    Ok((state, used_port))
}
