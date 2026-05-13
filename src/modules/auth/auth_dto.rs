use poem_openapi::Object;
use uuid::Uuid;

#[derive(Debug, Object)]
pub struct RegisterDto {
    pub phone: i64,
    #[oai(validator(min_length = 8))]
    pub password: String,
    #[oai(validator(min_length = 3, max_length = 60))]
    pub name: String,
}

#[derive(Debug, Object)]
pub struct LoginDto {
    pub phone: i64,
    pub password: String,
}

#[derive(Debug, Object)]
pub struct AccountDto {
    pub id: Uuid,
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
}
