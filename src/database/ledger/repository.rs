use std::sync::Arc;

use async_trait::async_trait;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;

/// Similar to above, we want to keep a reference count across threads so we can manage our connection pool.
pub type DynLedgersRepository = Arc<dyn LedgersRepository + Send + Sync>;

#[async_trait]
pub trait LedgersRepository {
    async fn create_ledger(
        &self,
        user_id: i64,
        name: String,
        amount: f64,
        description: String,
        frequency: String,
    ) -> anyhow::Result<Ledger>;

    async fn get_ledger_by_id(&self, id: i64) -> anyhow::Result<Option<Ledger>>;

    async fn get_ledgers(&self, user_id: i64) -> anyhow::Result<Vec<Ledger>>;

    async fn update_ledger(
        &self,
        id: i64,
        name: String,
        amount: f64,
        description: Option<String>,
        frequency: String,
    ) -> anyhow::Result<Ledger>;

    async fn delete_ledger(&self, id: i64) -> anyhow::Result<()>;
}

#[derive(FromRow, Debug)]
pub struct Ledger {
    pub id: i64,
    pub name: String,
    pub amount: f64,
    pub description: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
    pub user_id: i64,
    pub frequency: String,
}
