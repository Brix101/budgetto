use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Validate, Default)]
pub struct SignupUserRequest {
    #[validate]
    pub user: SignUpUserDto,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct SigninUserRequest {
    #[validate]
    pub user: SignInUserDto,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct UpdateUserRequest {
    pub user: UpdateUserDto,
}

#[derive(Clone, Serialize, Deserialize, Debug, Validate, Default)]
pub struct SignUpUserDto {
    #[validate(required, length(min = 1))]
    pub name: Option<String>,
    #[validate(required, length(min = 1), email(message = "email is invalid"))]
    pub email: Option<String>,
    #[validate(required, length(min = 6))]
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct SignInUserDto {
    #[validate(required, length(min = 1), email(message = "email is invalid"))]
    pub email: Option<String>,
    #[validate(required, length(min = 1))]
    pub password: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct UpdateUserDto {
    pub email: Option<String>,
    pub password: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
}

impl SignUpUserDto {
    pub fn new_stub() -> Self {
        Self {
            name: Some(String::from("stub name")),
            email: Some(String::from("stub email")),
            password: Some(String::from("stub password")),
        }
    }
}

impl SignInUserDto {
    pub fn new_stub() -> Self {
        Self {
            email: Some(String::from("stub email")),
            password: Some(String::from("stub password")),
        }
    }
}
