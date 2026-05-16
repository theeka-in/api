use super::UsersService;
use crate::errors::{ErrorDto, ServiceError};
use crate::guards::BearerAuth;
use crate::modules::users::users_dto::{
    CreateUserAddressDto, UpdateUserAddressDto, UpdateUserDto, UserAddressDto, UserDto,
};
use poem_openapi::{ApiResponse, OpenApi, param::Path, payload::Json};
use std::sync::Arc;
use uuid::Uuid;

pub struct UsersController {
    service: Arc<UsersService>,
}

#[derive(ApiResponse)]
pub enum GetMeResponse {
    #[oai(status = 200)]
    Ok(Json<UserDto>),
    #[oai(status = 404)]
    NotFound(Json<ErrorDto>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum UpdateMeResponse {
    #[oai(status = 200)]
    Ok(Json<UserDto>),
    #[oai(status = 404)]
    NotFound(Json<ErrorDto>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum DeleteMeResponse {
    #[oai(status = 204)]
    NoContent,
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum GetAddressesResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<UserAddressDto>>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum CreateAddressResponse {
    #[oai(status = 201)]
    Created(Json<UserAddressDto>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum UpdateAddressResponse {
    #[oai(status = 200)]
    Ok(Json<UserAddressDto>),
    #[oai(status = 404)]
    NotFound(Json<ErrorDto>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum DeleteAddressResponse {
    #[oai(status = 204)]
    NoContent,
    #[oai(status = 404)]
    NotFound(Json<ErrorDto>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[OpenApi(prefix_path = "/users")]
impl UsersController {
    pub fn new(service: Arc<UsersService>) -> Self {
        Self { service }
    }

    #[oai(path = "/me", method = "get")]
    pub async fn get(&self, auth: BearerAuth) -> GetMeResponse {
        let account_id = auth.0.0.id;

        match self.service.get(account_id).await {
            Ok(user) => GetMeResponse::Ok(Json(user)),
            Err(ServiceError::NotFound(e)) => GetMeResponse::NotFound(Json(e)),
            Err(_) => GetMeResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(path = "/me", method = "patch")]
    pub async fn update(&self, auth: BearerAuth, body: Json<UpdateUserDto>) -> UpdateMeResponse {
        let account_id = auth.0.0.id;

        match self.service.update(account_id, body.0).await {
            Ok(user) => UpdateMeResponse::Ok(Json(user)),
            Err(ServiceError::NotFound(e)) => UpdateMeResponse::NotFound(Json(e)),
            Err(_) => UpdateMeResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(path = "/me", method = "delete")]
    pub async fn delete(&self, auth: BearerAuth) -> DeleteMeResponse {
        let account_id = auth.0.0.id;

        match self.service.delete(account_id).await {
            Ok(_) => DeleteMeResponse::NoContent,
            Err(_) => DeleteMeResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(path = "/me/addresses", method = "get")]
    pub async fn get_addresses(&self, auth: BearerAuth) -> GetAddressesResponse {
        let account_id = auth.0.0.id;

        match self.service.get_addresses(account_id).await {
            Ok(addresses) => GetAddressesResponse::Ok(Json(addresses)),
            Err(_) => GetAddressesResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(path = "/me/addresses", method = "post")]
    pub async fn create_address(
        &self,
        auth: BearerAuth,
        body: Json<CreateUserAddressDto>,
    ) -> CreateAddressResponse {
        let account_id = auth.0.0.id;

        match self.service.create_address(account_id, body.0).await {
            Ok(address) => CreateAddressResponse::Created(Json(address)),
            Err(_) => CreateAddressResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(path = "/me/addresses/:id", method = "patch")]
    pub async fn update_address(
        &self,
        auth: BearerAuth,
        id: Path<Uuid>,
        body: Json<UpdateUserAddressDto>,
    ) -> UpdateAddressResponse {
        let account_id = auth.0.0.id;

        match self.service.update_address(account_id, id.0, body.0).await {
            Ok(address) => UpdateAddressResponse::Ok(Json(address)),
            Err(ServiceError::NotFound(e)) => UpdateAddressResponse::NotFound(Json(e)),
            Err(_) => UpdateAddressResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(path = "/me/addresses/:id", method = "delete")]
    pub async fn delete_address(&self, auth: BearerAuth, id: Path<Uuid>) -> DeleteAddressResponse {
        let account_id = auth.0.0.id;

        match self.service.delete_address(account_id, id.0).await {
            Ok(_) => DeleteAddressResponse::NoContent,
            Err(ServiceError::NotFound(e)) => DeleteAddressResponse::NotFound(Json(e)),
            Err(_) => DeleteAddressResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }
}
