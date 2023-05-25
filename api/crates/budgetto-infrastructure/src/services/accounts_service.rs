use async_trait::async_trait;
use tracing::info;
use uuid::Uuid;

use budgetto_core::{
    accounts::{
        repository::{Account, DynAccountsRepository},
        service::AccountsService,
    },
    errors::{AppResult, Error},
};
use budgetto_domain::accounts::{
    requests::{CreateAccountDto, UpdateAccountDto},
    AccountDto,
};

#[derive(Clone)]
pub struct BudgettoAccountsService {
    repository: DynAccountsRepository,
}

impl BudgettoAccountsService {
    pub fn new(repository: DynAccountsRepository) -> Self {
        Self { repository }
    }
    async fn map_to_accounts(&self, accounts: Vec<Account>) -> AppResult<Vec<AccountDto>> {
        info!("found {} accounts", accounts.len());

        let mut mapped_accounts: Vec<AccountDto> = Vec::new();

        if !accounts.is_empty() {
            for account in accounts {
                mapped_accounts.push(account.into_dto());
            }
        }

        Ok(mapped_accounts)
    }
}

#[async_trait]
impl AccountsService for BudgettoAccountsService {
    async fn create(&self, user_id: Uuid, request: CreateAccountDto) -> AppResult<AccountDto> {
        let name = request.name.unwrap();
        let balance = request.balance.unwrap();
        let note = request.note;

        let created_account = self.repository.create(name, balance, note, user_id).await?;

        info!("user {:?} created account successfully", user_id);

        Ok(created_account.into_dto())
    }

    async fn find_by_id(&self, id: Uuid, user_id: Uuid) -> AppResult<AccountDto> {
        info!("searching for existing account {:?}", id);
        let account = self.repository.find_by_id(id).await?;

        if let Some(existing_account) = account {
            // verify the user IDs match on the request and the account
            if existing_account.user_id != user_id {
                return Err(Error::Forbidden);
            }
            return Ok(existing_account.into_dto());
        }

        Err(Error::NotFound(String::from("account was not found")))
    }

    async fn find_many(&self, user_id: Uuid) -> AppResult<Vec<AccountDto>> {
        info!("searching accounts for user {:?}", user_id);
        let accounts = self.repository.find_many(user_id).await?;

        self.map_to_accounts(accounts).await
    }

    async fn updated(
        &self,
        id: Uuid,
        user_id: Uuid,
        request: UpdateAccountDto,
    ) -> AppResult<AccountDto> {
        let account_to_update = self.repository.find_by_id(id).await?;

        if let Some(existing_account) = account_to_update {
            // verify the user IDs match on the request and the account
            if existing_account.user_id != user_id {
                return Err(Error::Forbidden);
            }

            let updated_name = request.name.unwrap_or(existing_account.name);
            let updated_balance = request.balance.unwrap_or(existing_account.balance);
            let update_note = request.note.unwrap_or(existing_account.note.unwrap());

            info!("updating account {:?} for user {:?}", id, user_id);
            let updated_account = self
                .repository
                .update(id, updated_name, updated_balance, Some(update_note))
                .await?;

            return Ok(updated_account.into_dto());
        }

        Err(Error::NotFound(String::from("account was not found")))
    }

    async fn delete(&self, id: Uuid, user_id: Uuid) -> AppResult<()> {
        let account = self.repository.find_by_id(id).await?;

        if let Some(existing_acount) = account {
            // verify the user IDs match on the request and the account
            if existing_acount.user_id != user_id {
                return Err(Error::Forbidden);
            }

            self.repository.delete(existing_acount.id).await?;

            return Ok(());
        }

        Err(Error::NotFound(String::from("account was not found")))
    }
}
