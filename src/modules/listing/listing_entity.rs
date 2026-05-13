use sqlx::{FromRow, types::chrono};
use uuid::Uuid;

#[derive(Debug, sqlx::Type, PartialEq)]
#[sqlx(type_name = "media_type", rename_all = "snake_case")]
pub enum MediaType {
    Image,
    Video,
}

#[derive(Debug, FromRow)]
pub struct BusinessListingEntity {
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

#[derive(Debug, FromRow)]
pub struct ProductListingEntity {
    pub id: Uuid,
    pub price: f64,
    pub stock: i32,
}

#[derive(Debug, FromRow)]
pub struct ServiceListingEntity {
    pub id: Uuid,
    pub price: String,
    pub available: bool,
}

#[derive(Debug, FromRow)]
pub struct ListingCategoryEntity {
    pub id: Uuid,
    pub value: String,
    pub listing_id: Uuid,
}

#[derive(Debug, FromRow)]
pub struct ListingTagEntity {
    pub id: Uuid,
    pub value: String,
    pub listing_id: Uuid,
}

#[derive(Debug, FromRow)]
pub struct ListingProductSpecEntity {
    pub id: Uuid,
    pub group_name: String,
    pub key: String,
    pub value: String,
    pub product_listing_id: Uuid,
}

#[derive(Debug, FromRow)]
pub struct ListingMediaEntity {
    pub id: Uuid,
    pub media_type: MediaType,
    pub url: String,
    pub listing_id: Uuid,
}
