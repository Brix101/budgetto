use async_trait::async_trait;
use tracing::info;
use uuid::Uuid;

use budgetto_core::{
    errors::{AppResult, Error},
    transactions::{
        repository::{
            CreateTransaction, DynTransactionsRepository, Transaction, TransactionType,
            UpdateTransaction,
        },
        service::TransactionsService,
    },
};
use budgetto_domain::transactions::{
    requests::{CreateTransactionDto, UpdateTransactionDto},
    responses::TransactionsResponse,
    TransactionDto,
};

#[derive(Clone)]
pub struct BudgettoTransactionsService {
    repository: DynTransactionsRepository,
}

impl BudgettoTransactionsService {
    pub fn new(repository: DynTransactionsRepository) -> Self {
        Self { repository }
    }
    async fn map_to_transactions(
        &self,
        categories: Vec<Transaction>,
    ) -> AppResult<TransactionsResponse> {
        info!("found {} categories", categories.len());

        let mut mapped_transactions: Vec<TransactionDto> = Vec::new();

        if !categories.is_empty() {
            for transaction in categories {
                mapped_transactions.push(transaction.into_dto());
            }
        }

        Ok(TransactionsResponse {
            transactions: mapped_transactions,
        })
    }
}

#[async_trait]
impl TransactionsService for BudgettoTransactionsService {
    async fn create_transaction(
        &self,
        user_id: Uuid,
        transaction_type: TransactionType,
        request: CreateTransactionDto,
    ) -> AppResult<TransactionDto> {
        let amount = request.amount.unwrap();
        let note = request.note;
        let category_id = request.category_id.unwrap();
        let account_id = request.account_id.unwrap();

        let created_transaction = self
            .repository
            .create_transaction(CreateTransaction {
                amount,
                note,
                transaction_type,
                category_id,
                account_id,
                user_id,
            })
            .await?;

        info!("user {:?} created transaction successfully", user_id);

        Ok(created_transaction.into_dto())
    }

    async fn get_transaction_by_id(&self, id: Uuid, user_id: Uuid) -> AppResult<TransactionDto> {
        let transaction = self.repository.get_transaction_by_id(id).await?;

        if let Some(existing_transaction) = transaction {
            // verify the user IDs match on the request and the transaction
            if existing_transaction.user_id != user_id {
                return Err(Error::Forbidden);
            }

            info!("retrieving transaction {:?} for user {:?}", id, user_id);
            return Ok(existing_transaction.into_dto());
        }

        Err(Error::NotFound(String::from("transaction was not found")))
    }

    async fn get_transactions(&self, user_id: Uuid) -> AppResult<TransactionsResponse> {
        info!("retrieving transactions for user {:?}", user_id);
        let transactions = self.repository.get_transactions(user_id).await?;

        self.map_to_transactions(transactions).await
    }

    async fn updated_transaction(
        &self,
        id: Uuid,
        user_id: Uuid,
        transaction_type: Option<TransactionType>,
        request: UpdateTransactionDto,
    ) -> AppResult<TransactionDto> {
        let transaction_to_update = self.repository.get_transaction_by_id(id).await?;

        if let Some(existing_transaction) = transaction_to_update {
            // verify the user IDs match on the request and the transaction
            if existing_transaction.user_id != user_id {
                return Err(Error::Forbidden);
            }

            let updated_amount = request.amount.unwrap_or(existing_transaction.amount);
            let updated_note = request.note.unwrap_or(existing_transaction.note.unwrap());
            let updated_category_id = request
                .category_id
                .unwrap_or(existing_transaction.category_id);
            let updated_account_id = request
                .account_id
                .unwrap_or(existing_transaction.account_id);
            let updated_transaction_type =
                transaction_type.unwrap_or(existing_transaction.transaction_type);

            info!("updating transaction {:?} for user {:?}", id, user_id);
            let updated_transaction = self
                .repository
                .update_transaction(UpdateTransaction {
                    id: existing_transaction.id,
                    amount: updated_amount,
                    note: Some(updated_note),
                    category_id: updated_category_id,
                    account_id: updated_account_id,
                    transaction_type: updated_transaction_type,
                })
                .await?;

            return Ok(updated_transaction.into_dto());
        }

        Err(Error::NotFound(String::from("transaction was not found")))
    }

    async fn delete_transaction(&self, id: Uuid, user_id: Uuid) -> AppResult<()> {
        let transaction = self.repository.get_transaction_by_id(id).await?;

        if let Some(existing_transaction) = transaction {
            // verify the user IDs match on the request and the transaction
            if existing_transaction.user_id != user_id {
                return Err(Error::Forbidden);
            }

            info!("deleting transaction {:?} for user {:?}", id, user_id);
            self.repository
                .delete_transaction(existing_transaction.id)
                .await?;

            return Ok(());
        }

        Err(Error::NotFound(String::from("transaction was not found")))
    }
}
