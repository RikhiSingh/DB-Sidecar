#[cfg(feature = "qdrant")]
use super::VectorIndex;
#[cfg(feature = "qdrant")]
use async_trait::async_trait;
#[cfg(feature = "qdrant")]
use qdrant_client::prelude::*;

#[cfg(feature = "qdrant")]
pub struct QdrantIndex {
    client: QdrantClient,
    collection: String,
}

#[cfg(feature = "qdrant")]
impl QdrantIndex {
    pub fn new(url: &str, collection: &str) -> anyhow::Result<Self> {
        Ok(Self { client: QdrantClient::from_url(url).build()?, collection: collection.to_string() })
    }
}

#[cfg(feature = "qdrant")]
#[async_trait]
impl VectorIndex for QdrantIndex {
    async fn upsert(&self, _id: String, _vector: Vec<f32>) -> anyhow::Result<()> {
        // TODO: implement
        Ok(())
    }
    async fn search(&self, _vector: Vec<f32>, _top_k: usize) -> anyhow::Result<Vec<(String, f32)>> {
        // TODO: implement
        Ok(vec![])
    }
}


