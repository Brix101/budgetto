use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::users::{AuthClaims, UserDto};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    pub sub: Uuid,
    pub user: UserDto,
    pub exp: usize,
    pub iat: usize,
}

impl AccessTokenClaims {
    pub fn into_auth_claims(&self) -> AuthClaims {
        AuthClaims {
            session_id: None,
            user: Some(self.user.clone()),
            cookie: None,
        }
    }
}

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    pub sub: Uuid,
    pub exp: usize,
    pub iat: usize,
}
