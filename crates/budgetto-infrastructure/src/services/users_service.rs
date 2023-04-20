use async_trait::async_trait;
use budgetto_domain::{
    sessions::requests::NewSessionDto,
    users::{
        requests::{SignInUserDto, SignUpUserDto, UpdateUserDto},
        UserDto,
    },
};
use tracing::{error, info};

use budgetto_core::{
    errors::{AppResult, Error},
    sessions::service::DynSessionsService,
    users::{repository::DynUsersRepository, service::UsersService},
    utils::{security_service::DynSecurityService, token_service::DynTokenService},
};
use uuid::Uuid;

#[derive(Clone)]
pub struct BudgettoUsersService {
    repository: DynUsersRepository,
    security_service: DynSecurityService,
    token_service: DynTokenService,
    session_service: DynSessionsService,
}

impl BudgettoUsersService {
    pub fn new(
        repository: DynUsersRepository,
        security_service: DynSecurityService,
        token_service: DynTokenService,
        session_service: DynSessionsService,
    ) -> Self {
        Self {
            repository,
            security_service,
            token_service,
            session_service,
        }
    }
}

#[async_trait]
impl UsersService for BudgettoUsersService {
    async fn signup_user(&self, request: SignUpUserDto) -> AppResult<UserDto> {
        let email = request.email.unwrap();
        let name = request.name.unwrap();
        let password = request.password.unwrap();

        let existing_user = self.repository.get_user_by_email(&email).await?;

        if existing_user.is_some() {
            error!("user {:?} already exists", email);
            return Err(Error::ObjectConflict(format!("email {} is taken", email)));
        }

        info!("creating password hash for user {:?}", email);
        let hashed_password = self.security_service.hash_password(&password)?;

        info!("password hashed successfully, creating user {:?}", email);
        let created_user = self
            .repository
            .create_user(&email, &name, &hashed_password)
            .await?;

        Ok(created_user.into_dto(String::new()))
    }

    async fn signin_user(
        &self,
        request: SignInUserDto,
        user_agent: Option<String>,
    ) -> AppResult<(UserDto, String)> {
        let email = request.email.unwrap();
        let attempted_password = request.password.unwrap();

        info!("searching for existing user {:?}", email);
        let existing_user = self.repository.get_user_by_email(&email).await?;

        if existing_user.is_none() {
            return Err(Error::NotFound(String::from("user email does not exist")));
        }

        let user = existing_user.unwrap();

        info!("user found, verifying password hash for user {:?}", email);
        let is_valid_login_attempt = self
            .security_service
            .verify_password(&user.password, attempted_password)?;

        if !is_valid_login_attempt {
            error!("invalid login attempt for user {:?}", email);
            return Err(Error::InvalidLoginAttmpt);
        }

        if user_agent.is_some() {
            info!("user login successful, generating token");
        }

        let token = self
            .session_service
            .new_session(NewSessionDto {
                user_id: Some(user.id),
                user_agent,
            })
            .await
            .unwrap();

        Ok((user.into_dto(token.access_token), token.refresh_token))
    }

    async fn get_current_user(&self, user_id: Uuid) -> AppResult<UserDto> {
        info!("retrieving user {:?}", user_id);
        let user = self.repository.get_user_by_id(user_id).await?;

        info!(
            "user found with email {:?}, generating new token",
            user.email
        );
        let token = self
            .token_service
            .new_access_token(user.id, user.email.as_str())?;

        Ok(user.into_dto(token))
    }

    async fn updated_user(&self, user_id: Uuid, request: UpdateUserDto) -> AppResult<UserDto> {
        info!("retrieving user {:?}", user_id);
        let user = self.repository.get_user_by_id(user_id).await?;

        let updated_email = request.email.unwrap_or(user.email);
        let updated_name = request.name.unwrap_or(user.name);
        let updated_bio = request.bio.unwrap_or(user.bio);
        let updated_image = request.image.unwrap_or(user.image);
        let mut updated_hashed_password = user.password;

        // if the password is included on the request, hash it and update the stored password
        if request.password.is_some() && !request.password.as_ref().unwrap().is_empty() {
            info!(
                "new password found for user {:?}, hashing password",
                user_id
            );
            updated_hashed_password = self
                .security_service
                .hash_password(request.password.unwrap().as_str())?;
        }

        info!("updating user {:?}", user_id);
        let updated_user = self
            .repository
            .update_user(
                user_id,
                updated_email.clone(),
                updated_name,
                updated_hashed_password,
                updated_bio,
                updated_image,
            )
            .await?;

        info!("user {:?} updated, generating a new token", user_id);
        let token = self
            .token_service
            .new_access_token(user_id, updated_email.as_str())?;

        Ok(updated_user.into_dto(token))
    }

    async fn get_user_by_email(&self, email: String) -> AppResult<UserDto> {
        info!("retrieving user {:?}", email);
        let user = self.repository.get_user_by_email(&email).await?;

        if user.is_some() {
            info!("user found with email {:?}", email);
        }

        Ok(user.unwrap().into_dto(String::new()))
    }
}
