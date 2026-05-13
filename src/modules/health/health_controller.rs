use poem_openapi::{OpenApi, payload::PlainText};

#[derive(Debug)]
pub struct HealthController {}

#[OpenApi(prefix_path = "/health")]
impl HealthController {
    pub fn new() -> Self {
        Self {}
    }

    #[oai(path = "/", method = "get")]
    pub async fn check(&self) -> PlainText<String> {
        PlainText("Working 👍".to_owned())
    }
}
