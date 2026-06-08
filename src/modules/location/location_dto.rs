use poem_openapi::{Enum, Object};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PhotonResponse {
    pub features: Vec<PhotonFeature>,
}

#[derive(Deserialize, Clone)]
pub struct PhotonFeature {
    pub geometry: PhotonGeometry,
    pub properties: PhotonProperties,
}

#[derive(Deserialize, Clone)]
pub struct PhotonGeometry {
    pub coordinates: Vec<f64>,
}

#[derive(Deserialize, Clone)]
pub struct PhotonProperties {
    pub osm_id: Option<u64>,
    pub osm_key: String,
    pub osm_type: Option<String>,
    pub osm_value: String,
    pub name: Option<String>,
    pub country: Option<String>,
    pub countrycode: Option<String>,
    pub state: Option<String>,
    pub county: Option<String>,
    pub city: Option<String>,
    pub postcode: Option<String>,
    pub district: Option<String>,
    pub street: Option<String>,
    pub housenumber: Option<String>,
    pub extent: Option<Vec<f64>>,
}

#[derive(Debug, Object)]
pub struct LocationDto {
    pub id: u64,
    pub osm_key: String,
    pub osm_type: OsmType,
    pub osm_value: String,
    pub name: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub state: Option<String>,
    pub county: Option<String>,
    pub country: Option<String>,
    pub country_iso_code: Option<String>,
    pub postcode: Option<String>,
    pub street: Option<String>,
    pub house_number: Option<String>,
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Enum)]
pub enum OsmType {
    Node,
    Way,
    Relation,
}

impl From<String> for OsmType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "N" | "n" => Self::Node,
            "W" | "w" => Self::Way,
            "R" | "r" => Self::Relation,
            _ => panic!("unexpected osm_type: {}", s),
        }
    }
}

impl From<PhotonFeature> for LocationDto {
    fn from(f: PhotonFeature) -> Self {
        Self {
            id: f.properties.osm_id.unwrap(),
            osm_key: f.properties.osm_key,
            osm_type: f.properties.osm_type.unwrap().into(),
            osm_value: f.properties.osm_value,
            name: f.properties.name,
            city: f.properties.city,
            district: f.properties.district,
            state: f.properties.state,
            county: f.properties.county,
            country: f.properties.country,
            country_iso_code: f.properties.countrycode,
            postcode: f.properties.postcode,
            street: f.properties.street,
            house_number: f.properties.housenumber,
            lat: f.geometry.coordinates[1],
            lon: f.geometry.coordinates[0],
        }
    }
}
