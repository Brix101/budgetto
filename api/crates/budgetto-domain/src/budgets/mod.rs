use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod requests;
pub mod responses;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BudgetDto {
    // #[serde(skip_serializing, skip_deserializing)]
    pub id: Uuid,
    pub amount: f64,
    pub category_id: Uuid,
    // #[serde(rename = "createdAt")]
    pub created_at: String,
    // #[serde(rename = "updatedAt")]
    pub updated_at: String,
}
