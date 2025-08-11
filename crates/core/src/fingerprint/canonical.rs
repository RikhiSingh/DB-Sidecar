use sha2::{Digest, Sha256};
use sqlx::{Pool, Postgres, Row};

pub fn normalize_whitespace(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut last_was_ws = false;
    for ch in input.chars() {
        if ch.is_whitespace() {
            if !last_was_ws {
                out.push(' ');
                last_was_ws = true;
            }
        } else {
            out.push(ch);
            last_was_ws = false;
        }
    }
    out.trim().to_string()
}

pub fn hash_ddl(ddl: &str) -> String {
    let normalized = normalize_whitespace(ddl);
    let mut hasher = Sha256::new();
    hasher.update(normalized.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

pub async fn canonicalize_table(pool: &Pool<Postgres>, schema: &str, table: &str) -> anyhow::Result<String> {
    // Minimal canonicalization: select columns in ordinal order, lowercase identifiers, normalize defaults.
    let rows = sqlx::query(
        r#"
        SELECT a.attname as column_name,
               pg_catalog.format_type(a.atttypid, a.atttypmod) AS data_type,
               a.attnotnull as not_null,
               pg_get_expr(ad.adbin, ad.adrelid) as default_expr
        FROM pg_attribute a
        JOIN pg_class c ON a.attrelid = c.oid
        JOIN pg_namespace n ON n.oid = c.relnamespace
        LEFT JOIN pg_attrdef ad ON ad.adrelid = a.attrelid AND ad.adnum = a.attnum
        WHERE c.relname = $1 AND n.nspname = $2 AND a.attnum > 0 AND NOT a.attisdropped
        ORDER BY a.attnum
        "#
    )
    .bind(table)
    .bind(schema)
    .fetch_all(pool)
    .await?;

    let cols: Vec<String> = rows
        .into_iter()
        .map(|r| {
            let name: String = r.try_get::<String, _>("column_name").unwrap_or_default();
            let dtype: String = r.try_get::<String, _>("data_type").unwrap_or_else(|_| "text".to_string());
            let not_null: bool = r.try_get::<bool, _>("not_null").unwrap_or(false);
            let default_expr: Option<String> = r.try_get::<Option<String>, _>("default_expr").ok().flatten();
            let mut parts = vec![format!("\"{}\" {}", name.to_lowercase(), dtype.to_lowercase())];
            if not_null { parts.push("NOT NULL".to_string()); }
            if let Some(def) = default_expr { parts.push(format!("DEFAULT {}", normalize_default(&def))); }
            parts.join(" ")
        })
        .collect();

    let ddl = format!("CREATE TABLE \"{}\".\"{}\" ({});", schema.to_lowercase(), table.to_lowercase(), cols.join(", "));
    Ok(ddl)
}

fn normalize_default(expr: &str) -> String {
    let e = expr.trim();
    if e.eq_ignore_ascii_case("now()") { return "CURRENT_TIMESTAMP".to_string(); }
    e.replace('\n', " ").replace('\t', " ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_stable() {
        let a = "CREATE TABLE x (id INT, name TEXT)";
        let b = " CREATE   TABLE x ( id INT , name TEXT ) ";
        assert_eq!(hash_ddl(a), hash_ddl(b));
    }
}


