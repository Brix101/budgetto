use async_trait::async_trait;
use tracing::info;
use uuid::Uuid;

use budgetto_core::{
    categories::{
        repository::{Category, CreateCategory, DynCategoriesRepository, UpdateCategory},
        service::CategoriesService,
    },
    errors::{AppResult, Error},
};
use budgetto_domain::categories::{
    requests::{CreateCategoryDto, UpdateCategoryDto},
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
    async fn map_to_categories(&self, categories: Vec<Category>) -> AppResult<Vec<CategoryDto>> {
        info!("found {} categories", categories.len());

        let mut mapped_categories: Vec<CategoryDto> = Vec::new();

        if !categories.is_empty() {
            for category in categories {
                mapped_categories.push(category.into_dto());
            }
        }

        Ok(mapped_categories)
    }
}

#[async_trait]
impl CategoriesService for BudgettoCategoriesService {
    async fn create(&self, request: CreateCategoryDto) -> AppResult<CategoryDto> {
        let name = request.name.unwrap();
        let note = request.note;
        let user_id = request.user_id;

        let created_category = self
            .repository
            .create(CreateCategory {
                name,
                note,
                user_id,
            })
            .await?;

        if user_id.is_some() {
            info!("user {:?} created category successfully", user_id);
        }

        Ok(created_category.into_dto())
    }

    async fn find_by_id(&self, id: Uuid, user_id: Uuid) -> AppResult<CategoryDto> {
        let category = self.repository.find_by_id(id).await?;

        if let Some(existing_category) = category {
            // verify the user IDs match on the request and the category
            if let Some(category_user_id) = existing_category.user_id {
                if category_user_id == user_id {
                    info!("retrieving category {:?} for user {:?}", id, user_id);
                    return Ok(existing_category.into_dto());
                } else {
                    return Err(Error::Forbidden);
                }
            }

            return Err(Error::Forbidden);
        }

        Err(Error::NotFound(String::from("category was not found")))
    }

    async fn find_many(&self, user_id: Uuid) -> AppResult<Vec<CategoryDto>> {
        info!("retrieving categories for user {:?}", user_id);
        let categories = self.repository.find_many(user_id).await?;

        self.map_to_categories(categories).await
    }

    async fn updated(&self, request: UpdateCategoryDto) -> AppResult<CategoryDto> {
        let id = request.id.unwrap();
        let user_id = request.user_id.unwrap();
        let category_to_update = self.repository.find_by_id(id).await?;

        if let Some(existing_category) = category_to_update {
            // verify the user IDs match on the request and the category
            if let Some(category_user_id) = existing_category.user_id {
                if category_user_id == user_id {
                    let updated_name = request.name.unwrap_or(existing_category.name);
                    let update_note = match request.note {
                        Some(note) => note,
                        None => existing_category.note.unwrap_or(String::new()),
                    };

                    info!("updating category {:?} for user {:?}", id, user_id);
                    let updated_category = self
                        .repository
                        .update(UpdateCategory {
                            id,
                            name: updated_name,
                            note: Some(update_note),
                        })
                        .await?;

                    return Ok(updated_category.into_dto());
                } else {
                    return Err(Error::Forbidden);
                }
            }

            return Err(Error::Forbidden);
        }

        Err(Error::NotFound(String::from("category was not found")))
    }

    async fn delete(&self, id: Uuid, user_id: Uuid) -> AppResult<()> {
        let category = self.repository.find_by_id(id).await?;

        if let Some(existing_category) = category {
            // verify the user IDs match on the request and the category
            if let Some(category_user_id) = existing_category.user_id {
                if category_user_id == user_id {
                    info!("deleting category {:?} for user {:?}", id, user_id);
                    self.repository.delete(existing_category.id).await?;

                    return Ok(());
                } else {
                    return Err(Error::Forbidden);
                }
            }

            return Err(Error::Forbidden);
        }

        Err(Error::NotFound(String::from("category was not found")))
    }
}
