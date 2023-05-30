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
    pub session_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub cookie: Option<String>,
}

impl UserDto {
    pub fn into_auth_claims(&self, session_id: Option<Uuid>) -> AuthClaims {
        AuthClaims {
            session_id,
            user_id: Some(self.id.clone()),
            cookie: None,
        }
    }
}

impl AuthClaims {
    pub fn expired(cookie: Option<String>) -> Self {
        Self {
            session_id: None,
            user_id: None,
            cookie: Some(cookie.unwrap()),
        }
    }
}

impl Default for AuthClaims {
    fn default() -> Self {
        Self {
            session_id: None,
            user_id: None,
            cookie: None,
        }
    }
}
