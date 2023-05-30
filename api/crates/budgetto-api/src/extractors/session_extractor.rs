use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::header::COOKIE;
use axum::http::request::Parts;
use axum::Extension;
use axum_extra::extract::cookie::Cookie;

use budgetto_core::errors::Error;
use budgetto_domain::users::AuthClaims;

/// Extracts the JWT from the cookie token header.
pub struct RefreshTokenExtractor(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for RefreshTokenExtractor
where
    S: Send + Sync,
{
    type Rejection = Error;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Extension(auth_claims): Extension<AuthClaims> =
            Extension::from_request_parts(parts, state)
                .await
                .map_err(|_err| Error::Unauthorized)?;

        if auth_claims.user_id.is_none() {
            return Err(Error::Unauthorized);
        }

        if let Some(authorization_header) = parts.headers.get(COOKIE) {
            let header_cookie_value = authorization_header
                .to_str()
                .map_err(|_| Error::Forbidden)?;

            let cookie_value = Cookie::parse(header_cookie_value).unwrap();

            let token_value = cookie_value.value();

            return Ok(Self(token_value.to_string()));
        }

        Err(Error::Unauthorized)
    }
}
