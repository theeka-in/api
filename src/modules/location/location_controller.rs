use std::sync::Arc;

use super::LocationService;
use crate::modules::location::LocationDto;
use crate::shared::errors::ErrorDto;
use poem_openapi::ApiResponse;
use poem_openapi::OpenApi;
use poem_openapi::param::Query;
use poem_openapi::payload::Json;

pub struct LocationController {
    service: Arc<LocationService>,
}

#[derive(ApiResponse)]
pub enum SearchResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<LocationDto>>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[derive(ApiResponse)]
pub enum ReverseResponse {
    #[oai(status = 200)]
    Ok(Json<LocationDto>),
    #[oai(status = 500)]
    InternalError(Json<ErrorDto>),
}

#[OpenApi(prefix_path = "/location")]
impl LocationController {
    pub fn new(service: Arc<LocationService>) -> Self {
        Self { service }
    }

    #[oai(path = "/search", method = "get", operation_id = "location_search")]
    pub async fn search(&self, query: Query<String>) -> SearchResponse {
        match self.service.search(query.0).await {
            Ok(results) => SearchResponse::Ok(Json(results)),
            Err(_) => SearchResponse::InternalError(Json(ErrorDto {
                message: "internal server error".to_owned(),
            })),
        }
    }

    #[oai(path = "/reverse", method = "get", operation_id = "location_reverse")]
    pub async fn reverse(&self, lat: Query<f64>, lon: Query<f64>) -> ReverseResponse {
        match self.service.reverse(lat.0, lon.0).await {
            Ok(result) => ReverseResponse::Ok(Json(result)),
            Err(err) => {
                dbg!(err);
                ReverseResponse::InternalError(Json(ErrorDto {
                    message: "internal server error".to_owned(),
                }))
            }
        }
    }
}
