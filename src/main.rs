use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use dotenvy::dotenv;
use serde::Serialize;
use std::env;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::info;

use budgetto::core::config::AppConfig;
use budgetto::core::database::ConnectionManager;

#[derive(Serialize)]
struct Hello {
    msg: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = AppConfig::new();
    let cors = CorsLayer::new().allow_origin(Any);

    // Setup logging & RUST_LOG from args
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", format!("debug,hyper=info,mio=info"))
    }

    // enable console logging
    tracing_subscriber::fmt::init();
    info!("environment loaded and configuration parsed, initializing Postgres connection and running migrations...");
    let pg_pool = ConnectionManager::new_pool(&config.database_url, config.run_migrations)
        .await
        .expect("could not initialize the database connection pool");

    let api_routes = Router::new().route("/", get(root));

    let app = Router::new()
        .nest("/api", api_routes)
        .layer(cors)
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    println!("listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> impl IntoResponse {
    let hello = Hello {
        msg: String::from("☁️☁️🚀☁️ "),
    };
    (StatusCode::OK, Json(hello))
}
