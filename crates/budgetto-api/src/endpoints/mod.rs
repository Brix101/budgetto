use axum::routing::*;

use self::{categories_endpoints::CategoryController, users_endpoints::UserRouter};

pub mod categories_endpoints;
pub mod users_endpoints;

pub fn app() -> Router {
    Router::new()
        .nest("/users", UserRouter::app())
        .nest("/categories", CategoryController::app())
}
