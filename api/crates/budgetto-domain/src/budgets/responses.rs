use serde::{Deserialize, Serialize};

use super::BudgetDto;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BudgetResponse {
    pub budget: BudgetDto,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BudgetsResponse {
    pub budgets: Vec<BudgetDto>,
}
