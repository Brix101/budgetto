use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::database::category::repository::Category;

impl Category {
    pub fn into_dto(self) -> CategoryResponseDto {
        CategoryResponseDto {
            id: self.id,
            name: Some(self.name),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CategoryResponseDto {
    pub id: i64,
    pub name: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Validate, Default)]
pub struct CategoryCreateDto {
    #[validate(required, length(min = 1))]
    pub name: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct CategoryUpdateDto {
    pub name: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct CategoryQuery {
    pub category_id: Option<i64>,
}

impl CategoryCreateDto {
    pub fn new_stub() -> Self {
        Self {
            name: Some(String::from("stub category")),
        }
    }
}
