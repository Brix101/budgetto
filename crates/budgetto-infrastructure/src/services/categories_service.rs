use async_trait::async_trait;
use tracing::info;
use uuid::Uuid;

use budgetto_core::{
    categories::{
        repository::{Category, DynCategoriesRepository},
        service::CategoriesService,
    },
    errors::{AppResult, Error},
};
use budgetto_domain::categories::{
    requests::{CreateCategoryDto, UpdateCategoryDto},
    responses::CategoriesResponse,
    CategoryDto,
};

#[derive(Clone)]
pub struct BudgettoCategoriesService {
    repository: DynCategoriesRepository,
}

impl BudgettoCategoriesService {
    pub fn new(repository: DynCategoriesRepository) -> Self {
        Self { repository }
    }
    async fn map_to_categories(&self, categories: Vec<Category>) -> AppResult<CategoriesResponse> {
        info!("found {} categories", categories.len());

        let mut mapped_categories: Vec<CategoryDto> = Vec::new();

        if !categories.is_empty() {
            for category in categories {
                mapped_categories.push(category.into_dto());
            }
        }

        Ok(CategoriesResponse {
            categories: mapped_categories,
        })
    }
}

#[async_trait]
impl CategoriesService for BudgettoCategoriesService {
    async fn create_category(
        &self,
        user_id: Option<Uuid>,
        request: CreateCategoryDto,
    ) -> AppResult<CategoryDto> {
        let name = request.name.unwrap();
        let note = request.note;

        let created_category = self.repository.create_category(name, note, user_id).await?;

        if user_id.is_some() {
            info!("user {:?} created category successfully", user_id);
        }

        Ok(created_category.into_dto())
    }

    async fn get_category_by_id(&self, id: Uuid, user_id: Uuid) -> AppResult<CategoryDto> {
        let category = self.repository.get_category_by_id(id).await?;

        if let Some(existing_category) = category {
            // verify the user IDs match on the request and the category
            if let Some(category_user_id) = existing_category.user_id {
                if category_user_id != user_id {
                    return Err(Error::Forbidden);
                }
            }

            info!("retrieving category {:?} for user {:?}", id, user_id);
            return Ok(existing_category.into_dto());
        }

        Err(Error::NotFound(String::from("category was not found")))
    }

    async fn get_categories(&self, user_id: Uuid) -> AppResult<CategoriesResponse> {
        info!("retrieving categories for user {:?}", user_id);
        let categories = self.repository.get_categories(user_id).await?;

        self.map_to_categories(categories).await
    }

    async fn updated_category(
        &self,
        id: Uuid,
        user_id: Uuid,
        request: UpdateCategoryDto,
    ) -> AppResult<CategoryDto> {
        let category_to_update = self.repository.get_category_by_id(id).await?;

        if let Some(existing_category) = category_to_update {
            // verify the user IDs match on the request and the category
            if let Some(category_user_id) = existing_category.user_id {
                if category_user_id != user_id {
                    return Err(Error::Forbidden);
                }
            }

            let updated_name = request.name.unwrap_or(existing_category.name);
            let update_note = request.note.unwrap_or(existing_category.note.unwrap());

            info!("updating category {:?} for user {:?}", id, user_id);
            let updated_category = self
                .repository
                .update_category(id, updated_name, Some(update_note))
                .await?;

            return Ok(updated_category.into_dto());
        }

        Err(Error::NotFound(String::from("category was not found")))
    }

    async fn delete_category(&self, id: Uuid, user_id: Uuid) -> AppResult<()> {
        let category = self.repository.get_category_by_id(id).await?;

        if let Some(existing_category) = category {
            // verify the user IDs match on the request and the category
            if let Some(category_user_id) = existing_category.user_id {
                if category_user_id != user_id {
                    return Err(Error::Forbidden);
                }
            }

            info!("deleting category {:?} for user {:?}", id, user_id);
            self.repository
                .delete_category(existing_category.id)
                .await?;

            return Ok(());
        }

        Err(Error::NotFound(String::from("category was not found")))
    }
}
