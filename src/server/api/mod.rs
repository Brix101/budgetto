mod budget_api;
mod user;

use axum::routing::*;

use self::user::UsersRouter;
use budget_api::BudgetRouter;

pub async fn health() -> &'static str {
    "I am working!"
}

pub fn app() -> Router {
    Router::new()
        .nest("/", UsersRouter::app())
        .nest("/budgets", BudgetRouter::app())
        .route("/health", get(health))
}
