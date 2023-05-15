use anyhow::Context;
use async_trait::async_trait;
use budgetto_core::sessions::repository::{Session, SessionsRepository};
use budgetto_core::users::repository::User;
use sqlx::query_as;
use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;

use crate::connection_pool::ConnectionPool;

#[derive(Clone)]
pub struct PostgresSessionsRepository {
    pool: ConnectionPool,
}

impl PostgresSessionsRepository {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SessionsRepository for PostgresSessionsRepository {
    async fn new_session(
        &self,
        user_id: Uuid,
        user_agent: &str,
        exp: &OffsetDateTime,
    ) -> anyhow::Result<Session> {
        query_as!(
            Session,
            r#"
        insert into sessions (user_id,user_agent,exp)
        values ($1,$2,$3)
        returning *
            "#,
            user_id,
            user_agent,
            exp
        )
        .fetch_one(&self.pool)
        .await
        .context("an unexpected error occured while creating a session")
    }

    async fn get_user_by_session_id(&self, id: Uuid) -> anyhow::Result<Option<User>> {
        query_as!(
            User,
            r#"
        select users.* from users
        inner join sessions
        on users.id = sessions.user_id
        where sessions.exp >= now() and sessions.id = $1
            "#,
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .context("user was not found")
    }
}
