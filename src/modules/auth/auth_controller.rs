use std::sync::Arc;

use poem_openapi::{OpenApi, param::Query, payload::PlainText};

use super::AuthService;

pub struct AuthController {
    service: Arc<AuthService>,
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
    pub async fn db_health(&self) -> PlainText<String> {
        PlainText(self.service.db_health().await)
    }
}
