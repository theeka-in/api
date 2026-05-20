use crate::errors::DbError;
use crate::modules::listing::listing_entity::{
    BusinessListingEntity, ListingMediaEntity, ProductListingEntity, ServiceListingEntity,
};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug)]
pub struct ListingRepository {
    pg: PgPool,
}

impl ListingRepository {
    pub fn new(pg: PgPool) -> Self {
        Self { pg }
    }

    pub async fn find_all_products_listings_by_business(
        &self,
        business_id: Uuid,
    ) -> Result<Vec<(BusinessListingEntity, ProductListingEntity)>, DbError> {
        let rows = sqlx::query!(
            r#"SELECT
                bl.id, bl.title, bl.description, bl.logo, bl.is_active,
                bl.created_at, bl.updated_at, bl.business_id,
                bl.product_listing_id, bl.service_listing_id,
                pl.id as pl_id, pl.price, pl.stock
               FROM listing.business_listings bl
               INNER JOIN listing.product_listings pl ON pl.id = bl.product_listing_id
               WHERE bl.business_id = $1"#,
            business_id
        )
        .fetch_all(&self.pg)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| {
                let listing = BusinessListingEntity {
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
                };
                let product = ProductListingEntity {
                    id: row.pl_id,
                    price: row.price,
                    stock: row.stock,
                };
                (listing, product)
            })
            .collect())
    }

    pub async fn find_product_listing_by_id_and_business(
        &self,
        id: Uuid,
        business_id: Uuid,
    ) -> Result<Option<(BusinessListingEntity, ProductListingEntity)>, DbError> {
        let row = sqlx::query!(
            r#"SELECT
                bl.id, bl.title, bl.description, bl.logo, bl.is_active,
                bl.created_at, bl.updated_at, bl.business_id,
                bl.product_listing_id, bl.service_listing_id,
                pl.id as pl_id, pl.price, pl.stock
               FROM listing.business_listings bl
               INNER JOIN listing.product_listings pl ON pl.id = bl.product_listing_id
               WHERE bl.id = $1 AND bl.business_id = $2"#,
            id,
            business_id
        )
        .fetch_optional(&self.pg)
        .await?;

        Ok(row.map(|row| {
            let listing = BusinessListingEntity {
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
            };
            let product = ProductListingEntity {
                id: row.pl_id,
                price: row.price,
                stock: row.stock,
            };
            (listing, product)
        }))
    }

    pub async fn find_product_listing_by_id_and_business_and_owner(
        &self,
        id: Uuid,
        business_id: Uuid,
        owner_id: Uuid,
    ) -> Result<Option<(BusinessListingEntity, ProductListingEntity)>, DbError> {
        let row = sqlx::query!(
            r#"SELECT
                bl.id, bl.title, bl.description, bl.logo, bl.is_active,
                bl.created_at, bl.updated_at, bl.business_id,
                bl.product_listing_id, bl.service_listing_id,
                pl.id as pl_id, pl.price, pl.stock
               FROM listing.business_listings bl
               INNER JOIN listing.product_listings pl ON pl.id = bl.product_listing_id
               INNER JOIN business.businesses bb ON bb.id = bl.business_id
               WHERE bl.id = $1 AND bl.business_id = $2 AND bb.owner_id = $3"#,
            id,
            business_id,
            owner_id
        )
        .fetch_optional(&self.pg)
        .await?;

        Ok(row.map(|row| {
            let listing = BusinessListingEntity {
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
            };
            let product = ProductListingEntity {
                id: row.pl_id,
                price: row.price,
                stock: row.stock,
            };
            (listing, product)
        }))
    }

    pub async fn create_product_listing(
        &self,
        business_id: Uuid,
        title: String,
        description: Option<String>,
        logo: Option<String>,
        price: f64,
        stock: i32,
        categories: Option<Vec<String>>,
        tags: Option<Vec<String>>,
    ) -> Result<(BusinessListingEntity, ProductListingEntity), DbError> {
        let mut tx = self.pg.begin().await?;

        let product = sqlx::query_as!(
            ProductListingEntity,
            r#"INSERT INTO listing.product_listings (id, price, stock)
               VALUES (gen_random_uuid(), $1, $2)
               RETURNING id, price, stock"#,
            price,
            stock
        )
        .fetch_one(&mut *tx)
        .await?;

        let listing = sqlx::query_as!(
            BusinessListingEntity,
            r#"INSERT INTO listing.business_listings
                   (id, business_id, title, description, logo, product_listing_id, updated_at)
               VALUES (gen_random_uuid(), $1, $2, $3, $4, $5, now())
               RETURNING id, title, description, logo, is_active, created_at, updated_at,
                         business_id, product_listing_id, service_listing_id"#,
            business_id,
            title,
            description,
            logo,
            product.id
        )
        .fetch_one(&mut *tx)
        .await?;

        if let Some(categories) = categories {
            for category in categories {
                sqlx::query!(
                    r#"INSERT INTO listing.listing_categories (id, listing_id, value)
                       VALUES (gen_random_uuid(), $1, $2)"#,
                    listing.id,
                    category
                )
                .execute(&mut *tx)
                .await?;
            }
        }

        if let Some(tags) = tags {
            for tag in tags {
                sqlx::query!(
                    r#"INSERT INTO listing.listing_tags (id, listing_id, value)
                       VALUES (gen_random_uuid(), $1, $2)"#,
                    listing.id,
                    tag
                )
                .execute(&mut *tx)
                .await?;
            }
        }

        tx.commit().await?;

        Ok((listing, product))
    }

    pub async fn update_product_listing(
        &self,
        listing_id: Uuid,
        business_id: Uuid,
        title: Option<String>,
        description: Option<String>,
        logo: Option<String>,
        is_active: Option<bool>,
        price: Option<f64>,
        stock: Option<i32>,
    ) -> Result<(BusinessListingEntity, ProductListingEntity), DbError> {
        let mut tx = self.pg.begin().await?;

        let listing = sqlx::query_as!(
            BusinessListingEntity,
            r#"UPDATE listing.business_listings SET
               title = COALESCE($3, title),
               description = COALESCE($4, description),
               logo = COALESCE($5, logo),
               is_active = COALESCE($6, is_active),
               updated_at = now()
               WHERE id = $1 AND business_id = $2
               RETURNING id, title, description, logo, is_active, created_at, updated_at,
                         business_id, product_listing_id, service_listing_id"#,
            listing_id,
            business_id,
            title,
            description,
            logo,
            is_active
        )
        .fetch_one(&mut *tx)
        .await?;

        let product = sqlx::query_as!(
            ProductListingEntity,
            r#"UPDATE listing.product_listings SET
               price = COALESCE($2, price),
               stock = COALESCE($3, stock)
               WHERE id = $1
               RETURNING id, price, stock"#,
            listing.product_listing_id,
            price,
            stock
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok((listing, product))
    }

    pub async fn delete_product_listing(&self, id: Uuid, business_id: Uuid) -> Result<(), DbError> {
        sqlx::query!(
            "DELETE FROM listing.business_listings WHERE id = $1 AND business_id = $2 AND product_listing_id IS NOT NULL",
            id,
            business_id
        )
        .execute(&self.pg)
        .await?;

        Ok(())
    }

    pub async fn find_all_services_listings_by_business(
        &self,
        business_id: Uuid,
    ) -> Result<Vec<(BusinessListingEntity, ServiceListingEntity)>, DbError> {
        let rows = sqlx::query!(
            r#"SELECT
                bl.id, bl.title, bl.description, bl.logo, bl.is_active,
                bl.created_at, bl.updated_at, bl.business_id,
                bl.product_listing_id, bl.service_listing_id,
                sl.id as sl_id, sl.price, sl.available
               FROM listing.business_listings bl
               INNER JOIN listing.service_listings sl ON sl.id = bl.service_listing_id
               WHERE bl.business_id = $1"#,
            business_id
        )
        .fetch_all(&self.pg)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| {
                let listing = BusinessListingEntity {
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
                };
                let service = ServiceListingEntity {
                    id: row.sl_id,
                    price: row.price,
                    available: row.available,
                };
                (listing, service)
            })
            .collect())
    }

    pub async fn find_service_listing_by_id_and_business(
        &self,
        id: Uuid,
        business_id: Uuid,
    ) -> Result<Option<(BusinessListingEntity, ServiceListingEntity)>, DbError> {
        let row = sqlx::query!(
            r#"SELECT
                bl.id, bl.title, bl.description, bl.logo, bl.is_active,
                bl.created_at, bl.updated_at, bl.business_id,
                bl.product_listing_id, bl.service_listing_id,
                sl.id as sl_id, sl.price, sl.available
               FROM listing.business_listings bl
               INNER JOIN listing.service_listings sl ON sl.id = bl.service_listing_id
               WHERE bl.id = $1 AND bl.business_id = $2"#,
            id,
            business_id
        )
        .fetch_optional(&self.pg)
        .await?;

        Ok(row.map(|row| {
            let listing = BusinessListingEntity {
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
            };
            let service = ServiceListingEntity {
                id: row.sl_id,
                price: row.price,
                available: row.available,
            };
            (listing, service)
        }))
    }

    pub async fn find_service_listing_by_id_and_business_and_owner(
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
                sl.id as sl_id, sl.price, sl.available
               FROM listing.business_listings bl
               INNER JOIN listing.service_listings sl ON sl.id = bl.service_listing_id
               INNER JOIN business.businesses bb ON bb.id = bl.business_id
               WHERE bl.id = $1 AND bl.business_id = $2 AND bb.owner_id = $3"#,
            id,
            business_id,
            owner_id
        )
        .fetch_optional(&self.pg)
        .await?;

        Ok(row.map(|row| {
            let listing = BusinessListingEntity {
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
            };
            let service = ServiceListingEntity {
                id: row.sl_id,
                price: row.price,
                available: row.available,
            };
            (listing, service)
        }))
    }

    pub async fn create_service_listing(
        &self,
        business_id: Uuid,
        title: String,
        description: Option<String>,
        logo: Option<String>,
        price: String,
        available: bool,
        categories: Option<Vec<String>>,
        tags: Option<Vec<String>>,
    ) -> Result<(BusinessListingEntity, ServiceListingEntity), DbError> {
        let mut tx = self.pg.begin().await?;

        let service = sqlx::query_as!(
            ServiceListingEntity,
            r#"INSERT INTO listing.service_listings (id, price, available)
               VALUES (gen_random_uuid(), $1, $2)
               RETURNING id, price, available"#,
            price,
            available
        )
        .fetch_one(&mut *tx)
        .await?;

        let listing = sqlx::query_as!(
            BusinessListingEntity,
            r#"INSERT INTO listing.business_listings
                   (id, business_id, title, description, logo, service_listing_id, updated_at)
               VALUES (gen_random_uuid(), $1, $2, $3, $4, $5, now())
               RETURNING id, title, description, logo, is_active, created_at, updated_at,
                         business_id, product_listing_id, service_listing_id"#,
            business_id,
            title,
            description,
            logo,
            service.id
        )
        .fetch_one(&mut *tx)
        .await?;

        if let Some(categories) = categories {
            for category in categories {
                sqlx::query!(
                    r#"INSERT INTO listing.listing_categories (id, listing_id, value)
                       VALUES (gen_random_uuid(), $1, $2)"#,
                    listing.id,
                    category
                )
                .execute(&mut *tx)
                .await?;
            }
        }

        if let Some(tags) = tags {
            for tag in tags {
                sqlx::query!(
                    r#"INSERT INTO listing.listing_tags (id, listing_id, value)
                       VALUES (gen_random_uuid(), $1, $2)"#,
                    listing.id,
                    tag
                )
                .execute(&mut *tx)
                .await?;
            }
        }

        tx.commit().await?;

        Ok((listing, service))
    }

    pub async fn update_service_listing(
        &self,
        listing_id: Uuid,
        business_id: Uuid,
        title: Option<String>,
        description: Option<String>,
        logo: Option<String>,
        is_active: Option<bool>,
        price: Option<String>,
        available: Option<bool>,
    ) -> Result<(BusinessListingEntity, ServiceListingEntity), DbError> {
        let mut tx = self.pg.begin().await?;

        let listing = sqlx::query_as!(
            BusinessListingEntity,
            r#"UPDATE listing.business_listings SET
               title = COALESCE($3, title),
               description = COALESCE($4, description),
               logo = COALESCE($5, logo),
               is_active = COALESCE($6, is_active),
               updated_at = now()
               WHERE id = $1 AND business_id = $2
               RETURNING id, title, description, logo, is_active, created_at, updated_at,
                         business_id, product_listing_id, service_listing_id"#,
            listing_id,
            business_id,
            title,
            description,
            logo,
            is_active
        )
        .fetch_one(&mut *tx)
        .await?;

        let service = sqlx::query_as!(
            ServiceListingEntity,
            r#"UPDATE listing.service_listings SET
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

    pub async fn delete_service_listing(&self, id: Uuid, business_id: Uuid) -> Result<(), DbError> {
        sqlx::query!(
            "DELETE FROM listing.business_listings WHERE id = $1 AND business_id = $2 AND service_listing_id IS NOT NULL",
            id,
            business_id
        )
        .execute(&self.pg)
        .await?;

        Ok(())
    }

    pub async fn find_media(&self, listing_id: Uuid) -> Result<Vec<ListingMediaEntity>, DbError> {
        let media = sqlx::query_as!(
            ListingMediaEntity,
            r#"SELECT id, type AS "media_type: _", url, listing_id
               FROM listing.listing_media WHERE listing_id = $1"#,
            listing_id
        )
        .fetch_all(&self.pg)
        .await?;

        Ok(media)
    }

    pub async fn create_media(
        &self,
        listing_id: Uuid,
        media_type: String,
        url: String,
    ) -> Result<ListingMediaEntity, DbError> {
        let media = sqlx::query_as!(
            ListingMediaEntity,
            r#"INSERT INTO listing.listing_media (id, listing_id, type, url)
               VALUES (gen_random_uuid(), $1, $2::shared.media_type, $3)
               RETURNING id, type AS "media_type: _", url, listing_id"#,
            listing_id,
            media_type as _,
            url
        )
        .fetch_one(&self.pg)
        .await?;

        Ok(media)
    }

    pub async fn delete_media(&self, id: Uuid, listing_id: Uuid) -> Result<(), DbError> {
        sqlx::query!(
            "DELETE FROM listing.listing_media WHERE id = $1 AND listing_id = $2",
            id,
            listing_id
        )
        .execute(&self.pg)
        .await?;

        Ok(())
    }
}