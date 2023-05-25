use std::sync::Arc;

use tracing::info;

use budgetto_core::{
    accounts::{repository::DynAccountsRepository, service::DynAccountsService},
    budgets::{repository::DynBudgetsRepository, service::DynBudgetsService},
    categories::{repository::DynCategoriesRepository, service::DynCategoriesService},
    config::AppConfig,
    sessions::{repository::DynSessionsRepository, service::DynSessionsService},
    transactions::{repository::DynTransactionsRepository, service::DynTransactionsService},
    users::{repository::DynUsersRepository, service::DynUsersService},
    utils::{security_service::DynSecurityService, token_service::DynTokenService},
};

use crate::{
    connection_pool::ConnectionPool,
    repositories::{
        accounts_repository::PostgresAccountsRepository,
        budgets_repository::PostgresBudgetsRepository,
        categories_repository::PostgresCategoriesRepository,
        sessions_repository::PostgresSessionsRepository,
        transactions_repository::PostgresTransactionsRepository,
        users_repository::PostgresUsersRepository,
    },
    services::{
        accounts_service::BudgettoAccountsService,
        budgets_service::BudgettoBudgetsService,
        categories_service::BudgettoCategoriesService,
        sessions_service::BudgettoSessionsService,
        transactions_service::BudgettoTransactionsService,
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
    pub accounts: DynAccountsService,
    pub transactions: DynTransactionsService,
    pub budgets: DynBudgetsService,
}

/// A simple service container responsible for managing the various services our API endpoints will pull from through axum extensions.
impl ServiceRegister {
    pub fn init(pool: ConnectionPool, config: Arc<AppConfig>) -> Self {
        info!("initializing utility services...");
        let security_service =
            Arc::new(ArgonSecurityService::new(config.clone())) as DynSecurityService;
        let token_service = Arc::new(JwtService::new(config)) as DynTokenService;

        // token_service.clone().refresh_blacklist();

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
            sessions.clone(),
        )) as DynUsersService;

        let categories_repository =
            Arc::new(PostgresCategoriesRepository::new(pool.clone())) as DynCategoriesRepository;

        let categories =
            Arc::new(BudgettoCategoriesService::new(categories_repository)) as DynCategoriesService;

        let accounts_repository =
            Arc::new(PostgresAccountsRepository::new(pool.clone())) as DynAccountsRepository;

        let accounts =
            Arc::new(BudgettoAccountsService::new(accounts_repository)) as DynAccountsService;

        let transactions_repository = Arc::new(PostgresTransactionsRepository::new(pool.clone()))
            as DynTransactionsRepository;

        let transactions = Arc::new(BudgettoTransactionsService::new(transactions_repository))
            as DynTransactionsService;

        let budgets_repository =
            Arc::new(PostgresBudgetsRepository::new(pool.clone())) as DynBudgetsRepository;

        let budgets =
            Arc::new(BudgettoBudgetsService::new(budgets_repository)) as DynBudgetsService;

        info!("feature services successfully initialized!");
        ServiceRegister {
            token_service,
            users,
            sessions,
            categories,
            accounts,
            transactions,
            budgets,
        }
    }
}
