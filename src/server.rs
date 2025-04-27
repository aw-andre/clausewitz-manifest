use axum::{Router, routing::get};
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use tracing::info;

use crate::templates::index;

pub async fn run_server() -> anyhow::Result<()> {
    info!("initializing router...");

    let assets_path = std::env::current_dir().unwrap();
    let port = 8000_u16;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let router = Router::new().route("/", get(index)).nest_service(
        "/assets",
        ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
    );

    info!("router initialized, now listening on port {}", port);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();

    Ok(())
}
