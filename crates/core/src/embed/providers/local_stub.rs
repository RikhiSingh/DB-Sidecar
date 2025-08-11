use crate::embed::EmbeddingProvider;
use async_trait::async_trait;
use sha2::{Digest, Sha256};

pub struct LocalStubEmbeddingProvider {
    pub dim: usize,
}

#[async_trait]
impl EmbeddingProvider for LocalStubEmbeddingProvider {
    async fn embed_batch(&self, inputs: &[String]) -> anyhow::Result<Vec<Vec<f32>>> {
        // Deterministic pseudo-embeddings from SHA256 -> map to dim floats [0,1)
        let mut out = Vec::with_capacity(inputs.len());
        for text in inputs {
            let mut hasher = Sha256::new();
            hasher.update(text.as_bytes());
            let bytes = hasher.finalize();
            let mut vec = vec![0.0f32; self.dim];
            for (i, b) in bytes.iter().enumerate() {
                vec[i % self.dim] += (*b as f32) / 255.0;
            }
            out.push(vec);
        }
        Ok(out)
    }
}


