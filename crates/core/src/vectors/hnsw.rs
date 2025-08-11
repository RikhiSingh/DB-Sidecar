use super::VectorIndex;
use async_trait::async_trait;
use hnsw_rs::prelude::*;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct HnswIndex {
    inner: Arc<RwLock<Hnsw<f32, DistL2>>>,
    id_map: Arc<RwLock<HashMap<usize, String>>>,
    next_id: Arc<RwLock<usize>>,
}

impl HnswIndex {
    pub fn new(dim: usize) -> Self {
        let hnsw: Hnsw<f32, DistL2> = HnswBuilder::default()
            .num_neighbours(16)
            .max_nb_connection(32)
            .max_item(100_000)
            .ef(200)
            .build(dim, DistL2 {});
        Self {
            inner: Arc::new(RwLock::new(hnsw)),
            id_map: Arc::new(RwLock::new(HashMap::new())),
            next_id: Arc::new(RwLock::new(0)),
        }
    }
}

#[async_trait]
impl VectorIndex for HnswIndex {
    async fn upsert(&self, id: String, vector: Vec<f32>) -> anyhow::Result<()> {
        let point_id = {
            let mut idx = self.next_id.write();
            let pid = *idx;
            *idx += 1;
            self.id_map.write().insert(pid, id);
            pid
        };
        self.inner.write().insert((&vector).into(), point_id)?;
        Ok(())
    }

    async fn search(&self, vector: Vec<f32>, top_k: usize) -> anyhow::Result<Vec<(String, f32)>> {
        let results = self.inner.read().search((&vector).into(), top_k, 200);
        let map = self.id_map.read();
        Ok(results
            .into_iter()
            .filter_map(|r| map.get(&r.d_id).cloned().map(|id| (id, r.distance)))
            .collect())
    }
}


