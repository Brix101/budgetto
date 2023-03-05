use serde::Serialize;
use std::env;

#[derive(Serialize)]
pub struct AppConfig {
    pub database_url: String,
    pub port: u16,
    pub run_migrations: bool,
    pub rust_log: String,
    pub token_secret: String,
    pub argon_salt: String,
}

impl AppConfig {
    pub fn new() -> Self {
        AppConfig {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            port: env::var("PORT")
                .unwrap_or("8000".to_owned())
                .parse()
                .unwrap(),
            run_migrations: env::var("RUN_MIGRATIONS")
                .unwrap_or("false".to_owned())
                .parse::<bool>()
                .unwrap(),
            rust_log: env::var("RUST_LOG").expect("RUST_LOG must be set"),
            token_secret: env::var("TOKEN_SECRET").expect("TOKEN_SECRET must be set"),
            argon_salt: env::var("ARGON_SALT").expect("ARGON_SALT must be set"),
        }
    }
}
