use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationEdge {
    pub src_id: Uuid,
    pub dst_id: Uuid,
    pub kind: String,
}


