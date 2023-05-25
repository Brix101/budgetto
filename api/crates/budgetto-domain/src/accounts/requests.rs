use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Default, Validate)]
pub struct CreateAccountDto {
    #[validate(required)]
    pub name: Option<String>,
    #[validate(required)]
    pub balance: Option<f64>,
    pub note: Option<String>,
    pub user_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct UpdateAccountDto {
    pub id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub name: Option<String>,
    pub balance: Option<f64>,
    pub note: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct QueryAccount {
    pub id: Option<Uuid>,
}
