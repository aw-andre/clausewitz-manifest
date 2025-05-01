mod server;
mod templates;

use sqlx::postgres::PgPool;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("Error: DATABASE_URL is not set");
    let pool = PgPool::connect(&database_url).await?;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "pdx_manifest=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    server::run_server(pool).await?;

    Ok(())
}
