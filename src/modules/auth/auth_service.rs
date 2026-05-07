use std::sync::Arc;

use sea_orm::ConnectionTrait;

use crate::modules::{database::DatabaseService, users::UsersService};

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

    pub async fn db_health(&self) -> String {
        self.database_service.health_check().await
    }
}
