use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod requests;
pub mod responses;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UserDto {
    // #[serde(skip_serializing, skip_deserializing)]
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}
