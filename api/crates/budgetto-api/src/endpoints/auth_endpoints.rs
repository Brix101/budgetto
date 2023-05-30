use axum::extract::Json;
use axum::routing::{get, post};
use axum::{Extension, Router};
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;
use tracing::info;

use budgetto_core::errors::AppResult;
use budgetto_domain::sessions::responses::SessionResponse;
use budgetto_domain::users::requests::{SignInUserDto, SignUpUserDto};
use budgetto_domain::users::responses::{ReAuthResponse, UserAuthenicationResponse};
use budgetto_infrastructure::service_register::ServiceRegister;

use crate::extractors::required_authentication_extractor::RequiredAuthentication;
use crate::extractors::session_extractor::SessionExtractor;
use crate::extractors::user_agent_extractor::UserAgentExtractor;
use crate::extractors::validation_extractor::ValidationExtractor;

pub struct AuthRouter;

impl AuthRouter {
    pub fn app() -> Router {
        Router::new()
            .route("/sign-up", post(Self::signup_user))
            .route("/sign-in", post(Self::signin_user))
            .route("/whoami", get(Self::who_am_i))
            .route("/re-auth", post(Self::re_auth))
            .route("/sign-out", post(Self::signout_user))
    }

    pub async fn signup_user(
        Extension(services): Extension<ServiceRegister>,
        ValidationExtractor(request): ValidationExtractor<SignUpUserDto>,
    ) -> AppResult<Json<UserAuthenicationResponse>> {
        info!(
            "recieved request to sign up user {:?}/{:?}",
            request.email.as_ref().unwrap(),
            request.name.as_ref().unwrap()
        );

        let created_user = services.users.signup(request).await?;

        Ok(Json(UserAuthenicationResponse { user: created_user }))
    }

    pub async fn signin_user(
        jar: CookieJar,
        Extension(services): Extension<ServiceRegister>,
        UserAgentExtractor(user_agent): UserAgentExtractor,
        ValidationExtractor(mut request): ValidationExtractor<SignInUserDto>,
    ) -> AppResult<(CookieJar, Json<SessionResponse>)> {
        info!(
            "recieved request to sign in user {:?}",
            request.email.as_ref().unwrap()
        );

        request.user_agent = user_agent;
        let user = services.users.signin(request).await?;

        let cookie = Cookie::build("x-refresh", user.refresh_token.clone().unwrap())
            .path("/")
            .secure(false)
            .http_only(true)
            .finish();

        let cookie_jar = jar.add(cookie);

        Ok((cookie_jar, Json(user)))
    }

    pub async fn who_am_i(
        RequiredAuthentication(user): RequiredAuthentication,
    ) -> AppResult<Json<UserAuthenicationResponse>> {
        info!("recieved request to retrieve current user");

        Ok(Json(UserAuthenicationResponse { user }))
    }

    pub async fn re_auth(
        Extension(services): Extension<ServiceRegister>,
        SessionExtractor(_, refresh_token): SessionExtractor,
    ) -> AppResult<Json<ReAuthResponse>> {
        info!("recieved request to refresh access token");

        let new_token = services
            .sessions
            .create_access_token(&refresh_token)
            .await?;

        Ok(Json(new_token))
    }

    pub async fn signout_user(
        jar: CookieJar,
        RequiredAuthentication(user): RequiredAuthentication,
        Extension(services): Extension<ServiceRegister>,
        SessionExtractor(session_id, _): SessionExtractor,
    ) -> AppResult<CookieJar> {
        info!("recieved request to signout session");
        services.sessions.delete(session_id, user.id).await?;

        let cookie_jar = jar.remove(Cookie::named("x-refresh"));

        Ok(cookie_jar)
    }
}
