use std::sync::Arc;

use crate::errors::AppResult;
use async_trait::async_trait;

pub type DynService = Arc<dyn Services + Send + Sync>;

#[async_trait]
pub trait Services {
    async fn create<T, R>(&self, args: T) -> AppResult<R>;

    async fn find_by_id<T, R>(&self, args: T) -> AppResult<R>;

    async fn find_many<T, R>(&self, args: T) -> AppResult<R>;

    async fn updated<T, R>(&self, args: T) -> AppResult<R>;

    async fn delete<T, R>(&self, args: T) -> AppResult<R>;
}
