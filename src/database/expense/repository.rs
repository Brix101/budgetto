use std::sync::Arc;

use async_trait::async_trait;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;

/// Similar to above, we want to keep a reference count across threads so we can manage our connection pool.
pub type DynExpensesRepository = Arc<dyn ExpensesRepository + Send + Sync>;

#[async_trait]
pub trait ExpensesRepository {
    async fn create_expense(
        &self,
        amount: f64,
        category_id: i64,
        user_id: i64,
    ) -> anyhow::Result<Expense>;

    async fn get_expense_by_id(&self, id: i64) -> anyhow::Result<Option<Expense>>;

    async fn get_expenses(&self, user_id: i64) -> anyhow::Result<Vec<Expense>>;

    async fn update_expense(
        &self,
        id: i64,
        amount: f64,
        category_id: i64,
    ) -> anyhow::Result<Expense>;

    async fn delete_expense(&self, id: i64) -> anyhow::Result<()>;
}

#[derive(FromRow, Debug)]
pub struct Expense {
    pub id: i64,
    pub amount: f64,
    pub category_id: i64,
    pub user_id: i64,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}
