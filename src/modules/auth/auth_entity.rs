use sqlx::{FromRow, types::chrono};
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct AccountEntity {
    pub id: Uuid,
    pub phone: i64,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, FromRow)]
pub struct SessionEntity {
    pub token: String,
    pub user_agent: String,
    pub ip_address: String,
    pub created_at: chrono::NaiveDateTime,
    pub account_id: Uuid,
}
