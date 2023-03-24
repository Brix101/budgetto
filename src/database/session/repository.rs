use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
// use mockall::automock;
use sqlx::{types::time::OffsetDateTime, FromRow};
use uuid::Uuid;

use crate::database::user::User;

/// Similar to above, we want to keep a reference count across threads so we can manage our connection pool.
pub type DynSessionsRepository = Arc<dyn SessionsRepository + Send + Sync>;

#[automock]
#[async_trait]
pub trait SessionsRepository {
    async fn new_session(
        &self,
        user_id: Uuid,
        user_agent: &str,
        exp: &OffsetDateTime,
    ) -> anyhow::Result<Session>;

    async fn get_user_by_session_id(&self, id: Uuid) -> anyhow::Result<Option<User>>;
}

#[derive(FromRow, Debug)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub exp: OffsetDateTime,
    pub user_agent: String,
}

// impl Default for Session {
//     fn default() -> Self {
//         Self {
//             id: 1,
//             user_id: 1,
//             exp: OffsetDateTime::from(SystemTime::now()),
//             user_agent: String::from("stub user agent"),
//         }
//     }
// }
