mod routes;
mod server;
mod templates;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "pdx_manifest=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    server::run_server().await?;

    Ok(())
}
