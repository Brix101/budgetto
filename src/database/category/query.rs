use anyhow::Context;
use async_trait::async_trait;
use sqlx::{query, query_as};

use crate::database::Database;

use super::repository::{CategoriesRepository, Category};

#[async_trait]
impl CategoriesRepository for Database {
    async fn create_category(&self, user_id: i64, name: String) -> anyhow::Result<Category> {
        query_as!(
            Category,
            r#"
        insert into categories (created_at, updated_at, name, user_id)
        values (current_timestamp, current_timestamp, $1, $2)
        returning *
            "#,
            name,
            user_id
        )
        .fetch_one(&self.pool)
        .await
        .context("an unexpected error occured while creating the category")
    }

    async fn get_category_by_id(&self, id: i64) -> anyhow::Result<Option<Category>> {
        query_as!(
            Category,
            r#"
        select *
        from categories
        where id = $1
            "#,
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .context("category was not found")
    }

    async fn get_categories(&self, user_id: i64) -> anyhow::Result<Vec<Category>> {
        query_as!(
            Category,
            r#"
        select categories.*
        from categories
        inner join users on categories.user_id=users.id
        where users.id = $1
            "#,
            user_id,
        )
        .fetch_all(&self.pool)
        .await
        .context("category was not found")
    }

    async fn update_category(&self, id: i64, name: String) -> anyhow::Result<Category> {
        query_as!(
            Category,
            r#"
        update categories
        set
            name = $1
        where id = $2
        returning *
            "#,
            name,
            id
        )
        .fetch_one(&self.pool)
        .await
        .context("could not update the category")
    }

    async fn delete_category(&self, id: i64) -> anyhow::Result<()> {
        query!(
            r#"
        delete from categories
        where id = $1
        "#,
            id
        )
        .execute(&self.pool)
        .await
        .context("an unexpected error occurred deleting category")?;

        Ok(())
    }
}
