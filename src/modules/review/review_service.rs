use crate::errors::{DbError, ServiceError};
use crate::modules::review::review_dto::{CreateReviewDto, ReviewDto, UpdateReviewDto};
use crate::modules::review::review_repository::ReviewRepository;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct ReviewService {
    repo: ReviewRepository,
}

impl ReviewService {
    pub fn new(repo: ReviewRepository) -> Arc<Self> {
        Arc::new(Self { repo })
    }

    pub async fn get_by_listing(
        &self,
        business_id: Uuid,
        listing_id: Uuid,
    ) -> Result<Vec<ReviewDto>, ServiceError> {
        todo!()
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        business_id: Uuid,
        listing_id: Uuid,
        body: CreateReviewDto,
    ) -> Result<ReviewDto, ServiceError> {
        todo!()
    }

    pub async fn update(
        &self,
        user_id: Uuid,
        business_id: Uuid,
        listing_id: Uuid,
        review_id: Uuid,
        body: UpdateReviewDto,
    ) -> Result<ReviewDto, ServiceError> {
        todo!()
    }

    pub async fn delete(
        &self,
        user_id: Uuid,
        business_id: Uuid,
        listing_id: Uuid,
        review_id: Uuid,
    ) -> Result<(), ServiceError> {
        todo!()
    }
}
