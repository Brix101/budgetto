use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::header::COOKIE;
use axum::http::request::Parts;
use axum::Extension;
use axum_extra::extract::cookie::Cookie;
use budgetto_infrastructure::service_register::ServiceRegister;
use tracing::error;
use uuid::Uuid;

use budgetto_core::errors::Error;

/// Extracts the JWT from the cookie token header.
pub struct SessionExtractor(pub Uuid, pub String);

#[async_trait]
impl<S> FromRequestParts<S> for SessionExtractor
where
    S: Send + Sync,
{
    type Rejection = Error;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Extension(services): Extension<ServiceRegister> =
            Extension::from_request_parts(parts, state)
                .await
                .map_err(|err| Error::InternalServerErrorWithContext(err.to_string()))?;

        if let Some(authorization_header) = parts.headers.get(COOKIE) {
            let header_cookie_value = authorization_header
                .to_str()
                .map_err(|_| Error::Forbidden)?;

            let cookie_value = Cookie::parse(header_cookie_value).unwrap();

            let refresh_token_value = cookie_value.value();

            let session_id = services
                .token_service
                .get_session_id_from_token(String::from(refresh_token_value))
                .map_err(|err| {
                    error!("could not validate session ID from token: {:?}", err);
                    Error::Forbidden
                })?;

            Ok(SessionExtractor(
                session_id,
                refresh_token_value.to_string(),
            ))
        } else {
            Err(Error::Forbidden)
        }
    }
}
