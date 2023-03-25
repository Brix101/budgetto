use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;
use validator::Validate;

use crate::database::category::repository::{Category, CategoryType};

impl Category {
    pub fn into_dto(self) -> CategoryResponseDto {
        CategoryResponseDto {
            id: self.id,
            name: Some(self.name),
            cat_type: self.cat_type,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryResponseDto {
    pub id: Uuid,
    pub name: Option<String>,
    pub cat_type: CategoryType,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

#[derive(Clone, Serialize, Deserialize, Debug, Validate, Default)]
pub struct CategoryCreateDto {
    #[validate(required, length(min = 1))]
    pub name: Option<String>,
    pub cat_type: CategoryType,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct CategoryUpdateDto {
    pub name: Option<String>,
    pub cat_type: Option<CategoryType>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct CategoryQuery {
    pub category_id: Option<Uuid>,
}

impl CategoryCreateDto {
    pub fn new_stub() -> Self {
        Self {
            name: Some(String::from("stub category")),
            cat_type: CategoryType::NonEssential,
        }
    }
}
