use std::net::SocketAddr;            // ✅ needed for SocketAddr

use std::sync::Arc;
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
    let port: u16 = std::env::var("PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(8080);
    
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await.expect("bind failed");
    tracing::info!("🚀 Listening on http://{addr}");
    axum::serve(listener, app).await.expect("server crashed");
}
