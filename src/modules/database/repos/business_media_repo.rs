use crate::shared::errors::DbError;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, sqlx::Type, PartialEq)]
#[sqlx(type_name = "media_type", rename_all = "snake_case")]
pub enum MediaType {
    Image,
    Video,
}

#[derive(Debug, FromRow)]
pub struct BusinessMediaEntity {
    pub id: Uuid,
    pub media_type: MediaType,
    pub url: String,
    pub business_id: Uuid,
}

#[derive(Debug)]
pub struct BusinessMediaRepo {
    pg: PgPool,
}

impl BusinessMediaRepo {
    pub fn new(pg: PgPool) -> Self {
        Self { pg }
    }

    pub async fn find_all(&self, business_id: Uuid) -> Result<Vec<BusinessMediaEntity>, DbError> {
        let media = sqlx::query_as!(
            BusinessMediaEntity,
            r#"SELECT id, type AS "media_type: _", url, business_id
               FROM business_media WHERE business_id = $1"#,
            business_id
        )
        .fetch_all(&self.pg)
        .await?;

        Ok(media)
    }

    pub async fn create(
        &self,
        business_id: Uuid,
        media_type: String,
        url: String,
    ) -> Result<BusinessMediaEntity, DbError> {
        let media = sqlx::query_as!(
            BusinessMediaEntity,
            r#"INSERT INTO business_media (id, business_id, type, url)
               VALUES (gen_random_uuid(), $1, $2::media_type, $3)
               RETURNING id, type AS "media_type: _", url, business_id"#,
            business_id,
            media_type as _,
            url
        )
        .fetch_one(&self.pg)
        .await?;

        Ok(media)
    }

    pub async fn delete(&self, id: Uuid, business_id: Uuid) -> Result<(), DbError> {
        sqlx::query!(
            "DELETE FROM business_media WHERE id = $1 AND business_id = $2",
            id,
            business_id
        )
        .execute(&self.pg)
        .await?;

        Ok(())
    }
}