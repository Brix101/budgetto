mod budget_api;
mod user_api;

use axum::routing::*;

use budget_api::BudgetRouter;
use user_api::UsersRouter;

pub async fn health() -> &'static str {
    "I am working!"
}

pub fn app() -> Router {
    Router::new()
        .nest("/", UsersRouter::app())
        .nest("/budgets", BudgetRouter::app())
        .route("/health", get(health))
}
