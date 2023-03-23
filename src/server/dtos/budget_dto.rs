use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::database::budget::repository::Budget;

impl Budget {
    pub fn into_dto(self) -> ResponseBudgetDto {
        ResponseBudgetDto {
            id: self.id,
            name: Some(self.name),
            amount: Some(self.amount),
            description: Some(self.description),
            frequency: Some(self.frequency),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ResponseBudgetDto {
    pub id: i64,
    pub name: Option<String>,
    pub amount: Option<f64>,
    pub description: Option<String>,
    pub frequency: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Validate, Default)]
pub struct CreateBudgetDto {
    #[validate(required, length(min = 1))]
    pub name: Option<String>,
    #[validate(required, range(min = 0.00))]
    pub amount: Option<f64>,
    pub description: Option<String>,
    pub frequency: Option<String>,
    #[validate(required, range(min = 1))]
    pub category_id: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct UpdateBudgetDto {
    pub name: Option<String>,
    pub amount: Option<f64>,
    pub description: Option<String>,
    pub frequency: Option<String>,
    pub category_id: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct GetBudgetRequestDto {
    pub budget_id: Option<i64>,
}
