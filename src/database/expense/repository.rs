use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;
use uuid::Uuid;

/// Similar to above, we want to keep a reference count across threads so we can manage our connection pool.
pub type DynExpensesRepository = Arc<dyn ExpensesRepository + Send + Sync>;

#[automock]
#[async_trait]
pub trait ExpensesRepository {
    async fn create_expense(
        &self,
        category_id: Uuid,
        amount: f64,
        description: String,
    ) -> anyhow::Result<Expense>;

    async fn get_expense_by_id(&self, id: Uuid) -> anyhow::Result<Option<Expense>>;

    async fn get_expenses(&self, user_id: Uuid) -> anyhow::Result<Vec<Expense>>;

    async fn update_expense(
        &self,
        id: Uuid,
        category_id: Uuid,
        amount: f64,
        description: String,
    ) -> anyhow::Result<Expense>;

    async fn delete_expense(&self, id: Uuid) -> anyhow::Result<()>;
}

#[derive(FromRow, Debug)]
pub struct Expense {
    pub id: Uuid,
    pub category_id: Uuid,
    pub amount: f64,
    pub description: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}
