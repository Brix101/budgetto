use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;
use uuid::Uuid;

pub type DynTransactionsRepository = Arc<dyn TransactionsRepository + Send + Sync>;

#[automock]
#[async_trait]
pub trait TransactionsRepository {
    async fn create_transaction(
        &self,
        name: String,
        balance: f64,
        note: Option<String>,
        user_id: Uuid,
    ) -> anyhow::Result<Transaction>;

    async fn get_transactions(&self, user_id: Uuid) -> anyhow::Result<Vec<Transaction>>;

    async fn get_transaction_by_id(&self, id: Uuid) -> anyhow::Result<Option<Transaction>>;

    async fn update_transaction(
        &self,
        id: Uuid,
        name: String,
        balance: f64,
        note: Option<String>,
    ) -> anyhow::Result<Transaction>;

    async fn delete_transaction(&self, id: Uuid) -> anyhow::Result<()>;
}

#[derive(sqlx::Type, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[sqlx(type_name = "transaction_type")]
pub enum TransactionType {
    Expense,
    Income,
    Transfer,
    Refund,
}

impl Default for TransactionType {
    fn default() -> Self {
        Self::Expense
    }
}

#[derive(FromRow, Debug)]
pub struct Transaction {
    pub id: Uuid,
    pub amount: f64,
    pub note: Option<String>,
    pub transaction_type: TransactionType,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
