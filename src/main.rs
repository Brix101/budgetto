use std::sync::Arc;

use clap::Parser;
use dotenvy::dotenv;

use budgetto_api::{config::AppConfig, database::ConnectionManager};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let config = Arc::new(AppConfig::parse());

    info!("environment loaded and configuration parsed, initializing Postgres connection and running migrations...");
    ConnectionManager::new_pool(&config.database_url, config.run_migrations)
        .await
        .expect("could not initialize the database connection pool");
    Ok(())
}
