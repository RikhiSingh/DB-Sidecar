use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod pg;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ObjectKind {
    Table,
    View,
    Index,
    MaterializedView,
    Function,
    Schema,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CatalogObject {
    pub id: Uuid,
    pub kind: ObjectKind,
    pub qualified_name: String,
    pub canonical_ddl: String,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct NeighborRef {
    pub object_id: Uuid,
    pub kind: String,
}

#[async_trait::async_trait]
pub trait CatalogProvider: Send + Sync {
    async fn snapshot(&self) -> anyhow::Result<Vec<CatalogObject>>;
    async fn neighbors(&self, object_id: &Uuid) -> anyhow::Result<Vec<NeighborRef>>;
}


