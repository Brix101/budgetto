use axum::extract::Json;
use axum::routing::{post, put};
use axum::{Extension, Router};
use tracing::info;

use budgetto_core::errors::AppResult;
use budgetto_domain::users::requests::{SignUpUserDto, UpdateUserDto};
use budgetto_domain::users::responses::UserAuthenicationResponse;
use budgetto_infrastructure::service_register::ServiceRegister;

use crate::extractors::required_authentication_extractor::RequiredAuthentication;
use crate::extractors::validation_extractor::ValidationExtractor;

pub struct UserRouter;

impl UserRouter {
    pub fn app() -> Router {
        Router::new()
            .route("/sign-up", post(Self::signup_user_endpoint))
            .route("/", put(Self::update_user_endpoint))
    }

    pub async fn signup_user_endpoint(
        Extension(services): Extension<ServiceRegister>,
        ValidationExtractor(request): ValidationExtractor<SignUpUserDto>,
    ) -> AppResult<Json<UserAuthenicationResponse>> {
        info!(
            "recieved request to create user {:?}/{:?}",
            request.email.as_ref().unwrap(),
            request.name.as_ref().unwrap()
        );

        let created_user = services.users.signup_user(request).await?;

        Ok(Json(UserAuthenicationResponse { user: created_user }))
    }

    pub async fn update_user_endpoint(
        RequiredAuthentication(user): RequiredAuthentication,
        Extension(services): Extension<ServiceRegister>,
        Json(mut request): Json<UpdateUserDto>,
    ) -> AppResult<Json<UserAuthenicationResponse>> {
        info!("recieved request to update user {:?}", user.id);

        request.id = Some(user.id);
        let updated_user = services.users.updated_user(request).await?;

        Ok(Json(UserAuthenicationResponse { user: updated_user }))
    }
}
