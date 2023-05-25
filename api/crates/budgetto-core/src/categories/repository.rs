use std::sync::Arc;
use std::time::SystemTime;

use async_trait::async_trait;
use mockall::automock;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;
use uuid::{uuid, Uuid};

use budgetto_domain::categories::CategoryDto;
/// Similar to above, we want to keep a reference count across threads so we can manage our connection pool.
pub type DynCategoriesRepository = Arc<dyn CategoriesRepository + Send + Sync>;

#[automock]
#[async_trait]
pub trait CategoriesRepository {
    async fn create(&self, args: CreateCategory) -> anyhow::Result<CategoryEntity>;

    async fn find_many(&self, user_id: Uuid) -> anyhow::Result<Vec<CategoryEntity>>;

    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<CategoryEntity>>;

    async fn update(&self, args: UpdateCategory) -> anyhow::Result<CategoryEntity>;

    async fn delete(&self, id: Uuid) -> anyhow::Result<()>;
}

#[derive(Debug)]
pub struct CreateCategory {
    pub name: String,
    pub note: Option<String>,
    pub user_id: Option<Uuid>,
}

#[derive(Debug)]
pub struct UpdateCategory {
    pub id: Uuid,
    pub name: String,
    pub note: Option<String>,
}

#[derive(FromRow, Debug)]
pub struct CategoryEntity {
    pub id: Uuid,
    pub name: String,
    pub note: Option<String>,
    pub user_id: Option<Uuid>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}

impl CategoryEntity {
    pub fn into_dto(self) -> CategoryDto {
        CategoryDto {
            id: self.id,
            name: self.name,
            note: self.note,
            is_default: self.user_id.is_none(),
        }
    }
}

impl Default for CategoryEntity {
    fn default() -> Self {
        CategoryEntity {
            id: uuid!("e0edf148-717b-491c-ad58-402adf892313"),
            name: String::from("housing"),
            note: None,
            user_id: Some(uuid!("f3f898aa-ffa3-4b58-91b0-612a1c801a5e")),
            created_at: OffsetDateTime::from(SystemTime::now()),
            updated_at: OffsetDateTime::from(SystemTime::now()),
            deleted_at: None,
        }
    }
}

impl Default for CreateCategory {
    fn default() -> Self {
        CreateCategory {
            name: String::from("housing"),
            note: None,
            user_id: Some(uuid!("f3f898aa-ffa3-4b58-91b0-612a1c801a5e")),
        }
    }
}
