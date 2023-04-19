use axum::extract::Json;
use axum::routing::{get, post, put};
use axum::{Extension, Router};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use tracing::info;

use budgetto_core::errors::AppResult;
use budgetto_domain::users::requests::{SignInUserDto, SignUpUserDto, UpdateUserDto};
use budgetto_domain::users::responses::UserAuthenicationResponse;
use budgetto_infrastructure::service_register::ServiceRegister;

use crate::extractors::required_authentication_extractor::RequiredAuthentication;
use crate::extractors::session_extractor::SessionExtractor;
use crate::extractors::user_agent_extractor::UserAgentExtractor;
use crate::extractors::validation_extractor::ValidationExtractor;

pub struct UserRouter;

impl UserRouter {
    pub fn app() -> Router {
        Router::new()
            .route("/signup", post(Self::signup_user_endpoint))
            .route("/signin", post(Self::signin_user_endpoint))
            .route("/signout", post(Self::signout_user_endpoint))
            .route("/whoami", get(Self::get_current_user_endpoint))
            .route("/refresh", get(Self::refresh_user_endpoint))
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
    pub async fn signin_user_endpoint(
        jar: CookieJar,
        Extension(services): Extension<ServiceRegister>,
        UserAgentExtractor(user_agent): UserAgentExtractor,
        ValidationExtractor(request): ValidationExtractor<SignInUserDto>,
    ) -> AppResult<(CookieJar, Json<UserAuthenicationResponse>)> {
        info!(
            "recieved request to login user {:?}",
            request.email.as_ref().unwrap()
        );

        let (user, refresh_token) = services.users.signin_user(request, user_agent).await?;

        let cookie = jar.add(Cookie::new("refresh_token", refresh_token.to_string()));

        Ok((cookie, Json(UserAuthenicationResponse { user })))
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

    pub async fn refresh_user_endpoint(
        jar: CookieJar,
        Extension(services): Extension<ServiceRegister>,
        SessionExtractor(session_id, refresh_token): SessionExtractor,
    ) -> AppResult<(CookieJar, Json<UserAuthenicationResponse>)> {
        info!("recieved request to refresh access token {:?}", session_id);

        let user = services.sessions.refresh_access_token(session_id).await?;

        let cookie = jar.add(Cookie::new("refresh_token", refresh_token));

        Ok((cookie, Json(UserAuthenicationResponse { user })))
    }

    pub async fn signout_user_endpoint(
        jar: CookieJar,
        Extension(services): Extension<ServiceRegister>,
        SessionExtractor(session_id, _refresh_token): SessionExtractor,
    ) -> AppResult<CookieJar> {
        info!("recieved request to signout session {:?}", session_id);

        services.sessions.refresh_access_token(session_id).await?;

        let cookie = jar.remove(Cookie::named("refresh_token"));

        Ok(cookie)
    }
}
