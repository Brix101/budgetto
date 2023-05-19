use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;

use budgetto_domain::transactions::{
    requests::{CreateTransactionDto, UpdateTransactionDto},
    responses::TransactionsResponse,
    TransactionDto,
};

use crate::errors::AppResult;

pub type DynTransactionsService = Arc<dyn TransactionsService + Send + Sync>;

#[automock]
#[async_trait]
pub trait TransactionsService {
    async fn create_transaction(
        &self,
        user_id: Uuid,
        request: CreateTransactionDto,
    ) -> AppResult<TransactionDto>;

    async fn get_transaction_by_id(&self, id: Uuid, user_id: Uuid) -> AppResult<TransactionDto>;

    async fn get_transactions(&self, user_id: Uuid) -> AppResult<TransactionsResponse>;

    async fn updated_transaction(
        &self,
        id: Uuid,
        user_id: Uuid,
        request: UpdateTransactionDto,
    ) -> AppResult<TransactionDto>;

    async fn delete_transaction(&self, id: Uuid, user_id: Uuid) -> AppResult<()>;
}
