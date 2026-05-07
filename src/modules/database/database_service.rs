use std::sync::Arc;

use sea_orm::{ConnectionTrait, Database, DatabaseConnection, TryGetable};

#[derive(Debug)]
pub struct DatabaseService {
    db: DatabaseConnection,
}

impl DatabaseService {
    pub async fn new(database_url: String) -> Arc<Self> {
        let db = Database::connect(database_url).await.unwrap();
        Arc::new(Self { db })
    }

    pub async fn health_check(&self) -> String {
        use sea_orm::{DbBackend, Statement};
        let row = self
            .db
            .query_one(Statement::from_string(
                DbBackend::Postgres,
                "SELECT CAST((1 + 1 * 100 / 50 + 20 / 10) AS TEXT) as val".to_owned(),
            ))
            .await
            .unwrap()
            .unwrap();

        String::try_get(&row, "", "val").unwrap()
    }

    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }
}
