use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;

// use domain::accounts::requests::{SignInUserDto, SignUpUserDto, UpdateUserDto};
use budgetto_domain::categories::CategoryDto;

use crate::errors::AppResult;

pub type DynUsersService = Arc<dyn UsersService + Send + Sync>;

#[automock]
#[async_trait]
pub trait UsersService {
    async fn create_account(&self) -> AppResult<CategoryDto>;

    async fn get_account_by_id(&self, id: Uuid, user_id: Uuid) -> AppResult<CategoryDto>;

    async fn get_accounts(&self, user_id: Uuid) -> AppResult<Vec<CategoryDto>>;

    async fn updated_account(&self) -> AppResult<CategoryDto>;

    async fn delete_account(&self, id: Uuid) -> AppResult<()>;
}
