use anyhow::Ok;
use axum::{response::Redirect, routing::get, Router};
use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;
use std::net::{Ipv4Addr, SocketAddr};
use tower::ServiceBuilder;
use tower_http::{
    add_extension::AddExtensionLayer,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::info;

use budgetto::app::apis::ApiRoutes;
use budgetto::core::{config::AppConfig, database::ConnectionManager};

#[derive(Clone)]
struct ApiContext {
    db: PgPool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let config = AppConfig::new();
    let api_routes = ApiRoutes::new();
    let cors = CorsLayer::new().allow_origin(Any);

    // Setup logging & RUST_LOG from args
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", format!("debug,hyper=info,mio=info"))
    }

    // enable console logging
    tracing_subscriber::fmt::init();
    info!("environment loaded and configuration parsed, initializing Postgres connection and running migrations...");
    let db = ConnectionManager::new_pool(&config.database_url, config.run_migrations)
        .await
        .expect("could not initialize the database connection pool");

    let app = Router::new()
        .route("/", get(|| async { Redirect::permanent("/api") }))
        .nest("/api", api_routes)
        .layer(cors)
        .layer(
            ServiceBuilder::new()
                .layer(AddExtensionLayer::new(ApiContext { db }))
                .layer(TraceLayer::new_for_http()),
        );

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, config.port));
    println!("listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
