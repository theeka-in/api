use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct UserEntity {
    pub id: Uuid,
    pub name: String,
    pub avatar: Option<String>,
    pub account_id: Uuid,
}

#[derive(Debug, FromRow)]
pub struct UserAddressEntity {
    pub id: Uuid,
    pub name: String,
    pub complete_address: String,
    pub city: String,
    pub state: String,
    pub pincode: i32,
    pub latitude: f64,
    pub longitude: f64,
    pub user_id: Uuid,
}
