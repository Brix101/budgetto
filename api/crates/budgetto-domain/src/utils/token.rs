use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::users::UserDto;

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    pub sub: Uuid,
    pub user: UserDto,
    pub exp: usize,
    pub iat: usize,
}

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    pub sub: Uuid,
    pub exp: usize,
    pub iat: usize,
}
