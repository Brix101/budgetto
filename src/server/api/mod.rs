mod user;

use axum::routing::*;

use self::user::UsersRouter;

pub async fn health() -> &'static str {
    "I am working!"
}

pub fn app() -> Router {
    Router::new()
        .nest("/", UsersRouter::app())
        // .nest("/fs", fs::app())
        .route("/health", get(health))
}
