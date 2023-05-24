use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod requests;
pub mod responses;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct UserDto {
    // #[serde(skip_serializing, skip_deserializing)]
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AuthClaims {
    pub user: Option<UserDto>,
    pub is_expired: bool,
}

impl AuthClaims {
    pub fn into_auth(user: UserDto) -> Self {
        Self {
            user: Some(user),
            is_expired: false,
        }
    }

    pub fn expired() -> Self {
        Self {
            user: None,
            is_expired: true,
        }
    }
}

impl Default for AuthClaims {
    fn default() -> Self {
        Self {
            user: None,
            is_expired: false,
        }
    }
}
