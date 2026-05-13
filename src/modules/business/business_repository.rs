use crate::errors::DbError;
use crate::modules::business::business_entity::{BusinessAddressEntity, BusinessEntity, BusinessHourEntity, BusinessMediaEntity};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct BusinessRepository {
    pg: PgPool,
}

impl BusinessRepository {
    pub fn new(pg: PgPool) -> Self {
        Self { pg }
    }

    pub async fn find_nearby(&self, latitude: f64, longitude: f64) -> Result<Vec<BusinessEntity>, DbError> {
        todo!()
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<BusinessEntity>, DbError> {
        todo!()
    }

    pub async fn create(&self, owner_id: Uuid, phone_number: i64, title: String, logo: Option<String>, description: Option<String>) -> Result<BusinessEntity, DbError> {
        todo!()
    }

    pub async fn update(&self, id: Uuid, phone_number: Option<i64>, title: Option<String>, logo: Option<String>, description: Option<String>, is_closed: Option<bool>) -> Result<BusinessEntity, DbError> {
        todo!()
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), DbError> {
        todo!()
    }

    pub async fn find_hours(&self, business_id: Uuid) -> Result<Vec<BusinessHourEntity>, DbError> {
        todo!()
    }

    pub async fn create_hour(&self, business_id: Uuid, day: String, hours_type: String, open_time: Option<String>, close_time: Option<String>) -> Result<BusinessHourEntity, DbError> {
        todo!()
    }

    pub async fn update_hour(&self, business_id: Uuid, day: String, hours_type: Option<String>, open_time: Option<String>, close_time: Option<String>) -> Result<BusinessHourEntity, DbError> {
        todo!()
    }

    pub async fn delete_hour(&self, business_id: Uuid, day: String) -> Result<(), DbError> {
        todo!()
    }

    pub async fn find_media(&self, business_id: Uuid) -> Result<Vec<BusinessMediaEntity>, DbError> {
        todo!()
    }

    pub async fn create_media(&self, business_id: Uuid, media_type: String, url: String) -> Result<BusinessMediaEntity, DbError> {
        todo!()
    }

    pub async fn delete_media(&self, id: Uuid, business_id: Uuid) -> Result<(), DbError> {
        todo!()
    }
}
