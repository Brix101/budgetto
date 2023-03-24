use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;
use uuid::Uuid;

/// Similar to above, we want to keep a reference count across threads so we can manage our connection pool.
pub type DynBudgetsRepository = Arc<dyn BudgetsRepository + Send + Sync>;

#[derive(sqlx::Type, Serialize, Deserialize, Debug, Clone)]
#[sqlx(type_name = "plan_type")]
pub enum PlanType {
    Daily,
    Weekly,
    Monthly,
}

impl Default for PlanType {
    fn default() -> Self {
        Self::Monthly
    }
}

#[automock]
#[async_trait]
pub trait BudgetsRepository {
    async fn create_budget(
        &self,
        category_id: Uuid,
        amount: f64,
        description: String,
        plan: PlanType,
    ) -> anyhow::Result<Budget>;

    async fn get_budget_by_id(&self, id: Uuid) -> anyhow::Result<Option<Budget>>;

    async fn get_budgets(&self, user_id: Uuid) -> anyhow::Result<Vec<Budget>>;

    async fn update_budget(
        &self,
        id: Uuid,
        category_id: Uuid,
        amount: f64,
        description: String,
        plan: PlanType,
    ) -> anyhow::Result<Budget>;

    async fn delete_budget(&self, id: Uuid) -> anyhow::Result<()>;
}

#[derive(FromRow, Debug)]
pub struct Budget {
    pub id: Uuid,
    pub amount: f64,
    pub description: String,
    pub plan: PlanType,
    pub category_id: Uuid,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}
