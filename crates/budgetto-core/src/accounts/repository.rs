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
    async fn create_account(
        &self,
        name: String,
        balance: f64,
        note: Option<String>,
        user_id: Uuid,
    ) -> anyhow::Result<Account>;

    async fn get_accounts(&self, user_id: Uuid) -> anyhow::Result<Vec<Account>>;

    async fn get_account_by_id(&self, id: Uuid) -> anyhow::Result<Option<Account>>;

    async fn update_account(
        &self,
        id: Uuid,
        name: String,
        balance: f64,
        note: Option<String>,
    ) -> anyhow::Result<Account>;

    async fn delete_account(&self, id: Uuid) -> anyhow::Result<()>;
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
