use super::AuthService;
use crate::errors::DbError;
use crate::modules::auth::auth_dto::{CreateUserDto, UserDto};
use poem_openapi::{
    param::Query, payload::{Json, PlainText}, ApiResponse,
    Object,
    OpenApi,
};
use std::sync::Arc;

pub struct AuthController {
    service: Arc<AuthService>,
}

#[derive(Debug, Object)]
pub struct ErrorDto {
    pub message: String,
}

#[derive(ApiResponse)]
pub enum CreateUserResponse {
    #[oai(status = 201)]
    Created(Json<UserDto>),

    #[oai(status = 409)]
    Conflict(Json<ErrorDto>),

    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[OpenApi(prefix_path = "/auth")]
impl AuthController {
    pub fn new(service: Arc<AuthService>) -> Self {
        Self { service }
    }

    #[oai(path = "/hello", method = "get")]
    pub async fn hello_from_auth(
        &self,
        #[oai(validator(min_length = 2, max_length = 50))] name: Query<String>,
    ) -> PlainText<String> {
        PlainText(self.service.hello_from_auth(&name.0))
    }

    #[oai(path = "/db-health", method = "get")]
    pub async fn db_health(&self) -> Json<Vec<UserDto>> {
        let users = self.service.db_health().await;

        Json(users)
    }

    #[oai(path = "/create-user", method = "post")]
    pub async fn create_user(&self, body: Json<CreateUserDto>) -> CreateUserResponse {
        match self.service.create_user(body.0).await {
            Ok(user) => CreateUserResponse::Created(Json(user)),

            Err(DbError::UniqueViolation { constraint }) => {
                CreateUserResponse::Conflict(Json(ErrorDto {
                    message: format!("{constraint} already exists"),
                }))
            }

            Err(e) => CreateUserResponse::InternalError(Json(ErrorDto {
                message: e.to_string(),
            })),
        }
    }
}
