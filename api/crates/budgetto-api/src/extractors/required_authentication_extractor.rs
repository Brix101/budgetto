use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::Extension;

use budgetto_core::errors::Error;
use budgetto_domain::users::AuthClaims;
use uuid::Uuid;

/// Extracts the JWT from the Authorization token header.
pub struct RequiredAuthentication(pub Uuid);

#[async_trait]
impl<S> FromRequestParts<S> for RequiredAuthentication
where
    S: Send + Sync,
{
    type Rejection = Error;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Extension(auth_claims): Extension<AuthClaims> =
            Extension::from_request_parts(parts, state)
                .await
                .map_err(|_err| Error::Unauthorized)?;

        if let Some(user_id) = auth_claims.user_id {
            return Ok(RequiredAuthentication(user_id));
        }

        Err(Error::Unauthorized)
    }
}
