use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct CreateTransactionDto {
    pub amount: f64,
    pub note: Option<String>,
    // pub transaction_type: TransactionType,
    pub category_id: Uuid,
    pub account_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct UpdateTransactionDto {
    pub id: Uuid,
    pub amount: f64,
    pub note: Option<String>,
    // pub transaction_type: TransactionType,
    pub category_id: Uuid,
    pub account_id: Uuid,
}
