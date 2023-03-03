use axum::routing::{delete, get, post, put};
use axum::{http::StatusCode, response::IntoResponse, Json, Router};

pub struct UsersRouter;

impl UsersRouter {
    pub fn new() -> Router {
        Router::new()
            .route("/users", get(UsersRouter::get_users_endpoint))
            .route("/users", post(UsersRouter::create_user_endpoint))
            .route("/user", get(UsersRouter::get_user_endpoint))
            .route("/user", put(UsersRouter::update_user_endpoint))
            .route("/user", delete(UsersRouter::delete_user_endpoint))
    }

    pub async fn create_user_endpoint() -> impl IntoResponse {
        (StatusCode::OK, Json("create user"))
    }
    pub async fn get_users_endpoint() -> impl IntoResponse {
        (StatusCode::OK, Json("get all users"))
    }
    pub async fn get_user_endpoint() -> impl IntoResponse {
        (StatusCode::OK, Json("get user"))
    }
    pub async fn update_user_endpoint() -> impl IntoResponse {
        (StatusCode::OK, Json("update user"))
    }
    pub async fn delete_user_endpoint() -> impl IntoResponse {
        (StatusCode::OK, Json("delete users"))
    }
}
