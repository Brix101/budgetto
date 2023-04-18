use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;

// use domain::accounts::requests::{SignInUserDto, SignUpUserDto, UpdateUserDto};
use domain::{
    sessions::{requests::NewSessionDto, SessionDto},
    users::UserDto,
};

use crate::errors::AppResult;

pub type DynUsersService = Arc<dyn UsersService + Send + Sync>;

#[automock]
#[async_trait]
pub trait UsersService {
    async fn new_session(&self, request: NewSessionDto) -> AppResult<SessionDto>;

    async fn refresh_access_token(&self, id: Uuid) -> AppResult<UserDto>;
}
