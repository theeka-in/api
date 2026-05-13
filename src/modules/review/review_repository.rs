use crate::errors::DbError;
use crate::modules::review::review_entity::ReviewEntity;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct ReviewRepository {
    pg: PgPool,
}

impl ReviewRepository {
    pub fn new(pg: PgPool) -> Self {
        Self { pg }
    }

    pub async fn find_by_listing(
        &self,
        business_id: Uuid,
        listing_id: Uuid,
    ) -> Result<Vec<ReviewEntity>, DbError> {
        todo!()
    }

    pub async fn find_by_id(
        &self,
        id: Uuid,
        listing_id: Uuid,
    ) -> Result<Option<ReviewEntity>, DbError> {
        todo!()
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        business_id: Uuid,
        listing_id: Uuid,
        rating: i32,
        title: String,
        comment: String,
    ) -> Result<ReviewEntity, DbError> {
        todo!()
    }

    pub async fn update(
        &self,
        id: Uuid,
        user_id: Uuid,
        rating: Option<i32>,
        title: Option<String>,
        comment: Option<String>,
    ) -> Result<ReviewEntity, DbError> {
        todo!()
    }

    pub async fn delete(&self, id: Uuid, user_id: Uuid) -> Result<(), DbError> {
        todo!()
    }
}
