use axum::extract::{Json, Path, Query};
use axum::routing::{delete, get, post, put};
use axum::{Extension, Router};
use budgetto_core::errors::AppResult;
use budgetto_domain::budgets::requests::{CreateBudgetDto, QueryBudget, UpdateBudgetDto};
use budgetto_domain::budgets::responses::BudgetsResponse;
use budgetto_domain::budgets::BudgetDto;
use budgetto_infrastructure::service_register::ServiceRegister;
use tracing::info;
use uuid::Uuid;

use crate::extractors::required_authentication_extractor::RequiredAuthentication;
use crate::extractors::validation_extractor::ValidationExtractor;

pub struct BudgetRouter;

impl BudgetRouter {
    pub fn app() -> Router {
        Router::new()
            .route("/", get(Self::get_budgets))
            .route("/", post(Self::create_budget))
            .route("/:id", get(Self::get_budget))
            .route("/:id", put(Self::update_budget))
            .route("/:id", delete(Self::delete_budget))
    }

    pub async fn get_budgets(
        query_params: Query<QueryBudget>,
        RequiredAuthentication(user): RequiredAuthentication,
        Extension(services): Extension<ServiceRegister>,
    ) -> AppResult<Json<BudgetsResponse>> {
        info!("received request to get current user budgets");

        if let Some(id) = query_params.id {
            // return this function if the query params has value
            let budget = services.budgets.find_by_id(id, user.id).await?;

            return Ok(Json(BudgetsResponse {
                budgets: vec![budget],
            }));
        }

        let budgets = services.budgets.find_many(user.id).await?;

        Ok(Json(BudgetsResponse { budgets }))
    }
    pub async fn get_budget(
        Path(id): Path<Uuid>,
        RequiredAuthentication(user): RequiredAuthentication,
        Extension(services): Extension<ServiceRegister>,
    ) -> AppResult<Json<BudgetDto>> {
        info!("recieved request to get budget {:?}", id);

        let budget = services.budgets.find_by_id(id, user.id).await?;

        Ok(Json(budget))
    }
    pub async fn create_budget(
        RequiredAuthentication(user): RequiredAuthentication,
        Extension(services): Extension<ServiceRegister>,
        ValidationExtractor(mut request): ValidationExtractor<CreateBudgetDto>,
    ) -> AppResult<Json<BudgetDto>> {
        info!("received request to create budget");

        request.user_id = Some(user.id);

        let new_budget = services.budgets.create(request).await?;

        Ok(Json(new_budget))
    }

    pub async fn update_budget(
        Path(id): Path<Uuid>,
        RequiredAuthentication(user): RequiredAuthentication,
        Extension(services): Extension<ServiceRegister>,
        Json(mut request): Json<UpdateBudgetDto>,
    ) -> AppResult<Json<BudgetDto>> {
        info!("recieved request to update budget {:?}", id);

        request.id = Some(id);
        request.user_id = Some(user.id);
        let updated_budget = services.budgets.updated(request).await?;

        Ok(Json(updated_budget))
    }

    pub async fn delete_budget(
        Path(id): Path<Uuid>,
        RequiredAuthentication(user): RequiredAuthentication,
        Extension(services): Extension<ServiceRegister>,
    ) -> AppResult<()> {
        info!("recieved request to remove budget {:?}", id);

        services.budgets.delete(id, user.id).await?;

        Ok(())
    }
}
