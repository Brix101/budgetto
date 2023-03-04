use anyhow::{Context, Ok};
use axum::{response::Redirect, routing::get, Router};
use budgetto::app::services::ServiceRegister;
use dotenvy::dotenv;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::info;

use budgetto::app::apis::ApiRoutes;
use budgetto::core::{config::AppConfig, database::ConnectionManager};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let config = Arc::new(AppConfig::new());
    let cors = CorsLayer::new().allow_origin(Any);

    info!("environment loaded and configuration parsed, initializing Postgres connection and running migrations...");
    let pool = ConnectionManager::new(&config.database_url, config.run_migrations)
        .await
        .expect("could not initialize the database connection pool");

    let service_register = ServiceRegister::new(pool, config.clone());

    let api_routes = ApiRoutes::new(service_register);

    // enable console logging
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(|| async { Redirect::permanent("/api") }))
        .nest("/api", api_routes)
        .layer(cors)
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, config.port));
    println!("server on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context("could not initialize application")?;

    Ok(())
}
