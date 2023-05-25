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

pub struct AccountRouter;

impl AccountRouter {
    pub fn app() -> Router {
        Router::new()
            .route("/", get(Self::get_accounts))
            .route("/", post(Self::create_account))
            .route("/:id", get(Self::get_account))
            .route("/:id", put(Self::update_account))
            .route("/:id", delete(Self::delete_account))
    }

    pub async fn get_accounts(
        query_params: Query<QueryAccount>,
        RequiredAuthentication(user): RequiredAuthentication,
        Extension(services): Extension<ServiceRegister>,
    ) -> AppResult<Json<AccountsResponse>> {
        info!("received request to get current user accounts");

        if let Some(id) = query_params.id {
            // return this function if the query params has value
            let account = services.accounts.find_by_id(id, user.id).await?;

            return Ok(Json(AccountsResponse {
                accounts: vec![account],
            }));
        }

        let accounts = services.accounts.find_many(user.id).await?;

        Ok(Json(AccountsResponse { accounts }))
    }
    pub async fn get_account(
        Path(id): Path<Uuid>,
        RequiredAuthentication(user): RequiredAuthentication,
        Extension(services): Extension<ServiceRegister>,
    ) -> AppResult<Json<AccountDto>> {
        info!("recieved request to get account {:?}", id);

        let account = services.accounts.find_by_id(id, user.id).await?;

        Ok(Json(account))
    }
    pub async fn create_account(
        RequiredAuthentication(user): RequiredAuthentication,
        Extension(services): Extension<ServiceRegister>,
        ValidationExtractor(mut request): ValidationExtractor<CreateAccountDto>,
    ) -> AppResult<Json<AccountDto>> {
        info!("received request to create account");

        request.user_id = Some(user.id);

        let new_account = services.accounts.create(request).await?;

        Ok(Json(new_account))
    }

    pub async fn update_account(
        Path(id): Path<Uuid>,
        RequiredAuthentication(user): RequiredAuthentication,
        Extension(services): Extension<ServiceRegister>,
        Json(mut request): Json<UpdateAccountDto>,
    ) -> AppResult<Json<AccountDto>> {
        info!("recieved request to update account {:?}", id);

        request.id = Some(id);
        request.user_id = Some(user.id);
        let updated_account = services.accounts.updated(request).await?;

        Ok(Json(updated_account))
    }

    pub async fn delete_account(
        Path(id): Path<Uuid>,
        RequiredAuthentication(user): RequiredAuthentication,
        Extension(services): Extension<ServiceRegister>,
    ) -> AppResult<()> {
        info!("recieved request to remove account {:?}", id);

        services.accounts.delete(id, user.id).await?;

        Ok(())
    }
}
