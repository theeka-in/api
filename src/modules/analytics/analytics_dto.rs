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
