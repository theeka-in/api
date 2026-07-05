use crate::shared::errors::DbError;
use sqlx::types::chrono;
use sqlx::{prelude::FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct AccountEntity {
    pub id: Uuid,
    pub phone: i64,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug)]
pub struct AccountRepo {
    pg: PgPool,
}

impl AccountRepo {
    pub fn new(pg: PgPool) -> Self {
        Self { pg }
    }

    pub async fn create(&self, phone: i64, password: String) -> Result<AccountEntity, DbError> {
        let account = sqlx::query_as!(
            AccountEntity,
            r#"INSERT INTO accounts (id, phone, password)
               VALUES (gen_random_uuid(), $1, $2)
               RETURNING id, phone, password, created_at"#,
            phone,
            password,
        )
        .fetch_one(&self.pg)
        .await?;

        Ok(account)
    }

    pub async fn find_by_phone(&self, phone: i64) -> Result<Option<AccountEntity>, DbError> {
        let account = sqlx::query_as!(
            AccountEntity,
            r#"SELECT id, phone, password, created_at
               FROM accounts
               WHERE phone = $1"#,
            phone,
        )
        .fetch_optional(&self.pg)
        .await?;

        Ok(account)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<AccountEntity>, DbError> {
        let account = sqlx::query_as!(
            AccountEntity,
            r#"SELECT id, phone, password, created_at
               FROM accounts
               WHERE id = $1"#,
            id,
        )
        .fetch_optional(&self.pg)
        .await?;

        Ok(account)
    }

    pub async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<AccountEntity>, DbError> {
        let account = sqlx::query_as!(
            AccountEntity,
            r#"SELECT account.id, account.phone, account.password, account.created_at
               FROM accounts account
               INNER JOIN users u ON u.account_id = account.id
               WHERE u.id = $1"#,
            user_id,
        )
        .fetch_optional(&self.pg)
        .await?;

        Ok(account)
    }
}
