use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SessionResponse {
    #[serde(skip_serializing, skip_deserializing)]
    pub access_token: String,
    pub refresh_token: String,
}
