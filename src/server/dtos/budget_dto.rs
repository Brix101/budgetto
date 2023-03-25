use serde::{Deserialize, Serialize};
use sqlx::{types::time::OffsetDateTime, FromRow};
use uuid::Uuid;
use validator::Validate;

use crate::database::budget::repository::{Budget, PlanType};

impl Budget {
    pub fn into_dto(self) -> BudgetResponseDto {
        BudgetResponseDto {
            id: self.id,
            category_id: self.category_id,
            amount: Some(self.amount),
            description: Some(self.description),
            plan: self.plan,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct BudgetResponseDto {
    pub id: Uuid,
    pub category_id: Uuid,
    pub amount: Option<f64>,
    pub description: Option<String>,
    pub plan: PlanType,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

#[derive(Clone, Serialize, Deserialize, Debug, Validate, Default)]
pub struct BudgetCreateDto {
    #[validate(required)]
    pub category_id: Option<Uuid>,
    #[validate(required, range(min = 0.00))]
    pub amount: Option<f64>,
    pub description: Option<String>,
    #[validate(required)]
    pub plan: Option<PlanType>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct BudgetUpdateDto {
    pub category_id: Option<Uuid>,
    pub amount: Option<f64>,
    pub description: Option<String>,
    pub plan: Option<PlanType>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct BudgetQuery {
    pub budget_id: Option<Uuid>,
}
