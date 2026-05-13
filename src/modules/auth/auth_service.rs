use crate::errors::{DbError, ServiceError};
use crate::modules::auth::auth_dto::{AccountDto, LoginDto, RegisterDto, SessionDto};
use crate::modules::auth::auth_repository::AuthRepository;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct AuthService {
    repo: AuthRepository,
}

impl AuthService {
    pub fn new(repo: AuthRepository) -> Arc<Self> {
        Arc::new(Self { repo })
    }

    pub async fn register(&self, body: RegisterDto) -> Result<AccountDto, ServiceError> {
        todo!()
    }

    pub async fn login(
        &self,
        body: LoginDto,
        user_agent: String,
        ip_address: String,
    ) -> Result<SessionDto, ServiceError> {
        todo!()
    }

    pub async fn logout(&self, token: String) -> Result<(), ServiceError> {
        todo!()
    }

    pub async fn get_sessions(&self, account_id: Uuid) -> Result<Vec<SessionDto>, ServiceError> {
        todo!()
    }

    pub async fn delete_session(
        &self,
        account_id: Uuid,
        token: String,
    ) -> Result<(), ServiceError> {
        todo!()
    }
}
