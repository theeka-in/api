use sqlx::{PgPool, prelude::FromRow};
use uuid::Uuid;

use crate::shared::errors::DbError;

#[derive(Debug, FromRow)]
pub struct UserEntity {
    pub id: Uuid,
    pub name: String,
    pub avatar: Option<String>,
    pub account_id: Uuid,
}

#[derive(Debug)]
pub struct UserRepo {
    pg: PgPool,
}

impl UserRepo {
    pub fn new(pg: PgPool) -> Self {
        Self { pg }
    }

    pub async fn find_by_account(&self, account_id: Uuid) -> Result<Option<UserEntity>, DbError> {
        let user = sqlx::query_as!(
            UserEntity,
            r#"SELECT id, name, avatar, account_id
               FROM users
               WHERE account_id = $1"#,
            account_id
        )
        .fetch_optional(&self.pg)
        .await?;

        Ok(user)
    }

    pub async fn find(&self, id: Uuid) -> Result<Option<UserEntity>, DbError> {
        let user = sqlx::query_as!(
            UserEntity,
            r#"SELECT id, name, avatar, account_id
               FROM users
               WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pg)
        .await?;

        Ok(user)
    }

    pub async fn create(
        &self,
        account_id: Uuid,
        name: String,
        avatar: Option<String>,
    ) -> Result<UserEntity, DbError> {
        let user = sqlx::query_as!(
            UserEntity,
            r#"INSERT INTO users (id, account_id, name, avatar)
               VALUES (gen_random_uuid(), $1, $2, $3)
               RETURNING id, name, avatar, account_id"#,
            account_id,
            name,
            avatar
        )
        .fetch_one(&self.pg)
        .await?;

        Ok(user)
    }

    pub async fn update(
        &self,
        id: Uuid,
        name: Option<String>,
        avatar: Option<String>,
    ) -> Result<UserEntity, DbError> {
        let user = sqlx::query_as!(
            UserEntity,
            r#"UPDATE users
               SET
                   name   = COALESCE($2, name),
                   avatar = COALESCE($3, avatar)
               WHERE id = $1
               RETURNING id, name, avatar, account_id"#,
            id,
            name,
            avatar
        )
        .fetch_one(&self.pg)
        .await?;

        Ok(user)
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), DbError> {
        sqlx::query!(r#"DELETE FROM users WHERE id = $1"#, id)
            .execute(&self.pg)
            .await?;

        Ok(())
    }
}
