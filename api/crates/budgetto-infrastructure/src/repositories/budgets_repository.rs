use anyhow::Context;
use async_trait::async_trait;
use sqlx::{query, query_as};
use uuid::Uuid;

use budgetto_core::budgets::repository::{Budget, BudgetsRepository, CreateBudget, UpdateBudget};

use crate::connection_pool::ConnectionPool;

#[derive(Clone)]
pub struct PostgresBudgetsRepository {
    pool: ConnectionPool,
}

impl PostgresBudgetsRepository {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BudgetsRepository for PostgresBudgetsRepository {
    async fn create(&self, args: CreateBudget) -> anyhow::Result<Budget> {
        query_as!(
            Budget,
            r#"
        INSERT INTO "budgets" ("amount", "category_id", "user_id", "created_at", "updated_at", "deleted_at")
        VALUES ($1, $2, $3, current_timestamp, current_timestamp, NULL)
        RETURNING *
            "#,
            args.amount,
            args.category_id,
            args.user_id
        )
        .fetch_one(&self.pool)
        .await
        .context("an unexpected error occured while creating the budget")
    }

    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Budget>> {
        query_as!(
            Budget,
            r#"
        SELECT *
        FROM "budgets" 
        WHERE "id" = $1 
        AND "deleted_at" IS NULL
            "#,
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .context("budget was not found")
    }

    async fn find_many(&self, user_id: Uuid) -> anyhow::Result<Vec<Budget>> {
        query_as!(
            Budget,
            r#"
        SELECT *
        FROM "budgets"
        WHERE "user_id" IS NULL OR "user_id" = $1 AND "deleted_at" IS NULL
        ORDER BY created_at ASC
            "#,
            user_id,
        )
        .fetch_all(&self.pool)
        .await
        .context("transaction was not found")
    }

    async fn update(&self, args: UpdateBudget) -> anyhow::Result<Budget> {
        query_as!(
            Budget,
            r#"
        update budgets
        set
            amount = $1,
            category_id = $2,
            updated_at = current_timestamp
        where id = $3
        RETURNING * 
            "#,
            args.amount,
            args.category_id,
            args.id
        )
        .fetch_one(&self.pool)
        .await
        .context("could not update the budget")
    }

    async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        query!(
            r#"
        update budgets
        set
            deleted_at = current_timestamp,
            updated_at = current_timestamp
        where id = $1
        "#,
            id
        )
        .execute(&self.pool)
        .await
        .context("an unexpected error occurred deleting budget")?;

        Ok(())
    }
}
