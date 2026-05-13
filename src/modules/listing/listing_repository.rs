use crate::errors::DbError;
use crate::modules::listing::listing_entity::{BusinessListingEntity, ListingMediaEntity};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct ListingRepository {
    pg: PgPool,
}

impl ListingRepository {
    pub fn new(pg: PgPool) -> Self {
        Self { pg }
    }

    pub async fn find_by_business(&self, business_id: Uuid) -> Result<Vec<BusinessListingEntity>, DbError> {
        todo!()
    }

    pub async fn find_by_id(&self, id: Uuid, business_id: Uuid) -> Result<Option<BusinessListingEntity>, DbError> {
        todo!()
    }

    pub async fn create_product_listing(&self, business_id: Uuid, title: String, description: Option<String>, logo: Option<String>, price: f64, stock: i32, categories: Option<Vec<String>>, tags: Option<Vec<String>>) -> Result<BusinessListingEntity, DbError> {
        todo!()
    }

    pub async fn create_service_listing(&self, business_id: Uuid, title: String, description: Option<String>, logo: Option<String>, price: String, available: bool, categories: Option<Vec<String>>, tags: Option<Vec<String>>) -> Result<BusinessListingEntity, DbError> {
        todo!()
    }

    pub async fn update(&self, id: Uuid, business_id: Uuid, title: Option<String>, description: Option<String>, logo: Option<String>, is_active: Option<bool>) -> Result<BusinessListingEntity, DbError> {
        todo!()
    }

    pub async fn delete(&self, id: Uuid, business_id: Uuid) -> Result<(), DbError> {
        todo!()
    }

    pub async fn find_media(&self, listing_id: Uuid) -> Result<Vec<ListingMediaEntity>, DbError> {
        todo!()
    }

    pub async fn create_media(&self, listing_id: Uuid, media_type: String, url: String) -> Result<ListingMediaEntity, DbError> {
        todo!()
    }

    pub async fn delete_media(&self, id: Uuid, listing_id: Uuid) -> Result<(), DbError> {
        todo!()
    }
}
