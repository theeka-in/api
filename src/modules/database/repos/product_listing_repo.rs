use crate::{modules::database::BusinessListingEntity, shared::errors::DbError};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct ProductListingEntity {
    pub id: Uuid,
    pub price: f64,
    pub stock: i32,
}

#[derive(Debug)]
pub struct ProductListingRepo {
    pg: PgPool,
}

impl ProductListingRepo {
    pub fn new(pg: PgPool) -> Self {
        Self { pg }
    }

    pub async fn find_all_by_business(
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

        Ok(rows.into_iter().map(|row| {
            let listing = BusinessListingEntity {
                id: row.id, title: row.title, description: row.description,
                logo: row.logo, is_active: row.is_active, created_at: row.created_at,
                updated_at: row.updated_at, business_id: row.business_id,
                product_listing_id: row.product_listing_id,
                service_listing_id: row.service_listing_id, embedding: row.embedding,
            };
            let product = ProductListingEntity { id: row.pl_id, price: row.price, stock: row.stock };
            (listing, product)
        }).collect())
    }

    pub async fn find_by_id_and_business(
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

        Ok(row.map(|row| (
            BusinessListingEntity {
                id: row.id, title: row.title, description: row.description,
                logo: row.logo, is_active: row.is_active, created_at: row.created_at,
                updated_at: row.updated_at, business_id: row.business_id,
                product_listing_id: row.product_listing_id,
                service_listing_id: row.service_listing_id, embedding: row.embedding,
            },
            ProductListingEntity { id: row.pl_id, price: row.price, stock: row.stock },
        )))
    }

    pub async fn find_by_id_and_business_and_owner(
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

        Ok(row.map(|row| (
            BusinessListingEntity {
                id: row.id, title: row.title, description: row.description,
                logo: row.logo, is_active: row.is_active, created_at: row.created_at,
                updated_at: row.updated_at, business_id: row.business_id,
                product_listing_id: row.product_listing_id,
                service_listing_id: row.service_listing_id, embedding: row.embedding,
            },
            ProductListingEntity { id: row.pl_id, price: row.price, stock: row.stock },
        )))
    }

    pub async fn create(
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
            business_id, title, description, logo, product.id, embedding as _,
        )
        .fetch_one(&mut *tx)
        .await?;

        if let Some(categories) = categories {
            for category in categories {
                sqlx::query!(
                    r#"INSERT INTO listing.listing_categories (id, listing_id, value)
                       VALUES (gen_random_uuid(), $1, $2)"#,
                    listing.id, category
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
                    listing.id, tag
                )
                .execute(&mut *tx)
                .await?;
            }
        }

        tx.commit().await?;
        Ok((listing, product))
    }

    pub async fn update(
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
            listing_id, business_id, title, description, logo, is_active, embedding as _,
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
            listing.product_listing_id, price, stock
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok((listing, product))
    }

    pub async fn delete(&self, id: Uuid, business_id: Uuid) -> Result<(), DbError> {
        sqlx::query!(
            "DELETE FROM listing.business_listings WHERE id = $1 AND business_id = $2 AND product_listing_id IS NOT NULL",
            id, business_id
        )
        .execute(&self.pg)
        .await?;

        Ok(())
    }
}