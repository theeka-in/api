use super::ReviewService;
use crate::errors::{DbError, ErrorDto};
use crate::guards::AuthGuard;
use crate::modules::review::review_dto::{CreateReviewDto, ReviewDto, UpdateReviewDto};
use poem_openapi::{ApiResponse, Object, OpenApi, param::Path, payload::Json};
use std::sync::Arc;
use uuid::Uuid;

pub struct ReviewController {
    service: Arc<ReviewService>,
}

#[derive(ApiResponse)]
pub enum GetReviewsResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<ReviewDto>>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum CreateReviewResponse {
    #[oai(status = 201)]
    Created(Json<ReviewDto>),
    #[oai(status = 409)]
    Conflict(Json<ErrorDto>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum UpdateReviewResponse {
    #[oai(status = 200)]
    Ok(Json<ReviewDto>),
    #[oai(status = 403)]
    Forbidden(Json<ErrorDto>),
    #[oai(status = 404)]
    NotFound(Json<ErrorDto>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum DeleteReviewResponse {
    #[oai(status = 204)]
    NoContent,
    #[oai(status = 403)]
    Forbidden(Json<ErrorDto>),
    #[oai(status = 404)]
    NotFound(Json<ErrorDto>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[OpenApi(prefix_path = "/businesses")]
impl ReviewController {
    pub fn new(service: Arc<ReviewService>) -> Self {
        Self { service }
    }

    #[oai(path = "/:business_id/listings/:listing_id/reviews", method = "get", operation_id = "get_review_by_listing")]
    pub async fn get_review_by_listing(
        &self,
        business_id: Path<Uuid>,
        listing_id: Path<Uuid>,
    ) -> GetReviewsResponse {
        todo!()
    }

    #[oai(path = "/:business_id/listings/:listing_id/reviews", method = "post", operation_id = "create_review")]
    pub async fn create_review(
        &self,
        business_id: Path<Uuid>,
        listing_id: Path<Uuid>,
        body: Json<CreateReviewDto>,
        auth: AuthGuard,
    ) -> CreateReviewResponse {
        todo!()
    }

    #[oai(
        path = "/:business_id/listings/:listing_id/reviews/:review_id",
        method = "patch",
        operation_id = "update_review"
    )]
    pub async fn update_review(
        &self,
        business_id: Path<Uuid>,
        listing_id: Path<Uuid>,
        review_id: Path<Uuid>,
        body: Json<UpdateReviewDto>,
        auth: AuthGuard,
    ) -> UpdateReviewResponse {
        todo!()
    }

    #[oai(
        path = "/:business_id/listings/:listing_id/reviews/:review_id",
        method = "delete",
        operation_id = "delete_review"
    )]
    pub async fn delete_review(
        &self,
        business_id: Path<Uuid>,
        listing_id: Path<Uuid>,
        review_id: Path<Uuid>,
        auth: AuthGuard,
    ) -> DeleteReviewResponse {
        todo!()
    }
}