use super::BusinessService;
use crate::errors::{DbError, ErrorDto};
use crate::guards::BearerAuth;
use crate::modules::business::business_dto::{
    BusinessDto, BusinessHourDto, BusinessMediaDto, CreateBusinessDto, CreateBusinessHourDto,
    CreateBusinessMediaDto, UpdateBusinessDto, UpdateBusinessHourDto,
};
use poem_openapi::{
    ApiResponse, Object, OpenApi,
    param::{Path, Query},
    payload::Json,
};
use std::sync::Arc;
use uuid::Uuid;

pub struct BusinessController {
    service: Arc<BusinessService>,
}

#[derive(ApiResponse)]
pub enum GetBusinessesResponse {
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

    #[oai(path = "/", method = "get")]
    pub async fn get_nearby(&self, lat: Query<f64>, lng: Query<f64>) -> GetBusinessesResponse {
        todo!()
    }

    #[oai(path = "/:id", method = "get")]
    pub async fn get_by_id(&self, id: Path<Uuid>) -> GetBusinessResponse {
        todo!()
    }

    #[oai(path = "/", method = "post")]
    pub async fn create(
        &self,
        body: Json<CreateBusinessDto>,
        auth: BearerAuth,
    ) -> CreateBusinessResponse {
        todo!()
    }

    #[oai(path = "/:id", method = "patch")]
    pub async fn update(
        &self,
        id: Path<Uuid>,
        body: Json<UpdateBusinessDto>,
        auth: BearerAuth,
    ) -> UpdateBusinessResponse {
        todo!()
    }

    #[oai(path = "/:id", method = "delete")]
    pub async fn delete(&self, id: Path<Uuid>, auth: BearerAuth) -> DeleteBusinessResponse {
        todo!()
    }

    #[oai(path = "/:id/hours", method = "get")]
    pub async fn get_hours(&self, id: Path<Uuid>) -> GetHoursResponse {
        todo!()
    }

    #[oai(path = "/:id/hours", method = "post")]
    pub async fn create_hour(
        &self,
        id: Path<Uuid>,
        body: Json<CreateBusinessHourDto>,
        auth: BearerAuth,
    ) -> CreateHourResponse {
        todo!()
    }

    #[oai(path = "/:id/hours/:day", method = "patch")]
    pub async fn update_hour(
        &self,
        id: Path<Uuid>,
        day: Path<String>,
        body: Json<UpdateBusinessHourDto>,
        auth: BearerAuth,
    ) -> UpdateHourResponse {
        todo!()
    }

    #[oai(path = "/:id/hours/:day", method = "delete")]
    pub async fn delete_hour(
        &self,
        id: Path<Uuid>,
        day: Path<String>,
        auth: BearerAuth,
    ) -> DeleteHourResponse {
        todo!()
    }

    #[oai(path = "/:id/media", method = "get")]
    pub async fn get_media(&self, id: Path<Uuid>) -> GetMediaResponse {
        todo!()
    }

    #[oai(path = "/:id/media", method = "post")]
    pub async fn create_media(
        &self,
        id: Path<Uuid>,
        body: Json<CreateBusinessMediaDto>,
        auth: BearerAuth,
    ) -> CreateMediaResponse {
        todo!()
    }

    #[oai(path = "/:id/media/:media_id", method = "delete")]
    pub async fn delete_media(
        &self,
        id: Path<Uuid>,
        media_id: Path<Uuid>,
        auth: BearerAuth,
    ) -> DeleteMediaResponse {
        todo!()
    }
}
