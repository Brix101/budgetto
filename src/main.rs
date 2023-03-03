use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use clap::Parser;
use dotenvy::dotenv;
use serde::Serialize;
use std::env;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::info;

mod types;
use types::Person;
mod config;
use config::AppConfig;

// Setup the command line interface with clap.
#[derive(Parser, Debug)]
#[clap(name = "server", about = "A server for our wasm project!")]
struct Opt {
    /// set the log level
    #[clap(short = 'l', long = "log", default_value = "debug")]
    log_level: String,
}

#[derive(Serialize)]
struct Hello {
    msg: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = AppConfig::new();

    let opt = Opt::parse();

    println!("{:?}", config.database_url);
    // Setup logging & RUST_LOG from args
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level))
    }

    // enable console logging
    tracing_subscriber::fmt::init();
    let cors = CorsLayer::new().allow_origin(Any);

    let api_routes = Router::new()
        .route("/", get(root))
        .route("/people", get(get_people));

    let app = Router::new()
        .nest("/api", api_routes)
        .layer(cors)
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!("listening on http://{}", addr);

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

async fn get_people() -> impl IntoResponse {
    let people = vec![
        Person {
            name: String::from("Person A"),
            age: 36,
            favourite_food: Some(String::from("Pizza")),
        },
        Person {
            name: String::from("Person B"),
            age: 5,
            favourite_food: Some(String::from("Broccoli")),
        },
        Person {
            name: String::from("Person C"),
            age: 100,
            favourite_food: None,
        },
    ];
    (StatusCode::OK, Json(people))
}
