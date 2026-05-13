use crate::errors::{DbError, ServiceError};
use crate::modules::business::business_dto::{BusinessDto, BusinessHourDto, BusinessMediaDto, CreateBusinessDto, CreateBusinessHourDto, CreateBusinessMediaDto, UpdateBusinessDto, UpdateBusinessHourDto};
use crate::modules::business::business_repository::BusinessRepository;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct BusinessService {
    repo: BusinessRepository,
}

impl BusinessService {
    pub fn new(repo: BusinessRepository) -> Arc<Self> {
        Arc::new(Self { repo })
    }

    pub async fn get_nearby(&self, latitude: f64, longitude: f64) -> Result<Vec<BusinessDto>, ServiceError> {
        todo!()
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<BusinessDto, ServiceError> {
        todo!()
    }

    pub async fn create(&self, owner_id: Uuid, body: CreateBusinessDto) -> Result<BusinessDto, ServiceError> {
        todo!()
    }

    pub async fn update(&self, id: Uuid, owner_id: Uuid, body: UpdateBusinessDto) -> Result<BusinessDto, ServiceError> {
        todo!()
    }

    pub async fn delete(&self, id: Uuid, owner_id: Uuid) -> Result<(), ServiceError> {
        todo!()
    }

    pub async fn get_hours(&self, business_id: Uuid) -> Result<Vec<BusinessHourDto>, ServiceError> {
        todo!()
    }

    pub async fn create_hour(&self, business_id: Uuid, owner_id: Uuid, body: CreateBusinessHourDto) -> Result<BusinessHourDto, ServiceError> {
        todo!()
    }

    pub async fn update_hour(&self, business_id: Uuid, owner_id: Uuid, day: String, body: UpdateBusinessHourDto) -> Result<BusinessHourDto, ServiceError> {
        todo!()
    }

    pub async fn delete_hour(&self, business_id: Uuid, owner_id: Uuid, day: String) -> Result<(), ServiceError> {
        todo!()
    }

    pub async fn get_media(&self, business_id: Uuid) -> Result<Vec<BusinessMediaDto>, ServiceError> {
        todo!()
    }

    pub async fn create_media(&self, business_id: Uuid, owner_id: Uuid, body: CreateBusinessMediaDto) -> Result<BusinessMediaDto, ServiceError> {
        todo!()
    }

    pub async fn delete_media(&self, business_id: Uuid, owner_id: Uuid, media_id: Uuid) -> Result<(), ServiceError> {
        todo!()
    }
}
