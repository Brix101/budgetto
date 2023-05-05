use axum::extract::{Json, Path, Query};
use axum::routing::{delete, get, post, put};
use axum::{Extension, Router};
use budgetto_core::errors::AppResult;
use budgetto_domain::accounts::requests::{CreateAccountDto, QueryAccount, UpdateAccountDto};
use budgetto_domain::accounts::responses::AccountsResponse;
use budgetto_domain::accounts::AccountDto;
use budgetto_infrastructure::service_register::ServiceRegister;
use tracing::info;
use uuid::Uuid;

use crate::extractors::required_authentication_extractor::RequiredAuthentication;
use crate::extractors::validation_extractor::ValidationExtractor;

pub struct AccountController;

impl AccountController {
    pub fn app() -> Router {
        Router::new()
            .route("/", get(Self::get_accounts))
            .route("/", post(Self::create_account))
            .route("/:id", put(Self::update_account))
            .route("/:id", delete(Self::delete_account))
    }

    pub async fn get_accounts(
        query_params: Query<QueryAccount>,
        RequiredAuthentication(user): RequiredAuthentication,
        Extension(services): Extension<ServiceRegister>,
    ) -> AppResult<Json<AccountsResponse>> {
        info!("received request to get current user accounts");

        if let Some(id) = query_params.account_id {
            // return this function if the query params has value
            let account = services.accounts.get_account_by_id(id, user.id).await?;

            return Ok(Json(AccountsResponse {
                accounts: vec![account],
            }));
        }

        let accounts = services.accounts.get_accounts(user.id).await?;

        Ok(Json(accounts))
    }

    pub async fn create_account(
        RequiredAuthentication(user): RequiredAuthentication,
        Extension(services): Extension<ServiceRegister>,
        ValidationExtractor(request): ValidationExtractor<CreateAccountDto>,
    ) -> AppResult<Json<AccountDto>> {
        info!("received request to create account");

        let new_account = services.accounts.create_account(user.id, request).await?;

        Ok(Json(new_account))
    }

    pub async fn update_account(
        Path(id): Path<Uuid>,
        RequiredAuthentication(user): RequiredAuthentication,
        Extension(services): Extension<ServiceRegister>,
        Json(request): Json<UpdateAccountDto>,
    ) -> AppResult<Json<AccountDto>> {
        info!("recieved request to update account {:?}", id);

        let updated_account = services
            .accounts
            .updated_account(id, user.id, request)
            .await?;

        Ok(Json(updated_account))
    }

    pub async fn delete_account(
        Path(id): Path<Uuid>,
        RequiredAuthentication(user): RequiredAuthentication,
        Extension(services): Extension<ServiceRegister>,
    ) -> AppResult<()> {
        info!("recieved request to remove account {:?}", id);

        services.categories.delete_category(id, user.id).await?;

        Ok(())
    }
}
