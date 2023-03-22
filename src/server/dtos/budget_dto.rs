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
            user_id: self.user_id,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ResponseBudgetDto {
    #[serde(skip_serializing, skip_deserializing)]
    pub id: i64,
    pub name: Option<String>,
    pub amount: Option<f64>,
    pub description: Option<String>,
    pub user_id: i64,
}

#[derive(Clone, Serialize, Deserialize, Debug, Validate, Default)]
pub struct CreateBudgetDto {
    #[validate(required, length(min = 1))]
    pub name: Option<String>,
    #[validate(required, range(min = 0.00))]
    pub amount: Option<f64>,
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct UpdateBudgetDto {
    pub name: Option<String>,
    pub amount: Option<f64>,
    pub description: Option<String>,
}
