use anyhow::Context;
use async_trait::async_trait;
use sqlx::query_as;
use sqlx::types::time::OffsetDateTime;

use crate::database::user::User;
use crate::database::Database;

use super::{Session, SessionsRepository};

#[async_trait]
impl SessionsRepository for Database {
    async fn new_session(
        &self,
        user_id: &i64,
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

    async fn get_user_by_session_id(&self, id: i64) -> anyhow::Result<User> {
        query_as!(
            User,
            r#"
        select users.* from users
        inner join sessions
        on users.id = sessions.user_id
        where sessions.id = $1
            "#,
            id,
        )
        .fetch_one(&self.pool)
        .await
        .context("user was not found")
    }
}
