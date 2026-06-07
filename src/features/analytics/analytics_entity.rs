use sqlx::{FromRow, types::chrono};
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct ViewEntity {
    pub id: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub user_id: Uuid,
    pub business_id: Uuid,
    pub listing_id: Uuid,
}
