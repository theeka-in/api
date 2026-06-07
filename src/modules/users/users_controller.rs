use super::UsersService;
use crate::modules::users::users_dto::{
    CreateUserAddressDto, UpdateUserAddressDto, UpdateUserDto, UserAddressDto, UserDto,
};
use crate::shared::errors::{ErrorDto, ServiceError};
use crate::shared::guards::AuthGuard;
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

    #[oai(path = "/me", method = "get", operation_id = "get_myself")]
    pub async fn get_myself(&self, auth: AuthGuard) -> GetMeResponse {
        let account_id = auth.0.account_id;

        match self.service.get(account_id).await {
            Ok(user) => GetMeResponse::Ok(Json(user)),
            Err(ServiceError::NotFound(e)) => GetMeResponse::NotFound(Json(e)),
            Err(_) => GetMeResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(path = "/me", method = "patch", operation_id = "update_myself")]
    pub async fn update_myself(
        &self,
        auth: AuthGuard,
        body: Json<UpdateUserDto>,
    ) -> UpdateMeResponse {
        let account_id = auth.0.account_id;

        match self.service.update(account_id, body.0).await {
            Ok(user) => UpdateMeResponse::Ok(Json(user)),
            Err(ServiceError::NotFound(e)) => UpdateMeResponse::NotFound(Json(e)),
            Err(_) => UpdateMeResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(path = "/me", method = "delete", operation_id = "delete_myself")]
    pub async fn delete_myself(&self, auth: AuthGuard) -> DeleteMeResponse {
        let account_id = auth.0.account_id;

        match self.service.delete(account_id).await {
            Ok(_) => DeleteMeResponse::NoContent,
            Err(_) => DeleteMeResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(
        path = "/me/addresses",
        method = "get",
        operation_id = "get_my_addresses"
    )]
    pub async fn get_my_addresses(&self, auth: AuthGuard) -> GetAddressesResponse {
        let account_id = auth.0.account_id;

        match self.service.get_addresses(account_id).await {
            Ok(addresses) => GetAddressesResponse::Ok(Json(addresses)),
            Err(_) => GetAddressesResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(
        path = "/me/addresses",
        method = "post",
        operation_id = "create_my_address"
    )]
    pub async fn create_my_address(
        &self,
        auth: AuthGuard,
        body: Json<CreateUserAddressDto>,
    ) -> CreateAddressResponse {
        let account_id = auth.0.account_id;

        match self.service.create_address(account_id, body.0).await {
            Ok(address) => CreateAddressResponse::Created(Json(address)),
            Err(_) => CreateAddressResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(
        path = "/me/addresses/:address_id",
        method = "patch",
        operation_id = "update_my_address"
    )]
    pub async fn update_my_address(
        &self,
        auth: AuthGuard,
        address_id: Path<Uuid>,
        body: Json<UpdateUserAddressDto>,
    ) -> UpdateAddressResponse {
        let account_id = auth.0.account_id;

        match self
            .service
            .update_address(account_id, address_id.0, body.0)
            .await
        {
            Ok(address) => UpdateAddressResponse::Ok(Json(address)),
            Err(ServiceError::NotFound(e)) => UpdateAddressResponse::NotFound(Json(e)),
            Err(_) => UpdateAddressResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(
        path = "/me/addresses/:address_id",
        method = "delete",
        operation_id = "delete_my_address"
    )]
    pub async fn delete_my_address(
        &self,
        auth: AuthGuard,
        address_id: Path<Uuid>,
    ) -> DeleteAddressResponse {
        let account_id = auth.0.account_id;

        match self.service.delete_address(account_id, address_id.0).await {
            Ok(_) => DeleteAddressResponse::NoContent,
            Err(ServiceError::NotFound(e)) => DeleteAddressResponse::NotFound(Json(e)),
            Err(_) => DeleteAddressResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }
}
