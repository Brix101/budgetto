use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;
use validator::Validate;

use crate::database::expense::repository::Expense;

impl Expense {
    pub fn into_dto(self) -> ExpenseResponseDto {
        ExpenseResponseDto {
            id: self.id,
            amount: Some(self.amount),
            description: Some(self.description),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExpenseResponseDto {
    pub id: Uuid,
    pub amount: Option<f64>,
    pub description: Option<String>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

#[derive(Clone, Serialize, Deserialize, Debug, Validate, Default)]
pub struct ExpenseCreateDto {
    #[validate(required)]
    pub category_id: Option<Uuid>,
    #[validate(required, range(min = 0.00))]
    pub amount: Option<f64>,
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ExpenseUpdateDto {
    pub category_id: Option<Uuid>,
    pub amount: Option<f64>,
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ExpenseQuery {
    pub expense_id: Option<Uuid>,
}
