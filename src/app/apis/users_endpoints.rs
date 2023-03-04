use axum::routing::{delete, get, post, put};
use axum::{http::StatusCode, response::IntoResponse, Extension, Json, Router};

use crate::app::dto::users_dto::{RegisterUserDto, UserAuthenicationResponse};
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
    ) -> CustomResult<Json<UserAuthenicationResponse>> {
        let name = Some(String::from("Brix"));
        let email = Some(String::from("brixterporras@gmail.com"));
        let password = Some(String::from("password"));

        let created_user = users_service
            .register_user(RegisterUserDto {
                name,
                email,
                password,
            })
            .await?;

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
