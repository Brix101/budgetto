use axum::routing::*;

use self::{
    accounts_endpoints::AccountController, categories_endpoints::CategoryController,
    transactions_endpoints::TransactionController, users_endpoints::UserRouter,
};

pub mod accounts_endpoints;
pub mod categories_endpoints;
pub mod transactions_endpoints;
pub mod users_endpoints;

pub fn app() -> Router {
    Router::new()
        .nest("/users", UserRouter::app())
        .nest("/categories", CategoryController::app())
        .nest("/accounts", AccountController::app())
        .nest("/transactions", TransactionController::app())
}
