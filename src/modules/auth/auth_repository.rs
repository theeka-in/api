use crate::errors::DbError;
use crate::modules::auth::auth_entity::{AccountEntity, SessionEntity};
use sqlx::PgPool;
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
        let account = sqlx::query_as!(
            AccountEntity,
            r#"INSERT INTO auth.accounts (id, phone, password)
               VALUES (gen_random_uuid(), $1, $2)
               RETURNING id, phone, password, created_at"#,
            phone,
            password,
        )
        .fetch_one(&self.pg)
        .await?;

        Ok(account)
    }

    pub async fn find_account_by_phone(
        &self,
        phone: i64,
    ) -> Result<Option<AccountEntity>, DbError> {
        let account = sqlx::query_as!(
            AccountEntity,
            r#"SELECT id, phone, password, created_at
               FROM auth.accounts
               WHERE phone = $1"#,
            phone,
        )
        .fetch_optional(&self.pg)
        .await?;

        Ok(account)
    }

    pub async fn find_account_by_id(&self, id: Uuid) -> Result<Option<AccountEntity>, DbError> {
        let account = sqlx::query_as!(
            AccountEntity,
            r#"SELECT id, phone, password, created_at
               FROM auth.accounts
               WHERE id = $1"#,
            id,
        )
        .fetch_optional(&self.pg)
        .await?;

        Ok(account)
    }

    pub async fn create_session(
        &self,
        account_id: Uuid,
        token: String,
        user_agent: String,
        ip_address: String,
    ) -> Result<SessionEntity, DbError> {
        let session = sqlx::query_as!(
            SessionEntity,
            r#"INSERT INTO auth.sessions (token, account_id, user_agent, ip_address)
               VALUES ($1, $2, $3, $4)
               RETURNING token, account_id, user_agent, ip_address, created_at"#,
            token,
            account_id,
            user_agent,
            ip_address,
        )
        .fetch_one(&self.pg)
        .await?;

        Ok(session)
    }

    pub async fn find_sessions_by_account(
        &self,
        account_id: Uuid,
    ) -> Result<Vec<SessionEntity>, DbError> {
        let sessions = sqlx::query_as!(
            SessionEntity,
            r#"SELECT token, account_id, user_agent, ip_address, created_at
               FROM auth.sessions
               WHERE account_id = $1"#,
            account_id,
        )
        .fetch_all(&self.pg)
        .await?;

        Ok(sessions)
    }

    pub async fn find_session_by_token(
        &self,
        token: String,
    ) -> Result<Option<SessionEntity>, DbError> {
        let session = sqlx::query_as!(
            SessionEntity,
            r#"SELECT token, account_id, user_agent, ip_address, created_at
               FROM auth.sessions
               WHERE token = $1"#,
            token,
        )
        .fetch_optional(&self.pg)
        .await?;

        Ok(session)
    }

    pub async fn delete_session(&self, token: String) -> Result<(), DbError> {
        sqlx::query!(r#"DELETE FROM auth.sessions WHERE token = $1"#, token,)
            .execute(&self.pg)
            .await?;

        Ok(())
    }
}
