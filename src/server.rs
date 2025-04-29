use axum::{Router, routing::get};
use std::net::SocketAddr;
use tracing::info;

use crate::templates::*;

pub async fn run_server() -> anyhow::Result<()> {
    info!("initializing router...");

    let port = 8000_u16;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let router = Router::new()
        .route("/", get(index))
        .route("/form/{game}", get(form));

    info!("router initialized, now listening on port {}", port);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();

    Ok(())
}
