use std::sync::Arc;

use tracing::info;

use budgetto_core::{
    config::AppConfig,
    utils::{security_service::DynSecurityService, token_service::DynTokenService},
};

use crate::{
    connection_pool::ConnectionPool,
    services::utils::{argon_security_service::ArgonSecurityService, jwt_utils::JwtService},
};

#[derive(Clone)]
pub struct ServiceRegister {
    pub token_service: DynTokenService,
}

/// A simple service container responsible for managing the various services our API endpoints will pull from through axum extensions.
impl ServiceRegister {
    pub fn init(_pool: ConnectionPool, config: Arc<AppConfig>) -> Self {
        info!("initializing utility services...");
        let _security_service =
            Arc::new(ArgonSecurityService::new(config.clone())) as DynSecurityService;
        let token_service = Arc::new(JwtService::new(config)) as DynTokenService;

        info!("utility services initialized, building feature services...");

        info!("feature services successfully initialized!");
        ServiceRegister { token_service }
    }
}
