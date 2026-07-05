use super::AnalyticsService;
use crate::modules::analytics::analytics_dto::{
    BusinessAnalyticsDto, ListingAnalyticsDto, RecordBusinessViewDto, ViewDto,
};
use crate::shared::errors::ErrorDto;
use poem_openapi::{param::Path, payload::Json, ApiResponse, OpenApi};
use std::sync::Arc;
use uuid::Uuid;

pub struct AnalyticsController {
    service: Arc<AnalyticsService>,
}

#[derive(ApiResponse)]
pub enum RecordViewResponse {
    #[oai(status = 201)]
    Created(Json<ViewDto>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum GetBusinessAnalyticsResponse {
    #[oai(status = 200)]
    Ok(Json<BusinessAnalyticsDto>),
    #[oai(status = 403)]
    Forbidden(Json<ErrorDto>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum GetListingAnalyticsResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<ListingAnalyticsDto>>),
    #[oai(status = 403)]
    Forbidden(Json<ErrorDto>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[OpenApi(prefix_path = "/businesses")]
impl AnalyticsController {
    pub fn new(service: Arc<AnalyticsService>) -> Self {
        Self { service }
    }

    #[oai(
        path = "/:business_id/views",
        method = "post",
        operation_id = "record_business_view"
    )]
    pub async fn record_business_view(
        &self,
        business_id: Path<Uuid>,
        body: Json<RecordBusinessViewDto>,
    ) -> RecordViewResponse {
        todo!()
    }

    #[oai(
        path = "/:business_id/listings/:listing_id/views",
        method = "post",
        operation_id = "record_listing_view"
    )]
    pub async fn record_listing_view(
        &self,
        business_id: Path<Uuid>,
        listing_id: Path<Uuid>,
    ) -> RecordViewResponse {
        todo!()
    }

    #[oai(
        path = "/:business_id/analytics",
        method = "get",
        operation_id = "get_business_analytics"
    )]
    pub async fn get_business_analytics(
        &self,
        business_id: Path<Uuid>,
    ) -> GetBusinessAnalyticsResponse {
        todo!()
    }

    #[oai(
        path = "/:business_id/listings/analytics",
        method = "get",
        operation_id = "get_listing_analytics"
    )]
    pub async fn get_listing_analytics(
        &self,
        business_id: Path<Uuid>,
    ) -> GetListingAnalyticsResponse {
        todo!()
    }
}
