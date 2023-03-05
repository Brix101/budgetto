use axum::extract::Json;
use axum::routing::{delete, get, post, put};
use axum::{http::StatusCode, response::IntoResponse, Extension, Router};
use tracing::info;

use crate::app::dto::users_dto::{RegisterUserRequest, UserAuthenicationResponse};
use crate::app::services::users_service::DynUsersService;
use crate::app::services::ServiceRegister;
use crate::core::errors::CustomResult;

pub struct UsersRouter;

impl UsersRouter {
    pub fn new(service_register: ServiceRegister) -> Router {
        Router::new()
            .route("/users", get(UsersRouter::get_users_endpoint))
            .route("/users", post(UsersRouter::create_user_endpoint))
            .route("/user", get(UsersRouter::get_user_endpoint))
            .route("/user", put(UsersRouter::update_user_endpoint))
            .route("/user", delete(UsersRouter::delete_user_endpoint))
            .layer(Extension(service_register.users_service))
    }

    pub async fn create_user_endpoint(
        Extension(users_service): Extension<DynUsersService>,
        Json(request): Json<RegisterUserRequest>,
    ) -> CustomResult<Json<UserAuthenicationResponse>> {
        info!(
            "recieved request to create user {:?}/{:?}",
            request.user.email.as_ref().unwrap(),
            request.user.name.as_ref().unwrap()
        );
        println!(
            "recieved request to create user {:?}/{:?}",
            request.user.email.as_ref().unwrap(),
            request.user.name.as_ref().unwrap()
        );
        // let name = Some(String::from("John Doe"));
        // let email = Some(String::from("johndoe@example.com"));
        // let password = Some(String::from("password"));

        let created_user = users_service.register_user(request.user).await?;

        Ok(Json(UserAuthenicationResponse { user: created_user }))
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
