use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct EmbedRequest {
    pub model: String,
    pub input: String,
}

#[derive(Deserialize)]
pub struct EmbedResponse {
    pub embeddings: Vec<Vec<f32>>,
}
