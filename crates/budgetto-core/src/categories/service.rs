use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;

// use domain::categorys::requests::{SignInUserDto, SignUpUserDto, UpdateUserDto};
use budgetto_domain::categories::{
    requests::{CreateCategoryDto, UpdateCategoryDto},
    responses::CategoriesResponse,
    CategoryDto,
};

use crate::errors::AppResult;

pub type DynCategoriesService = Arc<dyn CategoriesService + Send + Sync>;

#[automock]
#[async_trait]
pub trait CategoriesService {
    async fn create_category(
        &self,
        user_id: Option<Uuid>,
        request: CreateCategoryDto,
    ) -> AppResult<CategoryDto>;

    async fn get_category_by_id(&self, id: Uuid, user_id: Uuid) -> AppResult<CategoryDto>;

    async fn get_categories(&self, user_id: Uuid) -> AppResult<CategoriesResponse>;

    async fn updated_category(
        &self,
        id: Uuid,
        user_id: Uuid,
        request: UpdateCategoryDto,
    ) -> AppResult<CategoryDto>;

    async fn delete_category(&self, id: Uuid, user_id: Uuid) -> AppResult<()>;
}
