use serde::{Deserialize, Serialize};

use crate::users::UserDto;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SessionResponse {
    // #[serde(skip_serializing, skip_deserializing)]
    pub user: Option<UserDto>,
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
}
