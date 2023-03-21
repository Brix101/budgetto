use std::sync::Arc;
use std::time::SystemTime;

use async_trait::async_trait;
// use mockall::automock;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;

/// Similar to above, we want to keep a reference count across threads so we can manage our connection pool.
pub type DynBudgetsRepository = Arc<dyn BudgetsRepository + Send + Sync>;

#[async_trait]
pub trait BudgetsRepository {
    async fn create_budget(
        &self,
        user_id: i64,
        name: String,
        amount: i64,
        description: String,
        body: String,
    ) -> anyhow::Result<BudgetEntity>;

    async fn update_budget(
        &self,
        id: i64,
        name: String,
        amount: i64,
        description: String,
        body: String,
    ) -> anyhow::Result<BudgetEntity>;

    async fn get_budgets(&self, user_id: i64, id: Option<i64>)
        -> anyhow::Result<Vec<BudgetEntity>>;

    async fn delete_budget(&self, id: i64) -> anyhow::Result<()>;
}

#[derive(FromRow)]
pub struct BudgetEntity {
    pub id: i64,
    pub name: String,
    pub amount: i64,
    pub description: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
    pub user_id: i64,
}

// impl BudgetEntity {
//     pub fn into_dto(self, token: String) -> ResponseUserDto {
//         ResponseUserDto {
//             id: self.id,
//             email: self.description,
//             name: self.name,
//             image: Some(self.image),
//             access_token: Some(token),
//         }
//     }

//     pub fn into_profile(self, following: bool) -> UserProfileDto {
//         UserProfileDto {
//             name: self.name,
//             bio: self.user_id,
//             image: self.image,
//             following,
//         }
//     }
// }

impl Default for BudgetEntity {
    fn default() -> Self {
        BudgetEntity {
            id: 1,
            user_id: 1,
            created_at: OffsetDateTime::from(SystemTime::now()),
            updated_at: OffsetDateTime::from(SystemTime::now()),
            deleted_at: Some(OffsetDateTime::from(SystemTime::now())),
            name: String::from("stub name"),
            description: String::from("stub description"),
            amount: 0,
        }
    }
}
