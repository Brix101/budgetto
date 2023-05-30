use std::sync::Arc;

use async_trait::async_trait;
use budgetto_domain::sessions::responses::SessionResponse;
use mockall::automock;
use uuid::Uuid;

use budgetto_domain::users::requests::{SignInUserDto, SignUpUserDto, UpdateUserDto};
use budgetto_domain::users::UserDto;

use crate::errors::AppResult;

/// A reference counter for our user service allows us safely pass instances user utils
/// around which themselves depend on the user repostiory, and ultimately, our Posgres connection pool.
pub type DynUsersService = Arc<dyn UsersService + Send + Sync>;

#[automock]
#[async_trait]
pub trait UsersService {
    async fn signup(&self, request: SignUpUserDto) -> AppResult<UserDto>;

    async fn signin(&self, request: SignInUserDto) -> AppResult<SessionResponse>;

    async fn find_by_id(&self, id: Uuid) -> AppResult<UserDto>;

    async fn updated(&self, request: UpdateUserDto) -> AppResult<UserDto>;

    async fn find_by_email(&self, email: String) -> AppResult<UserDto>;
}
