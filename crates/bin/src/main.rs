use std::sync::Arc;

// use anyhow::Context;
use clap::Parser;
use dotenvy::dotenv;

use core::config::AppConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let config = Arc::new(AppConfig::parse());

    println!("{:#?}", &config.database_url);
    Ok(())
}
