use crate::modules::analytics::analytics_dto::{
    BusinessAnalyticsDto, ListingAnalyticsDto, RecordBusinessViewDto, ViewDto,
};
use crate::modules::database::DatabaseService;
use crate::shared::errors::ServiceError;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct AnalyticsService {
    db: Arc<DatabaseService>,
}

impl AnalyticsService {
    pub fn new(db: Arc<DatabaseService>) -> Arc<Self> {
        Arc::new(Self { db })
    }

    pub async fn record_business_view(
        &self,
        user_id: Uuid,
        business_id: Uuid,
        body: RecordBusinessViewDto,
    ) -> Result<ViewDto, ServiceError> {
        todo!()
    }

    pub async fn record_listing_view(
        &self,
        user_id: Uuid,
        business_id: Uuid,
        listing_id: Uuid,
    ) -> Result<ViewDto, ServiceError> {
        todo!()
    }

    pub async fn get_business_analytics(
        &self,
        owner_id: Uuid,
        business_id: Uuid,
    ) -> Result<BusinessAnalyticsDto, ServiceError> {
        todo!()
    }

    pub async fn get_listing_analytics(
        &self,
        owner_id: Uuid,
        business_id: Uuid,
    ) -> Result<Vec<ListingAnalyticsDto>, ServiceError> {
        todo!()
    }
}
