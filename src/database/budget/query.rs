use anyhow::Context;
use async_trait::async_trait;
use sqlx::{query, query_as};

use crate::database::Database;

use super::repository::{Budget, BudgetsRepository,};

#[async_trait]
impl BudgetsRepository for Database {
    async fn create_budget(
        &self,
        user_id: i64,
        name: String,
        amount: f64,
        description: String,
        frequency: String,
    ) -> anyhow::Result<Budget> {        
        query_as!(
            Budget,
            r#"
        insert into budgets (created_at, updated_at, name, amount, description,user_id,frequency)
        values (current_timestamp, current_timestamp, $1::varchar, $2::float, $3::varchar, $4::bigint, $5::varchar)
        returning *
            "#,
            name,
            amount,
            description,
            user_id,
            frequency 
        )
        .fetch_one(&self.pool)
        .await
        .context("an unexpected error occured while creating the budget")
    }

    async fn get_budget_by_id(&self, id: i64) -> anyhow::Result<Option<Budget>> {
        query_as!(
            Budget,
            r#"
        select *
        from budgets
        where id = $1
            "#,
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .context("budget was not found")
    }

    async fn get_budgets(&self, user_id: i64) -> anyhow::Result<Vec<Budget>> {
        query_as!(
            Budget,
            r#"
        select budgets.*
        from budgets
        inner join users on budgets.user_id=users.id
        where users.id = $1
            "#,
            user_id,
        )
        .fetch_all(&self.pool)
        .await
        .context("budget was not found")
    }

    async fn update_budget(
        &self,
        id: i64,
        name: String,
        amount: f64,
        description: Option<String>,
        frequency: String,
    ) -> anyhow::Result<Budget> {
        query_as!(
            Budget,
            r#"
        update budgets
        set
            name = $1::varchar,
            amount = $2::float,
            description = $3::varchar,
            updated_at = current_timestamp,
            frequency = $4::varchar
        where id = $5
        returning *
            "#,
            name,
            amount,
            description,
            frequency,
            id
        )
        .fetch_one(&self.pool)
        .await
        .context("could not update the budget")
    }

    async fn delete_budget(&self, id: i64) -> anyhow::Result<()> {
        query!(
            r#"
        delete from budgets
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
