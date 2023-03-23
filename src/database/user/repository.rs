use std::sync::Arc;
use std::time::SystemTime;

use async_trait::async_trait;
use mockall::automock;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;

/// Similar to above, we want to keep a reference count across threads so we can manage our connection pool.
pub type DynUsersRepository = Arc<dyn UsersRepository + Send + Sync>;

#[automock]
#[async_trait]
pub trait UsersRepository {
    async fn create_user(
        &self,
        email: &str,
        name: &str,
        hash_password: &str,
    ) -> anyhow::Result<User>;

    async fn get_user_by_email(&self, email: &str) -> anyhow::Result<Option<User>>;

    async fn get_user_by_id(&self, id: i64) -> anyhow::Result<User>;

    async fn update_user(
        &self,
        id: i64,
        email: String,
        name: String,
        password: String,
        bio: String,
        image: String,
    ) -> anyhow::Result<User>;
}

#[derive(FromRow, Debug)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password: String,
    pub bio: String,
    pub image: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}

impl Default for User {
    fn default() -> Self {
        User {
            id: 1,
            bio: String::from("stub bio"),
            created_at: OffsetDateTime::from(SystemTime::now()),
            updated_at: OffsetDateTime::from(SystemTime::now()),
            deleted_at: Some(OffsetDateTime::from(SystemTime::now())),
            email: String::from("stub email"),
            name: String::from("stub name"),
            password: String::from("hashed password"),
            image: String::from("stub image"),
        }
    }
}
