use axum::{routing::get, Router, Json};
use serde::Serialize;
use std::{env, net::SocketAddr};
use tracing_subscriber::EnvFilter;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const DEFAULT_PORT: u16 = 8800;

#[derive(Serialize)]
struct Status { ok: bool, service: &'static str, version: &'static str }

async fn status() -> Json<Status> {
    Json(Status { ok: true, service: "hh-core", version: VERSION })
}

#[tokio::main]
async fn main() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "info".parse().unwrap());
    tracing_subscriber::fmt().with_env_filter(filter).init();

    let port = env::var("PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(DEFAULT_PORT);
    let app = Router::new()
        .route("/health", get(status))
        .route("/api/status", get(status));

    let addr = SocketAddr::from(([0,0,0,0], port));
    tracing::info!(%addr, "hh-core listening");
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
