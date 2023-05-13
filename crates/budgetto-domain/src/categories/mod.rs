use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod requests;
pub mod responses;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CategoryDto {
    pub id: Uuid,
    pub name: String,
    pub note: Option<String>,
    #[serde(rename = "userId")]
    pub user_id: Option<Uuid>,
}
