use sha2::{Digest, Sha256};
use uuid::Uuid;

pub fn deterministic_object_id(kind: &str, qualified_name: &str) -> Uuid {
    let mut hasher = Sha256::new();
    hasher.update(kind.as_bytes());
    hasher.update(b"::");
    hasher.update(qualified_name.as_bytes());
    let bytes = hasher.finalize();
    let mut b = [0u8; 16];
    b.copy_from_slice(&bytes[0..16]);
    Uuid::from_bytes(b)
}


