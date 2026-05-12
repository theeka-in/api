use crate::errors::DbError;
use crate::modules::auth::auth_dto::{CreateUserDto, UserDto};
use crate::modules::auth::auth_entity::UserEntity;
use crate::modules::users::UsersService;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct AuthService {
    users_service: Arc<UsersService>,
    pg: PgPool,
}

impl AuthService {
    pub fn new(users_service: Arc<UsersService>, pg: PgPool) -> Arc<Self> {
        Arc::new(Self { users_service, pg })
    }

    pub fn hello_from_auth(&self, name: &str) -> String {
        format!("{} from auth", self.users_service.hello(name))
    }

    pub async fn db_health(&self) -> Vec<UserDto> {
        let user_entities = sqlx::query_as!(
            UserEntity,
            r#"--sql
                SELECT * FROM users;
            "#
        )
        .fetch_all(&self.pg)
        .await
        .unwrap();

        user_entities.into_iter().map(UserDto::from).collect()
    }

    pub async fn create_user(&self, body: CreateUserDto) -> Result<UserDto, DbError> {
        let user_entity = sqlx::query_as!(
            UserEntity,
            r#"--sql
            INSERT INTO users (id, name, email, username) VALUES ($1, $2, $3, $4) RETURNING id, name, email, username
            "#,
            Uuid::new_v4(),
            body.name,
            body.email,
            body.username
        )
        .fetch_one(&self.pg)
        .await.map_err(DbError::from)?;

        Ok(UserDto::from(user_entity))
    }
}
