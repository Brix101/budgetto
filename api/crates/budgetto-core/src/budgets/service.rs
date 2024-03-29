use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;

use budgetto_domain::budgets::{
    requests::{CreateBudgetDto, UpdateBudgetDto},
    BudgetDto,
};

use crate::errors::AppResult;

pub type DynBudgetsService = Arc<dyn BudgetsService + Send + Sync>;

#[automock]
#[async_trait]
pub trait BudgetsService {
    async fn create(&self, args: CreateBudgetDto) -> AppResult<BudgetDto>;

    async fn find_by_id(&self, id: Uuid, user_id: Uuid) -> AppResult<BudgetDto>;

    async fn find_many(&self, user_id: Uuid) -> AppResult<Vec<BudgetDto>>;

    async fn updated(&self, args: UpdateBudgetDto) -> AppResult<BudgetDto>;

    async fn delete(&self, id: Uuid, user_id: Uuid) -> AppResult<()>;
}
