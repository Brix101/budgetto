use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;

use budgetto_domain::{
    sessions::{requests::NewSessionDto, responses::SessionResponse},
    users::responses::ReAuthResponse,
};

use crate::errors::AppResult;

pub type DynSessionsService = Arc<dyn SessionsService + Send + Sync>;

#[automock]
#[async_trait]
pub trait SessionsService {
    async fn new_session(&self, request: NewSessionDto) -> AppResult<SessionResponse>;

    async fn refresh_access_token(&self, refresh_token: &str) -> AppResult<ReAuthResponse>;
}
