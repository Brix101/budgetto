use std::sync::Arc;

use async_trait::async_trait;
use budgetto_domain::transactions::TransactionDto;
use mockall::automock;
use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;
use time::format_description::well_known::Rfc3339;
use uuid::Uuid;

pub type DynTransactionsRepository = Arc<dyn TransactionsRepository + Send + Sync>;

#[automock]
#[async_trait]
pub trait TransactionsRepository {
    async fn create_transaction(&self, request: CreateTransaction) -> anyhow::Result<Transaction>;

    async fn get_transactions(&self, user_id: Uuid) -> anyhow::Result<Vec<Transaction>>;

    async fn get_transaction_by_id(&self, id: Uuid) -> anyhow::Result<Option<Transaction>>;

    async fn update_transaction(&self, request: UpdateTransaction) -> anyhow::Result<Transaction>;

    async fn delete_transaction(&self, id: Uuid) -> anyhow::Result<()>;
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct CreateTransaction {
    pub amount: f64,
    pub note: Option<String>,
    pub transaction_type: TransactionType,
    pub category_id: Uuid,
    pub account_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct UpdateTransaction {
    pub id: Uuid,
    pub amount: f64,
    pub note: Option<String>,
    pub transaction_type: TransactionType,
    pub category_id: Uuid,
    pub account_id: Uuid,
}

#[warn(unused_imports)]
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
    pub category_id: Uuid,
    pub account_id: Uuid,
    pub user_id: Uuid,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}

impl Transaction {
    pub fn into_dto(self) -> TransactionDto {
        TransactionDto {
            id: self.id,
            amount: self.amount,
            note: self.note,
            created_at: self.created_at.format(&Rfc3339).unwrap(),
            updated_at: self.updated_at.format(&Rfc3339).unwrap(),
        }
    }
}
