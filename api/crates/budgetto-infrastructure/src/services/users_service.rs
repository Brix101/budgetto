use async_trait::async_trait;
use budgetto_domain::{
    sessions::{requests::CreateSessionDto, responses::SessionResponse},
    users::{
        requests::{SignInUserDto, SignUpUserDto, UpdateUserDto},
        UserDto,
    },
};
use tracing::{error, info};

use budgetto_core::{
    errors::{AppResult, Error},
    sessions::service::DynSessionsService,
    users::{
        repository::{CreateUser, DynUsersRepository, UpdateUser},
        service::UsersService,
    },
    utils::security_service::DynSecurityService,
};
use uuid::Uuid;

#[derive(Clone)]
pub struct BudgettoUsersService {
    repository: DynUsersRepository,
    security_service: DynSecurityService,
    session_service: DynSessionsService,
}

impl BudgettoUsersService {
    pub fn new(
        repository: DynUsersRepository,
        security_service: DynSecurityService,
        session_service: DynSessionsService,
    ) -> Self {
        Self {
            repository,
            security_service,
            session_service,
        }
    }
}

#[async_trait]
impl UsersService for BudgettoUsersService {
    async fn signup_user(&self, request: SignUpUserDto) -> AppResult<UserDto> {
        let email = request.email.unwrap();
        let name = request.name.unwrap();
        let req_password = request.password.unwrap();

        let existing_user = self.repository.find_by_email(&email).await?;

        if existing_user.is_some() {
            error!("user {:?} already exists", email);
            return Err(Error::ObjectConflict(format!("email {} is taken", email)));
        }

        info!("creating password hash for user {:?}", email);
        let password = self.security_service.hash_password(&req_password)?;

        info!("password hashed successfully, creating user {:?}", email);
        let created_user = self
            .repository
            .create(CreateUser {
                email,
                name,
                password,
            })
            .await?;

        Ok(created_user.into_dto())
    }

    async fn signin_user(
        &self,
        request: SignInUserDto,
        user_agent: Option<String>,
    ) -> AppResult<SessionResponse> {
        let email = request.email.unwrap();
        let attempted_password = request.password.unwrap();

        info!("searching for existing user {:?}", email);
        let existing_user = self.repository.find_by_email(&email).await?;

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

        let tokens = self
            .session_service
            .create(CreateSessionDto {
                user_id: Some(user.id),
                user_agent,
            })
            .await
            .unwrap();

        Ok(tokens)
    }

    async fn get_current_user(&self, user_id: Uuid) -> AppResult<UserDto> {
        info!("retrieving user {:?}", user_id);
        let user = self.repository.find_by_id(user_id).await?;

        info!(
            "user found with email {:?}, generating new token",
            user.email
        );

        Ok(user.into_dto())
    }

    async fn updated_user(&self, request: UpdateUserDto) -> AppResult<UserDto> {
        let user_id = request.id.unwrap();

        info!("retrieving user {:?}", user_id);
        let user = self.repository.find_by_id(user_id).await?;

        let updated_email = request.email.unwrap_or(user.email);
        let updated_name = request.name.unwrap_or(user.name);
        let updated_bio = request.bio.unwrap_or(user.bio);
        let updated_image = request.image.unwrap_or(user.image);
        let updated_password = match request.password {
            Some(password) => self.security_service.hash_password(&password).unwrap(),
            None => user.password,
        };

        info!("updating user {:?}", user_id);
        let updated_user = self
            .repository
            .update(UpdateUser {
                id: user_id,
                email: updated_email,
                name: updated_name,
                password: updated_password,
                bio: updated_bio,
                image: updated_image,
            })
            .await?;

        Ok(updated_user.into_dto())
    }

    async fn get_user_by_email(&self, email: String) -> AppResult<UserDto> {
        info!("retrieving user {:?}", email);
        let user = self.repository.find_by_email(&email).await?;

        if user.is_some() {
            info!("user found with email {:?}", email);
        }

        Ok(user.unwrap().into_dto())
    }
}
