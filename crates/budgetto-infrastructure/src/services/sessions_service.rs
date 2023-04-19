use async_trait::async_trait;
use budgetto_domain::sessions::responses::SessionResponse;
use sqlx::types::time::OffsetDateTime;
use std::time::{Duration, SystemTime};
use tracing::info;
use uuid::Uuid;

use budgetto_core::errors::{AppResult, Error};
use budgetto_core::sessions::repository::DynSessionsRepository;
use budgetto_core::sessions::service::SessionsService;
use budgetto_core::utils::token_service::DynTokenService;
use budgetto_domain::sessions::requests::NewSessionDto;
use budgetto_domain::users::UserDto;

#[derive(Clone)]
pub struct BudgettoSessionsService {
    repository: DynSessionsRepository,
    token_service: DynTokenService,
}

impl BudgettoSessionsService {
    pub fn new(repository: DynSessionsRepository, token_service: DynTokenService) -> Self {
        Self {
            repository,
            token_service,
        }
    }
}

#[async_trait]
impl SessionsService for BudgettoSessionsService {
    async fn new_session(&self, request: NewSessionDto) -> AppResult<SessionResponse> {
        let user_id = request.user_id.unwrap();
        let user_agent = request.user_agent.unwrap();
        let from_now = Duration::from_secs(604800);
        let expired_future_time = SystemTime::now().checked_add(from_now).unwrap();
        let exp = OffsetDateTime::from(expired_future_time);

        let created_session = self
            .repository
            .new_session(user_id, user_agent.as_str(), &exp)
            .await?;

        let user_session = self
            .repository
            .get_user_by_session_id(created_session.id)
            .await?
            .unwrap();

        info!("session successfully created, generating tokens");
        let access_token = self
            .token_service
            .new_access_token(user_session.id, &user_session.email)?;

        let refresh_token = self.token_service.new_refresh_token(created_session.id)?;

        Ok(SessionResponse {
            access_token,
            refresh_token,
        })
    }

    async fn refresh_access_token(&self, id: Uuid) -> AppResult<UserDto> {
        let user_in_session = self.repository.get_user_by_session_id(id).await?;

        if let Some(user) = user_in_session {
            info!("existing session found, generating access token");
            let access_token = self.token_service.new_access_token(user.id, &user.email)?;

            return Ok(user.into_dto(access_token));
        }

        Err(Error::Unauthorized)
    }
}
