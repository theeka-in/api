use super::ListingService;
use crate::errors::{ErrorDto, ServiceError};
use crate::guards::AuthGuard;
use crate::modules::listing::listing_dto::{
    CreateListingMediaDto, CreateProductListingDto, CreateServiceListingDto, ListingDto,
    ListingMediaDto, UpdateListingDto,
};
use poem_openapi::{ApiResponse, OpenApi, param::Path, payload::Json};
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
    #[oai(status = 404)]
    NotFound(Json<ErrorDto>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum CreateListingMediaResponse {
    #[oai(status = 201)]
    Created(Json<ListingMediaDto>),
    #[oai(status = 403)]
    Forbidden(Json<ErrorDto>),
    #[oai(status = 404)]
    NotFound(Json<ErrorDto>),
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
    pub async fn get_listing_by_business(&self, business_id: Path<Uuid>) -> GetListingsResponse {
        match self
            .service
            .get_all_listings_by_business(business_id.0)
            .await
        {
            Ok(listings) => GetListingsResponse::Ok(Json(listings)),
            Err(ServiceError::Internal(dto)) => GetListingsResponse::InternalError(Json(dto)),
            Err(_) => GetListingsResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(path = "/:business_id/listings/:listing_id", method = "get")]
    pub async fn get_listing_by_id(
        &self,
        business_id: Path<Uuid>,
        listing_id: Path<Uuid>,
    ) -> GetListingResponse {
        match self
            .service
            .get_listing_by_id_and_business(business_id.0, listing_id.0)
            .await
        {
            Ok(listing) => GetListingResponse::Ok(Json(listing)),
            Err(ServiceError::NotFound(dto)) => GetListingResponse::NotFound(Json(dto)),
            Err(ServiceError::Internal(dto)) => GetListingResponse::InternalError(Json(dto)),
            Err(_) => GetListingResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(path = "/:business_id/listings/product", method = "post")]
    pub async fn create_product(
        &self,
        business_id: Path<Uuid>,
        body: Json<CreateProductListingDto>,
        auth: AuthGuard,
    ) -> CreateListingResponse {
        let user_id = auth.0.user_id;

        match self
            .service
            .create_product(user_id, business_id.0, body.0)
            .await
        {
            Ok(listing) => CreateListingResponse::Created(Json(listing)),
            Err(ServiceError::Forbidden(dto)) => CreateListingResponse::Forbidden(Json(dto)),
            Err(ServiceError::Internal(dto)) => CreateListingResponse::InternalError(Json(dto)),
            Err(ServiceError::Conflict(dto)) => CreateListingResponse::InternalError(Json(dto)),
            Err(ServiceError::NotFound(dto)) => CreateListingResponse::InternalError(Json(dto)),
            Err(ServiceError::Unauthorized(dto)) => CreateListingResponse::InternalError(Json(dto)),
        }
    }

    #[oai(path = "/:business_id/listings/service", method = "post")]
    pub async fn create_service(
        &self,
        business_id: Path<Uuid>,
        body: Json<CreateServiceListingDto>,
        auth: AuthGuard,
    ) -> CreateListingResponse {
        let user_id = auth.0.user_id;

        match self
            .service
            .create_service(user_id, business_id.0, body.0)
            .await
        {
            Ok(listing) => CreateListingResponse::Created(Json(listing)),
            Err(ServiceError::Forbidden(dto)) => CreateListingResponse::Forbidden(Json(dto)),
            Err(ServiceError::Internal(dto)) => CreateListingResponse::InternalError(Json(dto)),
            Err(_) => CreateListingResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(path = "/:business_id/listings/:listing_id", method = "patch")]
    pub async fn update_listing(
        &self,
        business_id: Path<Uuid>,
        listing_id: Path<Uuid>,
        body: Json<UpdateListingDto>,
        auth: AuthGuard,
    ) -> UpdateListingResponse {
        let user_id = auth.0.user_id;

        match self
            .service
            .update_listing(user_id, business_id.0, listing_id.0, body.0)
            .await
        {
            Ok(listing) => UpdateListingResponse::Ok(Json(listing)),
            Err(ServiceError::Forbidden(dto)) => UpdateListingResponse::Forbidden(Json(dto)),
            Err(ServiceError::NotFound(dto)) => UpdateListingResponse::NotFound(Json(dto)),
            Err(ServiceError::Internal(dto)) => UpdateListingResponse::InternalError(Json(dto)),
            Err(_) => UpdateListingResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(path = "/:business_id/listings/:listing_id", method = "delete")]
    pub async fn delete_listing(
        &self,
        business_id: Path<Uuid>,
        listing_id: Path<Uuid>,
        auth: AuthGuard,
    ) -> DeleteListingResponse {
        let user_id = auth.0.user_id;

        match self
            .service
            .delete_listing(user_id, business_id.0, listing_id.0)
            .await
        {
            Ok(_) => DeleteListingResponse::NoContent,
            Err(ServiceError::Forbidden(dto)) => DeleteListingResponse::Forbidden(Json(dto)),
            Err(ServiceError::NotFound(dto)) => DeleteListingResponse::NotFound(Json(dto)),
            Err(ServiceError::Internal(dto)) => DeleteListingResponse::InternalError(Json(dto)),
            Err(_) => DeleteListingResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(path = "/:business_id/listings/:listing_id/media", method = "get")]
    pub async fn get_media(
        &self,
        business_id: Path<Uuid>,
        listing_id: Path<Uuid>,
    ) -> GetListingMediaResponse {
        match self.service.get_media(business_id.0, listing_id.0).await {
            Ok(media) => GetListingMediaResponse::Ok(Json(media)),
            Err(ServiceError::NotFound(dto)) => GetListingMediaResponse::NotFound(Json(dto)),
            Err(ServiceError::Internal(dto)) => GetListingMediaResponse::InternalError(Json(dto)),
            Err(_) => GetListingMediaResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(path = "/:business_id/listings/:listing_id/media", method = "post")]
    pub async fn create_media(
        &self,
        business_id: Path<Uuid>,
        listing_id: Path<Uuid>,
        body: Json<CreateListingMediaDto>,
        auth: AuthGuard,
    ) -> CreateListingMediaResponse {
        let user_id = auth.0.user_id;

        match self
            .service
            .create_media(user_id, business_id.0, listing_id.0, body.0)
            .await
        {
            Ok(media) => CreateListingMediaResponse::Created(Json(media)),
            Err(ServiceError::Forbidden(dto)) => CreateListingMediaResponse::Forbidden(Json(dto)),
            Err(ServiceError::NotFound(dto)) => CreateListingMediaResponse::NotFound(Json(dto)),
            Err(ServiceError::Internal(dto)) => {
                CreateListingMediaResponse::InternalError(Json(dto))
            }
            Err(_) => CreateListingMediaResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
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
        auth: AuthGuard,
    ) -> DeleteListingMediaResponse {
        let user_id = auth.0.user_id;

        match self
            .service
            .delete_media(user_id, business_id.0, listing_id.0, media_id.0)
            .await
        {
            Ok(_) => DeleteListingMediaResponse::NoContent,
            Err(ServiceError::Forbidden(dto)) => DeleteListingMediaResponse::Forbidden(Json(dto)),
            Err(ServiceError::NotFound(dto)) => DeleteListingMediaResponse::NotFound(Json(dto)),
            Err(ServiceError::Internal(dto)) => {
                DeleteListingMediaResponse::InternalError(Json(dto))
            }
            Err(_) => DeleteListingMediaResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }
}
