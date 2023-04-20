use std::sync::Arc;

// use anyhow::Context;
use clap::Parser;
use dotenvy::dotenv;
use tracing::info;

use budgetto_api::router::ApplicationController;
use budgetto_core::{config::AppConfig, logger};
use budgetto_infrastructure::{
    connection_pool::ConnectionManager, service_register::ServiceRegister,
    services::utils::seed_service::SeedService,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let config = Arc::new(AppConfig::parse());

    let _guard = logger::init(config.cargo_env);

    info!("environment loaded and configuration parsed, initializing Postgres connection and running migrations...");
    let pg_pool = ConnectionManager::new_pool(&config.database_url, config.run_migrations)
        .await
        .expect("could not initialize the database connection pool");

    let service_register = ServiceRegister::init(pg_pool, config.clone());

    if config.seed {
        info!("seeding enabled, creating test data...");
        SeedService::new(service_register.clone())
            .seed()
            .await
            .expect("unexpected error occurred while seeding application data");
    }

    info!("migrations successfully ran, initializing axum server...");
    ApplicationController::serve(config.port, &config.cors_origin, service_register).await?;

    Ok(())
}
