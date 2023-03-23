use std::sync::Arc;
use tracing::info;

use async_trait::async_trait;

use crate::{
    database::budget::repository::{Budget, DynBudgetsRepository},
    server::{
        dtos::budget_dto::{BudgetCreateDto, BudgetResponseDto, BudgetUpdateDto},
        error::{AppResult, Error},
    },
};

pub type DynBudgetsService = Arc<dyn BudgetsServiceTrait + Send + Sync>;

#[async_trait]
pub trait BudgetsServiceTrait {
    async fn create_budget(
        &self,
        user_id: i64,
        request: BudgetCreateDto,
    ) -> AppResult<BudgetResponseDto>;

    async fn get_budget_by_id(&self, id: i64, user_id: i64) -> AppResult<BudgetResponseDto>;

    async fn get_budgets(&self, user_id: i64) -> AppResult<Vec<BudgetResponseDto>>;

    async fn updated_budget(
        &self,
        id: i64,
        user_id: i64,
        request: BudgetUpdateDto,
    ) -> AppResult<BudgetResponseDto>;

    async fn delete_budget(&self, user_id: i64, id: i64) -> AppResult<()>;
}

#[derive(Clone)]
pub struct BudgetsService {
    repository: DynBudgetsRepository,
}

impl BudgetsService {
    pub fn new(repository: DynBudgetsRepository) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl BudgetsServiceTrait for BudgetsService {
    async fn create_budget(
        &self,
        user_id: i64,
        request: BudgetCreateDto,
    ) -> AppResult<BudgetResponseDto> {
        let name = request.name.unwrap();
        let amount = request.amount.unwrap();
        let description = request.description.unwrap_or(String::from(""));
        let frequency = request.frequency.unwrap_or(String::from("Monthly"));
        let category_id = request.category_id.unwrap();

        let created_budget = self
            .repository
            .create_budget(user_id, name, amount, description, frequency, category_id)
            .await?;

        info!("user created budget successfully");

        Ok(created_budget.into_dto())
    }

    async fn get_budget_by_id(&self, id: i64, user_id: i64) -> AppResult<BudgetResponseDto> {
        info!("searching for existing budget {:?}", id);
        let budget = self.repository.get_budget_by_id(id).await?;

        if let Some(existing_budget) = budget {
            // verify the user IDs match on the request and the budget
            if existing_budget.user_id != user_id {
                return Err(Error::Forbidden);
            }

            return Ok(existing_budget.into_dto());
        }

        Err(Error::NotFound(String::from("budget was not found")))
    }

    async fn get_budgets(&self, user_id: i64) -> AppResult<Vec<BudgetResponseDto>> {
        let budgets = self.repository.get_budgets(user_id).await?;

        self.map_to_budgets(budgets).await
    }

    async fn updated_budget(
        &self,
        id: i64,
        user_id: i64,
        request: BudgetUpdateDto,
    ) -> AppResult<BudgetResponseDto> {
        let budget_to_update = self.repository.get_budget_by_id(id).await?;

        if let Some(existing_budget) = budget_to_update {
            // verify the user IDs match on the request and the budget
            if existing_budget.user_id != user_id {
                return Err(Error::Forbidden);
            }

            let updated_name = request.name.unwrap_or(existing_budget.name);
            let updated_amount = request.amount.unwrap_or(existing_budget.amount);
            let updated_description = request.description.unwrap_or(existing_budget.description);
            let updated_frequency = request.frequency.unwrap_or(existing_budget.frequency);
            let updated_category_id = request.category_id.unwrap_or(existing_budget.category_id);

            let updated_budget = self
                .repository
                .update_budget(
                    id,
                    updated_name,
                    updated_amount,
                    updated_description,
                    updated_frequency,
                    updated_category_id,
                )
                .await?;

            return Ok(updated_budget.into_dto());
        }

        Err(Error::NotFound(String::from("budget was not found")))
    }

    async fn delete_budget(&self, user_id: i64, id: i64) -> AppResult<()> {
        let budget = self.repository.get_budget_by_id(id).await?;

        if let Some(existing_budget) = budget {
            // verify the user IDs match on the request and the budget
            if existing_budget.user_id != user_id {
                return Err(Error::Forbidden);
            }

            self.repository.delete_budget(existing_budget.id).await?;

            return Ok(());
        }

        Err(Error::NotFound(String::from("budget was not found")))
    }
}

impl BudgetsService {
    async fn map_to_budgets(&self, budgets: Vec<Budget>) -> AppResult<Vec<BudgetResponseDto>> {
        info!("found {} budgets", budgets.len());

        let mut mapped_budgets: Vec<BudgetResponseDto> = Vec::new();

        if !budgets.is_empty() {
            for budget in budgets {
                mapped_budgets.push(budget.into_dto());
            }
        }

        Ok(mapped_budgets)
    }
}
