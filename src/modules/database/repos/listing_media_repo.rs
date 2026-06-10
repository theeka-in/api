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
pub struct ListingMediaEntity {
    pub id: Uuid,
    pub media_type: MediaType,
    pub url: String,
    pub listing_id: Uuid,
}

#[derive(Debug)]
pub struct ListingMediaRepo {
    pg: PgPool,
}

impl ListingMediaRepo {
    pub fn new(pg: PgPool) -> Self {
        Self { pg }
    }

    pub async fn find_all(&self, listing_id: Uuid) -> Result<Vec<ListingMediaEntity>, DbError> {
        let media = sqlx::query_as!(
            ListingMediaEntity,
            r#"SELECT id, type AS "media_type: _", url, listing_id
               FROM listing_media WHERE listing_id = $1"#,
            listing_id
        )
        .fetch_all(&self.pg)
        .await?;

        Ok(media)
    }

    pub async fn create(
        &self,
        listing_id: Uuid,
        media_type: String,
        url: String,
    ) -> Result<ListingMediaEntity, DbError> {
        let media = sqlx::query_as!(
            ListingMediaEntity,
            r#"INSERT INTO listing_media (id, listing_id, type, url)
               VALUES (gen_random_uuid(), $1, $2::media_type, $3)
               RETURNING id, type AS "media_type: _", url, listing_id"#,
            listing_id, media_type as _, url
        )
        .fetch_one(&self.pg)
        .await?;

        Ok(media)
    }

    pub async fn delete(&self, id: Uuid, listing_id: Uuid) -> Result<(), DbError> {
        sqlx::query!(
            "DELETE FROM listing_media WHERE id = $1 AND listing_id = $2",
            id, listing_id
        )
        .execute(&self.pg)
        .await?;

        Ok(())
    }
}