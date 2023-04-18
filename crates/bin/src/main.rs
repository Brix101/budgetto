use std::sync::Arc;

// use anyhow::Context;
use clap::Parser;
use dotenvy::dotenv;
use tracing::info;

use api::router::ApplicationController;
use core::{config::AppConfig, logger};
use infrastructure::{connection_pool::ConnectionManager, service_register::ServiceRegister};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let config = Arc::new(AppConfig::parse());

    let _guard = logger::init(config.cargo_env);

    info!("environment loaded and configuration parsed, initializing Postgres connection and running migrations...");
    let pg_pool = ConnectionManager::new_pool(&config.database_url, config.run_migrations)
        .await
        .expect("could not initialize the database connection pool");

    let port = config.port;
    let service_register = ServiceRegister::new(pg_pool, config.clone());

    info!("migrations successfully ran, initializing axum server...");
    ApplicationController::serve(port, &config.cors_origin, service_register).await?;

    Ok(())
}
