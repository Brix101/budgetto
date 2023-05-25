use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use super::TransactionType;

#[derive(Debug, Deserialize, Serialize, Default, Validate)]
pub struct CreateTransactionDto {
    #[validate(required)]
    pub amount: Option<f64>,
    #[validate(required)]
    pub transaction_type: Option<TransactionType>,
    pub note: Option<String>,
    #[validate(required)]
    pub category_id: Option<Uuid>,
    #[validate(required)]
    pub account_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct UpdateTransactionDto {
    pub amount: Option<f64>,
    pub note: Option<String>,
    pub category_id: Option<Uuid>,
    pub account_id: Option<Uuid>,
    pub transaction_type: Option<TransactionType>,
    pub id: Option<Uuid>,
    pub user_id: Option<Uuid>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct QueryTransaction {
    pub id: Option<Uuid>,
}
