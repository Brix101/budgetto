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
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct UpdateAccountDto {
    pub name: Option<String>,
    pub balance: Option<f64>,
    pub note: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct QueryAccount {
    pub id: Option<Uuid>,
}
