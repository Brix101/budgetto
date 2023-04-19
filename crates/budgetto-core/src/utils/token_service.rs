use mockall::automock;
use std::sync::Arc;
use uuid::Uuid;

use crate::errors::AppResult;

/// A security service for handling JWT authentication.
pub type DynTokenService = Arc<dyn TokenService + Send + Sync>;

#[automock]
pub trait TokenService {
    fn new_access_token(&self, user_id: Uuid, email: &str) -> AppResult<String>;
    fn new_refresh_token(&self, sub: Uuid) -> AppResult<String>;
    fn get_user_id_from_token(&self, token: String) -> AppResult<Uuid>;
    fn get_session_id_from_token(&self, token: String) -> AppResult<Uuid>;
}
