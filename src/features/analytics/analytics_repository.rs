use crate::shared::errors::DbError;
use crate::features::analytics::analytics_entity::ViewEntity;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct AnalyticsRepository {
    pg: PgPool,
}

impl AnalyticsRepository {
    pub fn new(pg: PgPool) -> Self {
        Self { pg }
    }

    pub async fn record_view(
        &self,
        user_id: Uuid,
        business_id: Uuid,
        listing_id: Uuid,
    ) -> Result<ViewEntity, DbError> {
        todo!()
    }

    pub async fn get_business_view_count(&self, business_id: Uuid) -> Result<i64, DbError> {
        todo!()
    }

    pub async fn get_business_unique_viewer_count(
        &self,
        business_id: Uuid,
    ) -> Result<i64, DbError> {
        todo!()
    }

    pub async fn get_listing_view_count(&self, listing_id: Uuid) -> Result<i64, DbError> {
        todo!()
    }

    pub async fn get_listing_unique_viewer_count(&self, listing_id: Uuid) -> Result<i64, DbError> {
        todo!()
    }
}
