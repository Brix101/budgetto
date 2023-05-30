use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::Extension;
use tracing::error;

use budgetto_core::errors::Error;
use budgetto_domain::users::{AuthClaims, UserDto};
use budgetto_infrastructure::service_register::ServiceRegister;

/// Extracts the JWT from the Authorization token header.
pub struct RequiredAuthentication(pub UserDto);

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

        if let Some(auth_user) = auth_claims.user {
            let Extension(services): Extension<ServiceRegister> =
                Extension::from_request_parts(parts, state)
                    .await
                    .map_err(|err| Error::InternalServerErrorWithContext(err.to_string()))?;

            services
                .sessions
                .get_user(auth_claims.session_id.unwrap())
                .await?;

            let user = services
                .users
                .find_by_id(auth_user.id)
                .await
                .map_err(|err| {
                    error!("invalid user ID from token: {:?}", err);
                    Error::Unauthorized
                })?;

            return Ok(RequiredAuthentication(user));
        }

        Err(Error::Unauthorized)
    }
}
