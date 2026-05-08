use sea_orm::{ConnectionTrait, Database, DatabaseConnection, EntityTrait, TryGetable};
use std::sync::Arc;

use crate::entities::user;

#[derive(Debug)]
pub struct DatabaseService {
    db: DatabaseConnection,
}

impl DatabaseService {
    pub async fn new(database_url: String) -> Arc<Self> {
        let db = Database::connect(database_url).await.unwrap();
        Arc::new(Self { db })
    }

    pub async fn health_check(&self) -> Vec<user::Model> {
        let users = user::Entity::find().all(&self.db).await.unwrap();
        users
    }

    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }
}
