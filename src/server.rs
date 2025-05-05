use axum::{Router, routing::get};
use sqlx::{Pool, Postgres};
use std::net::SocketAddr;
use tracing::info;

use crate::templates::children::*;
use crate::templates::form::*;
use crate::templates::index::*;
use crate::templates::tree::*;

pub async fn run_server(pool: Pool<Postgres>) -> anyhow::Result<()> {
    info!("initializing router...");

    let port = 8000_u16;
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let router = Router::new()
        .route("/", get(index))
        .route("/form/{game}", get(form))
        .route("/tree/{game}", get(tree))
        .route("/children", get(children))
        .with_state(pool);

    info!("router initialized, now listening on port {}", port);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();

    Ok(())
}
