use super::ListingService;
use crate::errors::{ErrorDto, ServiceError};
use crate::guards::AuthGuard;
use crate::modules::listing::listing_dto::{
    CreateListingMediaDto, CreateProductListingDto, CreateServiceListingDto, ListingMediaDto,
    ProductListingDto, ServiceListingDto, UpdateProductListingDto, UpdateServiceListingDto,
};
use poem_openapi::param::Query;
use poem_openapi::{ApiResponse, OpenApi, param::Path, payload::Json};
use std::sync::Arc;
use uuid::Uuid;

pub struct ListingController {
    service: Arc<ListingService>,
}

#[derive(ApiResponse)]
pub enum ExploreProductListingsNearbyResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<ProductListingDto>>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum ExploreServiceListingsNearbyResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<ServiceListingDto>>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum GetProductListingsResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<ProductListingDto>>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum GetProductListingResponse {
    #[oai(status = 200)]
    Ok(Json<ProductListingDto>),
    #[oai(status = 404)]
    NotFound(Json<ErrorDto>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum CreateProductListingResponse {
    #[oai(status = 201)]
    Created(Json<ProductListingDto>),
    #[oai(status = 403)]
    Forbidden(Json<ErrorDto>),
    #[oai(status = 404)]
    NotFound(Json<ErrorDto>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum UpdateProductListingResponse {
    #[oai(status = 200)]
    Ok(Json<ProductListingDto>),
    #[oai(status = 403)]
    Forbidden(Json<ErrorDto>),
    #[oai(status = 404)]
    NotFound(Json<ErrorDto>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum GetServiceListingsResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<ServiceListingDto>>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum GetServiceListingResponse {
    #[oai(status = 200)]
    Ok(Json<ServiceListingDto>),
    #[oai(status = 404)]
    NotFound(Json<ErrorDto>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum CreateServiceListingResponse {
    #[oai(status = 201)]
    Created(Json<ServiceListingDto>),
    #[oai(status = 403)]
    Forbidden(Json<ErrorDto>),
    #[oai(status = 404)]
    NotFound(Json<ErrorDto>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum UpdateServiceListingResponse {
    #[oai(status = 200)]
    Ok(Json<ServiceListingDto>),
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

    #[oai(path = "/listings/product/explore", method = "get", operation_id = "explore_product_listings_nearby")]
    pub async fn explore_products_listings_nearby(
        &self,
        latitude: Query<f64>,
        longitude: Query<f64>,
        query: Query<String>,
    ) -> ExploreProductListingsNearbyResponse {
        match self
            .service
            .explore_products_listings_nearby(latitude.0, longitude.0, query.0)
            .await
        {
            Ok(listings) => ExploreProductListingsNearbyResponse::Ok(Json(listings)),
            Err(ServiceError::Internal(dto)) => {
                ExploreProductListingsNearbyResponse::InternalError(Json(dto))
            }
            Err(_) => ExploreProductListingsNearbyResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(path = "/listings/service/explore", method = "get", operation_id = "explore_service_listings_nearby")]
    pub async fn explore_services_listings_nearby(
        &self,
        latitude: Query<f64>,
        longitude: Query<f64>,
        query: Query<String>,
    ) -> ExploreServiceListingsNearbyResponse {
        match self
            .service
            .explore_services_listings_nearby(latitude.0, longitude.0, query.0)
            .await
        {
            Ok(listings) => ExploreServiceListingsNearbyResponse::Ok(Json(listings)),
            Err(ServiceError::Internal(dto)) => {
                ExploreServiceListingsNearbyResponse::InternalError(Json(dto))
            }
            Err(_) => ExploreServiceListingsNearbyResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(path = "/:business_id/listings/product", method = "get", operation_id = "get_product_listings")]
    pub async fn get_product_listings(
        &self,
        business_id: Path<Uuid>,
    ) -> GetProductListingsResponse {
        match self
            .service
            .get_all_product_listings_by_business(business_id.0)
            .await
        {
            Ok(listings) => GetProductListingsResponse::Ok(Json(listings)),
            Err(ServiceError::Internal(dto)) => {
                GetProductListingsResponse::InternalError(Json(dto))
            }
            Err(_) => GetProductListingsResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(path = "/:business_id/listings/product/:listing_id", method = "get", operation_id = "get_product_listing_by_id")]
    pub async fn get_product_listing_by_id(
        &self,
        business_id: Path<Uuid>,
        listing_id: Path<Uuid>,
    ) -> GetProductListingResponse {
        match self
            .service
            .get_product_listing_by_id_and_business(business_id.0, listing_id.0)
            .await
        {
            Ok(listing) => GetProductListingResponse::Ok(Json(listing)),
            Err(ServiceError::NotFound(dto)) => GetProductListingResponse::NotFound(Json(dto)),
            Err(ServiceError::Internal(dto)) => GetProductListingResponse::InternalError(Json(dto)),
            Err(_) => GetProductListingResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(path = "/:business_id/listings/product", method = "post", operation_id = "create_product_listing")]
    pub async fn create_product(
        &self,
        business_id: Path<Uuid>,
        body: Json<CreateProductListingDto>,
        auth: AuthGuard,
    ) -> CreateProductListingResponse {
        match self
            .service
            .create_product(auth.0.user_id, business_id.0, body.0)
            .await
        {
            Ok(listing) => CreateProductListingResponse::Created(Json(listing)),
            Err(ServiceError::Forbidden(dto)) => CreateProductListingResponse::Forbidden(Json(dto)),
            Err(ServiceError::NotFound(dto)) => CreateProductListingResponse::NotFound(Json(dto)),
            Err(ServiceError::Internal(dto)) => {
                CreateProductListingResponse::InternalError(Json(dto))
            }
            Err(_) => CreateProductListingResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(path = "/:business_id/listings/product/:listing_id", method = "patch", operation_id = "update_product_listing")]
    pub async fn update_product(
        &self,
        business_id: Path<Uuid>,
        listing_id: Path<Uuid>,
        body: Json<UpdateProductListingDto>,
        auth: AuthGuard,
    ) -> UpdateProductListingResponse {
        match self
            .service
            .update_product(auth.0.user_id, business_id.0, listing_id.0, body.0)
            .await
        {
            Ok(listing) => UpdateProductListingResponse::Ok(Json(listing)),
            Err(ServiceError::Forbidden(dto)) => UpdateProductListingResponse::Forbidden(Json(dto)),
            Err(ServiceError::NotFound(dto)) => UpdateProductListingResponse::NotFound(Json(dto)),
            Err(ServiceError::Internal(dto)) => {
                UpdateProductListingResponse::InternalError(Json(dto))
            }
            Err(_) => UpdateProductListingResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(path = "/:business_id/listings/product/:listing_id", method = "delete", operation_id = "delete_product_listing")]
    pub async fn delete_product(
        &self,
        business_id: Path<Uuid>,
        listing_id: Path<Uuid>,
        auth: AuthGuard,
    ) -> DeleteListingResponse {
        match self
            .service
            .delete_product(auth.0.user_id, business_id.0, listing_id.0)
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

    #[oai(path = "/:business_id/listings/service", method = "get", operation_id = "get_service_listings")]
    pub async fn get_service_listings(
        &self,
        business_id: Path<Uuid>,
    ) -> GetServiceListingsResponse {
        match self
            .service
            .get_all_service_listings_by_business(business_id.0)
            .await
        {
            Ok(listings) => GetServiceListingsResponse::Ok(Json(listings)),
            Err(ServiceError::Internal(dto)) => {
                GetServiceListingsResponse::InternalError(Json(dto))
            }
            Err(_) => GetServiceListingsResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(path = "/:business_id/listings/service/:listing_id", method = "get", operation_id = "get_service_listing_by_id")]
    pub async fn get_service_listing_by_id(
        &self,
        business_id: Path<Uuid>,
        listing_id: Path<Uuid>,
    ) -> GetServiceListingResponse {
        match self
            .service
            .get_service_listing_by_id_and_business(business_id.0, listing_id.0)
            .await
        {
            Ok(listing) => GetServiceListingResponse::Ok(Json(listing)),
            Err(ServiceError::NotFound(dto)) => GetServiceListingResponse::NotFound(Json(dto)),
            Err(ServiceError::Internal(dto)) => GetServiceListingResponse::InternalError(Json(dto)),
            Err(_) => GetServiceListingResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(path = "/:business_id/listings/service", method = "post", operation_id = "create_service_listing")]
    pub async fn create_service(
        &self,
        business_id: Path<Uuid>,
        body: Json<CreateServiceListingDto>,
        auth: AuthGuard,
    ) -> CreateServiceListingResponse {
        match self
            .service
            .create_service(auth.0.user_id, business_id.0, body.0)
            .await
        {
            Ok(listing) => CreateServiceListingResponse::Created(Json(listing)),
            Err(ServiceError::Forbidden(dto)) => CreateServiceListingResponse::Forbidden(Json(dto)),
            Err(ServiceError::NotFound(dto)) => CreateServiceListingResponse::NotFound(Json(dto)),
            Err(ServiceError::Internal(dto)) => {
                CreateServiceListingResponse::InternalError(Json(dto))
            }
            Err(_) => CreateServiceListingResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(path = "/:business_id/listings/service/:listing_id", method = "patch", operation_id = "update_service_listing")]
    pub async fn update_service(
        &self,
        business_id: Path<Uuid>,
        listing_id: Path<Uuid>,
        body: Json<UpdateServiceListingDto>,
        auth: AuthGuard,
    ) -> UpdateServiceListingResponse {
        match self
            .service
            .update_service(auth.0.user_id, business_id.0, listing_id.0, body.0)
            .await
        {
            Ok(listing) => UpdateServiceListingResponse::Ok(Json(listing)),
            Err(ServiceError::Forbidden(dto)) => UpdateServiceListingResponse::Forbidden(Json(dto)),
            Err(ServiceError::NotFound(dto)) => UpdateServiceListingResponse::NotFound(Json(dto)),
            Err(ServiceError::Internal(dto)) => {
                UpdateServiceListingResponse::InternalError(Json(dto))
            }
            Err(_) => UpdateServiceListingResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(path = "/:business_id/listings/service/:listing_id", method = "delete", operation_id = "delete_service_listing")]
    pub async fn delete_service(
        &self,
        business_id: Path<Uuid>,
        listing_id: Path<Uuid>,
        auth: AuthGuard,
    ) -> DeleteListingResponse {
        match self
            .service
            .delete_service(auth.0.user_id, business_id.0, listing_id.0)
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

    #[oai(
        path = "/:business_id/listings/product/:listing_id/media",
        method = "get",
        operation_id = "get_product_listing_media"
    )]
    pub async fn get_product_media(
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

    #[oai(
        path = "/:business_id/listings/product/:listing_id/media",
        method = "post",
        operation_id = "create_product_listing_media"
    )]
    pub async fn create_product_media(
        &self,
        business_id: Path<Uuid>,
        listing_id: Path<Uuid>,
        body: Json<CreateListingMediaDto>,
        auth: AuthGuard,
    ) -> CreateListingMediaResponse {
        match self
            .service
            .create_media(auth.0.user_id, business_id.0, listing_id.0, body.0)
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
        path = "/:business_id/listings/product/:listing_id/media/:media_id",
        method = "delete",
        operation_id = "delete_product_listing_media"
    )]
    pub async fn delete_product_media(
        &self,
        business_id: Path<Uuid>,
        listing_id: Path<Uuid>,
        media_id: Path<Uuid>,
        auth: AuthGuard,
    ) -> DeleteListingMediaResponse {
        match self
            .service
            .delete_media(auth.0.user_id, business_id.0, listing_id.0, media_id.0)
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

    #[oai(
        path = "/:business_id/listings/service/:listing_id/media",
        method = "get",
        operation_id = "get_service_listing_media"
    )]
    pub async fn get_service_media(
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

    #[oai(
        path = "/:business_id/listings/service/:listing_id/media",
        method = "post",
        operation_id = "create_service_listing_media"
    )]
    pub async fn create_service_media(
        &self,
        business_id: Path<Uuid>,
        listing_id: Path<Uuid>,
        body: Json<CreateListingMediaDto>,
        auth: AuthGuard,
    ) -> CreateListingMediaResponse {
        match self
            .service
            .create_media(auth.0.user_id, business_id.0, listing_id.0, body.0)
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
        path = "/:business_id/listings/service/:listing_id/media/:media_id",
        method = "delete",
        operation_id = "delete_service_listing_media"
    )]
    pub async fn delete_service_media(
        &self,
        business_id: Path<Uuid>,
        listing_id: Path<Uuid>,
        media_id: Path<Uuid>,
        auth: AuthGuard,
    ) -> DeleteListingMediaResponse {
        match self
            .service
            .delete_media(auth.0.user_id, business_id.0, listing_id.0, media_id.0)
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