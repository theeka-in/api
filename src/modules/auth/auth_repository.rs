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
            r#"SELECT 
                token, 
                account_id, 
                user_agent, 
                ip_address, 
                created_at
               FROM auth.sessions
               WHERE token = $1"#,
            token,
        )
        .fetch_optional(&self.pg)
        .await?;

        Ok(session)
    }

    pub async fn find_account_and_session_by_token(
        &self,
        token: &str,
    ) -> Result<Option<(AccountEntity, SessionEntity)>, DbError> {
        let row = sqlx::query!(
            r#"SELECT
                   account.id           AS account_id, 
                   account.phone        AS account_phone, 
                   account.password     AS account_password, 
                   account.created_at   AS account_created_at,
                   session.token        AS session_token, 
                   session.user_agent   AS session_user_agent, 
                   session.ip_address   AS session_ip_address, 
                   session.created_at   AS session_created_at,
                   session.account_id   AS session_account_id
               FROM auth.sessions session
               JOIN auth.accounts account ON account.id = session.account_id
               WHERE session.token = $1"#,
            token,
        )
        .fetch_optional(&self.pg)
        .await?;

        Ok(row.map(|r| {
            (
                AccountEntity {
                    id: r.account_id,
                    phone: r.account_phone,
                    password: r.account_password,
                    created_at: r.account_created_at,
                },
                SessionEntity {
                    token: r.session_token,
                    user_agent: r.session_user_agent,
                    ip_address: r.session_ip_address,
                    created_at: r.session_created_at,
                    account_id: r.session_account_id,
                },
            )
        }))
    }

    pub async fn delete_session(&self, token: String) -> Result<(), DbError> {
        sqlx::query!(r#"DELETE FROM auth.sessions WHERE token = $1"#, token,)
            .execute(&self.pg)
            .await?;

        Ok(())
    }
}
