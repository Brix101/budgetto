use std::ops::Add;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::{Duration, SystemTime};

use budgetto_core::config::AppConfig;
use budgetto_core::errors::{AppResult, Error};
use budgetto_core::utils::token_service::TokenService;
use budgetto_domain::users::UserDto;
use budgetto_domain::utils::token::{AccessTokenClaims, RefreshTokenClaims};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;

lazy_static! {
    static ref TOKEN_BLACKLIST: Mutex<Vec<String>> = Mutex::new(vec![]);
}

pub struct JwtService {
    config: Arc<AppConfig>,
}

impl JwtService {
    pub fn new(config: Arc<AppConfig>) -> Self {
        Self { config }
    }
}

impl TokenService for JwtService {
    fn new_access_token(&self, sub: Uuid, user: UserDto) -> AppResult<String> {
        let from_now = Duration::from_secs(900); //? expires every 15 min
        let expired_future_time = SystemTime::now().add(from_now);
        let exp = OffsetDateTime::from(expired_future_time);
        let now = OffsetDateTime::now_utc();

        let claims = AccessTokenClaims {
            sub,
            user,
            exp: exp.unix_timestamp() as usize,
            iat: now.unix_timestamp() as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.access_token_secret.as_bytes()),
        )
        .map_err(|err| Error::InternalServerErrorWithContext(err.to_string()))?;

        Ok(token)
    }

    fn new_refresh_token(&self, sub: Uuid) -> AppResult<String> {
        let exp_time = 3600 * 24 * 7 * 4; // expires in 1 month
        let from_now = Duration::from_secs(exp_time);
        let expired_future_time = SystemTime::now().add(from_now);
        let exp = OffsetDateTime::from(expired_future_time);
        let now = OffsetDateTime::now_utc();

        let claims = RefreshTokenClaims {
            sub,
            exp: exp.unix_timestamp() as usize,
            iat: now.unix_timestamp() as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.refresh_token_secret.as_bytes()),
        )
        .map_err(|err| Error::InternalServerErrorWithContext(err.to_string()))?;

        Ok(token)
    }

    fn verify_access_token(&self, token: &str) -> AppResult<AccessTokenClaims> {
        let decoded_token = decode::<AccessTokenClaims>(
            token,
            &DecodingKey::from_secret(self.config.access_token_secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|err| Error::InternalServerErrorWithContext(err.to_string()))?;

        // let blacklist = TOKEN_BLACKLIST.lock().unwrap().to_vec();
        //
        // if blacklist.contains(&decoded_token.claims.sub.to_string()) {
        //     Err(Error::InternalServerErrorWithContext(
        //         "Blacklisted Token".to_string(),
        //     ))
        // } else {
        //     Ok(decoded_token.claims)
        // }
        Ok(decoded_token.claims)
    }

    fn verify_refresh_token(&self, token: &str) -> AppResult<RefreshTokenClaims> {
        let decoded_token = decode::<RefreshTokenClaims>(
            token,
            &DecodingKey::from_secret(self.config.refresh_token_secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|err| Error::InternalServerErrorWithContext(err.to_string()))?;

        Ok(decoded_token.claims)
    }

    fn get_session_id_from_token(&self, token: String) -> AppResult<Uuid> {
        let decoded_token = decode::<RefreshTokenClaims>(
            token.as_str(),
            &DecodingKey::from_secret(self.config.refresh_token_secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|err| Error::InternalServerErrorWithContext(err.to_string()))?;

        Ok(decoded_token.claims.sub)
    }

    fn refresh_blacklist(&self) {
        TOKEN_BLACKLIST.lock().unwrap().clear();
    }

    fn add_blacklist(&self, id: Uuid) {
        TOKEN_BLACKLIST.lock().unwrap().push(id.to_string());
    }
}
