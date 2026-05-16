use super::ListingService;
use crate::errors::{DbError, ErrorDto};
use crate::guards::BearerAuth;
use crate::modules::listing::listing_dto::{
    CreateListingMediaDto, CreateProductListingDto, CreateServiceListingDto, ListingDto,
    ListingMediaDto, UpdateListingDto,
};
use poem_openapi::{ApiResponse, Object, OpenApi, param::Path, payload::Json};
use std::sync::Arc;
use uuid::Uuid;

pub struct ListingController {
    service: Arc<ListingService>,
}

#[derive(ApiResponse)]
pub enum GetListingsResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<ListingDto>>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum GetListingResponse {
    #[oai(status = 200)]
    Ok(Json<ListingDto>),
    #[oai(status = 404)]
    NotFound(Json<ErrorDto>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum CreateListingResponse {
    #[oai(status = 201)]
    Created(Json<ListingDto>),
    #[oai(status = 403)]
    Forbidden(Json<ErrorDto>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum UpdateListingResponse {
    #[oai(status = 200)]
    Ok(Json<ListingDto>),
    #[oai(status = 403)]
    Forbidden(Json<ErrorDto>),
    #[oai(status = 404)]
    NotFound(Json<ErrorDto>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum DeleteListingResponse {
    #[oai(status = 204)]
    NoContent,
    #[oai(status = 403)]
    Forbidden(Json<ErrorDto>),
    #[oai(status = 404)]
    NotFound(Json<ErrorDto>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum GetListingMediaResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<ListingMediaDto>>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum CreateListingMediaResponse {
    #[oai(status = 201)]
    Created(Json<ListingMediaDto>),
    #[oai(status = 403)]
    Forbidden(Json<ErrorDto>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum DeleteListingMediaResponse {
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
impl ListingController {
    pub fn new(service: Arc<ListingService>) -> Self {
        Self { service }
    }

    #[oai(path = "/:business_id/listings", method = "get")]
    pub async fn get_by_business(&self, business_id: Path<Uuid>) -> GetListingsResponse {
        todo!()
    }

    #[oai(path = "/:business_id/listings/:listing_id", method = "get")]
    pub async fn get_by_id(
        &self,
        business_id: Path<Uuid>,
        listing_id: Path<Uuid>,
    ) -> GetListingResponse {
        todo!()
    }

    #[oai(path = "/:business_id/listings/product", method = "post")]
    pub async fn create_product(
        &self,
        business_id: Path<Uuid>,
        body: Json<CreateProductListingDto>,
        auth: BearerAuth,
    ) -> CreateListingResponse {
        todo!()
    }

    #[oai(path = "/:business_id/listings/service", method = "post")]
    pub async fn create_service(
        &self,
        business_id: Path<Uuid>,
        body: Json<CreateServiceListingDto>,
        auth: BearerAuth,
    ) -> CreateListingResponse {
        todo!()
    }

    #[oai(path = "/:business_id/listings/:listing_id", method = "patch")]
    pub async fn update(
        &self,
        business_id: Path<Uuid>,
        listing_id: Path<Uuid>,
        body: Json<UpdateListingDto>,
        auth: BearerAuth,
    ) -> UpdateListingResponse {
        todo!()
    }

    #[oai(path = "/:business_id/listings/:listing_id", method = "delete")]
    pub async fn delete(
        &self,
        business_id: Path<Uuid>,
        listing_id: Path<Uuid>,
        auth: BearerAuth,
    ) -> DeleteListingResponse {
        todo!()
    }

    #[oai(path = "/:business_id/listings/:listing_id/media", method = "get")]
    pub async fn get_media(
        &self,
        business_id: Path<Uuid>,
        listing_id: Path<Uuid>,
    ) -> GetListingMediaResponse {
        todo!()
    }

    #[oai(path = "/:business_id/listings/:listing_id/media", method = "post")]
    pub async fn create_media(
        &self,
        business_id: Path<Uuid>,
        listing_id: Path<Uuid>,
        body: Json<CreateListingMediaDto>,
        auth: BearerAuth,
    ) -> CreateListingMediaResponse {
        todo!()
    }

    #[oai(
        path = "/:business_id/listings/:listing_id/media/:media_id",
        method = "delete"
    )]
    pub async fn delete_media(
        &self,
        business_id: Path<Uuid>,
        listing_id: Path<Uuid>,
        media_id: Path<Uuid>,
        auth: BearerAuth,
    ) -> DeleteListingMediaResponse {
        todo!()
    }
}
