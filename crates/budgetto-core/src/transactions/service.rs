use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;

// use domain::transactions::requests::{SignInUserDto, SignUpUserDto, UpdateUserDto};
use budgetto_domain::transactions::TransactionDto;

use crate::errors::AppResult;

pub type DynTransactionsService = Arc<dyn TransactionsService + Send + Sync>;

#[automock]
#[async_trait]
pub trait TransactionsService {
    async fn create_transaction(&self) -> AppResult<TransactionDto>;

    async fn get_transaction_by_id(&self, id: Uuid, user_id: Uuid) -> AppResult<TransactionDto>;

    async fn get_transactions(&self, user_id: Uuid) -> AppResult<Vec<TransactionDto>>;

    async fn updated_transaction(&self) -> AppResult<TransactionDto>;

    async fn delete_transaction(&self, id: Uuid) -> AppResult<()>;
}
