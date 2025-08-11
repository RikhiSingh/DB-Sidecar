use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectStat {
    pub object: String,
    pub metric: String,
    pub value: String,
}

pub async fn collect_basic_stats(pool: &Pool<Postgres>, schema: &str, table: &str) -> anyhow::Result<Vec<ObjectStat>> {
    let est_rows = sqlx::query_scalar!(
        r#"
        SELECT reltuples::bigint FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = $1 AND c.relname = $2
        "#,
        schema,
        table
    )
    .fetch_optional(pool)
    .await?
    .unwrap_or(0i64);

    Ok(vec![ObjectStat { object: format!("{}.{}", schema, table), metric: "estimated_rows".into(), value: est_rows.to_string() }])
}


