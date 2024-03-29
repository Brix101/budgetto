use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;
use time::format_description::well_known::Rfc3339;
use uuid::Uuid;

use budgetto_domain::accounts::AccountDto;

pub type DynAccountsRepository = Arc<dyn AccountsRepository + Send + Sync>;

#[automock]
#[async_trait]
pub trait AccountsRepository {
    async fn create(&self, args: CreateAccount) -> anyhow::Result<Account>;

    async fn find_many(&self, user_id: Uuid) -> anyhow::Result<Vec<Account>>;

    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Account>>;

    async fn update(&self, args: UpdateAccount) -> anyhow::Result<Account>;

    async fn delete(&self, id: Uuid) -> anyhow::Result<()>;
}

#[derive(Debug)]
pub struct CreateAccount {
    pub name: String,
    pub balance: f64,
    pub note: Option<String>,
    pub user_id: Uuid,
}

#[derive(Debug)]
pub struct UpdateAccount {
    pub id: Uuid,
    pub name: String,
    pub balance: f64,
    pub note: Option<String>,
}

#[derive(FromRow, Debug)]
pub struct Account {
    pub id: Uuid,
    pub name: String,
    pub balance: f64,
    pub note: Option<String>,
    pub user_id: Uuid,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}

impl Account {
    pub fn into_dto(self) -> AccountDto {
        AccountDto {
            id: self.id,
            name: self.name,
            balance: self.balance,
            note: self.note,
            created_at: self.created_at.format(&Rfc3339).unwrap(),
            updated_at: self.updated_at.format(&Rfc3339).unwrap(),
        }
    }
}
