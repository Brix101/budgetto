use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;
use uuid::Uuid;

/// Similar to above, we want to keep a reference count across threads so we can manage our connection pool.
pub type DynbudgetsRepository = Arc<dyn BudgetsRepository + Send + Sync>;

#[automock]
#[async_trait]
pub trait BudgetsRepository {
    async fn create_budget(
        &self,
        name: String,
        balance: f64,
        note: Option<String>,
        user_id: Uuid,
    ) -> anyhow::Result<Budget>;

    async fn get_budgets(&self, user_id: Uuid) -> anyhow::Result<Vec<Budget>>;

    async fn get_budget_by_id(&self, id: Uuid) -> anyhow::Result<Option<Budget>>;

    async fn update_budget(
        &self,
        id: Uuid,
        name: String,
        balance: f64,
        note: Option<String>,
    ) -> anyhow::Result<Budget>;

    async fn delete_budget(&self, id: Uuid) -> anyhow::Result<()>;
}

#[derive(FromRow, Debug)]
pub struct Budget {
    pub id: Uuid,
    pub name: String,
    pub balance: f64,
    pub note: Option<String>,
    pub user_id: Uuid,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
