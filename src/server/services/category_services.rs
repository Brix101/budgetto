use std::sync::Arc;
use tracing::info;

use async_trait::async_trait;

use crate::{
    database::category::repository::{Category, DynCategoriesRepository},
    server::{
        dtos::category_dto::{CategoryCreateDto, CategoryResponseDto, CategoryUpdateDto},
        error::{AppResult, Error},
    },
};

pub type DynCategoriesService = Arc<dyn CategoriesServiceTrait + Send + Sync>;

#[async_trait]
pub trait CategoriesServiceTrait {
    async fn create_category(
        &self,
        user_id: i64,
        request: CategoryCreateDto,
    ) -> AppResult<CategoryResponseDto>;

    async fn get_category_by_id(&self, id: i64, user_id: i64) -> AppResult<CategoryResponseDto>;

    async fn get_categories(&self, user_id: i64) -> AppResult<Vec<CategoryResponseDto>>;

    async fn updated_category(
        &self,
        id: i64,
        user_id: i64,
        request: CategoryUpdateDto,
    ) -> AppResult<CategoryResponseDto>;

    async fn delete_category(&self, user_id: i64, id: i64) -> AppResult<()>;
}

#[derive(Clone)]
pub struct CategoriesService {
    repository: DynCategoriesRepository,
}

impl CategoriesService {
    pub fn new(repository: DynCategoriesRepository) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl CategoriesServiceTrait for CategoriesService {
    async fn create_category(
        &self,
        user_id: i64,
        request: CategoryCreateDto,
    ) -> AppResult<CategoryResponseDto> {
        let name = request.name.unwrap();

        let created_category = self.repository.create_category(user_id, name).await?;

        info!("user created category successfully");

        Ok(created_category.into_dto())
    }

    async fn get_category_by_id(&self, id: i64, user_id: i64) -> AppResult<CategoryResponseDto> {
        info!("searching for existing category {:?}", id);
        let category = self.repository.get_category_by_id(id).await?;

        if let Some(existing_category) = category {
            // verify the user IDs match on the request and the category
            if existing_category.user_id != user_id {
                return Err(Error::Forbidden);
            }

            return Ok(existing_category.into_dto());
        }

        Err(Error::NotFound(String::from("category was not found")))
    }

    async fn get_categories(&self, user_id: i64) -> AppResult<Vec<CategoryResponseDto>> {
        let categories = self.repository.get_categories(user_id).await?;

        self.map_to_categories(categories).await
    }

    async fn updated_category(
        &self,
        id: i64,
        user_id: i64,
        request: CategoryUpdateDto,
    ) -> AppResult<CategoryResponseDto> {
        let category_to_update = self.repository.get_category_by_id(id).await?;

        if let Some(existing_category) = category_to_update {
            // verify the user IDs match on the request and the category
            if existing_category.user_id != user_id {
                return Err(Error::Forbidden);
            }

            let updated_name = request.name.unwrap_or(existing_category.name);

            let updated_category = self.repository.update_category(id, updated_name).await?;

            return Ok(updated_category.into_dto());
        }

        Err(Error::NotFound(String::from("category was not found")))
    }

    async fn delete_category(&self, user_id: i64, id: i64) -> AppResult<()> {
        let category = self.repository.get_category_by_id(id).await?;

        if let Some(existing_category) = category {
            // verify the user IDs match on the request and the category
            if existing_category.user_id != user_id {
                return Err(Error::Forbidden);
            }

            self.repository
                .delete_category(existing_category.id)
                .await?;

            return Ok(());
        }

        Err(Error::NotFound(String::from("category was not found")))
    }
}

impl CategoriesService {
    async fn map_to_categories(
        &self,
        categorys: Vec<Category>,
    ) -> AppResult<Vec<CategoryResponseDto>> {
        info!("found {} categorys", categorys.len());

        let mut mapped_categories: Vec<CategoryResponseDto> = Vec::new();

        if !categorys.is_empty() {
            for category in categorys {
                mapped_categories.push(category.into_dto());
            }
        }

        Ok(mapped_categories)
    }
}
