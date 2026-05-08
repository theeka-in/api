use poem_openapi::{
    Object, OpenApi,
    param::Query,
    payload::{Json, PlainText},
};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    Database,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::{entities::user, modules::database::DatabaseService};

use super::AuthService;

pub struct AuthController {
    service: Arc<AuthService>,
}

#[derive(Clone, Debug, Object)]
pub struct UserDto {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub username: Option<String>,
}

impl From<user::Model> for UserDto {
    fn from(m: user::Model) -> Self {
        Self {
            id: m.id.to_string(),
            name: m.name,
            email: m.email,
            username: m.username,
        }
    }
}

#[derive(Object)]
pub struct CreateUserRequest {
    #[oai(validator(min_length = 3, max_length = 60))]
    name: Option<String>,

    #[oai(validator(pattern = r"^[^@\s]+@[^@\s]+\.[^@\s]+$"))]
    email: String,

    #[oai(validator(min_length = 3, max_length = 60))]
    username: String,
}

impl From<CreateUserRequest> for user::ActiveModel {
    fn from(req: CreateUserRequest) -> Self {
        Self {
            id: Set(Uuid::new_v4()),
            name: Set(req.name),
            email: Set(req.email),
            username: Set(Some(req.username)),
            ..Default::default()
        }
    }
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

        Json(users.into_iter().map(|m| UserDto::from(m)).collect())
    }

    #[oai(path = "/create-user", method = "post")]
    pub async fn create_user(&self, body: Json<CreateUserRequest>) -> Json<UserDto> {
        let user = self.service.create_user(body.0).await;
        Json(user)
    }
}
