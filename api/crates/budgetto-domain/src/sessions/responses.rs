use serde::{Deserialize, Serialize};

use crate::users::UserDto;

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionResponse {
    // #[serde(skip_serializing, skip_deserializing)]
    pub user: Option<UserDto>,
    #[serde(rename = "accessToken")]
    pub access_token: Option<String>,
    #[serde(skip_serializing, skip_deserializing)]
    pub refresh_token: Option<String>,
}

impl Default for SessionResponse {
    fn default() -> Self {
        Self {
            user: None,
            access_token: None,
            refresh_token: None,
        }
    }
}
