use crate::{
    modules::database::{
        BusinessAddressEntity, BusinessEntity, ListingMediaEntity, ProductListingEntity,
        ServiceListingEntity, UserEntity,
    },
    shared::errors::DbError,
};
use sqlx::{types::chrono, FromRow, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct ListingEntity {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub is_active: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub business_id: Uuid,
    pub product_listing_id: Option<Uuid>,
    pub service_listing_id: Option<Uuid>,
}

#[derive(Debug)]
pub struct ExploreProductListingEntity {
    pub business: BusinessEntity,
    pub business_address: BusinessAddressEntity,
    pub listing: ListingEntity,
    pub listing_media: Vec<ListingMediaEntity>,
    pub product: ProductListingEntity,
    pub business_owner: UserEntity,
}

#[derive(Debug)]
pub struct ExploreServiceListingEntity {
    pub business: BusinessEntity,
    pub business_address: BusinessAddressEntity,
    pub listing: ListingEntity,
    pub listing_media: Vec<ListingMediaEntity>,
    pub service: ServiceListingEntity,
    pub business_owner: UserEntity,
}

#[derive(Debug)]
pub struct BusinessListingRepo {
    pg: PgPool,
}

impl BusinessListingRepo {
    pub fn new(pg: PgPool) -> Self {
        Self { pg }
    }

    // TODO: embedding col dropped from `listings`. Semantic rank + 0.4 threshold
    // gone. Need new ranking strategy for product explore (text search, popularity,
    // distance, etc). Currently just filters by radius + active/open, no rank.
    pub async fn explore_products_nearby(
        &self,
        latitude: f64,
        longitude: f64,
    ) -> Result<Vec<ExploreProductListingEntity>, DbError> {
        let rows = sqlx::query!(
            r#"SELECT
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
                ST_Y(business_addresses.location::geometry) AS latitude,

                owner.name AS owner_name,
                owner.avatar AS owner_avatar,
                owner.account_id AS owner_account_id
               FROM listings AS listing
               INNER JOIN product_listings ON product_listings.id = listing.product_listing_id
               INNER JOIN businesses ON businesses.id = listing.business_id
               INNER JOIN business_addresses ON business_addresses.business_id = businesses.id
               INNER JOIN users AS owner ON owner.id = businesses.owner_id
               WHERE listing.is_active = true
                 AND businesses.is_closed = false
                 AND ST_DWithin(
                     business_addresses.location,
                     ST_SetSRID(ST_MakePoint($1, $2), 4326)::geography,
                     business_addresses.radius
                 )
               LIMIT 100"#,
            longitude,
            latitude,
        )
        .fetch_all(&self.pg)
        .await?;

        let listing_ids: Vec<Uuid> = rows.iter().map(|r| r.id).collect();
        let all_media = sqlx::query_as!(
            ListingMediaEntity,
            r#"SELECT id, type AS "media_type: _", url, listing_id
               FROM listing_media WHERE listing_id = ANY($1)"#,
            &listing_ids[..]
        )
        .fetch_all(&self.pg)
        .await?;

        let mut media_map: HashMap<Uuid, Vec<ListingMediaEntity>> = HashMap::new();
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
                let listing = ListingEntity {
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
                let media = media_map.remove(&row.id).unwrap_or_default();
                let product = ProductListingEntity {
                    id: row.pl_id,
                    price: row.price,
                    stock: row.stock,
                };

                let business_owner = UserEntity {
                    id: row.b_owner_id,
                    name: row.owner_name,
                    avatar: row.owner_avatar,
                    account_id: row.owner_account_id,
                };

                ExploreProductListingEntity {
                    business,
                    business_address: address,
                    listing,
                    listing_media: media,
                    product,
                    business_owner,
                }
            })
            .collect())
    }

    // Semantic embedding rank replaced w/ full-text search on
    // service_listing_type.name (tsvector). `query` = raw search text from user.
    pub async fn explore_services_nearby(
        &self,
        latitude: f64,
        longitude: f64,
        query: &str,
    ) -> Result<Vec<ExploreServiceListingEntity>, DbError> {
        let rows = sqlx::query!(
            r#"SELECT
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
                service_listings.id AS sl_id,
                service_listings.price,
                service_listings.available,
                service_listings.service_listings_type_id,
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
                ST_Y(business_addresses.location::geometry) AS latitude,

                owner.name AS owner_name,
                owner.avatar AS owner_avatar,
                owner.account_id AS owner_account_id
               FROM listings AS listing
               INNER JOIN service_listings ON service_listings.id = listing.service_listing_id
               INNER JOIN service_listings_type ON service_listings_type.id = service_listings.service_listings_type_id
               INNER JOIN businesses ON businesses.id = listing.business_id
               INNER JOIN business_addresses ON business_addresses.business_id = businesses.id
               INNER JOIN users AS owner ON owner.id = businesses.owner_id
               WHERE listing.is_active = true
                 AND businesses.is_closed = false
                 AND ST_DWithin(
                     business_addresses.location,
                     ST_SetSRID(ST_MakePoint($1, $2), 4326)::geography,
                     business_addresses.radius
                 )
                 AND to_tsvector('english', service_listings_type.name) @@ plainto_tsquery('english', $3)
               ORDER BY ts_rank(
                   to_tsvector('english', service_listings_type.name),
                   plainto_tsquery('english', $3)
               ) DESC
               LIMIT 100"#,
            longitude,
            latitude,
            query,
        )
            .fetch_all(&self.pg)
            .await?;

        let listing_ids: Vec<Uuid> = rows.iter().map(|r| r.id).collect();
        let all_media = sqlx::query_as!(
            ListingMediaEntity,
            r#"SELECT id, type AS "media_type: _", url, listing_id
               FROM listing_media WHERE listing_id = ANY($1)"#,
            &listing_ids[..]
        )
        .fetch_all(&self.pg)
        .await?;

        let mut media_map: HashMap<Uuid, Vec<ListingMediaEntity>> = HashMap::new();
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
                let listing = ListingEntity {
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
                let media = media_map.remove(&row.id).unwrap_or_default();
                let service = ServiceListingEntity {
                    id: row.sl_id,
                    price: row.price,
                    available: row.available,
                    service_listings_type_id: row.service_listings_type_id,
                };
                let business_owner = UserEntity {
                    id: row.b_owner_id,
                    name: row.owner_name,
                    avatar: row.owner_avatar,
                    account_id: row.owner_account_id,
                };

                ExploreServiceListingEntity {
                    business,
                    business_address: address,
                    listing,
                    listing_media: media,
                    service,
                    business_owner,
                }
            })
            .collect())
    }
}
