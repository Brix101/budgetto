use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod requests;
pub mod responses;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategoryDto {
    pub id: Uuid,
    pub name: String,
    pub note: Option<String>,
    // #[serde(rename = "isDefault")]
    pub is_default: bool,
}
