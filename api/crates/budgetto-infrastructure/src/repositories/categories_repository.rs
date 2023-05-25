use anyhow::Context;
use async_trait::async_trait;
use sqlx::{query, query_as};
use uuid::Uuid;

use budgetto_core::categories::repository::{
    CategoriesRepository, Category, CreateCategory, UpdateCategory,
};

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
    async fn create(&self, args: CreateCategory) -> anyhow::Result<Category> {
        query_as!(
            Category,
            r#"
        insert into categories (name, note, user_id,created_at, updated_at)
        values ($1::varchar, $2, $3, current_timestamp, current_timestamp)
        returning * 
            "#,
            args.name,
            args.note,
            args.user_id
        )
        .fetch_one(&self.pool)
        .await
        .context("an unexpected error occured while creating the category")
    }

    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Category>> {
        query_as!(
            Category,
            r#"
        SELECT * 
        FROM "categories" 
        WHERE "id" = $1 
        AND "deleted_at" IS NULL
            "#,
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .context("category was not found")
    }

    async fn find_many(&self, user_id: Uuid) -> anyhow::Result<Vec<Category>> {
        query_as!(
            Category,
            r#"
        SELECT *
        FROM "categories"
        WHERE "user_id" IS NULL OR "user_id" = $1 AND "deleted_at" IS NULL
        ORDER BY name ASC
            "#,
            user_id,
        )
        .fetch_all(&self.pool)
        .await
        .context("category was not found")
    }

    async fn update(&self, args: UpdateCategory) -> anyhow::Result<Category> {
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
            args.name,
            args.note,
            args.id
        )
        .fetch_one(&self.pool)
        .await
        .context("could not update the category")
    }

    async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
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
