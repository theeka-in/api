use poem_openapi::{Enum, Object};
use sqlx::types::chrono::NaiveTime;
use uuid::Uuid;

use crate::modules::database::{BusinessAddressEntity, BusinessEntity, BusinessHourEntity, BusinessHourType, BusinessMediaEntity, DayOfWeekType};

#[derive(Debug, Object)]
pub struct BusinessDto {
    pub id: Uuid,
    pub phone_number: i64,
    pub is_closed: bool,
    pub title: String,
    pub logo: Option<String>,
    pub description: Option<String>,
    pub created_at: String,
    pub owner_id: Uuid,
    pub address: Option<BusinessAddressDto>,
}

#[derive(Debug, Object)]
pub struct CreateBusinessDto {
    #[oai(validator(minimum(value = "1111111111"), maximum(value = "9999999999")))]
    pub phone_number: Option<i64>,
    #[oai(validator(min_length = 3, max_length = 120))]
    pub title: String,
    pub logo: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Object)]
pub struct UpdateBusinessDto {
    pub phone_number: Option<i64>,
    #[oai(validator(min_length = 3, max_length = 120))]
    pub title: Option<String>,
    pub logo: Option<String>,
    pub description: Option<String>,
    pub is_closed: Option<bool>,
}

#[derive(Debug, Object)]
pub struct BusinessAddressDto {
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

#[derive(Debug, Object)]
pub struct CreateBusinessAddressDto {
    pub address_line1: String,
    pub address_line2: String,
    pub landmark: Option<String>,
    pub pincode: String,
    pub city: String,
    pub state: String,
    pub latitude: f64,
    pub longitude: f64,
    pub radius: f64,
}

#[derive(Debug, Object)]
pub struct UpdateBusinessAddressDto {
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub landmark: Option<String>,
    pub pincode: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub radius: Option<f64>,
}

const TIME_PATTERN: &str = r"^([01]\d|2[0-3]):[0-5]\d$";

#[derive(Debug, Object)]
pub struct BusinessHourDto {
    pub id: Uuid,
    pub day: DayOfWeekType,
    pub hours_type: BusinessHourType,
    pub open_time: Option<String>,
    pub close_time: Option<String>,
    pub business_id: Uuid,
}

#[derive(Debug, Object)]
pub struct CreateBusinessHourDto {
    pub day: DayOfWeekType,
    pub hours_type: BusinessHourType,
    #[oai(validator(pattern = "^([01]\\d|2[0-3]):[0-5]\\d$"))]
    pub open_time: Option<String>,
    #[oai(validator(pattern = "^([01]\\d|2[0-3]):[0-5]\\d$"))]
    pub close_time: Option<String>,
}

#[derive(Debug, Object)]
pub struct UpdateBusinessHourDto {
    pub hours_type: Option<BusinessHourType>,
    #[oai(validator(pattern = "^([01]\\d|2[0-3]):[0-5]\\d$"))]
    pub open_time: Option<String>,
    #[oai(validator(pattern = "^([01]\\d|2[0-3]):[0-5]\\d$"))]
    pub close_time: Option<String>,
}

#[derive(Debug, Object)]
pub struct BusinessMediaDto {
    pub id: Uuid,
    pub media_type: String,
    pub url: String,
    pub business_id: Uuid,
}

#[derive(Debug, Object)]
pub struct CreateBusinessMediaDto {
    pub media_type: String,
    pub url: String,
}

impl From<BusinessEntity> for BusinessDto {
    fn from(entity: BusinessEntity) -> Self {
        Self {
            id: entity.id,
            phone_number: entity.phone_number,
            is_closed: entity.is_closed,
            title: entity.title,
            logo: entity.logo,
            description: entity.description,
            created_at: entity.created_at.to_string(),
            owner_id: entity.owner_id,
            address: None,
        }
    }
}

impl From<BusinessHourEntity> for BusinessHourDto {
    fn from(entity: BusinessHourEntity) -> Self {
        Self {
            id: entity.id,
            day: entity.day.into(),
            hours_type: entity.hours_type.into(),
            open_time: entity.open_time.map(|t| t.to_string()),
            close_time: entity.close_time.map(|t| t.to_string()),
            business_id: entity.business_id,
        }
    }
}

impl From<BusinessMediaEntity> for BusinessMediaDto {
    fn from(entity: BusinessMediaEntity) -> Self {
        Self {
            id: entity.id,
            media_type: format!("{:?}", entity.media_type).to_lowercase(),
            url: entity.url,
            business_id: entity.business_id,
        }
    }
}

impl From<BusinessAddressEntity> for BusinessAddressDto {
    fn from(entity: BusinessAddressEntity) -> Self {
        Self {
            address_line1: entity.address_line1,
            address_line2: entity.address_line2,
            landmark: entity.landmark,
            pincode: entity.pincode,
            city: entity.city,
            state: entity.state,
            latitude: entity.latitude,
            longitude: entity.longitude,
            radius: entity.radius,
            business_id: entity.business_id,
        }
    }
}
