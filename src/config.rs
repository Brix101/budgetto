use serde::Serialize;
use std::env;

#[derive(Serialize)]
pub struct AppConfig {
    pub database_url: String,
    pub port: u16,
}

impl AppConfig {
    pub fn new() -> Self {
        AppConfig {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            port: env::var("PORT")
                .unwrap_or("8080".to_owned())
                .parse()
                .unwrap(),
        }
    }
}
