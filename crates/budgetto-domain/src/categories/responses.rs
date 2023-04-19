use serde::{Deserialize, Serialize};

use super::CategoryDto;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CategoryResponse {
    pub category: CategoryDto,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CategoriesResponse {
    pub categories: Vec<CategoryDto>,
}
