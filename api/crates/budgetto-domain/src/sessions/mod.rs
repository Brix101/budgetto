use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod requests;
pub mod responses;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SessionDto {
    #[serde(skip_serializing, skip_deserializing)]
    pub id: Uuid,
    pub name: String,
    pub balance: f64,
    pub note: Option<String>,
}
