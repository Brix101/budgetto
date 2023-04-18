use std::sync::Arc;

// use anyhow::Context;
use clap::Parser;
use dotenvy::dotenv;
use tracing::info;

use core::config::AppConfig;
use core::logger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let config = Arc::new(AppConfig::parse());

    let _guard = logger::init(config.cargo_env);

    info!("environment loaded and configuration parsed, initializing Postgres connection and running migrations...");
    println!("{:#?}", config);
    Ok(())
}
