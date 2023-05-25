use anyhow::Context;
use async_trait::async_trait;
use budgetto_core::sessions::repository::{CreateSession, Session, SessionsRepository};
use budgetto_core::users::repository::User;
use sqlx::{query, query_as};
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
    async fn create(&self, args: CreateSession) -> anyhow::Result<Session> {
        query_as!(
            Session,
            r#"
        insert into sessions (user_id,user_agent,exp)
        values ($1,$2,$3)
        returning *
            "#,
            args.user_id,
            args.user_agent,
            args.exp
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

    async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        query!(
            r#"
        DELETE FROM "sessions"
        WHERE (("id" = $1));
            "#,
            id
        )
        .execute(&self.pool)
        .await
        .context("an unexpected error occurred deleting category")?;

        Ok(())
    }
}
