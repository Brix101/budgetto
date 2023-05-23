use axum::extract::{Json, Path, Query};
use axum::routing::{delete, get, post, put};
use axum::{Extension, Router};
use tracing::info;
use uuid::Uuid;

use budgetto_core::errors::AppResult;
use budgetto_domain::transactions::requests::{
    CreateTransactionDto, QueryTransaction, UpdateTransactionDto,
};
use budgetto_domain::transactions::responses::TransactionsResponse;
use budgetto_domain::transactions::TransactionDto;
use budgetto_infrastructure::service_register::ServiceRegister;

use crate::extractors::required_authentication_extractor::RequiredAuthentication;
use crate::extractors::validation_extractor::ValidationExtractor;

pub struct TransactionController;

impl TransactionController {
    pub fn app() -> Router {
        Router::new()
            .route("/", get(Self::get_transactions))
            .route("/", post(Self::create_transaction))
            .route("/:id", get(Self::get_transaction))
            .route("/:id", put(Self::update_transaction))
            .route("/:id", delete(Self::delete_transaction))
    }

    pub async fn get_transactions(
        query_params: Query<QueryTransaction>,
        RequiredAuthentication(user): RequiredAuthentication,
        Extension(services): Extension<ServiceRegister>,
    ) -> AppResult<Json<TransactionsResponse>> {
        info!("received request to get current user transactions");

        if let Some(id) = query_params.id {
            // return this function if the query params has value
            let transaction = services
                .transactions
                .get_transaction_by_id(id, user.id)
                .await?;

            return Ok(Json(TransactionsResponse {
                transactions: vec![transaction],
            }));
        }

        let transactions = services.transactions.get_transactions(user.id).await?;

        Ok(Json(transactions))
    }
    pub async fn get_transaction(
        Path(id): Path<Uuid>,
        RequiredAuthentication(user): RequiredAuthentication,
        Extension(services): Extension<ServiceRegister>,
    ) -> AppResult<Json<TransactionDto>> {
        info!("recieved request to update transaction {:?}", id);

        let transaction = services
            .transactions
            .get_transaction_by_id(id, user.id)
            .await?;

        Ok(Json(transaction))
    }

    pub async fn create_transaction(
        RequiredAuthentication(user): RequiredAuthentication,
        Extension(services): Extension<ServiceRegister>,
        ValidationExtractor(request): ValidationExtractor<CreateTransactionDto>,
    ) -> AppResult<Json<TransactionDto>> {
        info!("received request to create transaction");

        let new_transaction = services
            .transactions
            .create_transaction(user.id, request)
            .await?;

        Ok(Json(new_transaction))
    }

    pub async fn update_transaction(
        Path(id): Path<Uuid>,
        RequiredAuthentication(user): RequiredAuthentication,
        Extension(services): Extension<ServiceRegister>,
        Json(request): Json<UpdateTransactionDto>,
    ) -> AppResult<Json<TransactionDto>> {
        info!("recieved request to update transaction {:?}", id);

        let updated_transaction = services
            .transactions
            .updated_transaction(id, user.id, request)
            .await?;

        Ok(Json(updated_transaction))
    }

    pub async fn delete_transaction(
        Path(id): Path<Uuid>,
        RequiredAuthentication(user): RequiredAuthentication,
        Extension(services): Extension<ServiceRegister>,
    ) -> AppResult<()> {
        info!("recieved request to remove transaction {:?}", id);

        services
            .transactions
            .delete_transaction(id, user.id)
            .await?;

        Ok(())
    }
}
