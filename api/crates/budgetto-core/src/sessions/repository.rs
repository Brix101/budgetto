use std::{sync::Arc, time::SystemTime};

use async_trait::async_trait;
use mockall::automock;
use sqlx::{types::time::OffsetDateTime, FromRow};
use uuid::{uuid, Uuid};

use crate::users::repository::User;

pub type DynSessionsRepository = Arc<dyn SessionsRepository + Send + Sync>;

#[automock]
#[async_trait]
pub trait SessionsRepository {
    async fn create(&self, args: CreateSession) -> anyhow::Result<Session>;

    async fn get_user_by_session_id(&self, id: Uuid) -> anyhow::Result<Option<User>>;

    async fn delete(&self, id: Uuid) -> anyhow::Result<()>;
}

#[derive(Debug)]
pub struct CreateSession {
    pub user_id: Uuid,
    pub user_agent: String,
    pub exp: OffsetDateTime,
}

#[derive(FromRow, Debug)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub exp: OffsetDateTime,
    pub user_agent: String,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            id: uuid!("8147a9f8-2845-4f92-9e1d-0c0c6c8db79b"),
            user_id: uuid!("f3f898aa-ffa3-4b58-91b0-612a1c801a5e"),
            exp: OffsetDateTime::from(SystemTime::now()),
            user_agent: String::from("stub user agent"),
        }
    }
}
