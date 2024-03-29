use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Default, Validate)]
pub struct CreateCategoryDto {
    #[validate(required)]
    pub name: Option<String>,
    pub note: Option<String>,
    pub user_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct UpdateCategoryDto {
    pub id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub name: Option<String>,
    pub note: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct QueryCategory {
    pub id: Option<Uuid>,
}
