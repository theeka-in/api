use crate::errors::{DbError, ServiceError};
use crate::modules::listing::listing_dto::{CreateProductListingDto, CreateServiceListingDto, CreateListingMediaDto, ListingDto, ListingMediaDto, UpdateListingDto};
use crate::modules::listing::listing_repository::ListingRepository;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct ListingService {
    repo: ListingRepository,
}

impl ListingService {
    pub fn new(repo: ListingRepository) -> Arc<Self> {
        Arc::new(Self { repo })
    }

    pub async fn get_by_business(&self, business_id: Uuid) -> Result<Vec<ListingDto>, ServiceError> {
        todo!()
    }

    pub async fn get_by_id(&self, business_id: Uuid, listing_id: Uuid) -> Result<ListingDto, ServiceError> {
        todo!()
    }

    pub async fn create_product(&self, business_id: Uuid, owner_id: Uuid, body: CreateProductListingDto) -> Result<ListingDto, ServiceError> {
        todo!()
    }

    pub async fn create_service(&self, business_id: Uuid, owner_id: Uuid, body: CreateServiceListingDto) -> Result<ListingDto, ServiceError> {
        todo!()
    }

    pub async fn update(&self, business_id: Uuid, listing_id: Uuid, owner_id: Uuid, body: UpdateListingDto) -> Result<ListingDto, ServiceError> {
        todo!()
    }

    pub async fn delete(&self, business_id: Uuid, listing_id: Uuid, owner_id: Uuid) -> Result<(), ServiceError> {
        todo!()
    }

    pub async fn get_media(&self, business_id: Uuid, listing_id: Uuid) -> Result<Vec<ListingMediaDto>, ServiceError> {
        todo!()
    }

    pub async fn create_media(&self, business_id: Uuid, listing_id: Uuid, owner_id: Uuid, body: CreateListingMediaDto) -> Result<ListingMediaDto, ServiceError> {
        todo!()
    }

    pub async fn delete_media(&self, business_id: Uuid, listing_id: Uuid, owner_id: Uuid, media_id: Uuid) -> Result<(), ServiceError> {
        todo!()
    }
}
