use axum::extract::Json;
use axum::routing::{get, post};
use axum::{Extension, Router};
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;
use budgetto_domain::sessions::responses::SessionResponse;
use tracing::info;

use budgetto_core::errors::AppResult;
use budgetto_domain::users::requests::SignInUserDto;
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
            .route("/sign-in", post(Self::signin_user_endpoint))
            .route("/whoami", get(Self::get_current_user_endpoint))
            .route("/re-auth", post(Self::re_auth_endpoint))
            .route("/sign-out", post(Self::signout_user_endpoint))
    }

    pub async fn signin_user_endpoint(
        jar: CookieJar,
        Extension(services): Extension<ServiceRegister>,
        UserAgentExtractor(user_agent): UserAgentExtractor,
        ValidationExtractor(request): ValidationExtractor<SignInUserDto>,
    ) -> AppResult<(CookieJar, Json<SessionResponse>)> {
        info!(
            "recieved request to login user {:?}",
            request.email.as_ref().unwrap()
        );

        let user = services.users.signin_user(request, user_agent).await?;

        let cookie = Cookie::build("x-refresh", user.refresh_token.clone().unwrap())
            .path("/")
            .secure(false)
            .http_only(true)
            .finish();

        let cookie_jar = jar.add(cookie);

        Ok((cookie_jar, Json(user)))
    }

    pub async fn get_current_user_endpoint(
        RequiredAuthentication(user): RequiredAuthentication,
    ) -> AppResult<Json<UserAuthenicationResponse>> {
        info!("recieved request to retrieve current user");

        Ok(Json(UserAuthenicationResponse { user }))
    }

    pub async fn re_auth_endpoint(
        Extension(services): Extension<ServiceRegister>,
        SessionExtractor(_session_id, refresh_token): SessionExtractor,
    ) -> AppResult<Json<ReAuthResponse>> {
        info!("recieved request to refresh access token");

        let new_token = services
            .sessions
            .refresh_access_token(&refresh_token)
            .await?;

        Ok(Json(new_token))
    }

    pub async fn signout_user_endpoint(
        jar: CookieJar,
        // Extension(_services): Extension<ServiceRegister>,
        // SessionExtractor(session_id, _refresh_token): SessionExtractor,
    ) -> AppResult<CookieJar> {
        info!("recieved request to signout session");
        // services.sessions.refresh_access_token(session_id).await?;

        let cookie_jar = jar.remove(Cookie::named("x-refresh"));
        Ok(cookie_jar)
    }
}
