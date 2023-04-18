use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;

// use domain::accounts::requests::{SignInUserDto, SignUpUserDto, UpdateUserDto};
use domain::transactions::TransactionDto;

use crate::errors::AppResult;

pub type DynUsersService = Arc<dyn UsersService + Send + Sync>;

#[automock]
#[async_trait]
pub trait UsersService {
    async fn create_account(&self) -> AppResult<TransactionDto>;

    async fn get_account_by_id(&self, id: Uuid, user_id: Uuid) -> AppResult<TransactionDto>;

    async fn get_accounts(&self, user_id: Uuid) -> AppResult<Vec<TransactionDto>>;

    async fn updated_account(&self) -> AppResult<TransactionDto>;

    async fn delete_account(&self, id: Uuid) -> AppResult<()>;
}
