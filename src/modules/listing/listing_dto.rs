use crate::modules::listing::listing_entity::{BusinessListingEntity, ListingMediaEntity};
use poem_openapi::Object;
use uuid::Uuid;

#[derive(Debug, Object)]
pub struct ListingDto {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
    pub business_id: Uuid,
    pub product_listing_id: Option<Uuid>,
    pub service_listing_id: Option<Uuid>,
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
pub struct UpdateListingDto {
    #[oai(validator(min_length = 3, max_length = 120))]
    pub title: Option<String>,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub is_active: Option<bool>,
    pub price: Option<String>,
    pub stock: Option<i32>,
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

impl From<BusinessListingEntity> for ListingDto {
    fn from(entity: BusinessListingEntity) -> Self {
        Self {
            id: entity.id,
            title: entity.title,
            description: entity.description,
            logo: entity.logo,
            is_active: entity.is_active,
            created_at: entity.created_at.to_string(),
            updated_at: entity.updated_at.to_string(),
            business_id: entity.business_id,
            product_listing_id: entity.product_listing_id,
            service_listing_id: entity.service_listing_id,
        }
    }
}
