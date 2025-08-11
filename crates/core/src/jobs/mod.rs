use crate::catalog::pg::PgCatalogProvider;
use crate::catalog::CatalogProvider;
use crate::fingerprint::merkle::merkle_root;
use crate::repo::sqlite::SqliteRepo;
use crate::Config;
use sqlx::{Pool, Postgres};
use tracing::info;

pub async fn execute_bootstrap(cfg: &Config) -> anyhow::Result<i64> {
    let pool = Pool::<Postgres>::connect(&cfg.database_url).await?;
    let provider = PgCatalogProvider::new(pool);
    let objects = provider.snapshot().await?;
    let leaves: Vec<(String, String)> = objects.iter().map(|o| (o.qualified_name.clone(), o.hash.clone())).collect();
    let root = merkle_root(leaves);
    let repo = SqliteRepo::open(&cfg.sidecar_db)?;
    repo.migrate()?;
    let version = 1i64; // TODO: compute from existing
    repo.insert_schema_version(version, &root)?;
    info!(version, merkle_root=%root, "bootstrap complete");
    Ok(version)
}

pub async fn execute_refresh(cfg: &Config) -> anyhow::Result<i64> {
    // TODO: diff old/new and store changed objects
    let v = execute_bootstrap(cfg).await?;
    Ok(v)
}


