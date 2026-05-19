use crate::errors::DbError;
use crate::modules::listing::listing_entity::{BusinessListingEntity, ListingMediaEntity};
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

    pub async fn find_all_listings_by_business(
        &self,
        business_id: Uuid,
    ) -> Result<Vec<BusinessListingEntity>, DbError> {
        let listings = sqlx::query_as!(
            BusinessListingEntity,
            r#"SELECT id, title, description, logo, is_active, created_at, updated_at,
                      business_id, product_listing_id, service_listing_id
               FROM listing.business_listings WHERE business_id = $1"#,
            business_id
        )
        .fetch_all(&self.pg)
        .await?;

        Ok(listings)
    }

    pub async fn find_listing_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<BusinessListingEntity>, DbError> {
        let listing = sqlx::query_as!(
            BusinessListingEntity,
            r#"SELECT id, title, description, logo, is_active, created_at, updated_at,
                      business_id, product_listing_id, service_listing_id
               FROM listing.business_listings WHERE id = $1"#,
            id,
        )
        .fetch_optional(&self.pg)
        .await?;

        Ok(listing)
    }

    pub async fn find_listing_by_id_and_business(
        &self,
        id: Uuid,
        business_id: Uuid,
    ) -> Result<Option<BusinessListingEntity>, DbError> {
        let listing = sqlx::query_as!(
            BusinessListingEntity,
            r#"SELECT id, title, description, logo, is_active, created_at, updated_at,
                      business_id, product_listing_id, service_listing_id
               FROM listing.business_listings WHERE id = $1 AND business_id = $2"#,
            id,
            business_id
        )
        .fetch_optional(&self.pg)
        .await?;

        Ok(listing)
    }

    pub async fn find_listing_by_id_and_business_and_owner(
        &self,
        id: Uuid,
        business_id: Uuid,
        owner_id: Uuid,
    ) -> Result<Option<BusinessListingEntity>, DbError> {
        let listing = sqlx::query_as!(
            BusinessListingEntity,
            r#"SELECT ll.id, ll.title, ll.description, ll.logo, ll.is_active, ll.created_at, ll.updated_at,
                ll.business_id, ll.product_listing_id, ll.service_listing_id
               FROM listing.business_listings ll
               INNER JOIN business.businesses bb ON bb.id = ll.business_id
               WHERE ll.id = $1 AND ll.business_id = $2 AND bb.owner_id = $3"#,
            id,
            business_id,
            owner_id
        )
        .fetch_optional(&self.pg)
        .await?;

        Ok(listing)
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
    ) -> Result<BusinessListingEntity, DbError> {
        let mut tx = self.pg.begin().await?;

        let product = sqlx::query!(
            r#"INSERT INTO listing.product_listings (id, price, stock)
               VALUES (gen_random_uuid(), $1, $2)
               RETURNING id"#,
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

        Ok(listing)
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
    ) -> Result<BusinessListingEntity, DbError> {
        let mut tx = self.pg.begin().await?;

        let service = sqlx::query!(
            r#"INSERT INTO listing.service_listings (id, price, available)
               VALUES (gen_random_uuid(), $1, $2)
               RETURNING id"#,
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

        if let Some(cats) = categories {
            for cat in cats {
                sqlx::query!(
                    r#"INSERT INTO listing.listing_categories (id, listing_id, value)
                       VALUES (gen_random_uuid(), $1, $2)"#,
                    listing.id,
                    cat
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

        Ok(listing)
    }

    pub async fn update_listing(
        &self,
        listing_id: Uuid,
        business_id: Uuid,
        title: Option<String>,
        description: Option<String>,
        logo: Option<String>,
        is_active: Option<bool>,
    ) -> Result<BusinessListingEntity, DbError> {
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
        .fetch_one(&self.pg)
        .await?;

        Ok(listing)
    }

    pub async fn delete_listing(&self, id: Uuid, business_id: Uuid) -> Result<(), DbError> {
        sqlx::query!(
            "DELETE FROM listing.business_listings WHERE id = $1 AND business_id = $2",
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
