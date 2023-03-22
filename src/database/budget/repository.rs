use std::sync::Arc;

use async_trait::async_trait;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;

/// Similar to above, we want to keep a reference count across threads so we can manage our connection pool.
pub type DynBudgetsRepository = Arc<dyn BudgetsRepository + Send + Sync>;

#[async_trait]
pub trait BudgetsRepository {
    async fn create_budget(
        &self,
        email: &str,
        name: &str,
        hash_password: &str,
    ) -> anyhow::Result<Budget>;

    async fn get_budget_by_id(&self, id: i64) -> anyhow::Result<Budget>;
    async fn get_budgets(&self, user_id: i64) -> anyhow::Result<Vec<Budget>>;

    async fn update_user(
        &self,
        id: i64,
        email: String,
        name: String,
        password: String,
        bio: String,
        image: String,
    ) -> anyhow::Result<Budget>;
}

#[derive(FromRow)]
pub struct Budget {
    pub id: i64,
    pub name: String,
    pub amount: f64,
    pub desciption: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}
