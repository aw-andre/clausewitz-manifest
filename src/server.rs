use axum::{Router, routing::get};
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use tracing::info;

use crate::templates::*;

pub async fn run_server() -> anyhow::Result<()> {
    info!("initializing router...");

    let assets_path = std::env::current_dir().unwrap();
    let port = 8000_u16;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let router = Router::new()
        .route("/", get(index))
        .route("/eu3", get(|| modifier_tree("eu3")))
        .route("/eu4", get(|| modifier_tree("eu4")))
        .route("/ck2", get(|| modifier_tree("ck2")))
        .route("/ck3", get(|| modifier_tree("ck3")))
        .route("/hoi3", get(|| modifier_tree("hoi3")))
        .route("/vic2", get(|| modifier_tree("vic2")))
        .route("/vic3", get(|| modifier_tree("vic3")))
        .route("/imperator", get(|| modifier_tree("imperator")))
        .route("/stellaris", get(|| modifier_tree("stellaris")));

    info!("router initialized, now listening on port {}", port);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();

    Ok(())
}
