use crate::modules::auth::auth_dto::{AccountDto, LoginDto, RegisterDto, SessionDto};
use crate::modules::database::DatabaseService;
use crate::modules::users::{CreateUserDto, UsersService};
use crate::shared::errors::{ErrorDto, ServiceError};
use crate::shared::utils::{generate_token, hash_password, verify_password};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct AuthService {
    db: Arc<DatabaseService>,
    users: Arc<UsersService>,
}

impl AuthService {
    pub fn new(db: Arc<DatabaseService>, users_service: Arc<UsersService>) -> Arc<Self> {
        Arc::new(Self {
            db,
            users: users_service,
        })
    }

    pub async fn register(
        &self,
        body: RegisterDto,
        user_agent: String,
        ip_address: String,
    ) -> Result<SessionDto, ServiceError> {
        if self.db.account.find_by_phone(body.phone).await?.is_some() {
            return Err(ServiceError::Conflict(ErrorDto {
                message: "phone already exists".to_owned(),
            }));
        }

        let hashed = hash_password(&body.password)?;
        let token = generate_token();

        let account = self.db.account.create(body.phone, hashed).await?;

        let user = self
            .users
            .create(
                account.id,
                CreateUserDto {
                    name: body.name,
                    avatar: body.avatar,
                },
            )
            .await?;

        let session = self
            .db
            .session
            .create(account.id, user.id, token, user_agent, ip_address)
            .await?;

        Ok(session.into())
    }

    pub async fn login(
        &self,
        body: LoginDto,
        user_agent: String,
        ip_address: String,
    ) -> Result<SessionDto, ServiceError> {
        let account =
            self.db
                .account
                .find_by_phone(body.phone)
                .await?
                .ok_or(ServiceError::Unauthorized(ErrorDto {
                    message: "user doesn't exist".to_owned(),
                }))?;

        let is_valid = verify_password(&body.password, &account.password)?;

        if !is_valid {
            return Err(ServiceError::Unauthorized(ErrorDto {
                message: "invalid password".to_owned(),
            }));
        }

        let user = self.users.get(account.id).await?;

        let token = generate_token();
        let session = self
            .db
            .session
            .create(account.id, user.id, token, user_agent, ip_address)
            .await?;

        Ok(session.into())
    }

    pub async fn logout(&self, token: String) -> Result<(), ServiceError> {
        self.db.session.delete(token).await?;

        Ok(())
    }

    pub async fn get_session(&self, token: String) -> Result<SessionDto, ServiceError> {
        let session =
            self.db
                .session
                .find_by_token(token)
                .await?
                .ok_or(ServiceError::Unauthorized(ErrorDto {
                    message: "invalid session".to_owned(),
                }))?;

        Ok(SessionDto::from(session))
    }

    pub async fn get_account_by_user_id(&self, user_id: Uuid) -> Result<AccountDto, ServiceError> {
        let account =
            self.db
                .account
                .find_by_user_id(user_id)
                .await?
                .ok_or(ServiceError::Unauthorized(ErrorDto {
                    message: "invalid session".to_owned(),
                }))?;

        Ok(AccountDto::from(account))
    }

    pub async fn get_sessions(&self, account_id: Uuid) -> Result<Vec<SessionDto>, ServiceError> {
        let sessions = self.db.session.find_by_account(account_id).await?;

        Ok(sessions.into_iter().map(SessionDto::from).collect())
    }

    pub async fn delete_session(
        &self,
        account_id: Uuid,
        token: String,
    ) -> Result<(), ServiceError> {
        let session =
            self.db
                .session
                .find_by_token(token.clone())
                .await?
                .ok_or(ServiceError::NotFound(ErrorDto {
                    message: "session not found".to_owned(),
                }))?;

        if session.account_id != account_id {
            return Err(ServiceError::Forbidden(ErrorDto {
                message: "session does not belong to this account".to_owned(),
            }));
        }

        self.db.session.delete(token).await?;

        Ok(())
    }
}
