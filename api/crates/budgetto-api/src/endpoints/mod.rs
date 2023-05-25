use axum::routing::*;

use self::{
    account_endpoints::AccountRouter, auth_endpoints::AuthRouter, budget_endpoints::BudgetRouter,
    category_endpoints::CategoryRouter, transaction_endpoints::TransactionRouter,
    user_endpoints::UserRouter,
};

pub mod account_endpoints;
pub mod auth_endpoints;
pub mod budget_endpoints;
pub mod category_endpoints;
pub mod transaction_endpoints;
pub mod user_endpoints;

pub fn app() -> Router {
    Router::new()
        .nest("/users", UserRouter::app())
        .nest("/categories", CategoryRouter::app())
        .nest("/accounts", AccountRouter::app())
        .nest("/transactions", TransactionRouter::app())
        .nest("/auth", AuthRouter::app())
        .nest("/bugets", BudgetRouter::app())
}
