use serde::{Deserialize, Serialize};

use super::AccountDto;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AccountResponse {
    pub account: AccountDto,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AccountsResponse {
    pub accounts: Vec<AccountDto>,
}
