use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Default, Validate)]
pub struct CreateBudgetDto {
    #[validate(required)]
    pub category_id: Option<Uuid>,
    #[validate(required)]
    pub amount: Option<f64>,
    pub user_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct UpdateBudgetDto {
    pub id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
    pub amount: Option<f64>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct QueryBudget {
    pub id: Option<Uuid>,
}
