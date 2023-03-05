use axum::extract::Json;
use axum::routing::{get, post, put};
use axum::{Extension, Router};
use tracing::info;

use crate::app::dto::users_dto::{
    LoginUserRequest, RegisterUserRequest, UpdateUserRequest, UserAuthenicationResponse,
};
use crate::app::services::users_service::DynUsersService;
use crate::app::services::ServiceRegister;
use crate::core::errors::CustomResult;
use crate::core::extractors::required_authentication_extractor::RequiredAuthentication;
use crate::core::extractors::validation_extractor::ValidationExtractor;

pub struct UsersRouter;

impl UsersRouter {
    pub fn new(service_register: ServiceRegister) -> Router {
        Router::new()
            .route("/users", post(UsersRouter::register_user_endpoint))
            .route("/users/login", post(UsersRouter::login_user_endpoint))
            .route("/user", get(UsersRouter::get_current_user_endpoint))
            .route("/user", put(UsersRouter::update_user_endpoint))
            .layer(Extension(service_register.users_service))
            .layer(Extension(service_register.token_service))
    }

    pub async fn register_user_endpoint(
        Extension(users_service): Extension<DynUsersService>,
        ValidationExtractor(request): ValidationExtractor<RegisterUserRequest>,
    ) -> CustomResult<Json<UserAuthenicationResponse>> {
        info!(
            "recieved request to create user {:?}/{:?}",
            request.user.email.as_ref().unwrap(),
            request.user.name.as_ref().unwrap()
        );

        let created_user = users_service.register_user(request.user).await?;

        Ok(Json(UserAuthenicationResponse { user: created_user }))
    }
    pub async fn login_user_endpoint(
        Extension(users_service): Extension<DynUsersService>,
        Json(request): Json<LoginUserRequest>,
    ) -> CustomResult<Json<UserAuthenicationResponse>> {
        info!(
            "recieved request to login user {:?}",
            request.user.email.as_ref().unwrap()
        );

        let created_user = users_service.login_user(request.user).await?;

        Ok(Json(UserAuthenicationResponse { user: created_user }))
    }

    pub async fn get_current_user_endpoint(
        Extension(users_service): Extension<DynUsersService>,
        RequiredAuthentication(user_id): RequiredAuthentication,
    ) -> CustomResult<Json<UserAuthenicationResponse>> {
        info!("recieved request to retrieve current user");

        let current_user = users_service.get_current_user(user_id).await?;

        Ok(Json(UserAuthenicationResponse { user: current_user }))
    }

    pub async fn update_user_endpoint(
        Extension(users_service): Extension<DynUsersService>,
        RequiredAuthentication(user_id): RequiredAuthentication,
        Json(request): Json<UpdateUserRequest>,
    ) -> CustomResult<Json<UserAuthenicationResponse>> {
        info!("recieved request to update user {:?}", user_id);

        let updated_user = users_service.updated_user(user_id, request.user).await?;

        Ok(Json(UserAuthenicationResponse { user: updated_user }))
    }
}
