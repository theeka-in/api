use sqlx::{PgPool, prelude::FromRow};
use uuid::Uuid;

use crate::shared::errors::DbError;

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

#[derive(Debug)]
pub struct UserAddressRepo {
    pg: PgPool,
}

impl UserAddressRepo {
    pub fn new(pg: PgPool) -> Self {
        Self { pg }
    }

    pub async fn find_all(&self, user_id: Uuid) -> Result<Vec<UserAddressEntity>, DbError> {
        let addresses = sqlx::query_as!(
            UserAddressEntity,
            r#"SELECT id, name, complete_address, city, state, pincode, latitude, longitude, user_id
               FROM user_addresses
               WHERE user_id = $1"#,
            user_id
        )
        .fetch_all(&self.pg)
        .await?;

        Ok(addresses)
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        name: String,
        complete_address: String,
        city: String,
        state: String,
        pincode: i32,
        latitude: f64,
        longitude: f64,
    ) -> Result<UserAddressEntity, DbError> {
        let address = sqlx::query_as!(
            UserAddressEntity,
            r#"INSERT INTO user_addresses
                   (id, user_id, name, complete_address, city, state, pincode, latitude, longitude)
               VALUES
                   (gen_random_uuid(), $1, $2, $3, $4, $5, $6, $7, $8)
               RETURNING id, name, complete_address, city, state, pincode, latitude, longitude, user_id"#,
            user_id,
            name,
            complete_address,
            city,
            state,
            pincode,
            latitude,
            longitude
        )
        .fetch_one(&self.pg)
        .await?;

        Ok(address)
    }

    pub async fn update(
        &self,
        id: Uuid,
        user_id: Uuid,
        name: Option<String>,
        complete_address: Option<String>,
        city: Option<String>,
        state: Option<String>,
        pincode: Option<i32>,
        latitude: Option<f64>,
        longitude: Option<f64>,
    ) -> Result<UserAddressEntity, DbError> {
        let address = sqlx::query_as!(
            UserAddressEntity,
            r#"UPDATE user_addresses
               SET
                   name             = COALESCE($3, name),
                   complete_address = COALESCE($4, complete_address),
                   city             = COALESCE($5, city),
                   state            = COALESCE($6, state),
                   pincode          = COALESCE($7, pincode),
                   latitude         = COALESCE($8, latitude),
                   longitude        = COALESCE($9, longitude)
               WHERE id = $1 AND user_id = $2
               RETURNING id, name, complete_address, city, state, pincode, latitude, longitude, user_id"#,
            id,
            user_id,
            name,
            complete_address,
            city,
            state,
            pincode,
            latitude,
            longitude
        )
        .fetch_one(&self.pg)
        .await?;

        Ok(address)
    }

    pub async fn delete(&self, id: Uuid, user_id: Uuid) -> Result<(), DbError> {
        sqlx::query!(
            r#"DELETE FROM user_addresses WHERE id = $1 AND user_id = $2"#,
            id,
            user_id
        )
        .execute(&self.pg)
        .await?;

        Ok(())
    }
}
