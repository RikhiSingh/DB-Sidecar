use crate::embed::EmbeddingProvider;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub struct OpenAiEmbeddingProvider {
    pub api_key: String,
    pub model: String,
    pub dim: usize,
}

#[derive(Serialize)]
struct EmbeddingsRequestBody<'a> {
    input: &'a [String],
    model: &'a str,
}

#[derive(Deserialize)]
struct EmbeddingsResponse {
    data: Vec<EmbeddingItem>,
}

#[derive(Deserialize)]
struct EmbeddingItem {
    embedding: Vec<f32>,
}

#[async_trait]
impl EmbeddingProvider for OpenAiEmbeddingProvider {
    async fn embed_batch(&self, inputs: &[String]) -> anyhow::Result<Vec<Vec<f32>>> {
        if inputs.is_empty() {
            return Ok(Vec::new());
        }
        let client = reqwest::Client::new();
        let body = EmbeddingsRequestBody {
            input: inputs,
            model: &self.model,
        };
        let res = client
            .post("https://api.openai.com/v1/embeddings")
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await?;
        if !res.status().is_success() {
            let text = res.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(format!(
                "openai embeddings error: {}",
                text
            )));
        }
        let parsed: EmbeddingsResponse = res.json().await?;
        let mut out: Vec<Vec<f32>> = Vec::with_capacity(parsed.data.len());
        for item in parsed.data {
            let mut vec = item.embedding;
            if self.dim > 0 && vec.len() != self.dim {
                // If dimension differs, pad or truncate to configured dim for consistency
                if vec.len() < self.dim {
                    vec.resize(self.dim, 0.0);
                } else {
                    vec.truncate(self.dim);
                }
            }
            out.push(vec);
        }
        Ok(out)
    }
}
