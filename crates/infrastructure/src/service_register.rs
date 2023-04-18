use std::sync::Arc;

use tracing::info;

use core::config::AppConfig;

use crate::connection_pool::ConnectionPool;

#[derive(Clone)]
pub struct ServiceRegister {}

/// A simple service container responsible for managing the various services our API endpoints will pull from through axum extensions.
impl ServiceRegister {
    pub fn init(pool: ConnectionPool, config: Arc<AppConfig>) -> Self {
        info!("initializing utility services...");

        info!("utility services initialized, building feature services...");

        info!("feature services successfully initialized!");
        ServiceRegister {}
    }
}
