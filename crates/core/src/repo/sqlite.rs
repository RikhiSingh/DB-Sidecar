use anyhow::Context;
use rusqlite::{Connection, params};
use std::path::Path;
use uuid::Uuid;

pub struct SqliteRepo {
    conn: Connection,
}

impl SqliteRepo {
    pub fn open(path: &str) -> anyhow::Result<Self> {
        let parent = Path::new(path).parent().map(|p| p.to_path_buf()).unwrap_or_default();
        if !parent.as_os_str().is_empty() { std::fs::create_dir_all(&parent).ok(); }
        let conn = Connection::open(path).context("open sqlite")?;
        Ok(Self { conn })
    }

    pub fn migrate(&self) -> anyhow::Result<()> {
        let sql = include_str!("../../../../migrations/sqlite/0001_init.sql");
        self.conn.execute_batch(sql).context("apply migrations")?;
        Ok(())
    }

    pub fn insert_schema_version(&self, version: i64, merkle_root: &str) -> anyhow::Result<()> {
        let now = chrono::Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT OR REPLACE INTO schema_versions(version, merkle_root, created_at) VALUES (?,?,?)",
            params![version, merkle_root, now],
        )?;
        Ok(())
    }

    pub fn get_next_version(&self) -> anyhow::Result<i64> {
        let ver: Option<i64> = self.conn.query_row(
            "SELECT MAX(version) FROM schema_versions",
            [],
            |row| row.get(0),
        ).ok();
        Ok(ver.unwrap_or(0) + 1)
    }

    pub fn upsert_objects(&self, version: i64, objects: &[(&Uuid, &str, &str, &str, &str)]) -> anyhow::Result<()> {
        let now = chrono::Utc::now().to_rfc3339();
        let tx = self.conn.transaction()?;
        {
            let mut stmt = tx.prepare("INSERT OR REPLACE INTO objects(id, kind, qualified_name, canonical_ddl, hash, version, changed_at) VALUES (?,?,?,?,?,?,?)")?;
            for (id, kind, qualified_name, canonical_ddl, hash) in objects {
                stmt.execute(params![id.to_string(), *kind, *qualified_name, *canonical_ddl, *hash, version, now])?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    pub fn fetch_latest_hashes(&self) -> anyhow::Result<std::collections::HashMap<String, String>> {
        let mut map = std::collections::HashMap::new();
        let mut stmt = self.conn.prepare("SELECT qualified_name, hash FROM objects WHERE version = (SELECT MAX(version) FROM objects)")?;
        let rows = stmt.query_map([], |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)))?;
        for r in rows { let (k, v) = r?; map.insert(k, v); }
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

    pub fn insert_object_stat(&self, object_id: &str, metric: &str, value: &str) -> anyhow::Result<()> {
        let now = chrono::Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT INTO object_stats(object_id, metric, value, collected_at) VALUES (?,?,?,?)",
            params![object_id, metric, value, now],
        )?;
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
}


