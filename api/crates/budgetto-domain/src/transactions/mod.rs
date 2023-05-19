use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod requests;
pub mod responses;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct TransactionDto {
    #[serde(skip_serializing, skip_deserializing)]
    pub id: Uuid,
    pub amount: f64,
    pub note: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
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
