use anyhow::Context;
use async_trait::async_trait;
use sqlx::{query, query_as};

use crate::database::Database;

use super::repository::{Expense, ExpensesRepository};

#[async_trait]
impl ExpensesRepository for Database {
    async fn create_expense(
        &self,
        amount: f64,
        description: String,
        category_id: i64,
        user_id: i64,
    ) -> anyhow::Result<Expense> {
        query_as!(
            Expense,
            r#"
        insert into expenses (created_at, updated_at, amount, description, category_id, user_id)
        values (current_timestamp, current_timestamp, $1::float, $2::varchar, $3::bigint, $4::bigint)
        returning *
            "#,
            amount,
            description,
            category_id,
            user_id
        )
        .fetch_one(&self.pool)
        .await
        .context("an unexpected error occured while creating the expense")
    }

    async fn get_expense_by_id(&self, id: i64) -> anyhow::Result<Option<Expense>> {
        query_as!(
            Expense,
            r#"
        select *
        from expenses
        where id = $1
            "#,
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .context("expense was not found")
    }

    async fn get_expenses(&self, user_id: i64) -> anyhow::Result<Vec<Expense>> {
        query_as!(
            Expense,
            r#"
        select expenses.*
        from expenses
        inner join users on expenses.user_id=users.id
        where users.id = $1
            "#,
            user_id,
        )
        .fetch_all(&self.pool)
        .await
        .context("expense was not found")
    }

    async fn update_expense(
        &self,
        id: i64,
        amount: f64,
        description: String,
        category_id: i64,
    ) -> anyhow::Result<Expense> {
        query_as!(
            Expense,
            r#"
        update expenses
        set
            amount = $1::float,
            description = $2::varchar,
            category_id = $3::bigint
        where id = $4
        returning *
            "#,
            amount,
            description,
            category_id,
            id
        )
        .fetch_one(&self.pool)
        .await
        .context("could not update the expense")
    }

    async fn delete_expense(&self, id: i64) -> anyhow::Result<()> {
        query!(
            r#"
        delete from expenses
        where id = $1
        "#,
            id
        )
        .execute(&self.pool)
        .await
        .context("an unexpected error occurred deleting expense")?;

        Ok(())
    }
}
