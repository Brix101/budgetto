use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Clone, Serialize, Deserialize, Debug, Validate, Default)]
pub struct CreateSessionDto {
    #[validate(required)]
    pub user_id: Option<Uuid>,
    #[validate(required, length(min = 1))]
    pub user_agent: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Validate)]
pub struct NewAccessTokenRequest {
    #[validate(required, length(min = 1))]
    pub refresh_token: Option<String>,
}
