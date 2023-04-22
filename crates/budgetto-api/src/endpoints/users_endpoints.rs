use axum::extract::Json;
use axum::routing::{get, post, put};
use axum::{Extension, Router};
use budgetto_domain::sessions::requests::NewAccessTokenRequest;
use budgetto_domain::sessions::responses::SessionResponse;
use tracing::info;

use budgetto_core::errors::AppResult;
use budgetto_domain::users::requests::{SignInUserDto, SignUpUserDto, UpdateUserDto};
use budgetto_domain::users::responses::{ReAuthResponse, UserAuthenicationResponse};
use budgetto_infrastructure::service_register::ServiceRegister;

use crate::extractors::required_authentication_extractor::RequiredAuthentication;
use crate::extractors::user_agent_extractor::UserAgentExtractor;
use crate::extractors::validation_extractor::ValidationExtractor;

pub struct UserRouter;

impl UserRouter {
    pub fn app() -> Router {
        Router::new()
            .route("/signup", post(Self::signup_user_endpoint))
            .route("/signin", post(Self::signin_user_endpoint))
            .route("/whoami", get(Self::get_current_user_endpoint))
            .route("/", put(Self::update_user_endpoint))
            .route("/reAuth", post(Self::re_auth_endpoint))
        // .route("/signout", post(Self::signout_user_endpoint))
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
    pub async fn signin_user_endpoint(
        Extension(services): Extension<ServiceRegister>,
        UserAgentExtractor(user_agent): UserAgentExtractor,
        ValidationExtractor(request): ValidationExtractor<SignInUserDto>,
    ) -> AppResult<Json<SessionResponse>> {
        info!(
            "recieved request to login user {:?}",
            request.email.as_ref().unwrap()
        );

        let tokens = services.users.signin_user(request, user_agent).await?;

        // let cookie = jar.add(Cookie::new("refresh_token", refresh_token.to_string()));

        Ok(Json(tokens))
    }

    pub async fn get_current_user_endpoint(
        RequiredAuthentication(user, services): RequiredAuthentication,
    ) -> AppResult<Json<UserAuthenicationResponse>> {
        info!("recieved request to retrieve current user");

        let current_user = services.users.get_current_user(user.id).await?;

        Ok(Json(UserAuthenicationResponse { user: current_user }))
    }

    pub async fn update_user_endpoint(
        RequiredAuthentication(user, services): RequiredAuthentication,
        Json(request): Json<UpdateUserDto>,
    ) -> AppResult<Json<UserAuthenicationResponse>> {
        info!("recieved request to update user {:?}", user.id);

        let updated_user = services.users.updated_user(user.id, request).await?;

        Ok(Json(UserAuthenicationResponse { user: updated_user }))
    }

    pub async fn re_auth_endpoint(
        Extension(services): Extension<ServiceRegister>,
        ValidationExtractor(request): ValidationExtractor<NewAccessTokenRequest>,
    ) -> AppResult<Json<ReAuthResponse>> {
        info!("recieved request to refresh access token");

        let new_token = services.sessions.refresh_access_token(request).await?;

        Ok(Json(new_token))
    }

    pub async fn signout_user_endpoint(// jar: CookieJar,
        // SessionResponsextension(services): Extension<ServiceRegister>,
        // SessionExtractor(session_id, _refresh_token): SessionExtractor,
    ) -> AppResult<SessionResponse> {
        info!("recieved request to signout session");
        // TODO! update this function
        // services.sessions.refresh_access_token(session_id).await?;

        // let cookie = jar.remove(Cookie::named("refresh_token"));
        todo!()
        // Ok(SessionResponse {
        //     access_token: String::new(),
        //     refresh_token: String::new(),
        // })
    }
}
