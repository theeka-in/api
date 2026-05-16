use crate::errors::DbError;
use crate::modules::users::users_entity::{UserAddressEntity, UserEntity};
use sqlx::PgPool;
use std::sync::Arc;
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
        todo!()
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<UserEntity>, DbError> {
        todo!()
    }

    pub async fn create(&self, account_id: Uuid, name: String) -> Result<UserEntity, DbError> {
        todo!()
    }

    pub async fn update(
        &self,
        id: Uuid,
        name: Option<String>,
        avatar: Option<String>,
    ) -> Result<UserEntity, DbError> {
        todo!()
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), DbError> {
        todo!()
    }

    pub async fn find_addresses(&self, user_id: Uuid) -> Result<Vec<UserAddressEntity>, DbError> {
        todo!()
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
        todo!()
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
        todo!()
    }

    pub async fn delete_address(&self, id: Uuid, user_id: Uuid) -> Result<(), DbError> {
        todo!()
    }
}
