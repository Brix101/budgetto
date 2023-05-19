use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod requests;
pub mod responses;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransactionDto {
    // #[serde(skip_serializing, skip_deserializing)]
    pub id: Uuid,
    pub amount: f64,
    pub transaction_type: TransactionType,
    pub note: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[warn(unused_imports)]
#[derive(sqlx::Type, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[sqlx(type_name = "transaction_type")]
pub enum TransactionType {
    Expense,
    Income,
    Transfer,
    Refund,
}

impl Default for TransactionType {
    fn default() -> Self {
        Self::Expense
    }
}
