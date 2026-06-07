use crate::{modules::database::repos::account_repo::AccountEntity, shared::errors::DbError};
use sqlx::types::chrono;
use sqlx::{PgPool, prelude::FromRow};
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct SessionEntity {
    pub token: String,
    pub user_agent: String,
    pub ip_address: String,
    pub account_id: Uuid,
    pub user_id: Uuid,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug)]
pub struct SessionRepo {
    pg: PgPool,
}

impl SessionRepo {
    pub fn new(pg: PgPool) -> Self {
        Self { pg }
    }

    pub async fn create(
        &self,
        account_id: Uuid,
        user_id: Uuid,
        token: String,
        user_agent: String,
        ip_address: String,
    ) -> Result<SessionEntity, DbError> {
        let session = sqlx::query_as!(
            SessionEntity,
            r#"INSERT INTO auth.sessions (token, account_id, user_id, user_agent, ip_address)
               VALUES ($1, $2, $3, $4, $5)
               RETURNING token, account_id, user_id, user_agent, ip_address, created_at"#,
            token,
            account_id,
            user_id,
            user_agent,
            ip_address,
        )
        .fetch_one(&self.pg)
        .await?;

        Ok(session)
    }

    pub async fn find_by_account(&self, account_id: Uuid) -> Result<Vec<SessionEntity>, DbError> {
        let sessions = sqlx::query_as!(
            SessionEntity,
            r#"SELECT token, account_id, user_id, user_agent, ip_address, created_at
               FROM auth.sessions
               WHERE account_id = $1"#,
            account_id,
        )
        .fetch_all(&self.pg)
        .await?;

        Ok(sessions)
    }

    pub async fn find_by_token(&self, token: String) -> Result<Option<SessionEntity>, DbError> {
        let session = sqlx::query_as!(
            SessionEntity,
            r#"SELECT token, account_id, user_id, user_agent, ip_address, created_at
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
                   session.account_id   AS session_account_id,
                   session.user_id      AS session_user_id
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
                    user_id: r.session_user_id,
                },
            )
        }))
    }

    pub async fn delete(&self, token: String) -> Result<(), DbError> {
        sqlx::query!(r#"DELETE FROM auth.sessions WHERE token = $1"#, token)
            .execute(&self.pg)
            .await?;

        Ok(())
    }
}
