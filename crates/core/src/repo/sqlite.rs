use anyhow::Context;
use rusqlite::{Connection, params};
use std::path::Path;

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
}


