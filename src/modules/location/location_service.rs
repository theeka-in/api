use crate::modules::location::{LocationDto, PhotonFeature, PhotonResponse};
use crate::shared::errors::{ErrorDto, ServiceError};
use reqwest::Client;
use serde::Deserialize;
use std::sync::Arc;

const PHOTON_BASE_URL: &str = "https://photon.komoot.io";
const INDIA_BBOX: &str = "68.1,6.7,97.4,35.7";

pub struct LocationService {
    client: Client,
}

impl LocationService {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            client: Client::new(),
        })
    }

    pub async fn search(&self, query: String) -> Result<Vec<LocationDto>, ServiceError> {
        let response = self
            .client
            .get(format!("{}/api", PHOTON_BASE_URL))
            .query(&[
                ("q", query.as_str()),
                ("limit", "30"),
                ("bbox", INDIA_BBOX),
                ("lang", "en"),
            ])
            .send()
            .await
            .map_err(|e| {
                ServiceError::Internal(ErrorDto {
                    message: e.to_string(),
                })
            })?
            .json::<PhotonResponse>()
            .await
            .map_err(|e| {
                ServiceError::Internal(ErrorDto {
                    message: e.to_string(),
                })
            })?;

        Ok(response
            .features
            .into_iter()
            .filter(|p| match p.properties.countrycode.as_deref() {
                Some(code) => match code {
                    "IN" => true,
                    _ => false,
                },
                None => false,
            })
            .filter(|p| match p.properties.osm_id {
                Some(_) => true,
                None => false,
            })
            .filter(|p| match p.properties.osm_type {
                Some(_) => true,
                None => false,
            })
            .map(LocationDto::from)
            .filter(|p| match &p.osm_type {
                super::OsmType::Node => true,
                super::OsmType::Relation => false,
                super::OsmType::Way => false,
            })
            .collect())
    }

    pub async fn reverse(&self, lat: f64, lon: f64) -> Result<LocationDto, ServiceError> {
        let response = self
            .client
            .get(format!("{}/reverse", PHOTON_BASE_URL))
            .query(&[
                ("lat", lat.to_string()),
                ("lon", lon.to_string()),
                ("limit", "5".to_string()),
                ("lang", "en".to_string()),
            ])
            .send()
            .await
            .map_err(|e| {
                ServiceError::Internal(ErrorDto {
                    message: e.to_string(),
                })
            })?
            .json::<PhotonResponse>()
            .await
            .map_err(|e| {
                ServiceError::Internal(ErrorDto {
                    message: e.to_string(),
                })
            })?;

        let results: Vec<PhotonFeature> = response
            .features
            .into_iter()
            .filter(|p| match p.properties.osm_id {
                Some(_) => true,
                None => false,
            })
            .filter(|p| match p.properties.osm_type {
                Some(_) => true,
                None => false,
            })
            .collect();

        Ok(results[0].clone().into())
    }
}
