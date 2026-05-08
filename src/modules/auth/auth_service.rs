use std::sync::Arc;

use sea_orm::{ActiveModelTrait, ConnectionTrait};

use crate::{
    entities::user,
    modules::{
        auth::auth_controller::{CreateUserRequest, UserDto},
        database::DatabaseService,
        users::UsersService,
    },
};

#[derive(Debug)]
pub struct AuthService {
    users_service: Arc<UsersService>,
    database_service: Arc<DatabaseService>,
}

impl AuthService {
    pub fn new(
        database_service: Arc<DatabaseService>,
        users_service: Arc<UsersService>,
    ) -> Arc<Self> {
        Arc::new(Self {
            database_service,
            users_service,
        })
    }

    pub fn hello_from_auth(&self, name: &str) -> String {
        format!("{} from auth", self.users_service.hello(name))
    }

    pub async fn db_health(&self) -> Vec<user::Model> {
        self.database_service.health_check().await
    }

    pub async fn create_user(&self, body: CreateUserRequest) -> UserDto {
        let user = user::ActiveModel::from(body)
            .insert(self.database_service.db())
            .await
            .unwrap();

        UserDto::from(user)
    }
}
