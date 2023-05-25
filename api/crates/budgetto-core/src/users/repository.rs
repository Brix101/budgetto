use std::{sync::Arc, time::SystemTime};

use async_trait::async_trait;
use mockall::automock;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;
use uuid::{uuid, Uuid};

use budgetto_domain::users::UserDto;

/// Similar to above, we want to keep a reference count across threads so we can manage our connection pool.
pub type DynUsersRepository = Arc<dyn UsersRepository + Send + Sync>;

#[automock]
#[async_trait]
pub trait UsersRepository {
    async fn create(&self, args: CreateUser) -> anyhow::Result<User>;

    async fn find_by_email(&self, email: &str) -> anyhow::Result<Option<User>>;

    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<User>;

    async fn update(&self, args: UpdateUser) -> anyhow::Result<User>;
}

#[derive(Debug)]
pub struct CreateUser {
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Debug)]
pub struct UpdateUser {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub password: String,
    pub bio: String,
    pub image: String,
}

#[derive(FromRow, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub bio: String,
    pub image: String,
    pub is_active: Option<bool>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl User {
    pub fn into_dto(self) -> UserDto {
        UserDto {
            id: self.id,
            email: self.email,
            name: self.name,
            bio: Some(self.bio),
            image: Some(self.image),
        }
    }
}

impl Default for User {
    fn default() -> Self {
        User {
            id: uuid!("f3f898aa-ffa3-4b58-91b0-612a1c801a5e"),
            bio: String::from("stub bio"),
            email: String::from("stub email"),
            name: String::from("stub name"),
            password: String::from("hashed password"),
            image: String::from("stub image"),
            is_active: Some(true),
            created_at: OffsetDateTime::from(SystemTime::now()),
            updated_at: OffsetDateTime::from(SystemTime::now()),
        }
    }
}
