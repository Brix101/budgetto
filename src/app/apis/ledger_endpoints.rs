use axum::routing::{delete, get, post, put};
use axum::{http::StatusCode, response::IntoResponse, Json, Router};

pub struct LedgerRouter;

impl LedgerRouter {
    pub fn new() -> Router {
        Router::new()
            .route("/ledgers", get(LedgerRouter::get_ledgers_endpoint))
            .route("/ledgers", post(LedgerRouter::create_ledger_endpoint))
            .route("/ledger", get(LedgerRouter::get_ledger_endpoint))
            .route("/ledger", put(LedgerRouter::update_ledger_endpoint))
            .route("/ledger", delete(LedgerRouter::delete_ledger_endpoint))
    }

    pub async fn create_ledger_endpoint() -> impl IntoResponse {
        (StatusCode::OK, Json("create ledger"))
    }
    pub async fn get_ledgers_endpoint() -> impl IntoResponse {
        (StatusCode::OK, Json("get all ledgers"))
    }
    pub async fn get_ledger_endpoint() -> impl IntoResponse {
        (StatusCode::OK, Json("get ledger"))
    }
    pub async fn update_ledger_endpoint() -> impl IntoResponse {
        (StatusCode::OK, Json("update ledger"))
    }
    pub async fn delete_ledger_endpoint() -> impl IntoResponse {
        (StatusCode::OK, Json("delete ledgers"))
    }
}
