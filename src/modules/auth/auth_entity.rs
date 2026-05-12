use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct UserEntity {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub username: String,
}
