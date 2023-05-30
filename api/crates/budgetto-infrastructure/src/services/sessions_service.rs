use async_trait::async_trait;
use budgetto_domain::sessions::responses::SessionResponse;
use budgetto_domain::users::responses::ReAuthResponse;
use sqlx::types::time::OffsetDateTime;
use std::time::{Duration, SystemTime};
use tracing::info;

use budgetto_core::errors::{AppResult, Error};
use budgetto_core::sessions::repository::{CreateSession, DynSessionsRepository};
use budgetto_core::sessions::service::SessionsService;
use budgetto_core::utils::token_service::DynTokenService;
use budgetto_domain::sessions::requests::CreateSessionDto;

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
    async fn create(&self, request: CreateSessionDto) -> AppResult<SessionResponse> {
        let user_id = request.user_id.unwrap();
        let user_agent = request.user_agent;
        let from_now = Duration::from_secs(604800);
        let expired_future_time = SystemTime::now().checked_add(from_now).unwrap();
        let exp = OffsetDateTime::from(expired_future_time);

        if user_agent.is_some() {
            info!("user login successful, creating session");
        }

        match user_agent {
            Some(user_agent) => {
                let new_session = self
                    .repository
                    .create(CreateSession {
                        user_id,
                        user_agent,
                        exp,
                    })
                    .await?;

                let user = self
                    .repository
                    .get_user_by_session_id(new_session.id)
                    .await?
                    .unwrap();

                info!("session successfully created, generating tokens");

                let access_token = self
                    .token_service
                    .new_access_token(new_session.id, user.id)?;

                let refresh_token = self.token_service.new_refresh_token(new_session.id)?;

                Ok(SessionResponse {
                    user: Some(user.into_dto()),
                    access_token: Some(access_token),
                    refresh_token: Some(refresh_token),
                })
            }
            None => Ok(SessionResponse {
                user: None,
                access_token: None,
                refresh_token: None,
            }),
        }
    }

    async fn create_access_token(&self, refresh_token: &str) -> AppResult<ReAuthResponse> {
        let session_id = self
            .token_service
            .get_session_id_from_token(refresh_token.to_string())
            .map_err(|err| {
                info!("could not validate session ID from token: {:?}", err);
                Error::Unauthorized
            })?;

        let user_in_session = self.repository.get_user_by_session_id(session_id).await?;

        if let Some(user) = user_in_session {
            info!("existing session found, generating access token");
            let access_token = self.token_service.new_access_token(session_id, user.id)?;

            return Ok(ReAuthResponse {
                user: user.into_dto(),
                access_token,
            });
        }

        Err(Error::Unauthorized)
    }

    async fn delete(&self, refresh_token: &str) -> AppResult<()> {
        let session_id = self
            .token_service
            .get_session_id_from_token(refresh_token.to_string())
            .map_err(|err| {
                info!("could not validate session ID from token: {:?}", err);
                Error::Unauthorized
            })?;

        // verify the user IDs match on the request and the session

        info!("deleting session {:?}", session_id);
        self.repository.delete(session_id).await?;
        self.token_service.add_blacklist(session_id);

        return Ok(());
    }
}
