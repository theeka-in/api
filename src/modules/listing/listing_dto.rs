use crate::modules::listing::listing_entity::{
    BusinessListingEntity, ListingMediaEntity, ProductListingEntity, ServiceListingEntity,
};
use poem_openapi::Object;
use uuid::Uuid;

#[derive(Debug, Object)]
pub struct ProductListingDto {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
    pub business_id: Uuid,
    pub product_listing_id: Uuid,
    pub price: f64,
    pub stock: i32,
}

#[derive(Debug, Object)]
pub struct ServiceListingDto {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
    pub business_id: Uuid,
    pub service_listing_id: Uuid,
    pub price: String,
    pub available: bool,
}

#[derive(Debug, Object)]
pub struct CreateProductListingDto {
    #[oai(validator(min_length = 3, max_length = 120))]
    pub title: String,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub price: f64,
    pub stock: i32,
    pub categories: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Object)]
pub struct CreateServiceListingDto {
    #[oai(validator(min_length = 3, max_length = 120))]
    pub title: String,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub price: String,
    pub available: bool,
    pub categories: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Object)]
pub struct UpdateProductListingDto {
    #[oai(validator(min_length = 3, max_length = 120))]
    pub title: Option<String>,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub is_active: Option<bool>,
    pub price: Option<f64>,
    pub stock: Option<i32>,
}

#[derive(Debug, Object)]
pub struct UpdateServiceListingDto {
    #[oai(validator(min_length = 3, max_length = 120))]
    pub title: Option<String>,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub is_active: Option<bool>,
    pub price: Option<String>,
    pub available: Option<bool>,
}

#[derive(Debug, Object)]
pub struct ListingMediaDto {
    pub id: Uuid,
    pub media_type: String,
    pub url: String,
    pub listing_id: Uuid,
}

#[derive(Debug, Object)]
pub struct CreateListingMediaDto {
    pub media_type: String,
    pub url: String,
}

impl From<(BusinessListingEntity, ProductListingEntity)> for ProductListingDto {
    fn from((bl, pl): (BusinessListingEntity, ProductListingEntity)) -> Self {
        Self {
            id: bl.id,
            title: bl.title,
            description: bl.description,
            logo: bl.logo,
            is_active: bl.is_active,
            created_at: bl.created_at.to_string(),
            updated_at: bl.updated_at.to_string(),
            business_id: bl.business_id,
            product_listing_id: pl.id,
            price: pl.price,
            stock: pl.stock,
        }
    }
}

impl From<(BusinessListingEntity, ServiceListingEntity)> for ServiceListingDto {
    fn from((bl, sl): (BusinessListingEntity, ServiceListingEntity)) -> Self {
        Self {
            id: bl.id,
            title: bl.title,
            description: bl.description,
            logo: bl.logo,
            is_active: bl.is_active,
            created_at: bl.created_at.to_string(),
            updated_at: bl.updated_at.to_string(),
            business_id: bl.business_id,
            service_listing_id: sl.id,
            price: sl.price,
            available: sl.available,
        }
    }
}

impl From<ListingMediaEntity> for ListingMediaDto {
    fn from(entity: ListingMediaEntity) -> Self {
        Self {
            id: entity.id,
            media_type: format!("{:?}", entity.media_type).to_lowercase(),
            url: entity.url,
            listing_id: entity.listing_id,
        }
    }
}