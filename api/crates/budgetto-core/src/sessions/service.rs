use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;

use budgetto_domain::{
    sessions::{requests::CreateSessionDto, responses::SessionResponse},
    users::responses::ReAuthResponse,
};

use crate::errors::AppResult;

pub type DynSessionsService = Arc<dyn SessionsService + Send + Sync>;

#[automock]
#[async_trait]
pub trait SessionsService {
    async fn create(&self, request: CreateSessionDto) -> AppResult<SessionResponse>;

    async fn create_access_token(&self, refresh_token: &str) -> AppResult<ReAuthResponse>;

    async fn delete(&self, refresh_token: &str) -> AppResult<()>;
}
