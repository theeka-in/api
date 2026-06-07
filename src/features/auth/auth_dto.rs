use poem_openapi::Object;
use sqlx::prelude::Type;
use uuid::Uuid;

use crate::features::auth::auth_entity::{AccountEntity, SessionEntity};


#[derive(Debug, Object)]
pub struct RegisterDto {
    #[oai(validator(minimum(value = "1111111111"), maximum(value = "9999999999")))]
    pub phone: i64,
    #[oai(validator(min_length = 8))]
    pub password: String,
    #[oai(validator(min_length = 3, max_length = 60))]
    pub name: String,
    pub avatar: Option<String>,
}

#[derive(Debug, Object)]
pub struct LoginDto {
    #[oai(validator(minimum(value = "1111111111"), maximum(value = "9999999999")))]
    pub phone: i64,
    #[oai(validator(min_length = 8))]
    pub password: String,
}

#[derive(Debug, Object)]
pub struct AccountDto {
    pub id: Uuid,
    #[oai(validator(minimum(value = "1111111111"), maximum(value = "9999999999")))]
    pub phone: i64,
    pub created_at: String,
}

#[derive(Debug, Object)]
pub struct SessionDto {
    pub token: String,
    pub user_agent: String,
    pub ip_address: String,
    pub created_at: String,
    pub account_id: Uuid,
    pub user_id: Uuid,
}

impl From<AccountEntity> for AccountDto {
    fn from(e: AccountEntity) -> Self {
        Self {
            id: e.id,
            phone: e.phone,
            created_at: e.created_at.to_string(),
        }
    }
}

impl From<SessionEntity> for SessionDto {
    fn from(e: SessionEntity) -> Self {
        Self {
            token: e.token,
            user_agent: e.user_agent,
            ip_address: e.ip_address,
            created_at: e.created_at.to_string(),
            account_id: e.account_id,
            user_id: e.user_id
        }
    }
}
