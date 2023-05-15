use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::Extension;
use budgetto_domain::users::UserDto;

use budgetto_core::errors::Error;

/// Extracts the JWT from the Authorization token header.
pub struct RequiredAuthentication(pub UserDto);

#[async_trait]
impl<S> FromRequestParts<S> for RequiredAuthentication
where
    S: Send + Sync,
{
    type Rejection = Error;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Extension(user): Extension<UserDto> = Extension::from_request_parts(parts, state)
            .await
            .map_err(|_err| Error::Unauthorized)?;

        Ok(RequiredAuthentication(user))
    }
}
