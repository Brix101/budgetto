use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;
use uuid::Uuid;

use budgetto_domain::categories::CategoryDto;
/// Similar to above, we want to keep a reference count across threads so we can manage our connection pool.
pub type DynCategoriesRepository = Arc<dyn CategoriesRepository + Send + Sync>;

#[automock]
#[async_trait]
pub trait CategoriesRepository {
    async fn create_category(
        &self,
        name: String,
        note: Option<String>,
        user_id: Option<Uuid>,
    ) -> anyhow::Result<Category>;

    async fn get_categories(&self, user_id: Uuid) -> anyhow::Result<Vec<Category>>;

    async fn get_category_by_id(&self, id: Uuid) -> anyhow::Result<Option<Category>>;

    async fn update_category(
        &self,
        id: Uuid,
        name: String,
        note: Option<String>,
    ) -> anyhow::Result<Category>;

    async fn delete_category(&self, id: Uuid) -> anyhow::Result<()>;
}

#[derive(FromRow, Debug)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub note: Option<String>,
    pub user_id: Option<Uuid>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}

impl Category {
    pub fn into_dto(self) -> CategoryDto {
        CategoryDto {
            id: self.id,
            name: self.name,
            note: self.note,
        }
    }
}
