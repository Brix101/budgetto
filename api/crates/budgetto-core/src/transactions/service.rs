use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;

use budgetto_domain::transactions::{
    requests::{CreateTransactionDto, UpdateTransactionDto},
    TransactionDto,
};

use crate::errors::AppResult;

pub type DynTransactionsService = Arc<dyn TransactionsService + Send + Sync>;

#[automock]
#[async_trait]
pub trait TransactionsService {
    async fn create(&self, request: CreateTransactionDto) -> AppResult<TransactionDto>;

    async fn find_by_id(&self, id: Uuid, user_id: Uuid) -> AppResult<TransactionDto>;

    async fn find_many(&self, user_id: Uuid) -> AppResult<Vec<TransactionDto>>;

    async fn updated(&self, request: UpdateTransactionDto) -> AppResult<TransactionDto>;

    async fn delete(&self, id: Uuid, user_id: Uuid) -> AppResult<()>;
}
