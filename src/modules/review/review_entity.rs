use sqlx::{FromRow, types::chrono};
use uuid::Uuid;

#[derive(Debug, sqlx::Type, PartialEq)]
#[sqlx(type_name = "media_type", rename_all = "snake_case")]
pub enum MediaType {
    Image,
    Video,
}

#[derive(Debug, FromRow)]
pub struct ReviewEntity {
    pub id: Uuid,
    pub rating: i32,
    pub title: String,
    pub comment: String,
    pub created_at: chrono::NaiveDateTime,
    pub user_id: Uuid,
    pub business_id: Uuid,
    pub listing_id: Uuid,
}

#[derive(Debug, FromRow)]
pub struct ReviewMediaEntity {
    pub id: Uuid,
    pub media_type: MediaType,
    pub url: String,
    pub review_id: Uuid,
}
