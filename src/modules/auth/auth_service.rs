use crate::errors::{ErrorDto, ServiceError};
use crate::modules::auth::auth_dto::{AccountDto, LoginDto, RegisterDto, SessionDto};
use crate::modules::auth::auth_repository::AuthRepository;
use crate::modules::users::{CreateUserDto, UsersService};
use crate::utils::{generate_token, hash_password, verify_password};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct AuthService {
    repo: AuthRepository,
    users: Arc<UsersService>,
}

impl AuthService {
    pub fn new(repo: AuthRepository, users_service: Arc<UsersService>) -> Arc<Self> {
        Arc::new(Self {
            repo,
            users: users_service,
        })
    }

    pub async fn register(
        &self,
        body: RegisterDto,
        user_agent: String,
        ip_address: String,
    ) -> Result<SessionDto, ServiceError> {
        if self.repo.find_account_by_phone(body.phone).await?.is_some() {
            return Err(ServiceError::Conflict(ErrorDto {
                message: "phone already exists".to_owned(),
            }));
        }

        let hashed = hash_password(&body.password)?;
        let token = generate_token();

        let account = self.repo.create_account(body.phone, hashed).await?;

        let (session, _) = tokio::try_join!(
            async {
                self.repo
                    .create_session(account.id, token, user_agent, ip_address)
                    .await
                    .map_err(ServiceError::from)
            },
            self.users.create(
                account.id,
                CreateUserDto {
                    name: body.name,
                    avatar: body.avatar,
                },
            )
        )?;

        Ok(session.into())
    }

    pub async fn login(
        &self,
        body: LoginDto,
        user_agent: String,
        ip_address: String,
    ) -> Result<SessionDto, ServiceError> {
        let account = self.repo.find_account_by_phone(body.phone).await?.ok_or(
            ServiceError::Unauthorized(ErrorDto {
                message: "invalid credentials".to_owned(),
            }),
        )?;

        let is_valid = verify_password(&body.password, &account.password)?;

        if !is_valid {
            return Err(ServiceError::Unauthorized(ErrorDto {
                message: "invalid credentials".to_owned(),
            }));
        }

        let token = generate_token();
        let session = self
            .repo
            .create_session(account.id, token, user_agent, ip_address)
            .await?;

        Ok(session.into())
    }

    pub async fn logout(&self, token: String) -> Result<(), ServiceError> {
        self.repo.delete_session(token).await?;

        Ok(())
    }

    pub async fn get_sessions(&self, account_id: Uuid) -> Result<Vec<SessionDto>, ServiceError> {
        let sessions = self.repo.find_sessions_by_account(account_id).await?;

        Ok(sessions.into_iter().map(Into::into).collect())
    }

    pub async fn delete_session(
        &self,
        account_id: Uuid,
        token: String,
    ) -> Result<(), ServiceError> {
        let session = self
            .repo
            .find_session_by_token(token.clone())
            .await?
            .ok_or(ServiceError::NotFound(ErrorDto {
                message: "session not found".to_owned(),
            }))?;

        if session.account_id != account_id {
            return Err(ServiceError::Forbidden(ErrorDto {
                message: "session does not belong to this account".to_owned(),
            }));
        }

        self.repo.delete_session(token).await?;

        Ok(())
    }
}
