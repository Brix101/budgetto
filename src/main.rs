use axum::{response::Redirect, routing::get, Router};
use dotenvy::dotenv;
use sqlx::PgPool;
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

struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = AppConfig::new();
    let api_routes = ApiRoutes::new();
    let cors = CorsLayer::new().allow_origin(Any);

    info!("environment loaded and configuration parsed, initializing Postgres connection and running migrations...");
    let db = ConnectionManager::new(&config.database_url, config.run_migrations)
        .await
        .expect("could not initialize the database connection pool");
    let shared_state = Arc::new(AppState { db });

    // enable console logging
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .with_state(shared_state)
        .route("/", get(|| async { Redirect::permanent("/api") }))
        .nest("/api", api_routes)
        .layer(cors)
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, config.port));
    println!("listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
