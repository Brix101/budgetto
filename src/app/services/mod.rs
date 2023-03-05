use std::sync::Arc;

use tracing::info;

use crate::{
    app::repositories::user_repository::DynUsersRepository,
    core::{
        config::AppConfig,
        database::ConnectionPool,
        queries::user_query::UsersQuery,
        utils::{
            security_utils::{ArgonSecurityService, DynSecurityService},
            token_service::{DynTokenService, JwtService},
        },
    },
};

use self::users_service::{DynUsersService, UsersService};

pub mod users_service;

#[derive(Clone)]
pub struct ServiceRegister {
    pub users_service: DynUsersService,
    pub token_service: DynTokenService,
}

impl ServiceRegister {
    pub fn new(pool: ConnectionPool, config: Arc<AppConfig>) -> Self {
        info!("initializing utility services...");
        let security_service =
            Arc::new(ArgonSecurityService::new(config.clone())) as DynSecurityService;
        let token_service = Arc::new(JwtService::new(config)) as DynTokenService;

        info!("utility services initialized, building feature services...");
        let users_repository = Arc::new(UsersQuery::new(pool.clone())) as DynUsersRepository;
        let users_service = Arc::new(UsersService::new(
            users_repository.clone(),
            security_service,
            token_service.clone(),
        )) as DynUsersService;

        ServiceRegister {
            users_service,
            token_service,
        }
    }
}
