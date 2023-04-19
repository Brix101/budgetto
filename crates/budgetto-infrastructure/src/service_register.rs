use std::sync::Arc;

use tracing::info;

use budgetto_core::{
    categories::{repository::DynCategoriesRepository, service::DynCategoriesService},
    config::AppConfig,
    sessions::{repository::DynSessionsRepository, service::DynSessionsService},
    users::{repository::DynUsersRepository, service::DynUsersService},
    utils::{security_service::DynSecurityService, token_service::DynTokenService},
};

use crate::{
    connection_pool::ConnectionPool,
    repositories::{
        categories_repository::PostgresCategoriesRepository,
        sessions_repository::PostgresSessionsRepository, users_repository::PostgresUsersRepository,
    },
    services::{
        categories_service::BudgettoCategoriesService,
        sessions_service::BudgettoSessionsService,
        users_service::BudgettoUsersService,
        utils::{argon_security_service::ArgonSecurityService, jwt_utils::JwtService},
    },
};

#[derive(Clone)]
pub struct ServiceRegister {
    pub token_service: DynTokenService,
    pub users: DynUsersService,
    pub sessions: DynSessionsService,
    pub categories: DynCategoriesService,
}

/// A simple service container responsible for managing the various services our API endpoints will pull from through axum extensions.
impl ServiceRegister {
    pub fn init(pool: ConnectionPool, config: Arc<AppConfig>) -> Self {
        info!("initializing utility services...");
        let security_service =
            Arc::new(ArgonSecurityService::new(config.clone())) as DynSecurityService;
        let token_service = Arc::new(JwtService::new(config)) as DynTokenService;

        info!("utility services initialized, building feature services...");
        let sessions_repository =
            Arc::new(PostgresSessionsRepository::new(pool.clone())) as DynSessionsRepository;

        let sessions = Arc::new(BudgettoSessionsService::new(
            sessions_repository.clone(),
            token_service.clone(),
        )) as DynSessionsService;

        let users_repository =
            Arc::new(PostgresUsersRepository::new(pool.clone())) as DynUsersRepository;

        let users = Arc::new(BudgettoUsersService::new(
            users_repository.clone(),
            security_service,
            token_service.clone(),
            sessions.clone(),
        )) as DynUsersService;

        let categories_repository =
            Arc::new(PostgresCategoriesRepository::new(pool.clone())) as DynCategoriesRepository;

        let categories =
            Arc::new(BudgettoCategoriesService::new(categories_repository)) as DynCategoriesService;

        info!("feature services successfully initialized!");
        ServiceRegister {
            token_service,
            users,
            sessions,
            categories,
        }
    }
}
