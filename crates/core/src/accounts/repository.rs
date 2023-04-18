use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;
use uuid::Uuid;

/// Similar to above, we want to keep a reference count across threads so we can manage our connection pool.
pub type DynAccountsRepository = Arc<dyn UsersRepository + Send + Sync>;

#[automock]
#[async_trait]
pub trait UsersRepository {
    async fn create_account(
        &self,
        email: &str,
        name: &str,
        hash_password: &str,
    ) -> anyhow::Result<Account>;

    async fn get_accounts(&self, user_id: Uuid) -> anyhow::Result<Option<Account>>;

    async fn get_account_by_id(&self, id: Uuid) -> anyhow::Result<Account>;

    async fn update_account(
        &self,
        id: Uuid,
        email: String,
        name: String,
        password: String,
        bio: String,
        image: String,
    ) -> anyhow::Result<Account>;

    async fn delete_account(&self, id: Uuid) -> anyhow::Result<()>;
}

#[derive(FromRow, Debug)]
pub struct Account {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub bio: String,
    pub image: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
