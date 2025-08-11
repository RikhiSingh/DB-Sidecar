use crate::catalog::{CatalogObject, CatalogProvider, NeighborRef, ObjectKind};
use crate::fingerprint::canonical::{canonicalize_table, hash_ddl};
use crate::utils::ids::deterministic_object_id;
use anyhow::Context;
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

pub struct PgCatalogProvider {
    pool: Pool<Postgres>,
}

impl PgCatalogProvider {
    pub fn new(pool: Pool<Postgres>) -> Self { Self { pool } }
}

#[async_trait::async_trait]
impl CatalogProvider for PgCatalogProvider {
    async fn snapshot(&self) -> anyhow::Result<Vec<CatalogObject>> {
        // Minimal snapshot: list tables and views with schema-qualified names.
        // We build canonical DDLs minimally for tables using columns.
        let rows = sqlx::query(
            r#"
            SELECT n.nspname AS schema, c.relname AS name, c.relkind AS kind
            FROM pg_class c
            JOIN pg_namespace n ON n.oid = c.relnamespace
            WHERE n.nspname NOT IN ('pg_catalog','information_schema')
              AND c.relkind IN ('r','v','m','i')
            ORDER BY 1,2
            "#
        )
        .fetch_all(&self.pool)
        .await
        .context("query pg_class")?;

        let mut objects = Vec::with_capacity(rows.len());
        for r in rows {
            let schema_name: String = r.try_get("schema").unwrap_or_default();
            let object_name: String = r.try_get("name").unwrap_or_default();
            let relkind: String = r.try_get::<String, _>("kind").unwrap_or_else(|_| "r".to_string());
            let qualified = format!("{}.{}", schema_name, object_name);
            let kind = match relkind.as_str() {
                "r" => ObjectKind::Table,
                "v" => ObjectKind::View,
                "m" => ObjectKind::MaterializedView,
                "i" => ObjectKind::Index,
                _ => ObjectKind::Table,
            };

            let ddl = match kind {
                ObjectKind::Table => canonicalize_table(&self.pool, &schema_name, &object_name).await?,
                _ => format!("-- {} {}", match kind { ObjectKind::View => "VIEW", ObjectKind::MaterializedView => "MATERIALIZED VIEW", ObjectKind::Index => "INDEX", _ => "OBJECT" }, qualified),
            };
            let hash = hash_ddl(&ddl);
            let id = deterministic_object_id(match kind { ObjectKind::Table=>"table", ObjectKind::View=>"view", ObjectKind::MaterializedView=>"materialized_view", ObjectKind::Index=>"index", ObjectKind::Function=>"function", ObjectKind::Schema=>"schema" }, &qualified);
            objects.push(CatalogObject { id, kind, qualified_name: qualified, canonical_ddl: ddl, hash });
        }

        Ok(objects)
    }

    async fn neighbors(&self, _object_id: &Uuid) -> anyhow::Result<Vec<NeighborRef>> {
        // TODO: populate via pg_depend relationships
        Ok(Vec::new())
    }
}


