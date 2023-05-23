use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::Extension;
use budgetto_domain::users::UserDto;

use budgetto_core::errors::Error;
use budgetto_domain::users::responses::ReAuthResponse;

/// Extracts the JWT from the Authorization token header.
pub struct RequiredAuthentication(pub UserDto);

#[async_trait]
impl<S> FromRequestParts<S> for RequiredAuthentication
where
    S: Send + Sync,
{
    type Rejection = Error;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Extension(re_auth_response): Extension<ReAuthResponse> =
            Extension::from_request_parts(parts, state)
                .await
                .map_err(|_err| Error::Unauthorized)?;

        if let Some(user) = re_auth_response.user {
            return Ok(RequiredAuthentication(user));
        }

        Err(Error::Unauthorized)
    }
}
