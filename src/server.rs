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
        .route("/eu3.html", get(|| modifier_tree("eu3")))
        .route("/eu4.html", get(|| modifier_tree("eu4")))
        .route("/ck2.html", get(|| modifier_tree("ck2")))
        .route("/ck3.html", get(|| modifier_tree("ck3")))
        .route("/hoi3.html", get(|| modifier_tree("hoi3")))
        .route("/vic2.html", get(|| modifier_tree("vic2")))
        .route("/vic3.html", get(|| modifier_tree("vic3")))
        .route("/imperator.html", get(|| modifier_tree("imperator")))
        .route("/stellaris.html", get(|| modifier_tree("stellaris")))
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        );

    info!("router initialized, now listening on port {}", port);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();

    Ok(())
}
