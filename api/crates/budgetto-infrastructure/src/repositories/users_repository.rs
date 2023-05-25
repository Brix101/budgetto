use anyhow::Context;
use async_trait::async_trait;
use sqlx::query_as;
use uuid::Uuid;

use budgetto_core::users::repository::{CreateUser, UpdateUser, User, UsersRepository};

use crate::connection_pool::ConnectionPool;

#[derive(Clone)]
pub struct PostgresUsersRepository {
    pool: ConnectionPool,
}

impl PostgresUsersRepository {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UsersRepository for PostgresUsersRepository {
    async fn create(&self, args: CreateUser) -> anyhow::Result<User> {
        query_as!(
            User,
            r#"
        insert into users (created_at, updated_at, name, email, password, image)
        values (current_timestamp, current_timestamp, $1::varchar, $2::varchar, $3::varchar, '')
        returning *
            "#,
            args.name,
            args.email,
            args.password
        )
        .fetch_one(&self.pool)
        .await
        .context("an unexpected error occured while creating the user")
    }

    async fn find_by_email(&self, email: &str) -> anyhow::Result<Option<User>> {
        query_as!(
            User,
            r#"
        select *
        from users
        where email = $1::varchar
            "#,
            email,
        )
        .fetch_optional(&self.pool)
        .await
        .context("unexpected error while querying for user by email")
    }

    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<User> {
        query_as!(
            User,
            r#"
        select *
        from users
        where id = $1
            "#,
            id,
        )
        .fetch_one(&self.pool)
        .await
        .context("user was not found")
    }

    async fn update(&self, args: UpdateUser) -> anyhow::Result<User> {
        query_as!(
            User,
            r#"
        update users
        set
            name = $1::varchar,
            email = $2::varchar,
            password = $3::varchar,
            bio = $4::varchar,
            image = $5::varchar,
            updated_at = current_timestamp
        where id = $6
        returning *
            "#,
            args.name,
            args.email,
            args.password,
            args.bio,
            args.image,
            args.id
        )
        .fetch_one(&self.pool)
        .await
        .context("could not update the user")
    }
}
