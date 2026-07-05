use crate::{modules::database::ListingEntity, shared::errors::DbError};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct ServiceListingEntity {
    pub id: Uuid,
    pub price: String,
    pub available: bool,
    pub service_listings_type_id: Uuid,
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
    ) -> Result<Vec<(ListingEntity, ServiceListingEntity)>, DbError> {
        let rows = sqlx::query!(
            r#"
            SELECT
                listing.id,
                listing.title,
                listing.description,
                listing.logo,
                listing.is_active,
                listing.created_at,
                listing.updated_at,
                listing.business_id,
                listing.product_listing_id,
                listing.service_listing_id,
                service_listing.id                       AS service_listing_id_alias,
                service_listing.price,
                service_listing.available,
                service_listing.service_listings_type_id
            FROM listings AS listing
            INNER JOIN service_listings AS service_listing
                ON service_listing.id = listing.service_listing_id
            WHERE listing.business_id = $1
            "#,
            business_id
        )
            .fetch_all(&self.pg)
            .await?;

        Ok(rows
            .into_iter()
            .map(|row| {
                (
                    ListingEntity {
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
                    },
                    ServiceListingEntity {
                        id: row.service_listing_id_alias,
                        price: row.price,
                        available: row.available,
                        service_listings_type_id: row.service_listings_type_id,
                    },
                )
            })
            .collect())
    }

    pub async fn find_by_id_and_business(
        &self,
        id: Uuid,
        business_id: Uuid,
    ) -> Result<Option<(ListingEntity, ServiceListingEntity)>, DbError> {
        let row = sqlx::query!(
            r#"
            SELECT
                listing.id,
                listing.title,
                listing.description,
                listing.logo,
                listing.is_active,
                listing.created_at,
                listing.updated_at,
                listing.business_id,
                listing.product_listing_id,
                listing.service_listing_id,
                service_listing.id AS service_listing_id_alias,
                service_listing.price,
                service_listing.available,
                service_listing.service_listings_type_id
            FROM listings AS listing
            INNER JOIN service_listings AS service_listing
                ON service_listing.id = listing.service_listing_id
            WHERE listing.id = $1 AND listing.business_id = $2
            "#,
            id,
            business_id
        )
            .fetch_optional(&self.pg)
            .await?;

        Ok(row.map(|row| {
            (
                ListingEntity {
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
                },
                ServiceListingEntity {
                    id: row.service_listing_id_alias,
                    price: row.price,
                    available: row.available,
                    service_listings_type_id: row.service_listings_type_id,
                },
            )
        }))
    }

    pub async fn find_by_id_and_business_and_owner(
        &self,
        id: Uuid,
        business_id: Uuid,
        owner_id: Uuid,
    ) -> Result<Option<(ListingEntity, ServiceListingEntity)>, DbError> {
        let row = sqlx::query!(
            r#"
            SELECT
                listing.id,
                listing.title,
                listing.description,
                listing.logo,
                listing.is_active,
                listing.created_at,
                listing.updated_at,
                listing.business_id,
                listing.product_listing_id,
                listing.service_listing_id,
                service_listing.id AS service_listing_id_alias,
                service_listing.price,
                service_listing.available,
                service_listing.service_listings_type_id
            FROM listings AS listing
            INNER JOIN service_listings AS service_listing
                ON service_listing.id = listing.service_listing_id
            INNER JOIN businesses AS business
                ON business.id = listing.business_id
            WHERE listing.id = $1 AND listing.business_id = $2 AND business.owner_id = $3
            "#,
            id,
            business_id,
            owner_id
        )
            .fetch_optional(&self.pg)
            .await?;

        Ok(row.map(|row| {
            (
                ListingEntity {
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
                },
                ServiceListingEntity {
                    id: row.service_listing_id_alias,
                    price: row.price,
                    available: row.available,
                    service_listings_type_id: row.service_listings_type_id,
                },
            )
        }))
    }

    pub async fn create(
        &self,
        business_id: Uuid,
        service_listing_type_id: Uuid,
        title: String,
        description: Option<String>,
        logo: Option<String>,
        price: String,
        available: bool,
    ) -> Result<(ListingEntity, ServiceListingEntity), DbError> {
        let mut tx = self.pg.begin().await?;

        let service = sqlx::query_as!(
            ServiceListingEntity,
            r#"
            INSERT INTO service_listings (id, price, available, service_listings_type_id)
            VALUES (gen_random_uuid(), $1, $2, $3)
            RETURNING id, price, available, service_listings_type_id
            "#,
            price,
            available,
            service_listing_type_id
        )
            .fetch_one(&mut *tx)
            .await?;

        let listing = sqlx::query_as!(
            ListingEntity,
            r#"
            INSERT INTO listings
                (id, business_id, title, description, logo, service_listing_id, updated_at)
            VALUES (gen_random_uuid(), $1, $2, $3, $4, $5, now())
            RETURNING id, title, description, logo, is_active, created_at, updated_at,
                      business_id, product_listing_id, service_listing_id
            "#,
            business_id,
            title,
            description,
            logo,
            service.id,
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
        service_listing_type_id: Option<Uuid>,
        title: Option<String>,
        description: Option<String>,
        logo: Option<String>,
        is_active: Option<bool>,
        price: Option<String>,
        available: Option<bool>,
    ) -> Result<(ListingEntity, ServiceListingEntity), DbError> {
        let mut tx = self.pg.begin().await?;

        let listing = sqlx::query_as!(
            ListingEntity,
            r#"
            UPDATE listings SET
                title       = COALESCE($3, title),
                description = COALESCE($4, description),
                logo        = COALESCE($5, logo),
                is_active   = COALESCE($6, is_active),
                updated_at  = now()
            WHERE id = $1 AND business_id = $2
            RETURNING id, title, description, logo, is_active, created_at, updated_at,
                      business_id, product_listing_id, service_listing_id
            "#,
            listing_id,
            business_id,
            title,
            description,
            logo,
            is_active,
        )
            .fetch_one(&mut *tx)
            .await?;

        let service = sqlx::query_as!(
            ServiceListingEntity,
            r#"
            UPDATE service_listings SET
                price                    = COALESCE($2, price),
                available                = COALESCE($3, available),
                service_listings_type_id  = COALESCE($4, service_listings_type_id)
            WHERE id = $1
            RETURNING id, price, available, service_listings_type_id
            "#,
            listing.service_listing_id,
            price,
            available,
            service_listing_type_id
        )
            .fetch_one(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok((listing, service))
    }

    pub async fn delete(&self, id: Uuid, business_id: Uuid) -> Result<(), DbError> {
        sqlx::query!(
            r#"
            DELETE FROM listings
            WHERE id = $1 AND business_id = $2 AND service_listing_id IS NOT NULL
            "#,
            id,
            business_id
        )
            .execute(&self.pg)
            .await?;

        Ok(())
    }
}