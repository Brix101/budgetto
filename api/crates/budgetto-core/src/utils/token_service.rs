use budgetto_domain::utils::token::RefreshTokenClaims;
use mockall::automock;
use std::sync::Arc;
use uuid::Uuid;

use budgetto_domain::utils::token::AccessTokenClaims;

use crate::errors::AppResult;

/// A security service for handling JWT authentication.
pub type DynTokenService = Arc<dyn TokenService + Send + Sync>;

#[automock]
pub trait TokenService {
    fn new_access_token(&self, sub: Uuid, user_id: Uuid) -> AppResult<String>;
    fn new_refresh_token(&self, sub: Uuid) -> AppResult<String>;
    fn verify_access_token(&self, token: &str) -> AppResult<AccessTokenClaims>;
    fn verify_refresh_token(&self, token: &str) -> AppResult<RefreshTokenClaims>;
    fn get_session_id_from_token(&self, token: String) -> AppResult<Uuid>;
    fn refresh_blacklist(&self);
    fn add_blacklist(&self, id: Uuid);
}
