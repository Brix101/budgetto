use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;

mod auth_endpoints;
mod budget_endpoints;
mod ledger_endpoints;
mod users_endpoints;

use auth_endpoints::AuthRouter;
use budget_endpoints::BudgetRouter;
use ledger_endpoints::LedgerRouter;
use users_endpoints::UsersRouter;

pub struct ApiRoutes;

#[derive(Serialize)]
struct Hello {
    msg: String,
}

impl ApiRoutes {
    pub fn new() -> Router {
        Router::new()
            .route("/", get(ApiRoutes::hello))
            .merge(AuthRouter::new())
            .merge(BudgetRouter::new())
            .merge(LedgerRouter::new())
            .merge(UsersRouter::new())
    }

    async fn hello() -> impl IntoResponse {
        let hello = Hello {
            msg: String::from("☁️☁️🚀☁️ "),
        };

        (StatusCode::OK, Json(hello))
    }
}
