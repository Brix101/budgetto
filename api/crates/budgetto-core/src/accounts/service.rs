use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;

use budgetto_domain::accounts::{
    requests::{CreateAccountDto, UpdateAccountDto},
    AccountDto,
};

use crate::errors::AppResult;

pub type DynAccountsService = Arc<dyn AccountsService + Send + Sync>;

#[automock]
#[async_trait]
pub trait AccountsService {
    async fn create(&self, args: CreateAccountDto) -> AppResult<AccountDto>;

    async fn find_by_id(&self, id: Uuid, user_id: Uuid) -> AppResult<AccountDto>;

    async fn find_many(&self, user_id: Uuid) -> AppResult<Vec<AccountDto>>;

    async fn updated(&self, args: UpdateAccountDto) -> AppResult<AccountDto>;

    async fn delete(&self, id: Uuid, user_id: Uuid) -> AppResult<()>;
}
