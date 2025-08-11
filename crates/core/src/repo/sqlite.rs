use anyhow::Context;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Row, Sqlite};
use std::path::Path;
use uuid::Uuid;

#[derive(Clone)]
pub struct SqliteRepo {
    pool: Pool<Sqlite>,
}

impl SqliteRepo {
    pub async fn open(path: &str) -> anyhow::Result<Self> {
        let parent = Path::new(path).parent().map(|p| p.to_path_buf()).unwrap_or_default();
        if !parent.as_os_str().is_empty() { std::fs::create_dir_all(&parent).ok(); }
        let url = if path.starts_with("sqlite:") { path.to_string() } else { format!("sqlite://{}", path) };
        let pool = SqlitePoolOptions::new().max_connections(5).connect(&url).await.context("open sqlite")?;
        Ok(Self { pool })
    }

    pub async fn migrate(&self) -> anyhow::Result<()> {
        let sql = include_str!("../../../../migrations/sqlite/0001_init.sql");
        for stmt in sql.split(';') {
            let s = stmt.trim();
            if s.is_empty() { continue; }
            sqlx::query(s).execute(&self.pool).await?;
        }
        Ok(())
    }

    pub async fn insert_schema_version(&self, version: i64, merkle_root: &str) -> anyhow::Result<()> {
        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query("INSERT OR REPLACE INTO schema_versions(version, merkle_root, created_at) VALUES (?,?,?)")
            .bind(version)
            .bind(merkle_root)
            .bind(&now)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_next_version(&self) -> anyhow::Result<i64> {
        let v: (Option<i64>,) = sqlx::query_as("SELECT MAX(version) FROM schema_versions")
            .fetch_one(&self.pool)
            .await?;
        Ok(v.0.unwrap_or(0) + 1)
    }

    pub async fn upsert_objects(&self, version: i64, objects: &[(&Uuid, &str, &str, &str, &str)]) -> anyhow::Result<()> {
        let now = chrono::Utc::now().to_rfc3339();
        let mut tx = self.pool.begin().await?;
        for (id, kind, qualified_name, canonical_ddl, hash) in objects {
            sqlx::query("INSERT OR REPLACE INTO objects(id, kind, qualified_name, canonical_ddl, hash, version, changed_at) VALUES (?,?,?,?,?,?,?)")
                .bind(id.to_string())
                .bind(*kind)
                .bind(*qualified_name)
                .bind(*canonical_ddl)
                .bind(*hash)
                .bind(version)
                .bind(&now)
                .execute(&mut *tx)
                .await?;
        }
        tx.commit().await?;
        Ok(())
    }

    pub async fn fetch_latest_hashes(&self) -> anyhow::Result<std::collections::HashMap<String, String>> {
        let rows = sqlx::query("SELECT qualified_name, hash FROM objects WHERE version = (SELECT MAX(version) FROM objects)")
            .fetch_all(&self.pool)
            .await?;
        let mut map = std::collections::HashMap::with_capacity(rows.len());
        for r in rows { map.insert(r.try_get::<String, _>("qualified_name")?, r.try_get::<String, _>("hash")?); }
        Ok(map)
    }

    pub fn upsert_object_stats(&self, stats: &[(&str, &str, &str)]) -> anyhow::Result<()> {
        let now = chrono::Utc::now().to_rfc3339();
        let tx = self.conn.transaction()?;
        {
            let mut stmt = tx.prepare("INSERT INTO object_stats(object_id, metric, value, collected_at) VALUES (?,?,?,?)")?;
            for (object_id, metric, value) in stats {
                stmt.execute(params![*object_id, *metric, *value, now])?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    pub async fn insert_object_stat(&self, object_id: &str, metric: &str, value: &str) -> anyhow::Result<()> {
        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query("INSERT INTO object_stats(object_id, metric, value, collected_at) VALUES (?,?,?,?)")
            .bind(object_id)
            .bind(metric)
            .bind(value)
            .bind(&now)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub fn upsert_samples(&self, object_id: &str, column_names_json: &str, rows_json: &str) -> anyhow::Result<()> {
        let now = chrono::Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT INTO samples(object_id, column_names, rows_json, sampled_at) VALUES (?,?,?,?)",
            params![object_id, column_names_json, rows_json, now],
        )?;
        Ok(())
    }

    pub fn fetch_latest_samples(&self, object_id: &str) -> anyhow::Result<Option<(String, String, String)>> {
        let mut stmt = self.conn.prepare("SELECT column_names, rows_json, sampled_at FROM samples WHERE object_id = ? ORDER BY sampled_at DESC LIMIT 1")?;
        let mut rows = stmt.query(params![object_id])?;
        if let Some(row) = rows.next()? {
            let col: String = row.get(0)?;
            let rows_json: String = row.get(1)?;
            let when: String = row.get(2)?;
            Ok(Some((col, rows_json, when)))
        } else {
            Ok(None)
        }
    }

    pub fn fetch_stats_for_object(&self, object_id: &str, limit: usize) -> anyhow::Result<Vec<(String, String, String)>> {
        let mut stmt = self.conn.prepare("SELECT metric, value, collected_at FROM object_stats WHERE object_id = ? ORDER BY collected_at DESC LIMIT ?")?;
        let rows = stmt.query_map(params![object_id, limit as i64], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?))
        })?;
        let mut out = Vec::new();
        for r in rows { out.push(r?); }
        Ok(out)
    }

    pub fn fetch_objects_for_version(&self, version: i64) -> anyhow::Result<Vec<(String, String, String, String)>> {
        let mut stmt = self.conn.prepare("SELECT id, kind, qualified_name, canonical_ddl FROM objects WHERE version = ? ORDER BY qualified_name")?;
        let rows = stmt.query_map(params![version], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?, row.get::<_, String>(3)?))
        })?;
        let mut out = Vec::new();
        for r in rows { out.push(r?); }
        Ok(out)
    }

    pub fn fetch_object_by_id(&self, id: &str) -> anyhow::Result<Option<(String, String, String)>> {
        let mut stmt = self.conn.prepare("SELECT kind, qualified_name, canonical_ddl FROM objects WHERE id = ? ORDER BY version DESC LIMIT 1")?;
        let mut rows = stmt.query(params![id])?;
        if let Some(row) = rows.next()? {
            let kind: String = row.get(0)?;
            let qn: String = row.get(1)?;
            let ddl: String = row.get(2)?;
            Ok(Some((kind, qn, ddl)))
        } else {
            Ok(None)
        }
    }

    pub fn insert_object_stat(&self, object_id: &str, metric: &str, value: &str) -> anyhow::Result<()> {
        let now = chrono::Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT INTO object_stats(object_id, metric, value, collected_at) VALUES (?,?,?,?)",
            params![object_id, metric, value, now],
        )?;
        Ok(())
    }

    pub async fn upsert_samples(&self, object_id: &str, columns_json: &str, rows_json: &str) -> anyhow::Result<()> {
        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query("INSERT INTO object_samples(object_id, columns_json, rows_json, sampled_at) VALUES (?,?,?,?)")
            .bind(object_id)
            .bind(columns_json)
            .bind(rows_json)
            .bind(&now)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn fetch_objects_for_version(&self, version: i64) -> anyhow::Result<Vec<(String, String, String, String)>> {
        let rows = sqlx::query("SELECT id, kind, qualified_name, canonical_ddl FROM objects WHERE version = ? ORDER BY qualified_name")
            .bind(version)
            .fetch_all(&self.pool)
            .await?;
        let mut out = Vec::with_capacity(rows.len());
        for r in rows { out.push((r.try_get("id")?, r.try_get("kind")?, r.try_get("qualified_name")?, r.try_get("canonical_ddl")?)); }
        Ok(out)
    }

    pub async fn fetch_object_by_id(&self, id: &str) -> anyhow::Result<Option<(String, String, String)>> {
        let row = sqlx::query("SELECT kind, qualified_name, canonical_ddl FROM objects WHERE id = ? ORDER BY version DESC LIMIT 1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(|r| (r.try_get("kind").unwrap_or_default(), r.try_get("qualified_name").unwrap_or_default(), r.try_get("canonical_ddl").unwrap_or_default())))
    }

    pub async fn fetch_stats_for_object(&self, id: &str, limit: usize) -> anyhow::Result<Vec<(String, String, String)>> {
        let rows = sqlx::query("SELECT metric, value, collected_at FROM object_stats WHERE object_id = ? ORDER BY collected_at DESC LIMIT ?")
            .bind(id)
            .bind(limit as i64)
            .fetch_all(&self.pool)
            .await?;
        let mut out = Vec::with_capacity(rows.len());
        for r in rows { out.push((r.try_get("metric")?, r.try_get("value")?, r.try_get("collected_at")?)); }
        Ok(out)
    }

    pub async fn fetch_latest_samples(&self, id: &str) -> anyhow::Result<Option<(String, String, String)>> {
        let row = sqlx::query("SELECT columns_json, rows_json, sampled_at FROM object_samples WHERE object_id = ? ORDER BY sampled_at DESC LIMIT 1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(|r| (r.try_get("columns_json").unwrap_or_default(), r.try_get("rows_json").unwrap_or_default(), r.try_get("sampled_at").unwrap_or_default())))
    }
}


