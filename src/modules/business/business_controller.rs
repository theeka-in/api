use super::BusinessService;
use crate::errors::{ErrorDto, ServiceError};
use crate::guards::BearerAuth;
use crate::modules::business::business_dto::{
    BusinessDto, BusinessHourDto, BusinessMediaDto, CreateBusinessDto, CreateBusinessHourDto,
    CreateBusinessMediaDto, UpdateBusinessDto, UpdateBusinessHourDto,
};
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

    #[oai(path = "/nearby", method = "get")]
    pub async fn get_nearby(
        &self,
        lat: Query<f64>,
        lng: Query<f64>,
    ) -> GetNearbyBusinessesResponse {
        match self.service.get_nearby(lat.0, lng.0).await {
            Ok(businesses) => GetNearbyBusinessesResponse::Ok(Json(businesses)),
            Err(ServiceError::Internal(dto)) => {
                GetNearbyBusinessesResponse::InternalError(Json(dto))
            }
            Err(_) => GetNearbyBusinessesResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(path = "/", method = "get")]
    pub async fn get_my_businesses(&self, auth: BearerAuth) -> GetMyBusinessesResponse {
        let (account, _) = auth.0;

        match self.service.get_all_by_owner(account.id).await {
            Ok(businesses) => GetMyBusinessesResponse::Ok(Json(businesses)),
            Err(ServiceError::Internal(dto)) => GetMyBusinessesResponse::InternalError(Json(dto)),
            Err(_) => GetMyBusinessesResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(path = "/:id", method = "get")]
    pub async fn get_by_id(&self, id: Path<Uuid>) -> GetBusinessResponse {
        match self.service.get_by_id(id.0).await {
            Ok(business) => GetBusinessResponse::Ok(Json(business)),
            Err(ServiceError::NotFound(dto)) => GetBusinessResponse::NotFound(Json(dto)),
            Err(ServiceError::Internal(dto)) => GetBusinessResponse::InternalError(Json(dto)),
            Err(_) => GetBusinessResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(path = "/", method = "post")]
    pub async fn create(
        &self,
        body: Json<CreateBusinessDto>,
        auth: BearerAuth,
    ) -> CreateBusinessResponse {
        let (account, _) = auth.0;

        match self.service.create(account.id, account.phone, body.0).await {
            Ok(business) => CreateBusinessResponse::Created(Json(business)),
            Err(ServiceError::Internal(dto)) => CreateBusinessResponse::InternalError(Json(dto)),
            Err(_) => CreateBusinessResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(path = "/:id", method = "patch")]
    pub async fn update(
        &self,
        id: Path<Uuid>,
        body: Json<UpdateBusinessDto>,
        auth: BearerAuth,
    ) -> UpdateBusinessResponse {
        let (account, _) = auth.0;

        match self.service.update(id.0, account.id, body.0).await {
            Ok(business) => UpdateBusinessResponse::Ok(Json(business)),
            Err(ServiceError::Forbidden(dto)) => UpdateBusinessResponse::Forbidden(Json(dto)),
            Err(ServiceError::NotFound(dto)) => UpdateBusinessResponse::NotFound(Json(dto)),
            Err(ServiceError::Internal(dto)) => UpdateBusinessResponse::InternalError(Json(dto)),
            Err(_) => UpdateBusinessResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(path = "/:id", method = "delete")]
    pub async fn delete(&self, id: Path<Uuid>, auth: BearerAuth) -> DeleteBusinessResponse {
        let (account, _) = auth.0;

        match self.service.delete(id.0, account.id).await {
            Ok(_) => DeleteBusinessResponse::NoContent,
            Err(ServiceError::Forbidden(dto)) => DeleteBusinessResponse::Forbidden(Json(dto)),
            Err(ServiceError::NotFound(dto)) => DeleteBusinessResponse::NotFound(Json(dto)),
            Err(ServiceError::Internal(dto)) => DeleteBusinessResponse::InternalError(Json(dto)),
            Err(_) => DeleteBusinessResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(path = "/:id/hours", method = "get")]
    pub async fn get_hours(&self, id: Path<Uuid>) -> GetHoursResponse {
        match self.service.get_hours(id.0).await {
            Ok(hours) => GetHoursResponse::Ok(Json(hours)),
            Err(ServiceError::Internal(dto)) => GetHoursResponse::InternalError(Json(dto)),
            Err(_) => GetHoursResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(path = "/:id/hours", method = "post")]
    pub async fn create_hour(
        &self,
        id: Path<Uuid>,
        body: Json<CreateBusinessHourDto>,
        auth: BearerAuth,
    ) -> CreateHourResponse {
        let (account, _) = auth.0;

        match self.service.create_hour(id.0, account.id, body.0).await {
            Ok(hour) => CreateHourResponse::Created(Json(hour)),
            Err(ServiceError::Conflict(dto)) => CreateHourResponse::Conflict(Json(dto)),
            Err(ServiceError::Internal(dto)) => CreateHourResponse::InternalError(Json(dto)),
            Err(_) => CreateHourResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(path = "/:id/hours/:day", method = "patch")]
    pub async fn update_hour(
        &self,
        id: Path<Uuid>,
        day: Path<String>,
        body: Json<UpdateBusinessHourDto>,
        auth: BearerAuth,
    ) -> UpdateHourResponse {
        let (account, _) = auth.0;

        match self
            .service
            .update_hour(id.0, account.id, day.0, body.0)
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

    #[oai(path = "/:id/hours/:day", method = "delete")]
    pub async fn delete_hour(
        &self,
        id: Path<Uuid>,
        day: Path<String>,
        auth: BearerAuth,
    ) -> DeleteHourResponse {
        let (account, _) = auth.0;

        match self.service.delete_hour(id.0, account.id, day.0).await {
            Ok(_) => DeleteHourResponse::NoContent,
            Err(ServiceError::NotFound(dto)) => DeleteHourResponse::NotFound(Json(dto)),
            Err(ServiceError::Internal(dto)) => DeleteHourResponse::InternalError(Json(dto)),
            Err(_) => DeleteHourResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(path = "/:id/media", method = "get")]
    pub async fn get_media(&self, id: Path<Uuid>) -> GetMediaResponse {
        match self.service.get_media(id.0).await {
            Ok(media) => GetMediaResponse::Ok(Json(media)),
            Err(ServiceError::Internal(dto)) => GetMediaResponse::InternalError(Json(dto)),
            Err(_) => GetMediaResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(path = "/:id/media", method = "post")]
    pub async fn create_media(
        &self,
        id: Path<Uuid>,
        body: Json<CreateBusinessMediaDto>,
        auth: BearerAuth,
    ) -> CreateMediaResponse {
        let (account, _) = auth.0;

        match self.service.create_media(id.0, account.id, body.0).await {
            Ok(media) => CreateMediaResponse::Created(Json(media)),
            Err(ServiceError::Internal(dto)) => CreateMediaResponse::InternalError(Json(dto)),
            Err(_) => CreateMediaResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(path = "/:id/media/:media_id", method = "delete")]
    pub async fn delete_media(
        &self,
        id: Path<Uuid>,
        media_id: Path<Uuid>,
        auth: BearerAuth,
    ) -> DeleteMediaResponse {
        let (account, _) = auth.0;

        match self
            .service
            .delete_media(id.0, account.id, media_id.0)
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
