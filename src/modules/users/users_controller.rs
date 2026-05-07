use std::sync::Arc;

use poem_openapi::{OpenApi, param::Query, payload::PlainText};

use super::UsersService;

pub struct UsersController {
    service: Arc<UsersService>,
}

#[OpenApi(prefix_path = "/users")]
impl UsersController {
    pub fn new(service: Arc<UsersService>) -> Self {
        Self { service }
    }

    #[oai(path = "/hello", method = "get")]
    pub async fn hello(
        &self,
        #[oai(validator(min_length = 2, max_length = 50))] name: Query<String>,
    ) -> PlainText<String> {
        PlainText(self.service.hello(&name.0))
    }
}
