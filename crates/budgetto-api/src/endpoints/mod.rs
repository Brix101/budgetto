use axum::routing::*;

use self::users_endpoints::UserRouter;

pub mod users_endpoints;

pub fn app() -> Router {
    Router::new().nest("/users", UserRouter::app())
}
