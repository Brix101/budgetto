use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;

use budgetto_domain::accounts::{
    requests::{CreateAccountDto, UpdateAccountDto},
    responses::AccountsResponse,
    AccountDto,
};

use crate::errors::AppResult;

pub type DynAccountsService = Arc<dyn AccountsService + Send + Sync>;

#[automock]
#[async_trait]
pub trait AccountsService {
    async fn create_account(
        &self,
        user_id: Uuid,
        request: CreateAccountDto,
    ) -> AppResult<AccountDto>;

    async fn get_account_by_id(&self, id: Uuid, user_id: Uuid) -> AppResult<AccountDto>;

    async fn get_accounts(&self, user_id: Uuid) -> AppResult<AccountsResponse>;

    async fn updated_account(
        &self,
        id: Uuid,
        user_id: Uuid,
        request: UpdateAccountDto,
    ) -> AppResult<AccountDto>;

    async fn delete_account(&self, id: Uuid, user_id: Uuid) -> AppResult<()>;
}
