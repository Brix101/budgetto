use serde::{Deserialize, Serialize};

use super::TransactionDto;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TransactionResponse {
    pub transaction: TransactionDto,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TransactionsResponse {
    pub transactions: Vec<TransactionDto>,
}
