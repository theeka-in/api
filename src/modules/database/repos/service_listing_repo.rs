use crate::{modules::database::BusinessListingEntity, shared::errors::DbError};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct ServiceListingEntity {
    pub id: Uuid,
    pub price: String,
    pub available: bool,
}

#[derive(Debug)]
pub struct ServiceListingRepo {
    pg: PgPool,
}

impl ServiceListingRepo {
    pub fn new(pg: PgPool) -> Self {
        Self { pg }
    }

    pub async fn find_all_by_business(
        &self,
        business_id: Uuid,
    ) -> Result<Vec<(BusinessListingEntity, ServiceListingEntity)>, DbError> {
        let rows = sqlx::query!(
            r#"SELECT
                bl.id, bl.title, bl.description, bl.logo, bl.is_active,
                bl.created_at, bl.updated_at, bl.business_id,
                bl.product_listing_id, bl.service_listing_id,
                bl.embedding AS "embedding: pgvector::Vector",
                sl.id as sl_id, sl.price, sl.available
               FROM business_listings bl
               INNER JOIN service_listings sl ON sl.id = bl.service_listing_id
               WHERE bl.business_id = $1"#,
            business_id
        )
        .fetch_all(&self.pg)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| {
                (
                    BusinessListingEntity {
                        id: row.id,
                        title: row.title,
                        description: row.description,
                        logo: row.logo,
                        is_active: row.is_active,
                        created_at: row.created_at,
                        updated_at: row.updated_at,
                        business_id: row.business_id,
                        product_listing_id: row.product_listing_id,
                        service_listing_id: row.service_listing_id,
                        embedding: row.embedding,
                    },
                    ServiceListingEntity {
                        id: row.sl_id,
                        price: row.price,
                        available: row.available,
                    },
                )
            })
            .collect())
    }

    pub async fn find_by_id_and_business(
        &self,
        id: Uuid,
        business_id: Uuid,
    ) -> Result<Option<(BusinessListingEntity, ServiceListingEntity)>, DbError> {
        let row = sqlx::query!(
            r#"SELECT
                bl.id, bl.title, bl.description, bl.logo, bl.is_active,
                bl.created_at, bl.updated_at, bl.business_id,
                bl.product_listing_id, bl.service_listing_id,
                bl.embedding AS "embedding: pgvector::Vector",
                sl.id as sl_id, sl.price, sl.available
               FROM business_listings bl
               INNER JOIN service_listings sl ON sl.id = bl.service_listing_id
               WHERE bl.id = $1 AND bl.business_id = $2"#,
            id,
            business_id
        )
        .fetch_optional(&self.pg)
        .await?;

        Ok(row.map(|row| {
            (
                BusinessListingEntity {
                    id: row.id,
                    title: row.title,
                    description: row.description,
                    logo: row.logo,
                    is_active: row.is_active,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                    business_id: row.business_id,
                    product_listing_id: row.product_listing_id,
                    service_listing_id: row.service_listing_id,
                    embedding: row.embedding,
                },
                ServiceListingEntity {
                    id: row.sl_id,
                    price: row.price,
                    available: row.available,
                },
            )
        }))
    }

    pub async fn find_by_id_and_business_and_owner(
        &self,
        id: Uuid,
        business_id: Uuid,
        owner_id: Uuid,
    ) -> Result<Option<(BusinessListingEntity, ServiceListingEntity)>, DbError> {
        let row = sqlx::query!(
            r#"SELECT
                bl.id, bl.title, bl.description, bl.logo, bl.is_active,
                bl.created_at, bl.updated_at, bl.business_id,
                bl.product_listing_id, bl.service_listing_id,
                bl.embedding AS "embedding: pgvector::Vector",
                sl.id as sl_id, sl.price, sl.available
               FROM business_listings bl
               INNER JOIN service_listings sl ON sl.id = bl.service_listing_id
               INNER JOIN businesses bb ON bb.id = bl.business_id
               WHERE bl.id = $1 AND bl.business_id = $2 AND bb.owner_id = $3"#,
            id,
            business_id,
            owner_id
        )
        .fetch_optional(&self.pg)
        .await?;

        Ok(row.map(|row| {
            (
                BusinessListingEntity {
                    id: row.id,
                    title: row.title,
                    description: row.description,
                    logo: row.logo,
                    is_active: row.is_active,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                    business_id: row.business_id,
                    product_listing_id: row.product_listing_id,
                    service_listing_id: row.service_listing_id,
                    embedding: row.embedding,
                },
                ServiceListingEntity {
                    id: row.sl_id,
                    price: row.price,
                    available: row.available,
                },
            )
        }))
    }

    pub async fn create(
        &self,
        business_id: Uuid,
        title: String,
        description: Option<String>,
        logo: Option<String>,
        price: String,
        available: bool,
        embedding: pgvector::Vector,
    ) -> Result<(BusinessListingEntity, ServiceListingEntity), DbError> {
        let mut tx = self.pg.begin().await?;

        let service = sqlx::query_as!(
            ServiceListingEntity,
            r#"INSERT INTO service_listings (id, price, available)
               VALUES (gen_random_uuid(), $1, $2)
               RETURNING id, price, available"#,
            price,
            available
        )
        .fetch_one(&mut *tx)
        .await?;

        let listing = sqlx::query_as!(
            BusinessListingEntity,
            r#"INSERT INTO business_listings
                   (id, business_id, title, description, logo, service_listing_id, updated_at, embedding)
               VALUES (gen_random_uuid(), $1, $2, $3, $4, $5, now(), $6)
               RETURNING id, title, description, logo, is_active, created_at, updated_at,
                         business_id, product_listing_id, service_listing_id,
                         embedding AS "embedding: pgvector::Vector""#,
            business_id, title, description, logo, service.id, embedding as _,
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok((listing, service))
    }

    pub async fn update(
        &self,
        listing_id: Uuid,
        business_id: Uuid,
        title: Option<String>,
        description: Option<String>,
        logo: Option<String>,
        is_active: Option<bool>,
        price: Option<String>,
        available: Option<bool>,
        embedding: Option<pgvector::Vector>,
    ) -> Result<(BusinessListingEntity, ServiceListingEntity), DbError> {
        let mut tx = self.pg.begin().await?;

        let listing = sqlx::query_as!(
            BusinessListingEntity,
            r#"UPDATE business_listings SET
               title = COALESCE($3, title),
               description = COALESCE($4, description),
               logo = COALESCE($5, logo),
               is_active = COALESCE($6, is_active),
               embedding = COALESCE($7, embedding),
               updated_at = now()
               WHERE id = $1 AND business_id = $2
               RETURNING id, title, description, logo, is_active, created_at, updated_at,
                         business_id, product_listing_id, service_listing_id,
                         embedding AS "embedding: pgvector::Vector""#,
            listing_id,
            business_id,
            title,
            description,
            logo,
            is_active,
            embedding as _,
        )
        .fetch_one(&mut *tx)
        .await?;

        let service = sqlx::query_as!(
            ServiceListingEntity,
            r#"UPDATE service_listings SET
               price = COALESCE($2, price),
               available = COALESCE($3, available)
               WHERE id = $1
               RETURNING id, price, available"#,
            listing.service_listing_id,
            price,
            available
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok((listing, service))
    }

    pub async fn delete(&self, id: Uuid, business_id: Uuid) -> Result<(), DbError> {
        sqlx::query!(
            "DELETE FROM business_listings WHERE id = $1 AND business_id = $2 AND service_listing_id IS NOT NULL",
            id, business_id
        )
        .execute(&self.pg)
        .await?;

        Ok(())
    }
}
