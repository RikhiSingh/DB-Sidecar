use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub sidecar_db: String,
    pub embed_provider: String,
    pub embed_dim: usize,
    pub vector_backend: String,
    pub qdrant_url: Option<String>,
    pub poll_interval_secs: u64,
    pub sample_rows: usize,
    pub masking_rules: Option<String>,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        let _ = dotenvy::dotenv();
        Self {
            database_url: std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/postgres".to_string()),
            sidecar_db: std::env::var("SIDECAR_DB").unwrap_or_else(|_| "./data/sidecar.sqlite".to_string()),
            embed_provider: std::env::var("EMBED_PROVIDER").unwrap_or_else(|_| "local_stub".to_string()),
            embed_dim: std::env::var("EMBED_DIM").ok().and_then(|v| v.parse().ok()).unwrap_or(768),
            vector_backend: std::env::var("VECTOR_BACKEND").unwrap_or_else(|_| "hnsw".to_string()),
            qdrant_url: std::env::var("QDRANT_URL").ok(),
            poll_interval_secs: std::env::var("POLL_INTERVAL_SECS").ok().and_then(|v| v.parse().ok()).unwrap_or(300),
            sample_rows: std::env::var("SAMPLE_ROWS").ok().and_then(|v| v.parse().ok()).unwrap_or(200),
            masking_rules: std::env::var("MASKING_RULES").ok(),
            port: std::env::var("PORT").ok().and_then(|v| v.parse().ok()).unwrap_or(8080),
        }
    }
}


