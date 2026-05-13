use crate::errors::DbError;
use crate::modules::auth::auth_entity::{AccountEntity, SessionEntity};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct AuthRepository {
    pg: PgPool,
}

impl AuthRepository {
    pub fn new(pg: PgPool) -> Self {
        Self { pg }
    }

    pub async fn create_account(
        &self,
        phone: i64,
        password: String,
    ) -> Result<AccountEntity, DbError> {
        todo!()
    }

    pub async fn find_account_by_phone(
        &self,
        phone: i64,
    ) -> Result<Option<AccountEntity>, DbError> {
        todo!()
    }

    pub async fn find_account_by_id(&self, id: Uuid) -> Result<Option<AccountEntity>, DbError> {
        todo!()
    }

    pub async fn create_session(
        &self,
        account_id: Uuid,
        token: String,
        user_agent: String,
        ip_address: String,
    ) -> Result<SessionEntity, DbError> {
        todo!()
    }

    pub async fn find_sessions_by_account(
        &self,
        account_id: Uuid,
    ) -> Result<Vec<SessionEntity>, DbError> {
        todo!()
    }

    pub async fn delete_session(&self, token: String) -> Result<(), DbError> {
        todo!()
    }

    pub async fn find_session_by_token(
        &self,
        token: String,
    ) -> Result<Option<SessionEntity>, DbError> {
        todo!()
    }
}
