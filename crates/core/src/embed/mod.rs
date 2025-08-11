use async_trait::async_trait;

pub mod providers {
    pub mod local_stub;
}

#[async_trait]
pub trait EmbeddingProvider: Send + Sync {
    async fn embed_batch(&self, inputs: &[String]) -> anyhow::Result<Vec<Vec<f32>>>;
}

pub fn build_input_text(object_name: &str, canonical_ddl: &str) -> String {
    format!("OBJECT {}\nDDL:\n{}", object_name, canonical_ddl)
}


