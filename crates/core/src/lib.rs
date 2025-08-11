pub mod config;
pub mod catalog;
pub mod fingerprint;
pub mod stats;
pub mod graph;
pub mod embed;
pub mod vectors;
pub mod repo;
pub mod jobs;
pub mod utils;

pub use config::Config;

// Re-export key traits for ease of use in other crates
pub use catalog::CatalogProvider;
pub use embed::EmbeddingProvider;
pub use vectors::VectorIndex;


