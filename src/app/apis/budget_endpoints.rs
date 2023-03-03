use axum::routing::{delete, get, post, put};
use axum::{http::StatusCode, response::IntoResponse, Json, Router};

pub struct BudgetRouter;

impl BudgetRouter {
    pub fn new() -> Router {
        Router::new()
            .route("/budgets", get(BudgetRouter::get_budgets_endpoint))
            .route("/budgets", post(BudgetRouter::create_budget_endpoint))
            .route("/budget", get(BudgetRouter::get_budget_endpoint))
            .route("/budget", put(BudgetRouter::update_budget_endpoint))
            .route("/budget", delete(BudgetRouter::delete_budget_endpoint))
    }

    pub async fn create_budget_endpoint() -> impl IntoResponse {
        (StatusCode::OK, Json("create budget"))
    }
    pub async fn get_budgets_endpoint() -> impl IntoResponse {
        (StatusCode::OK, Json("get all budgets"))
    }
    pub async fn get_budget_endpoint() -> impl IntoResponse {
        (StatusCode::OK, Json("get budget"))
    }
    pub async fn update_budget_endpoint() -> impl IntoResponse {
        (StatusCode::OK, Json("update budget"))
    }
    pub async fn delete_budget_endpoint() -> impl IntoResponse {
        (StatusCode::OK, Json("delete budgets"))
    }
}
