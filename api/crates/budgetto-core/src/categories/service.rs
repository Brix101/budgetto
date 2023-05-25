use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;

use budgetto_domain::categories::{
    requests::{CreateCategoryDto, UpdateCategoryDto},
    CategoryDto,
};

use crate::errors::AppResult;

pub type DynCategoriesService = Arc<dyn CategoriesService + Send + Sync>;

#[automock]
#[async_trait]
pub trait CategoriesService {
    async fn create(
        &self,
        user_id: Option<Uuid>,
        request: CreateCategoryDto,
    ) -> AppResult<CategoryDto>;

    async fn find_by_id(&self, id: Uuid, user_id: Uuid) -> AppResult<CategoryDto>;

    async fn find_many(&self, user_id: Uuid) -> AppResult<Vec<CategoryDto>>;

    async fn updated(
        &self,
        id: Uuid,
        user_id: Uuid,
        request: UpdateCategoryDto,
    ) -> AppResult<CategoryDto>;

    async fn delete(&self, id: Uuid, user_id: Uuid) -> AppResult<()>;
}
