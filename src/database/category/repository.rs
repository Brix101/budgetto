use std::sync::Arc;
use std::time::SystemTime;

use async_trait::async_trait;
use mockall::automock;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;

/// Similar to above, we want to keep a reference count across threads so we can manage our connection pool.
pub type DynCategoriesRepository = Arc<dyn CategoriesRepository + Send + Sync>;

#[automock]
#[async_trait]
pub trait CategoriesRepository {
    async fn create_category(&self, user_id: i64, name: String) -> anyhow::Result<Category>;

    async fn get_category_by_id(&self, id: i64) -> anyhow::Result<Option<Category>>;

    async fn get_categories(&self, user_id: i64) -> anyhow::Result<Vec<Category>>;

    async fn update_category(&self, id: i64, name: String) -> anyhow::Result<Category>;

    async fn delete_category(&self, id: i64) -> anyhow::Result<()>;
}

#[derive(FromRow, Debug)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub user_id: i64,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}

impl Default for Category {
    fn default() -> Self {
        Self {
            id: 1,
            name: String::from("stub category"),
            user_id: 1,
            created_at: OffsetDateTime::from(SystemTime::now()),
            updated_at: OffsetDateTime::from(SystemTime::now()),
            deleted_at: Some(OffsetDateTime::from(SystemTime::now())),
        }
    }
}
