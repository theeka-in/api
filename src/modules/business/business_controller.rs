use super::BusinessService;
use crate::errors::{ErrorDto, ServiceError};
use crate::guards::AuthGuard;
use crate::modules::business::business_dto::{
    BusinessAddressDto, BusinessDto, BusinessHourDto, BusinessMediaDto, CreateBusinessDto,
    CreateBusinessHourDto, CreateBusinessMediaDto, UpdateBusinessDto, UpdateBusinessHourDto,
};
use crate::modules::business::business_entity::DayOfWeekType;
use crate::modules::business::{CreateBusinessAddressDto, UpdateBusinessAddressDto};
use poem_openapi::{
    ApiResponse, OpenApi,
    param::{Path, Query},
    payload::Json,
};
use std::sync::Arc;
use uuid::Uuid;

pub struct BusinessController {
    service: Arc<BusinessService>,
}

#[derive(ApiResponse)]
pub enum GetNearbyBusinessesResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<BusinessDto>>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum GetMyBusinessesResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<BusinessDto>>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum GetBusinessResponse {
    #[oai(status = 200)]
    Ok(Json<BusinessDto>),
    #[oai(status = 404)]
    NotFound(Json<ErrorDto>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum CreateBusinessResponse {
    #[oai(status = 201)]
    Created(Json<BusinessDto>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum UpdateBusinessResponse {
    #[oai(status = 200)]
    Ok(Json<BusinessDto>),
    #[oai(status = 403)]
    Forbidden(Json<ErrorDto>),
    #[oai(status = 404)]
    NotFound(Json<ErrorDto>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum DeleteBusinessResponse {
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
pub enum GetAddressResponse {
    #[oai(status = 200)]
    Ok(Json<BusinessAddressDto>),
    #[oai(status = 404)]
    NotFound(Json<ErrorDto>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum InsertAddressResponse {
    #[oai(status = 200)]
    Ok(Json<BusinessAddressDto>),
    #[oai(status = 403)]
    Forbidden(Json<ErrorDto>),
    #[oai(status = 404)]
    NotFound(Json<ErrorDto>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum UpdateAddressResponse {
    #[oai(status = 200)]
    Ok(Json<BusinessAddressDto>),
    #[oai(status = 403)]
    Forbidden(Json<ErrorDto>),
    #[oai(status = 404)]
    NotFound(Json<ErrorDto>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum DeleteAddressResponse {
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
pub enum GetHoursResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<BusinessHourDto>>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum CreateHourResponse {
    #[oai(status = 201)]
    Created(Json<BusinessHourDto>),
    #[oai(status = 409)]
    Conflict(Json<ErrorDto>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum UpdateHourResponse {
    #[oai(status = 200)]
    Ok(Json<BusinessHourDto>),
    #[oai(status = 404)]
    NotFound(Json<ErrorDto>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum DeleteHourResponse {
    #[oai(status = 204)]
    NoContent,
    #[oai(status = 404)]
    NotFound(Json<ErrorDto>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum GetMediaResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<BusinessMediaDto>>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum CreateMediaResponse {
    #[oai(status = 201)]
    Created(Json<BusinessMediaDto>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum DeleteMediaResponse {
    #[oai(status = 204)]
    NoContent,
    #[oai(status = 404)]
    NotFound(Json<ErrorDto>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[OpenApi(prefix_path = "/businesses")]
impl BusinessController {
    pub fn new(service: Arc<BusinessService>) -> Self {
        Self { service }
    }

    #[oai(path = "/", method = "get", operation_id = "get_my_businesses")]
    pub async fn get_all_businesses(&self, auth: AuthGuard) -> GetMyBusinessesResponse {
        let user_id = auth.0.user_id;

        match self.service.get_all_businesses_by_owner(user_id).await {
            Ok(businesses) => GetMyBusinessesResponse::Ok(Json(businesses)),
            Err(ServiceError::Internal(dto)) => GetMyBusinessesResponse::InternalError(Json(dto)),
            Err(_) => GetMyBusinessesResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(
        path = "/:business_id",
        method = "get",
        operation_id = "get_business_by_id"
    )]
    pub async fn get_by_id(&self, business_id: Path<Uuid>) -> GetBusinessResponse {
        match self.service.get_business_by_id(business_id.0).await {
            Ok(business) => GetBusinessResponse::Ok(Json(business)),
            Err(ServiceError::NotFound(dto)) => GetBusinessResponse::NotFound(Json(dto)),
            Err(ServiceError::Internal(dto)) => GetBusinessResponse::InternalError(Json(dto)),
            Err(_) => GetBusinessResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(path = "/", method = "post", operation_id = "create_business")]
    pub async fn create_business(
        &self,
        body: Json<CreateBusinessDto>,
        auth: AuthGuard,
    ) -> CreateBusinessResponse {
        let user_id = auth.0.user_id;

        match self.service.create_business(user_id, body.0).await {
            Ok(business) => CreateBusinessResponse::Created(Json(business)),
            Err(ServiceError::Internal(dto)) => CreateBusinessResponse::InternalError(Json(dto)),
            Err(_) => CreateBusinessResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(
        path = "/:business_id",
        method = "patch",
        operation_id = "update_business"
    )]
    pub async fn update_business(
        &self,
        business_id: Path<Uuid>,
        body: Json<UpdateBusinessDto>,
        auth: AuthGuard,
    ) -> UpdateBusinessResponse {
        let user_id = auth.0.user_id;

        match self
            .service
            .update_business(business_id.0, user_id, body.0)
            .await
        {
            Ok(business) => UpdateBusinessResponse::Ok(Json(business)),
            Err(ServiceError::Forbidden(dto)) => UpdateBusinessResponse::Forbidden(Json(dto)),
            Err(ServiceError::NotFound(dto)) => UpdateBusinessResponse::NotFound(Json(dto)),
            Err(ServiceError::Internal(dto)) => UpdateBusinessResponse::InternalError(Json(dto)),
            Err(_) => UpdateBusinessResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(
        path = "/:business_id",
        method = "delete",
        operation_id = "delete_business"
    )]
    pub async fn delete_business(
        &self,
        business_id: Path<Uuid>,
        auth: AuthGuard,
    ) -> DeleteBusinessResponse {
        let user_id = auth.0.user_id;

        match self.service.delete_business(business_id.0, user_id).await {
            Ok(_) => DeleteBusinessResponse::NoContent,
            Err(ServiceError::Forbidden(dto)) => DeleteBusinessResponse::Forbidden(Json(dto)),
            Err(ServiceError::NotFound(dto)) => DeleteBusinessResponse::NotFound(Json(dto)),
            Err(ServiceError::Internal(dto)) => DeleteBusinessResponse::InternalError(Json(dto)),
            Err(_) => DeleteBusinessResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(
        path = "/:business_id/address",
        method = "get",
        operation_id = "get_business_address"
    )]
    pub async fn get_address(&self, business_id: Path<Uuid>) -> GetAddressResponse {
        match self.service.get_address(business_id.0).await {
            Ok(address) => GetAddressResponse::Ok(Json(address)),
            Err(ServiceError::NotFound(dto)) => GetAddressResponse::NotFound(Json(dto)),
            Err(ServiceError::Internal(dto)) => GetAddressResponse::InternalError(Json(dto)),
            Err(_) => GetAddressResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(
        path = "/:business_id/address",
        method = "post",
        operation_id = "create_business_address"
    )]
    pub async fn insert_address(
        &self,
        business_id: Path<Uuid>,
        body: Json<CreateBusinessAddressDto>,
        auth: AuthGuard,
    ) -> InsertAddressResponse {
        let user_id = auth.0.user_id;

        match self
            .service
            .create_address(business_id.0, user_id, body.0)
            .await
        {
            Ok(address) => InsertAddressResponse::Ok(Json(address)),
            Err(ServiceError::Forbidden(dto)) => InsertAddressResponse::Forbidden(Json(dto)),
            Err(ServiceError::NotFound(dto)) => InsertAddressResponse::NotFound(Json(dto)),
            Err(ServiceError::Internal(dto)) => InsertAddressResponse::InternalError(Json(dto)),
            Err(_) => InsertAddressResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(
        path = "/:business_id/address",
        method = "patch",
        operation_id = "update_business_address"
    )]
    pub async fn update_address(
        &self,
        business_id: Path<Uuid>,
        body: Json<UpdateBusinessAddressDto>,
        auth: AuthGuard,
    ) -> UpdateAddressResponse {
        let user_id = auth.0.user_id;

        match self
            .service
            .update_address(business_id.0, user_id, body.0)
            .await
        {
            Ok(address) => UpdateAddressResponse::Ok(Json(address)),
            Err(ServiceError::Forbidden(dto)) => UpdateAddressResponse::Forbidden(Json(dto)),
            Err(ServiceError::NotFound(dto)) => UpdateAddressResponse::NotFound(Json(dto)),
            Err(ServiceError::Internal(dto)) => UpdateAddressResponse::InternalError(Json(dto)),
            Err(_) => UpdateAddressResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(
        path = "/:business_id/address",
        method = "delete",
        operation_id = "delete_business_address"
    )]
    pub async fn delete_address(
        &self,
        business_id: Path<Uuid>,
        auth: AuthGuard,
    ) -> DeleteAddressResponse {
        let user_id = auth.0.user_id;

        match self.service.delete_address(business_id.0, user_id).await {
            Ok(_) => DeleteAddressResponse::NoContent,
            Err(ServiceError::Forbidden(dto)) => DeleteAddressResponse::Forbidden(Json(dto)),
            Err(ServiceError::NotFound(dto)) => DeleteAddressResponse::NotFound(Json(dto)),
            Err(ServiceError::Internal(dto)) => DeleteAddressResponse::InternalError(Json(dto)),
            Err(_) => DeleteAddressResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(
        path = "/:business_id/hours",
        method = "get",
        operation_id = "get_business_hours"
    )]
    pub async fn get_hours(&self, business_id: Path<Uuid>) -> GetHoursResponse {
        match self.service.get_hours(business_id.0).await {
            Ok(hours) => GetHoursResponse::Ok(Json(hours)),
            Err(ServiceError::Internal(dto)) => GetHoursResponse::InternalError(Json(dto)),
            Err(_) => GetHoursResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(
        path = "/:business_id/hours",
        method = "post",
        operation_id = "create_business_hour"
    )]
    pub async fn create_hour(
        &self,
        business_id: Path<Uuid>,
        body: Json<CreateBusinessHourDto>,
        auth: AuthGuard,
    ) -> CreateHourResponse {
        let user_id = auth.0.user_id;

        match self
            .service
            .create_hour(business_id.0, user_id, body.0)
            .await
        {
            Ok(hour) => CreateHourResponse::Created(Json(hour)),
            Err(ServiceError::Conflict(dto)) => CreateHourResponse::Conflict(Json(dto)),
            Err(ServiceError::Internal(dto)) => CreateHourResponse::InternalError(Json(dto)),
            Err(_) => CreateHourResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(
        path = "/:business_id/hours/:day",
        method = "patch",
        operation_id = "update_business_hour"
    )]
    pub async fn update_hour(
        &self,
        business_id: Path<Uuid>,
        day: Path<DayOfWeekType>,
        body: Json<UpdateBusinessHourDto>,
        auth: AuthGuard,
    ) -> UpdateHourResponse {
        let user_id = auth.0.user_id;

        match self
            .service
            .update_hour(business_id.0, user_id, day.0, body.0)
            .await
        {
            Ok(hour) => UpdateHourResponse::Ok(Json(hour)),
            Err(ServiceError::NotFound(dto)) => UpdateHourResponse::NotFound(Json(dto)),
            Err(ServiceError::Internal(dto)) => UpdateHourResponse::InternalError(Json(dto)),
            Err(_) => UpdateHourResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(
        path = "/:business_id/hours/:day",
        method = "delete",
        operation_id = "delete_business_hour"
    )]
    pub async fn delete_hour(
        &self,
        business_id: Path<Uuid>,
        day: Path<DayOfWeekType>,
        auth: AuthGuard,
    ) -> DeleteHourResponse {
        let user_id = auth.0.user_id;

        match self
            .service
            .delete_hour(business_id.0, user_id, day.0)
            .await
        {
            Ok(_) => DeleteHourResponse::NoContent,
            Err(ServiceError::NotFound(dto)) => DeleteHourResponse::NotFound(Json(dto)),
            Err(ServiceError::Internal(dto)) => DeleteHourResponse::InternalError(Json(dto)),
            Err(_) => DeleteHourResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(
        path = "/:business_id/media",
        method = "get",
        operation_id = "get_business_media"
    )]
    pub async fn get_media(&self, business_id: Path<Uuid>) -> GetMediaResponse {
        match self.service.get_media(business_id.0).await {
            Ok(media) => GetMediaResponse::Ok(Json(media)),
            Err(ServiceError::Internal(dto)) => GetMediaResponse::InternalError(Json(dto)),
            Err(_) => GetMediaResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(
        path = "/:business_id/media",
        method = "post",
        operation_id = "create_business_media"
    )]
    pub async fn create_media(
        &self,
        business_id: Path<Uuid>,
        body: Json<CreateBusinessMediaDto>,
        auth: AuthGuard,
    ) -> CreateMediaResponse {
        let user_id = auth.0.user_id;

        match self
            .service
            .create_media(business_id.0, user_id, body.0)
            .await
        {
            Ok(media) => CreateMediaResponse::Created(Json(media)),
            Err(ServiceError::Internal(dto)) => CreateMediaResponse::InternalError(Json(dto)),
            Err(_) => CreateMediaResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(
        path = "/:business_id/media/:media_id",
        method = "delete",
        operation_id = "delete_business_media"
    )]
    pub async fn delete_media(
        &self,
        business_id: Path<Uuid>,
        media_id: Path<Uuid>,
        auth: AuthGuard,
    ) -> DeleteMediaResponse {
        let user_id = auth.0.user_id;

        match self
            .service
            .delete_media(business_id.0, user_id, media_id.0)
            .await
        {
            Ok(_) => DeleteMediaResponse::NoContent,
            Err(ServiceError::NotFound(dto)) => DeleteMediaResponse::NotFound(Json(dto)),
            Err(ServiceError::Internal(dto)) => DeleteMediaResponse::InternalError(Json(dto)),
            Err(_) => DeleteMediaResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }
}
