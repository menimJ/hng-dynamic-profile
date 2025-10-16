use std::{io, sync::Arc};
use axum::{Router, routing::get, middleware::from_fn}; // ✅ get + from_fn
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod config;
mod handlers;
mod models;
mod routes;
mod services;
mod metrics; // ensure this exists

use config::AppConfig;
use routes::create_router;
use crate::metrics::{metrics_handler, track_metrics}; // ✅ import handlers

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = AppConfig::from_env();
    let client = Arc::new(reqwest::Client::new());

    let app: Router = Router::new()
        .merge(create_router(client.clone()))
        .route("/metrics", get(metrics_handler))      // ✅ now in scope
        .layer(from_fn(track_metrics))               // ✅ now in scope
        .layer(CorsLayer::very_permissive());

    // robust bind (as you already had)
    let listener = match TcpListener::bind(("0.0.0.0", config.port)).await {
        Ok(l) => l,
        Err(e) if e.kind() == io::ErrorKind::AddrInUse => {
            tracing::warn!("Port {} busy; trying 127.0.0.1", config.port);
            match TcpListener::bind(("127.0.0.1", config.port)).await {
                Ok(l) => l,
                Err(_) => {
                    tracing::warn!("Falling back to ephemeral port");
                    TcpListener::bind(("0.0.0.0", 0)).await.expect("fallback bind failed")
                }
            }
        }
        Err(e) => panic!("bind failed: {e}"),
    };

    let addr = listener.local_addr().unwrap();
    tracing::info!("🚀 Listening on http://{addr}");
    tracing::info!("Try: http://127.0.0.1:{}/me", addr.port());

    axum::serve(listener, app).await.expect("server crashed");
}
