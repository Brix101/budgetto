use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::users::UserDto;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UserAuthenicationResponse {
    pub user: UserDto,
}

impl UserAuthenicationResponse {
    pub fn new(
        id: Uuid,
        name: String,
        email: String,
        // unfortunately, while our implementation returns thes optional fields as empty strings,
        // the realworld demo API enables nullable serializing by default, so we have to wrap these
        // strings as `Option` option values for now
        bio: Option<String>,
        image: Option<String>,
    ) -> Self {
        UserAuthenicationResponse {
            user: UserDto {
                id,
                name,
                email,
                bio,
                image,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReAuthResponse {
    pub user: Option<UserDto>,
    #[serde(rename = "accessToken")]
    pub access_token: Option<String>,
}

impl Default for ReAuthResponse {
    fn default() -> Self {
        Self {
            user: None,
            access_token: None,
        }
    }
}
