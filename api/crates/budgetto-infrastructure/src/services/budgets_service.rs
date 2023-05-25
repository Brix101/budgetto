use async_trait::async_trait;
use tracing::info;
use uuid::Uuid;

use budgetto_core::{
    budgets::{
        repository::{Budget, CreateBudget, DynBudgetsRepository, UpdateBudget},
        service::BudgetsService,
    },
    errors::{AppResult, Error},
};
use budgetto_domain::{
    budgets::requests::{CreateBudgetDto, UpdateBudgetDto},
    budgets::BudgetDto,
};

#[derive(Clone)]
pub struct BudgettoBudgetsService {
    repository: DynBudgetsRepository,
}

impl BudgettoBudgetsService {
    pub fn new(repository: DynBudgetsRepository) -> Self {
        Self { repository }
    }
    async fn map_to_budgets(&self, budgets: Vec<Budget>) -> AppResult<Vec<BudgetDto>> {
        info!("found {} budgets", budgets.len());

        let mut mapped_budgets: Vec<BudgetDto> = Vec::new();

        if !budgets.is_empty() {
            for budget in budgets {
                mapped_budgets.push(budget.into_dto());
            }
        }

        Ok(mapped_budgets)
    }
}

#[async_trait]
impl BudgetsService for BudgettoBudgetsService {
    async fn create(&self, args: CreateBudgetDto) -> AppResult<BudgetDto> {
        let amount = args.amount.unwrap();
        let category_id = args.category_id.unwrap();
        let user_id = args.user_id.unwrap();

        let created_budget = self
            .repository
            .create(CreateBudget {
                amount,
                category_id,
                user_id,
            })
            .await?;

        info!("user {:?} created budget successfully", user_id);

        Ok(created_budget.into_dto())
    }

    async fn find_by_id(&self, id: Uuid, user_id: Uuid) -> AppResult<BudgetDto> {
        info!("searching for existing budget {:?}", id);
        let budget = self.repository.find_by_id(id).await?;

        if let Some(existing_budget) = budget {
            // verify the user IDs match on the request and the budget
            if existing_budget.user_id != user_id {
                return Err(Error::Forbidden);
            }
            return Ok(existing_budget.into_dto());
        }

        Err(Error::NotFound(String::from("budget was not found")))
    }

    async fn find_many(&self, user_id: Uuid) -> AppResult<Vec<BudgetDto>> {
        info!("searching budgets for user {:?}", user_id);
        let budgets = self.repository.find_many(user_id).await?;

        self.map_to_budgets(budgets).await
    }

    async fn updated(&self, request: UpdateBudgetDto) -> AppResult<BudgetDto> {
        let id = request.id.unwrap();
        let user_id = request.user_id.unwrap();
        let budget_to_update = self.repository.find_by_id(id).await?;

        if let Some(existing_budget) = budget_to_update {
            // verify the user IDs match on the request and the budget
            if existing_budget.user_id != user_id {
                return Err(Error::Forbidden);
            }

            let updated_amount = request.amount.unwrap_or(existing_budget.amount);
            let updated_category_id = request.category_id.unwrap_or(existing_budget.category_id);

            info!("updating budget {:?} for user {:?}", id, user_id);
            let updated_budget = self
                .repository
                .update(UpdateBudget {
                    id,
                    amount: updated_amount,
                    category_id: updated_category_id,
                })
                .await?;

            return Ok(updated_budget.into_dto());
        }

        Err(Error::NotFound(String::from("budget was not found")))
    }

    async fn delete(&self, id: Uuid, user_id: Uuid) -> AppResult<()> {
        let budget = self.repository.find_by_id(id).await?;

        if let Some(existing_budget) = budget {
            // verify the user IDs match on the request and the budget
            if existing_budget.user_id != user_id {
                return Err(Error::Forbidden);
            }

            self.repository.delete(existing_budget.id).await?;

            return Ok(());
        }

        Err(Error::NotFound(String::from("budget was not found")))
    }
}
