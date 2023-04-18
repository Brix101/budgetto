use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;

use domain::users::requests::{SignInUserDto, SignUpUserDto, UpdateUserDto};
use domain::users::UserDto;

use crate::errors::AppResult;

/// A reference counter for our user service allows us safely pass instances user utils
/// around which themselves depend on the user repostiory, and ultimately, our Posgres connection pool.
pub type DynUsersService = Arc<dyn UsersService + Send + Sync>;

#[automock]
#[async_trait]
pub trait UsersService {
    async fn signup_user(&self, request: SignUpUserDto) -> AppResult<UserDto>;

    async fn signin_user(
        &self,
        request: SignInUserDto,
        user_agent: Option<String>,
    ) -> AppResult<(UserDto, String)>;

    async fn get_current_user(&self, user_id: Uuid) -> AppResult<UserDto>;

    async fn updated_user(&self, user_id: Uuid, request: UpdateUserDto) -> AppResult<UserDto>;
}
