use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;

/// Similar to above, we want to keep a reference count across threads so we can manage our connection pool.
pub type DynBudgetsRepository = Arc<dyn BudgetsRepository + Send + Sync>;

#[automock]
#[async_trait]
pub trait BudgetsRepository {
    async fn create_budget(
        &self,
        user_id: i64,
        name: String,
        amount: f64,
        description: String,
        frequency: String,
        category_id: i64,
    ) -> anyhow::Result<Budget>;

    async fn get_budget_by_id(&self, id: i64) -> anyhow::Result<Option<Budget>>;

    async fn get_budgets(&self, user_id: i64) -> anyhow::Result<Vec<Budget>>;

    async fn update_budget(
        &self,
        id: i64,
        name: String,
        amount: f64,
        description: String,
        frequency: String,
        category_id: i64,
    ) -> anyhow::Result<Budget>;

    async fn delete_budget(&self, id: i64) -> anyhow::Result<()>;
}

#[derive(FromRow, Debug)]
pub struct Budget {
    pub id: i64,
    pub name: String,
    pub amount: f64,
    pub description: String,
    pub frequency: String,
    pub category_id: i64,
    pub user_id: i64,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}
