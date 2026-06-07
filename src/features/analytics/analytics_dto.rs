use crate::features::analytics::analytics_entity::ViewEntity;
use poem_openapi::Object;
use uuid::Uuid;

#[derive(Debug, Object)]
pub struct ViewDto {
    pub id: Uuid,
    pub created_at: String,
    pub user_id: Uuid,
    pub business_id: Uuid,
    pub listing_id: Uuid,
}

#[derive(Debug, Object)]
pub struct RecordBusinessViewDto {
    pub listing_id: Uuid,
}

#[derive(Debug, Object)]
pub struct BusinessAnalyticsDto {
    pub total_views: i64,
    pub unique_viewers: i64,
    pub business_id: Uuid,
}

#[derive(Debug, Object)]
pub struct ListingAnalyticsDto {
    pub total_views: i64,
    pub unique_viewers: i64,
    pub listing_id: Uuid,
    pub business_id: Uuid,
}

impl From<ViewEntity> for ViewDto {
    fn from(entity: ViewEntity) -> Self {
        Self {
            id: entity.id,
            created_at: entity.created_at.to_string(),
            user_id: entity.user_id,
            business_id: entity.business_id,
            listing_id: entity.listing_id,
        }
    }
}
