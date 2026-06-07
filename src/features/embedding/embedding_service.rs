use std::collections::HashMap;
use std::sync::{Arc, LazyLock};
use std::vec;

use reqwest::Client;
use tokio::sync::Mutex;

use crate::features::embedding::EmbedRequest;
use crate::features::embedding::EmbedResponse;
use crate::shared::errors::{ErrorDto, ServiceError};

#[derive(Debug)]
pub struct EmbeddingService {
    client: Client,
    base_url: String,
    cache: HashMap<String, pgvector::Vector>,
}

impl EmbeddingService {
    pub fn new(base_url: String) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            client: Client::new(),
            base_url,
            cache: HashMap::new(),
        }))
    }

    pub async fn embed(&mut self, text: String) -> Result<pgvector::Vector, ServiceError> {
        let cache = self.cache.get(&text);

        if let Some(cache) = cache {
            return Ok(cache.clone());
        }

        let res = self
            .client
            .post(format!("{}/api/embed", self.base_url))
            .json(&EmbedRequest {
                model: "leoipulsar/harrier-0.6b".to_owned(),
                input: text.clone(),
            })
            .send()
            .await
            .map_err(|e| {
                ServiceError::Internal(ErrorDto {
                    message: e.to_string(),
                })
            })?
            .json::<EmbedResponse>()
            .await
            .map_err(|e| {
                ServiceError::Internal(ErrorDto {
                    message: e.to_string(),
                })
            })?;

        let vector = pgvector::Vector::from(res.embeddings[0].clone());

        self.cache.insert(text, vector.clone());

        Ok(vector)
    }
}
