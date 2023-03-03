use axum::routing::{delete, get, post, put};
use axum::{http::StatusCode, response::IntoResponse, Json, Router};

pub struct AuthRouter;

impl AuthRouter {
    pub fn new() -> Router {
        Router::new()
            .route("/auths", get(AuthRouter::get_auths_endpoint))
            .route("/auths", post(AuthRouter::create_auth_endpoint))
            .route("/auth", get(AuthRouter::get_auth_endpoint))
            .route("/auth", put(AuthRouter::update_auth_endpoint))
            .route("/auth", delete(AuthRouter::delete_auth_endpoint))
    }

    pub async fn create_auth_endpoint() -> impl IntoResponse {
        (StatusCode::OK, Json("create auth"))
    }
    pub async fn get_auths_endpoint() -> impl IntoResponse {
        (StatusCode::OK, Json("get all auths"))
    }
    pub async fn get_auth_endpoint() -> impl IntoResponse {
        (StatusCode::OK, Json("get auth"))
    }
    pub async fn update_auth_endpoint() -> impl IntoResponse {
        (StatusCode::OK, Json("update auth"))
    }
    pub async fn delete_auth_endpoint() -> impl IntoResponse {
        (StatusCode::OK, Json("delete auths"))
    }
}
