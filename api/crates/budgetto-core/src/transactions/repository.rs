use std::sync::Arc;

use async_trait::async_trait;
use budgetto_domain::transactions::{TransactionDto, TransactionType};
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
    async fn create(&self, args: CreateTransaction) -> anyhow::Result<Transaction>;

    async fn find_many(&self, user_id: Uuid) -> anyhow::Result<Vec<Transaction>>;

    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Transaction>>;

    async fn update(&self, args: UpdateTransaction) -> anyhow::Result<Transaction>;

    async fn delete(&self, id: Uuid) -> anyhow::Result<()>;
}

#[derive(Debug)]
pub struct CreateTransaction {
    pub amount: f64,
    pub note: Option<String>,
    pub transaction_type: TransactionType,
    pub category_id: Uuid,
    pub account_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Debug)]
pub struct UpdateTransaction {
    pub id: Uuid,
    pub amount: f64,
    pub note: Option<String>,
    pub transaction_type: TransactionType,
    pub category_id: Uuid,
    pub account_id: Uuid,
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
            transaction_type: self.transaction_type,
            created_at: self.created_at.format(&Rfc3339).unwrap(),
            updated_at: self.updated_at.format(&Rfc3339).unwrap(),
        }
    }
}
