use std::collections::HashMap;

use crate::errors::DbError;
use crate::modules::business::{BusinessAddressEntity, BusinessEntity};
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

    pub async fn explore_products_listings_nearby(
        &self,
        latitude: f64,
        longitude: f64,
        query_embedding: pgvector::Vector,
    ) -> Result<
        Vec<(
            BusinessEntity,
            BusinessAddressEntity,
            BusinessListingEntity,
            Vec<ListingMediaEntity>,
            ProductListingEntity,
        )>,
        DbError,
    > {
        let rows = sqlx::query!(
            r#"SELECT
                business_listings.id,
                business_listings.title,
                business_listings.description,
                business_listings.logo,
                business_listings.is_active,
                business_listings.created_at,
                business_listings.updated_at,
                business_listings.business_id,
                business_listings.product_listing_id,
                business_listings.service_listing_id,
                business_listings.embedding AS "embedding: pgvector::Vector",

                product_listings.id AS pl_id,
                product_listings.price,
                product_listings.stock,

                businesses.phone_number AS b_phone_number,
                businesses.is_closed AS b_is_closed,
                businesses.title AS b_title,
                businesses.logo AS b_logo,
                businesses.description AS b_description,
                businesses.created_at AS b_created_at,
                businesses.owner_id AS b_owner_id,

                business_addresses.address_line1,
                business_addresses.address_line2,
                business_addresses.landmark,
                business_addresses.city,
                business_addresses.state,
                business_addresses.pincode,
                business_addresses.radius,
                ST_X(business_addresses.location::geometry) AS longitude,
                ST_Y(business_addresses.location::geometry) AS latitude
               FROM listing.business_listings
               INNER JOIN listing.product_listings ON product_listings.id = business_listings.product_listing_id
               INNER JOIN business.businesses ON businesses.id = business_listings.business_id
               INNER JOIN business.business_addresses ON business_addresses.business_id = businesses.id
               WHERE business_listings.is_active = true
                 AND businesses.is_closed = false
                 AND ST_DWithin(
                     business_addresses.location,
                     ST_SetSRID(ST_MakePoint($1, $2), 4326)::geography,
                     business_addresses.radius
                 )
                 AND 1 - (business_listings.embedding <=> $3) >= 0.4
                 ORDER BY business_listings.embedding <=> $3
               LIMIT 100"#,
            longitude,
            latitude,
            query_embedding as _,
        )
        .fetch_all(&self.pg)
        .await?;

        let listing_ids: Vec<Uuid> = rows.iter().map(|r| r.id).collect();

        let all_media = sqlx::query_as!(
            ListingMediaEntity,
            r#"SELECT id, type AS "media_type: _", url, listing_id
            FROM listing.listing_media WHERE listing_id = ANY($1)"#,
            &listing_ids[..]
        )
        .fetch_all(&self.pg)
        .await?;

        let mut media_map: HashMap<uuid::Uuid, Vec<ListingMediaEntity>> = HashMap::new();
        for media in all_media {
            media_map.entry(media.listing_id).or_default().push(media);
        }

        Ok(rows
            .into_iter()
            .map(|row| {
                let business = BusinessEntity {
                    id: row.business_id,
                    phone_number: row.b_phone_number,
                    is_closed: row.b_is_closed,
                    title: row.b_title,
                    logo: row.b_logo,
                    description: row.b_description,
                    created_at: row.b_created_at,
                    owner_id: row.b_owner_id,
                };

                let address = BusinessAddressEntity {
                    address_line1: row.address_line1,
                    address_line2: row.address_line2,
                    landmark: row.landmark,
                    pincode: row.pincode,
                    city: row.city,
                    state: row.state,
                    latitude: row.latitude.unwrap_or(0.0),
                    longitude: row.longitude.unwrap_or(0.0),
                    radius: row.radius,
                    business_id: row.business_id,
                };

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
                    embedding: row.embedding,
                };

                let media = media_map.remove(&row.id).unwrap_or_default();

                let product = ProductListingEntity {
                    id: row.pl_id,
                    price: row.price,
                    stock: row.stock,
                };

                (business, address, listing, media, product)
            })
            .collect())
    }

    pub async fn explore_services_listings_nearby(
        &self,
        latitude: f64,
        longitude: f64,
        query_embedding: pgvector::Vector,
    ) -> Result<
        Vec<(
            BusinessEntity,
            BusinessAddressEntity,
            BusinessListingEntity,
            Vec<ListingMediaEntity>,
            ServiceListingEntity,
        )>,
        DbError,
    > {
        let rows = sqlx::query!(
            r#"SELECT
                -- Business Listing Properties
                business_listings.id,
                business_listings.title,
                business_listings.description,
                business_listings.logo,
                business_listings.is_active,
                business_listings.created_at,
                business_listings.updated_at,
                business_listings.business_id,
                business_listings.product_listing_id,
                business_listings.service_listing_id,
                business_listings.embedding AS "embedding: pgvector::Vector",
                
                -- Service Listing Properties
                service_listings.id AS sl_id,
                service_listings.price,
                service_listings.available,
        
                -- Business Properties
                businesses.phone_number AS b_phone_number,
                businesses.is_closed AS b_is_closed,
                businesses.title AS b_title,
                businesses.logo AS b_logo,
                businesses.description AS b_description,
                businesses.created_at AS b_created_at,
                businesses.owner_id AS b_owner_id,
        
                -- Business Address Properties
                business_addresses.address_line1,
                business_addresses.address_line2,
                business_addresses.landmark,
                business_addresses.city,
                business_addresses.state,
                business_addresses.pincode,
                business_addresses.radius,
                ST_X(business_addresses.location::geometry) AS longitude,
                ST_Y(business_addresses.location::geometry) AS latitude
               FROM listing.business_listings
               INNER JOIN listing.service_listings ON service_listings.id = business_listings.service_listing_id
               INNER JOIN business.businesses ON businesses.id = business_listings.business_id
               INNER JOIN business.business_addresses ON business_addresses.business_id = businesses.id
               WHERE business_listings.is_active = true
                 AND businesses.is_closed = false
                 AND ST_DWithin(
                     business_addresses.location,
                     ST_SetSRID(ST_MakePoint($1, $2), 4326)::geometry,
                     business_addresses.radius
                 )
                 AND 1 - (business_listings.embedding <=> $3) >= 0.4
               ORDER BY business_listings.embedding <=> $3
               LIMIT 100"#,
            longitude,
            latitude,
            query_embedding as _
        )
        .fetch_all(&self.pg)
        .await?;

        let listing_ids: Vec<Uuid> = rows.iter().map(|r| r.id).collect();

        let all_media = sqlx::query_as!(
            ListingMediaEntity,
            r#"SELECT id, type AS "media_type: _", url, listing_id
            FROM listing.listing_media WHERE listing_id = ANY($1)"#,
            &listing_ids[..]
        )
        .fetch_all(&self.pg)
        .await?;

        let mut media_map: HashMap<uuid::Uuid, Vec<ListingMediaEntity>> = HashMap::new();
        for media in all_media {
            media_map.entry(media.listing_id).or_default().push(media);
        }

        Ok(rows
            .into_iter()
            .map(|row| {
                let business = BusinessEntity {
                    id: row.business_id,
                    phone_number: row.b_phone_number,
                    is_closed: row.b_is_closed,
                    title: row.b_title,
                    logo: row.b_logo,
                    description: row.b_description,
                    created_at: row.b_created_at,
                    owner_id: row.b_owner_id,
                };

                let address = BusinessAddressEntity {
                    address_line1: row.address_line1,
                    address_line2: row.address_line2,
                    landmark: row.landmark,
                    pincode: row.pincode,
                    city: row.city,
                    state: row.state,
                    latitude: row.latitude.unwrap_or(0.0),
                    longitude: row.longitude.unwrap_or(0.0),
                    radius: row.radius,
                    business_id: row.business_id,
                };

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
                    embedding: row.embedding,
                };

                let media = media_map.remove(&row.id).unwrap_or_default();

                let service = ServiceListingEntity {
                    id: row.sl_id,
                    price: row.price,
                    available: row.available,
                };

                (business, address, listing, media, service)
            })
            .collect())
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
                bl.embedding AS "embedding: pgvector::Vector",
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
                    embedding: row.embedding,
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
                bl.embedding AS "embedding: pgvector::Vector",
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
                embedding: row.embedding,
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
                bl.embedding AS "embedding: pgvector::Vector",
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
                embedding: row.embedding,
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
        embedding: pgvector::Vector,
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
                   (id, business_id, title, description, logo, product_listing_id, updated_at, embedding)
               VALUES (gen_random_uuid(), $1, $2, $3, $4, $5, now(), $6)
               RETURNING id, title, description, logo, is_active, created_at, updated_at,
                         business_id, product_listing_id, service_listing_id,
                         embedding AS "embedding: pgvector::Vector""#,
            business_id,
            title,
            description,
            logo,
            product.id,
            embedding as _,
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
        embedding: Option<pgvector::Vector>,
    ) -> Result<(BusinessListingEntity, ProductListingEntity), DbError> {
        let mut tx = self.pg.begin().await?;

        let listing = sqlx::query_as!(
            BusinessListingEntity,
            r#"UPDATE listing.business_listings SET
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
                bl.embedding AS "embedding: pgvector::Vector",
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
                    embedding: row.embedding,
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
                bl.embedding AS "embedding: pgvector::Vector",
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
                embedding: row.embedding,
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
                bl.embedding AS "embedding: pgvector::Vector",
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
                embedding: row.embedding,
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
        embedding: pgvector::Vector,
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
                   (id, business_id, title, description, logo, service_listing_id, updated_at, embedding)
               VALUES (gen_random_uuid(), $1, $2, $3, $4, $5, now(), $6)
               RETURNING id, title, description, logo, is_active, created_at, updated_at,
                         business_id, product_listing_id, service_listing_id,
                         embedding AS "embedding: pgvector::Vector""#,
            business_id,
            title,
            description,
            logo,
            service.id,
            embedding as _,
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
        embedding: Option<pgvector::Vector>,
    ) -> Result<(BusinessListingEntity, ServiceListingEntity), DbError> {
        let mut tx = self.pg.begin().await?;

        let listing = sqlx::query_as!(
            BusinessListingEntity,
            r#"UPDATE listing.business_listings SET
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
