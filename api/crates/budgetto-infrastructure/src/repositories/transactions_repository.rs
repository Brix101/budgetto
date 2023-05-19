use anyhow::Context;
use async_trait::async_trait;
use sqlx::{query, query_as};
use uuid::Uuid;

use budgetto_core::transactions::repository::UpdateTransaction;
use budgetto_core::transactions::repository::{
    CreateTransaction, Transaction, TransactionsRepository,
};
use budgetto_domain::transactions::TransactionType;

use crate::connection_pool::ConnectionPool;

#[derive(Clone)]
pub struct PostgresTransactionsRepository {
    pool: ConnectionPool,
}

impl PostgresTransactionsRepository {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TransactionsRepository for PostgresTransactionsRepository {
    async fn create_transaction(&self, request: CreateTransaction) -> anyhow::Result<Transaction> {
        query_as!(
            Transaction,
            r#"
        INSERT INTO "transactions" ("amount", "note", "transaction_type", "account_id", "category_id", "user_id", "created_at", "updated_at", "deleted_at")
        VALUES ($1, $2, $3, $4, $5, $6, current_timestamp, current_timestamp, NULL)
        RETURNING id, amount, note, transaction_type as "transaction_type: TransactionType", account_id, category_id, user_id, created_at, updated_at, deleted_at
            "#,
            request.amount,
            request.note,
            request.transaction_type as _,
            request.account_id,
            request.category_id,
            request.user_id
        )
        .fetch_one(&self.pool)
        .await
        .context("an unexpected error occured while creating the transaction")
    }

    async fn get_transaction_by_id(&self, id: Uuid) -> anyhow::Result<Option<Transaction>> {
        query_as!(
            Transaction,
            r#"
        SELECT id, amount, note, transaction_type as "transaction_type: TransactionType", account_id, category_id, user_id, created_at, updated_at, deleted_at
        FROM "transactions" 
        WHERE "id" = $1 
        AND "deleted_at" IS NULL
            "#,
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .context("transaction was not found")
    }

    async fn get_transactions(&self, user_id: Uuid) -> anyhow::Result<Vec<Transaction>> {
        query_as!(
            Transaction,
            r#"
        SELECT id, amount, note, transaction_type as "transaction_type: TransactionType", account_id, category_id, user_id, created_at, updated_at, deleted_at
        FROM "transactions"
        WHERE "user_id" IS NULL OR "user_id" = $1 AND "deleted_at" IS NULL
        ORDER BY created_at ASC
            "#,
            user_id,
        )
        .fetch_all(&self.pool)
        .await
        .context("transaction was not found")
    }

    async fn update_transaction(&self, request: UpdateTransaction) -> anyhow::Result<Transaction> {
        query_as!(
            Transaction,
            r#"
        update transactions
        set
            amount = $1,
            note = $2,
            transaction_type = $3,
            account_id = $4,
            category_id = $5,
            updated_at = current_timestamp
        where id = $6
        RETURNING id, amount, note, transaction_type as "transaction_type: TransactionType", account_id, category_id, user_id, created_at, updated_at, deleted_at
            "#,
            request.amount,
            request.note,
            request.transaction_type as _,
            request.account_id,
            request.category_id,
            request.id
        )
        .fetch_one(&self.pool)
        .await
        .context("could not update the transaction")
    }

    async fn delete_transaction(&self, id: Uuid) -> anyhow::Result<()> {
        query!(
            r#"
        update transactions
        set
            deleted_at = current_timestamp,
            updated_at = current_timestamp
        where id = $1
        "#,
            id
        )
        .execute(&self.pool)
        .await
        .context("an unexpected error occurred deleting transaction")?;

        Ok(())
    }
}
