use poem_openapi::Object;
use uuid::Uuid;

use crate::modules::database::{UserAddressEntity, UserEntity};

#[derive(Debug, Object)]
pub struct UserDto {
    pub id: Uuid,
    #[oai(validator(min_length = 3, max_length = 60))]
    pub name: String,
    pub avatar: Option<String>,
    pub account_id: Uuid,
}

#[derive(Debug, Object)]
pub struct CreateUserDto {
    #[oai(validator(min_length = 3, max_length = 60))]
    pub name: String,
    pub avatar: Option<String>,
}

#[derive(Debug, Object)]
pub struct UpdateUserDto {
    #[oai(validator(min_length = 3, max_length = 60))]
    pub name: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Object)]
pub struct UserAddressDto {
    pub id: Uuid,
    #[oai(validator(min_length = 1, max_length = 60))]
    pub name: String,
    pub complete_address: String,
    pub city: String,
    pub state: String,
    pub pincode: i32,
    pub latitude: f64,
    pub longitude: f64,
    pub user_id: Uuid,
}

#[derive(Debug, Object)]
pub struct CreateUserAddressDto {
    #[oai(validator(min_length = 1, max_length = 60))]
    pub name: String,
    pub complete_address: String,
    pub city: String,
    pub state: String,
    pub pincode: i32,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Object)]
pub struct UpdateUserAddressDto {
    #[oai(validator(min_length = 1, max_length = 60))]
    pub name: Option<String>,
    pub complete_address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub pincode: Option<i32>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

impl From<UserEntity> for UserDto {
    fn from(entity: UserEntity) -> Self {
        Self {
            id: entity.id,
            name: entity.name,
            avatar: entity.avatar,
            account_id: entity.account_id,
        }
    }
}

impl From<UserAddressEntity> for UserAddressDto {
    fn from(entity: UserAddressEntity) -> Self {
        Self {
            id: entity.id,
            name: entity.name,
            complete_address: entity.complete_address,
            city: entity.city,
            state: entity.state,
            pincode: entity.pincode,
            latitude: entity.latitude,
            longitude: entity.longitude,
            user_id: entity.user_id,
        }
    }
}
