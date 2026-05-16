use super::AuthService;
use crate::errors::{ErrorDto, ServiceError};
use crate::guards::BearerAuth;
use crate::modules::auth::auth_dto::{LoginDto, RegisterDto, SessionDto};
use poem::{Request, web::RemoteAddr};
use poem_openapi::{ApiResponse, OpenApi, param::Path, payload::Json};
use std::sync::Arc;

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
    #[oai(status = 403)]
    Forbidden(Json<ErrorDto>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[OpenApi(prefix_path = "/auth")]
impl AuthController {
    pub fn new(service: Arc<AuthService>) -> Self {
        Self { service }
    }

    #[oai(path = "/register", method = "post")]
    pub async fn register(
        &self,
        req: &Request,
        remote_addr: &RemoteAddr,
        body: Json<RegisterDto>,
    ) -> RegisterResponse {
        let user_agent = req
            .headers()
            .get("user-agent")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("unknown")
            .to_owned();

        match self
            .service
            .register(body.0, user_agent, remote_addr.to_string())
            .await
        {
            Ok(session) => RegisterResponse::Created(Json(session)),
            Err(ServiceError::Conflict(dto)) => RegisterResponse::Conflict(Json(dto)),
            Err(ServiceError::Internal(dto)) => RegisterResponse::InternalError(Json(dto)),
            Err(_) => RegisterResponse::InternalError(Json(ErrorDto {
                message: "unexpected error".to_owned(),
            })),
        }
    }

    #[oai(path = "/login", method = "post")]
    pub async fn login(
        &self,
        req: &Request,
        remote_addr: &RemoteAddr,
        body: Json<LoginDto>,
    ) -> LoginResponse {
        let user_agent = req
            .headers()
            .get("user-agent")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("unknown")
            .to_owned();

        match self
            .service
            .login(body.0, user_agent, remote_addr.to_string())
            .await
        {
            Ok(session) => LoginResponse::Ok(Json(session)),
            Err(ServiceError::Unauthorized(dto)) => LoginResponse::Unauthorized(Json(dto)),
            Err(ServiceError::Internal(dto)) => LoginResponse::InternalError(Json(dto)),
            Err(_) => LoginResponse::InternalError(Json(ErrorDto {
                message: "unexpected error".to_owned(),
            })),
        }
    }

    #[oai(path = "/logout", method = "post")]
    pub async fn logout(&self, auth: BearerAuth) -> LogoutResponse {
        let (_, session) = auth.0;

        match self.service.logout(session.token).await {
            Ok(_) => LogoutResponse::NoContent,
            Err(ServiceError::Internal(dto)) => LogoutResponse::InternalError(Json(dto)),
            Err(_) => LogoutResponse::NoContent,
        }
    }

    #[oai(path = "/sessions", method = "get")]
    pub async fn get_sessions(&self, auth: BearerAuth) -> GetSessionsResponse {
        let (account, _) = auth.0;

        match self.service.get_sessions(account.id).await {
            Ok(sessions) => GetSessionsResponse::Ok(Json(sessions)),
            Err(ServiceError::Internal(dto)) => GetSessionsResponse::InternalError(Json(dto)),
            Err(_) => GetSessionsResponse::InternalError(Json(ErrorDto {
                message: "unexpected error".to_owned(),
            })),
        }
    }

    #[oai(path = "/sessions/:token", method = "delete")]
    pub async fn delete_session(
        &self,
        auth: BearerAuth,
        token: Path<String>,
    ) -> DeleteSessionResponse {
        let (account, session) = auth.0;

        if session.token == token.0 {
            return DeleteSessionResponse::Forbidden(Json(ErrorDto {
                message: "you can't delete your current token".to_owned(),
            }));
        }

        match self.service.delete_session(account.id, token.0).await {
            Ok(_) => DeleteSessionResponse::NoContent,
            Err(ServiceError::NotFound(dto)) => DeleteSessionResponse::NotFound(Json(dto)),
            Err(ServiceError::Forbidden(dto)) => DeleteSessionResponse::Forbidden(Json(dto)),
            Err(ServiceError::Internal(dto)) => DeleteSessionResponse::InternalError(Json(dto)),
            Err(_) => DeleteSessionResponse::InternalError(Json(ErrorDto {
                message: "unexpected error".to_owned(),
            })),
        }
    }
}
