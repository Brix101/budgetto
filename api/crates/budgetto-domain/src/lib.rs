use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod accounts;
pub mod budgets;
pub mod categories;
pub mod sessions;
pub mod transactions;
pub mod users;
pub mod utils;

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiError {
    pub errors: HashMap<String, Vec<String>>,
}

impl ApiError {
    pub fn new(error: String) -> Self {
        let mut error_map: HashMap<String, Vec<String>> = HashMap::new();
        error_map.insert("message".to_owned(), vec![error]);
        Self { errors: error_map }
    }
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct PingResponse {
    pub message: String,
}

impl PingResponse {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Default for PingResponse {
    fn default() -> Self {
        Self::new(String::from("API is responsive"))
    }
}
