use anyhow::Context;
use async_trait::async_trait;
use sqlx::{query, query_as};
use uuid::Uuid;

use budgetto_core::accounts::repository::{Account, AccountsRepository};

use crate::connection_pool::ConnectionPool;

#[derive(Clone)]
pub struct PostgresAccountsRepository {
    pool: ConnectionPool,
}

impl PostgresAccountsRepository {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AccountsRepository for PostgresAccountsRepository {
    async fn create(
        &self,
        name: String,
        balance: f64,
        note: Option<String>,
        user_id: Uuid,
    ) -> anyhow::Result<Account> {
        query_as!(
            Account,
            r#"
        INSERT INTO "accounts" ("name", "balance", "note", "user_id", "created_at", "updated_at")
        VALUES ($1, $2, $3, $4, current_timestamp, current_timestamp)
        RETURNING * 
            "#,
            name,
            balance,
            note,
            user_id
        )
        .fetch_one(&self.pool)
        .await
        .context("an unexpected error occured while creating the account")
    }

    async fn find_many(&self, user_id: Uuid) -> anyhow::Result<Vec<Account>> {
        query_as!(
            Account,
            r#"
        SELECT *
        FROM "accounts"
        WHERE "deleted_at" IS NULL AND "user_id" = $1
        ORDER BY name ASC
            "#,
            user_id,
        )
        .fetch_all(&self.pool)
        .await
        .context("account was not found")
    }

    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Account>> {
        query_as!(
            Account,
            r#"
        SELECT * 
        FROM "accounts" 
        WHERE "id" = $1 
        AND "deleted_at" IS NULL
            "#,
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .context("account was not found")
    }

    async fn update(
        &self,
        id: Uuid,
        name: String,
        balance: f64,
        note: Option<String>,
    ) -> anyhow::Result<Account> {
        query_as!(
            Account,
            r#"
        update accounts
        set
            name = $1,
            balance = $2,
            note = $3,
            updated_at = current_timestamp
        where id = $4
        returning *
            "#,
            name,
            balance,
            note,
            id
        )
        .fetch_one(&self.pool)
        .await
        .context("could not update the account")
    }

    async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        query!(
            r#"
        update accounts
        set
            deleted_at = current_timestamp,
            updated_at = current_timestamp
        where id = $1
        "#,
            id
        )
        .execute(&self.pool)
        .await
        .context("an unexpected error occurred deleting account")?;

        Ok(())
    }
}
