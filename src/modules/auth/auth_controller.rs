use poem_openapi::{OpenApi, param::Query, payload::PlainText};

use super::AuthService;

pub struct AuthController {
    service: AuthService,
}

#[OpenApi(prefix_path = "/auth")]
impl AuthController {
    pub fn new(service: AuthService) -> Self {
        Self { service }
    }

    #[oai(path = "/hello", method = "get")]
    pub async fn hello_from_auth(
        &self,
        #[oai(validator(min_length = 2, max_length = 50))] name: Query<String>,
    ) -> PlainText<String> {
        PlainText(self.service.hello_from_auth(&name.0))
    }
}
