use super::UsersService;
use crate::errors::{DbError, ErrorDto};
use crate::modules::users::users_dto::{
    CreateUserAddressDto, UpdateUserAddressDto, UpdateUserDto, UserAddressDto, UserDto,
};
use poem_openapi::{ApiResponse, Object, OpenApi, param::Path, payload::Json};
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
    pub async fn get_me(&self) -> GetMeResponse {
        todo!()
    }

    #[oai(path = "/me", method = "patch")]
    pub async fn update_me(&self, body: Json<UpdateUserDto>) -> UpdateMeResponse {
        todo!()
    }

    #[oai(path = "/me", method = "delete")]
    pub async fn delete_me(&self) -> DeleteMeResponse {
        todo!()
    }

    #[oai(path = "/me/addresses", method = "get")]
    pub async fn get_addresses(&self) -> GetAddressesResponse {
        todo!()
    }

    #[oai(path = "/me/addresses", method = "post")]
    pub async fn create_address(&self, body: Json<CreateUserAddressDto>) -> CreateAddressResponse {
        todo!()
    }

    #[oai(path = "/me/addresses/:id", method = "patch")]
    pub async fn update_address(
        &self,
        id: Path<Uuid>,
        body: Json<UpdateUserAddressDto>,
    ) -> UpdateAddressResponse {
        todo!()
    }

    #[oai(path = "/me/addresses/:id", method = "delete")]
    pub async fn delete_address(&self, id: Path<Uuid>) -> DeleteAddressResponse {
        todo!()
    }
}
