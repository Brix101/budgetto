use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::database::expense::repository::Expense;

impl Expense {
    pub fn into_dto(self) -> ExpenseResponseDto {
        ExpenseResponseDto {
            id: self.id,
            amount: Some(self.amount),
            description: Some(self.description),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ExpenseResponseDto {
    pub id: i64,
    pub amount: Option<f64>,
    pub description: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Validate, Default)]
pub struct ExpenseCreateDto {
    #[validate(required, range(min = 0.00))]
    pub amount: Option<f64>,
    pub description: Option<String>,
    #[validate(required, range(min = 1))]
    pub category_id: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ExpenseUpdateDto {
    pub amount: Option<f64>,
    pub description: Option<String>,
    pub category_id: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ExpenseGetQueryDto {
    pub expense_id: Option<i64>,
}
