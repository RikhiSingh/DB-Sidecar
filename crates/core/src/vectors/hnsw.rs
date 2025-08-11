use super::VectorIndex;
use async_trait::async_trait;
// Disable hnsw_rs for now: comment out to avoid unresolved crate on some toolchains
// use hnsw_rs::prelude::*;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct HnswIndex {
    storage: Arc<RwLock<HashMap<String, Vec<f32>>>>,
}

impl HnswIndex {
    pub fn new(_dim: usize) -> Self { Self { storage: Arc::new(RwLock::new(HashMap::new())) } }
}

#[async_trait]
impl VectorIndex for HnswIndex {
    async fn upsert(&self, id: String, vector: Vec<f32>) -> anyhow::Result<()> {
        self.storage.write().insert(id, vector);
        Ok(())
    }

    async fn search(&self, vector: Vec<f32>, top_k: usize) -> anyhow::Result<Vec<(String, f32)>> {
        let store = self.storage.read();
        let mut scored: Vec<(String, f32)> = store.iter().map(|(id, v)| (id.clone(), cosine_sim(v, &vector))).collect();
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scored.truncate(top_k);
        Ok(scored)
    }
}

fn cosine_sim(a: &[f32], b: &[f32]) -> f32 {
    let mut dot = 0.0f32;
    let mut na = 0.0f32;
    let mut nb = 0.0f32;
    let len = a.len().min(b.len());
    for i in 0..len {
        dot += a[i] * b[i];
        na += a[i] * a[i];
        nb += b[i] * b[i];
    }
    if na == 0.0 || nb == 0.0 { return 0.0; }
    dot / (na.sqrt() * nb.sqrt())
}


