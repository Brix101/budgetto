use serde::Serialize;
use std::env;

#[derive(Serialize)]
pub struct AppConfig {
    pub database_url: String,
    pub port: u16,
    pub run_migrations: bool,
    pub rust_log: String,
}

impl AppConfig {
    pub fn new() -> Self {
        AppConfig {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            port: env::var("PORT")
                .unwrap_or("8080".to_owned())
                .parse()
                .unwrap(),
            run_migrations: env::var("RUN_MIGRATIONS")
                .unwrap_or("false".to_owned())
                .parse::<bool>()
                .unwrap(),
            rust_log: env::var("RUST_LOG").expect("DATABASE_URL must be set"),
        }
    }
}
