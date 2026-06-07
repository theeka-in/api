use poem_openapi::Enum;
use sqlx::{FromRow, types::chrono};
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct BusinessEntity {
    pub id: Uuid,
    pub phone_number: i64,
    pub is_closed: bool,
    pub title: String,
    pub logo: Option<String>,
    pub description: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub owner_id: Uuid,
}

#[derive(Debug, FromRow)]
pub struct BusinessAddressEntity {
    pub address_line1: String,
    pub address_line2: String,
    pub landmark: Option<String>,
    pub pincode: String,
    pub city: String,
    pub state: String,
    pub latitude: f64,
    pub longitude: f64,
    pub radius: f64,
    pub business_id: Uuid,
}

#[derive(Debug, sqlx::Type, PartialEq)]
#[sqlx(type_name = "business.day_of_week", rename_all = "snake_case")]
#[derive(Enum)]
pub enum DayOfWeekType {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Debug, sqlx::Type, PartialEq)]
#[sqlx(type_name = "business.business_hour_type", rename_all = "snake_case")]
#[derive(Enum)]
pub enum BusinessHourType {
    Closed,
    #[sqlx(rename = "open_24_hours")]
    Open24Hours,
    CustomRange,
}

#[derive(Debug, FromRow)]
pub struct BusinessHourEntity {
    pub id: Uuid,
    pub day: DayOfWeekType,
    pub hours_type: BusinessHourType,
    pub open_time: Option<chrono::NaiveTime>,
    pub close_time: Option<chrono::NaiveTime>,
    pub business_id: Uuid,
}

#[derive(Debug, sqlx::Type, PartialEq)]
#[sqlx(type_name = "business.media_type", rename_all = "snake_case")]
pub enum MediaType {
    Image,
    Video,
}

#[derive(Debug, FromRow)]
pub struct BusinessMediaEntity {
    pub id: Uuid,
    pub media_type: MediaType,
    pub url: String,
    pub business_id: Uuid,
}
