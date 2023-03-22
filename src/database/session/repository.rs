use std::sync::Arc;

use async_trait::async_trait;
// use mockall::automock;
use sqlx::{types::time::OffsetDateTime, FromRow};

use crate::database::user::User;

/// Similar to above, we want to keep a reference count across threads so we can manage our connection pool.
pub type DynSessionsRepository = Arc<dyn SessionsRepository + Send + Sync>;

#[async_trait]
pub trait SessionsRepository {
    async fn new_session(
        &self,
        user_id: &i64,
        user_agent: &str,
        exp: &OffsetDateTime,
    ) -> anyhow::Result<Session>;

    async fn get_user_session_by_id(&self, id: i64) -> anyhow::Result<User>;
}

#[derive(FromRow)]
pub struct Session {
    pub id: i64,
    pub user_id: i64,
    pub exp: OffsetDateTime,
    pub user_agent: String,
}
