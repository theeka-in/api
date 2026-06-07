use poem_openapi::{Object, OpenApi, payload::Json};

#[derive(Debug, Object)]
pub struct HealthCheckResponse {
    pub status: String,
}

#[derive(Debug)]
pub struct HealthController {}

#[OpenApi(prefix_path = "/health")]
impl HealthController {
    pub fn new() -> Self {
        Self {}
    }

    #[oai(path = "/", method = "get", operation_id = "check")]
    pub async fn check(&self) -> Json<HealthCheckResponse> {
        Json(HealthCheckResponse {
            status: "Working".to_owned(),
        })
    }
}
