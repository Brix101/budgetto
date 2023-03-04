use std::sync::Arc;

use argon2::Config;
// use mockall::automock;

use crate::core::{
    config::AppConfig,
    errors::{CustomError, CustomResult},
};

/// A security service for handling JWT authentication.
pub type DynSecurityService = Arc<dyn SecurityService + Send + Sync>;

// #[automock]
pub trait SecurityService {
    fn hash_password(&self, raw_password: &str) -> CustomResult<String>;

    fn verify_password(
        &self,
        stored_password: &str,
        attempted_password: String,
    ) -> CustomResult<bool>;
}

pub struct ArgonSecurityService {
    config: Arc<AppConfig>,
}

impl ArgonSecurityService {
    pub fn new(config: Arc<AppConfig>) -> Self {
        Self { config }
    }
}

impl SecurityService for ArgonSecurityService {
    fn hash_password(&self, raw_password: &str) -> CustomResult<String> {
        let password_bytes = raw_password.as_bytes();
        let hashed_password = argon2::hash_encoded(
            password_bytes,
            self.config.argon_salt.as_bytes(),
            &Config::default(),
        )
        .unwrap();

        Ok(hashed_password)
    }

    fn verify_password(
        &self,
        stored_password: &str,
        attempted_password: String,
    ) -> CustomResult<bool> {
        let hashes_match =
            argon2::verify_encoded(stored_password, attempted_password.as_bytes())
                .map_err(|err| CustomError::InternalServerErrorWithContext(err.to_string()))?;

        Ok(hashes_match)
    }
}
