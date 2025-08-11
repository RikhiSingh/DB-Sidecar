use async_trait::async_trait;

pub mod hnsw;
#[cfg(feature = "qdrant")]
pub mod qdrant;

#[async_trait]
pub trait VectorIndex: Send + Sync {
    async fn upsert(&self, id: String, vector: Vec<f32>) -> anyhow::Result<()>;
    async fn search(&self, vector: Vec<f32>, top_k: usize) -> anyhow::Result<Vec<(String, f32)>>;
}


