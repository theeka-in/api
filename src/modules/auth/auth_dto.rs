use crate::modules::auth::auth_entity::UserEntity;
use poem_openapi::Object;
use uuid::Uuid;

#[derive(Debug, Object)]
pub struct CreateUserDto {
    #[oai(validator(min_length = 3, max_length = 60))]
    pub name: String,

    #[oai(validator(pattern = r"^[^@\s]+@[^@\s]+\.[^@\s]+$"))]
    pub email: String,

    #[oai(validator(min_length = 3, max_length = 60))]
    pub username: String,
}

#[derive(Debug, Object)]
pub struct UserDto {
    pub id: Uuid,

    #[oai(validator(min_length = 3, max_length = 60))]
    pub name: String,

    #[oai(validator(pattern = r"^[^@\s]+@[^@\s]+\.[^@\s]+$"))]
    pub email: String,

    #[oai(validator(min_length = 3, max_length = 60))]
    pub username: String,
}

impl From<UserEntity> for UserDto {
    fn from(entity: UserEntity) -> Self {
        Self {
            id: entity.id,
            name: entity.name,
            email: entity.email,
            username: entity.username,
        }
    }
}
