use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Default, Validate)]
pub struct CreateTransactionDto {
    #[validate(required)]
    pub amount: Option<f64>,
    pub note: Option<String>,
    #[validate(required)]
    pub category_id: Option<Uuid>,
    #[validate(required)]
    pub account_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct UpdateTransactionDto {
    pub amount: Option<f64>,
    pub note: Option<String>,
    pub category_id: Option<Uuid>,
    pub account_id: Option<Uuid>,
}
