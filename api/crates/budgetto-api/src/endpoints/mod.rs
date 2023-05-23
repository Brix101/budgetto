use axum::routing::*;

use self::{
    account_endpoints::AccountController, auth_endpoints::AuthRouter,
    category_endpoints::CategoryController, transaction_endpoints::TransactionController,
    user_endpoints::UserRouter,
};

pub mod account_endpoints;
pub mod auth_endpoints;
pub mod category_endpoints;
pub mod transaction_endpoints;
pub mod user_endpoints;

pub fn app() -> Router {
    Router::new()
        .nest("/users", UserRouter::app())
        .nest("/categories", CategoryController::app())
        .nest("/accounts", AccountController::app())
        .nest("/transactions", TransactionController::app())
        .nest("/auth", AuthRouter::app())
}
