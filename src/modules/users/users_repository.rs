use crate::errors::DbError;
use crate::modules::users::users_entity::{UserAddressEntity, UserEntity};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug)]
pub struct UsersRepository {
    pg: PgPool,
}

impl UsersRepository {
    pub fn new(pg: PgPool) -> Self {
        Self { pg }
    }

    pub async fn find_by_account_id(
        &self,
        account_id: Uuid,
    ) -> Result<Option<UserEntity>, DbError> {
        let user = sqlx::query_as!(
            UserEntity,
            r#"SELECT id, name, avatar, account_id
               FROM users.users
               WHERE account_id = $1"#,
            account_id
        )
        .fetch_optional(&self.pg)
        .await?;

        Ok(user)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<UserEntity>, DbError> {
        let user = sqlx::query_as!(
            UserEntity,
            r#"SELECT id, name, avatar, account_id
               FROM users.users
               WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pg)
        .await?;

        Ok(user)
    }

    pub async fn create(&self, account_id: Uuid, name: String, avatar: Option<String>) -> Result<UserEntity, DbError> {
        let user = sqlx::query_as!(
            UserEntity,
            r#"INSERT INTO users.users (id, account_id, name, avatar)
               VALUES (gen_random_uuid(), $1, $2, $3)
               RETURNING id, name, avatar, account_id"#,
            account_id,
            name,
            avatar
        )
        .fetch_one(&self.pg)
        .await?;

        Ok(user)
    }

    pub async fn update(
        &self,
        id: Uuid,
        name: Option<String>,
        avatar: Option<String>,
    ) -> Result<UserEntity, DbError> {
        let user = sqlx::query_as!(
            UserEntity,
            r#"UPDATE users.users
               SET
                   name   = COALESCE($2, name),
                   avatar = COALESCE($3, avatar)
               WHERE id = $1
               RETURNING id, name, avatar, account_id"#,
            id,
            name,
            avatar
        )
        .fetch_one(&self.pg)
        .await?;

        Ok(user)
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), DbError> {
        sqlx::query!(r#"DELETE FROM users.users WHERE id = $1"#, id)
            .execute(&self.pg)
            .await?;

        Ok(())
    }

    pub async fn find_addresses(&self, user_id: Uuid) -> Result<Vec<UserAddressEntity>, DbError> {
        let addresses = sqlx::query_as!(
            UserAddressEntity,
            r#"SELECT id, name, complete_address, city, state, pincode, latitude, longitude, user_id
               FROM users.user_addresses
               WHERE user_id = $1"#,
            user_id
        )
        .fetch_all(&self.pg)
        .await?;

        Ok(addresses)
    }

    pub async fn create_address(
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
            r#"INSERT INTO users.user_addresses
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

    pub async fn update_address(
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
            r#"UPDATE users.user_addresses
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

    pub async fn delete_address(&self, id: Uuid, user_id: Uuid) -> Result<(), DbError> {
        sqlx::query!(
            r#"DELETE FROM users.user_addresses WHERE id = $1 AND user_id = $2"#,
            id,
            user_id
        )
        .execute(&self.pg)
        .await?;

        Ok(())
    }
}
