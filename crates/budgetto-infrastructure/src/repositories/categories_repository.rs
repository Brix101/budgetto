use anyhow::Context;
use async_trait::async_trait;
use sqlx::{query, query_as};
use uuid::Uuid;

use budgetto_core::categories::repository::{CategoriesRepository, Category};

use crate::connection_pool::ConnectionPool;

#[derive(Clone)]
pub struct PostgresCategoriesRepository {
    pool: ConnectionPool,
}

impl PostgresCategoriesRepository {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CategoriesRepository for PostgresCategoriesRepository {
    async fn create_category(
        &self,
        name: String,
        note: Option<String>,
        user_id: Option<Uuid>,
    ) -> anyhow::Result<Category> {
        query_as!(
            Category,
            r#"
        insert into categories (name, note, user_id,created_at, updated_at)
        values ($1::varchar, $2, $3, current_timestamp, current_timestamp)
        returning * 
            "#,
            name,
            note,
            user_id
        )
        .fetch_one(&self.pool)
        .await
        .context("an unexpected error occured while creating the category")
    }

    async fn get_category_by_id(&self, id: Uuid) -> anyhow::Result<Option<Category>> {
        query_as!(
            Category,
            r#"
        select *
        from categories
        where id = $1 AND "deleted_at" IS NULL
            "#,
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .context("category was not found")
    }

    async fn get_categories(&self, user_id: Uuid) -> anyhow::Result<Vec<Category>> {
        query_as!(
            Category,
            r#"
        SELECT *
        FROM "categories"
        WHERE "user_id" IS NULL OR "user_id" = $1 AND "deleted_at" IS NULL
            "#,
            user_id,
        )
        .fetch_all(&self.pool)
        .await
        .context("category was not found")
    }

    async fn update_category(
        &self,
        id: Uuid,
        name: String,
        note: Option<String>,
    ) -> anyhow::Result<Category> {
        query_as!(
            Category,
            r#"
        update categories
        set
            name = $1::varchar,
            note = $2,
            updated_at = current_timestamp
        where id = $3
        returning *
            "#,
            name,
            note,
            id
        )
        .fetch_one(&self.pool)
        .await
        .context("could not update the category")
    }

    async fn delete_category(&self, id: Uuid) -> anyhow::Result<()> {
        query!(
            r#"
        update categories
        set
            deleted_at = current_timestamp,
            updated_at = current_timestamp
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
