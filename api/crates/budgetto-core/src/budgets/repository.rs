use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;
use time::format_description::well_known::Rfc3339;
use uuid::Uuid;

use budgetto_domain::budgets::BudgetDto;

/// Similar to above, we want to keep a reference count across threads so we can manage our connection pool.
pub type DynbudgetsRepository = Arc<dyn BudgetsRepository + Send + Sync>;

#[automock]
#[async_trait]
pub trait BudgetsRepository {
    async fn create(
        &self,
        name: String,
        balance: f64,
        note: Option<String>,
        user_id: Uuid,
    ) -> anyhow::Result<Budget>;

    async fn find_many(&self, user_id: Uuid) -> anyhow::Result<Vec<Budget>>;

    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Budget>>;

    async fn update(
        &self,
        id: Uuid,
        name: String,
        balance: f64,
        note: Option<String>,
    ) -> anyhow::Result<Budget>;

    async fn delete(&self, id: Uuid) -> anyhow::Result<()>;
}

#[derive(FromRow, Debug)]
pub struct Budget {
    pub id: Uuid,
    pub name: String,
    pub balance: f64,
    pub note: String,
    pub user_id: Uuid,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Budget {
    pub fn into_dto(self) -> BudgetDto {
        BudgetDto {
            id: self.id,
            name: self.name,
            balance: self.balance,
            note: Some(self.note),
            created_at: self.created_at.format(&Rfc3339).unwrap(),
            updated_at: self.updated_at.format(&Rfc3339).unwrap(),
        }
    }
}
