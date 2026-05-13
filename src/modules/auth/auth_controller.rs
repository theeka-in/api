use super::AuthService;
use crate::errors::{DbError, ErrorDto};
use crate::modules::auth::auth_dto::{AccountDto, LoginDto, RegisterDto, SessionDto};
use poem_openapi::{ApiResponse, Object, OpenApi, param::Path, payload::Json};
use std::sync::Arc;
use uuid::Uuid;

pub struct AuthController {
    service: Arc<AuthService>,
}

#[derive(ApiResponse)]
pub enum RegisterResponse {
    #[oai(status = 201)]
    Created(Json<SessionDto>),
    #[oai(status = 409)]
    Conflict(Json<ErrorDto>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum LoginResponse {
    #[oai(status = 200)]
    Ok(Json<SessionDto>),
    #[oai(status = 401)]
    Unauthorized(Json<ErrorDto>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum LogoutResponse {
    #[oai(status = 204)]
    NoContent,
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum GetSessionsResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<SessionDto>>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum DeleteSessionResponse {
    #[oai(status = 204)]
    NoContent,
    #[oai(status = 404)]
    NotFound(Json<ErrorDto>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[OpenApi(prefix_path = "/auth")]
impl AuthController {
    pub fn new(service: Arc<AuthService>) -> Self {
        Self { service }
    }

    #[oai(path = "/register", method = "post")]
    pub async fn register(&self, body: Json<RegisterDto>) -> RegisterResponse {
        todo!()
    }

    #[oai(path = "/login", method = "post")]
    pub async fn login(&self, body: Json<LoginDto>) -> LoginResponse {
        todo!()
    }

    #[oai(path = "/logout", method = "post")]
    pub async fn logout(&self) -> LogoutResponse {
        todo!()
    }

    #[oai(path = "/sessions", method = "get")]
    pub async fn get_sessions(&self) -> GetSessionsResponse {
        todo!()
    }

    #[oai(path = "/sessions/:token", method = "delete")]
    pub async fn delete_session(&self, token: Path<String>) -> DeleteSessionResponse {
        todo!()
    }
}
