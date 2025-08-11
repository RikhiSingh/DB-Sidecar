use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, Row};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectStat {
    pub object: String,
    pub metric: String,
    pub value: String,
}

pub async fn collect_basic_stats(
    pool: &Pool<Postgres>,
    schema: &str,
    table: &str,
) -> anyhow::Result<Vec<ObjectStat>> {
    let est_rows: i64 = sqlx::query_scalar(
        r#"
        SELECT reltuples::bigint FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = $1 AND c.relname = $2
        "#,
    )
    .bind(schema)
    .bind(table)
    .fetch_optional(pool)
    .await?
    .unwrap_or(0i64);

    Ok(vec![ObjectStat {
        object: format!("{}.{}", schema, table),
        metric: "estimated_rows".into(),
        value: est_rows.to_string(),
    }])
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnStat {
    pub column: String,
    pub null_frac: f64,
    pub n_distinct: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableStatsSummary {
    pub object: String,
    pub estimated_rows: i64,
    pub columns: Vec<ColumnStat>,
}

pub async fn collect_pg_stats(
    pool: &Pool<Postgres>,
    schema: &str,
    table: &str,
) -> anyhow::Result<TableStatsSummary> {
    let est_rows: i64 = sqlx::query_scalar(
        r#"
        SELECT reltuples::bigint FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = $1 AND c.relname = $2
        "#,
    )
    .bind(schema)
    .bind(table)
    .fetch_optional(pool)
    .await?
    .unwrap_or(0i64);

    let rows = sqlx::query(
        r#"
        SELECT a.attname AS column,
               s.null_frac,
               s.n_distinct
        FROM pg_attribute a
        JOIN pg_class c ON c.oid = a.attrelid
        JOIN pg_namespace n ON n.oid = c.relnamespace
        LEFT JOIN pg_stats s ON s.schemaname = n.nspname AND s.tablename = c.relname AND s.attname = a.attname
        WHERE n.nspname = $1 AND c.relname = $2 AND a.attnum > 0 AND NOT a.attisdropped
        ORDER BY a.attnum
        "#
    )
    .bind(schema)
    .bind(table)
    .fetch_all(pool)
    .await?;

    let mut columns: Vec<ColumnStat> = Vec::with_capacity(rows.len());
    for r in rows {
        let column: String = r.try_get("column").unwrap_or_default();
        let null_frac: Option<f64> = r.try_get("null_frac").ok();
        let n_distinct: Option<f64> = r.try_get("n_distinct").ok();
        columns.push(ColumnStat {
            column,
            null_frac: null_frac.unwrap_or(0.0),
            n_distinct: n_distinct.unwrap_or(0.0),
        });
    }

    Ok(TableStatsSummary {
        object: format!("{}.{}", schema, table),
        estimated_rows: est_rows,
        columns,
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SampledTable {
    pub object: String,
    pub column_names: Vec<String>,
    pub rows: Vec<serde_json::Value>,
}

pub async fn sample_table_with_masking(
    pool: &Pool<Postgres>,
    schema: &str,
    table: &str,
    limit_rows: usize,
    masking_rules: Option<&str>,
) -> anyhow::Result<SampledTable> {
    let qualified = format!("{}.{}", schema, table);
    // Use a subselect and to_jsonb to get stable JSON objects with column names
    let sql = format!(
        "SELECT to_jsonb(t) AS row FROM (SELECT * FROM \"{}\".\"{}\" TABLESAMPLE SYSTEM (1) LIMIT $1) t",
        schema.replace('"', ""),
        table.replace('"', "")
    );
    let rows = sqlx::query(&sql)
        .bind(limit_rows as i64)
        .fetch_all(pool)
        .await
        .unwrap_or_default();

    let mut json_rows: Vec<serde_json::Value> = Vec::with_capacity(rows.len());
    for row in rows {
        let v: serde_json::Value = row.try_get("row").unwrap_or(serde_json::Value::Null);
        json_rows.push(v);
    }

    // Column names from the first row's object keys
    let column_names: Vec<String> = json_rows
        .get(0)
        .and_then(|v| v.as_object())
        .map(|m| m.keys().cloned().collect())
        .unwrap_or_else(|| Vec::new());

    let masked_rows = apply_masking(json_rows, masking_rules)?;
    Ok(SampledTable {
        object: qualified,
        column_names,
        rows: masked_rows,
    })
}

fn apply_masking(
    rows: Vec<serde_json::Value>,
    masking_rules: Option<&str>,
) -> anyhow::Result<Vec<serde_json::Value>> {
    if masking_rules.is_none() {
        return Ok(rows);
    }
    let rules_str = masking_rules.unwrap();
    // Masking rule format: comma-separated regex=kind pairs, e.g. "email=.*email.*,name=.*name.*"
    let mut column_to_kind: Vec<(Regex, String)> = Vec::new();
    for part in rules_str.split(',') {
        if part.trim().is_empty() {
            continue;
        }
        if let Some((lhs, rhs)) = part.split_once('=') {
            if let Ok(re) = Regex::new(lhs.trim()) {
                column_to_kind.push((re, rhs.trim().to_string()));
            }
        }
    }
    let masked = rows
        .into_iter()
        .map(|mut row| {
            if let serde_json::Value::Object(ref mut map) = row {
                for (col, val) in map.clone() {
                    for (re, kind) in &column_to_kind {
                        if re.is_match(&col) {
                            let masked_val = mask_value(&val, kind);
                            map.insert(col.clone(), masked_val);
                            break;
                        }
                    }
                }
            }
            row
        })
        .collect();
    Ok(masked)
}

fn mask_value(_value: &serde_json::Value, kind: &str) -> serde_json::Value {
    match kind {
        "email" => serde_json::Value::String("***@***".to_string()),
        "name" => serde_json::Value::String("REDACTED".to_string()),
        "phone" => serde_json::Value::String("***-***-****".to_string()),
        "id" => serde_json::Value::String("*****".to_string()),
        _ => serde_json::Value::String("***".to_string()),
    }
}
