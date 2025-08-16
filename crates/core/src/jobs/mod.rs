use crate::catalog::pg::PgCatalogProvider;
use crate::catalog::CatalogProvider;
use crate::embed::{build_embedding_provider_from_config, build_input_text};
use crate::fingerprint::merkle::merkle_root;
use crate::repo::sqlite::SqliteRepo;
use crate::stats;
use crate::Config;
use once_cell::sync::OnceCell;
use regex::Regex;
use sqlx::{postgres::PgListener, Pool, Postgres};
use tokio::time::{sleep, Duration};
use tracing::info;
use uuid::Uuid;

pub async fn execute_bootstrap(cfg: &Config) -> anyhow::Result<i64> {
    let pool = get_pg_pool(cfg).await?;
    let provider = PgCatalogProvider::new(pool.clone());
    let objects = provider.snapshot().await?;
    println!("🔍 Debug: Snapshot found {} objects", objects.len());
    for obj in &objects {
        println!("🔍 Debug: Object: {} ({:?})", obj.qualified_name, obj.kind);
    }
    let leaves: Vec<(String, String)> = objects
        .iter()
        .map(|o| (o.qualified_name.clone(), o.hash.clone()))
        .collect();
    let root = merkle_root(leaves);
    let repo = SqliteRepo::open(&cfg.sidecar_db).await?;
    repo.migrate().await?;
    let version = repo.get_next_version().await?;
    let rows: Vec<(&Uuid, &str, &str, &str, &str)> = objects
        .iter()
        .map(|o| {
            (
                &o.id,
                kind_str(o.kind),
                o.qualified_name.as_str(),
                o.canonical_ddl.as_str(),
                o.hash.as_str(),
            )
        })
        .collect();
    repo.upsert_objects(version, &rows).await?;
    repo.insert_schema_version(version, &root).await?;

    // Generate and persist embeddings for all objects
    let provider = build_embedding_provider_from_config(cfg)?;
    let inputs: Vec<String> = objects
        .iter()
        .map(|o| build_input_text(&o.qualified_name, &o.canonical_ddl))
        .collect();

    println!(
        "🔍 Debug: Generating embeddings for {} objects",
        objects.len()
    );
    let vectors = provider.embed_batch(&inputs).await?;
    println!("🔍 Debug: Generated {} embeddings", vectors.len());

    let _ = repo.clear_embeddings().await; // best-effort on bootstrap
    for (o, vec) in objects.iter().zip(vectors.into_iter()) {
        match repo
            .upsert_embedding(&o.id.to_string(), &vec, &cfg.embed_provider, cfg.embed_dim)
            .await
        {
            Ok(_) => println!("🔍 Debug: Stored embedding for {}", o.qualified_name),
            Err(e) => println!(
                "❌ Debug: Failed to store embedding for {}: {}",
                o.qualified_name, e
            ),
        }
    }

    // Collect stats and samples for tables
    for o in &objects {
        if matches!(o.kind, crate::catalog::ObjectKind::Table) {
            if let Some((schema, table)) = o.qualified_name.split_once('.') {
                if let Ok(summary) = stats::collect_pg_stats(&pool, schema, table).await {
                    let object_id_owned = o.id.to_string();
                    let _ = repo.insert_object_stat(
                        object_id_owned.as_str(),
                        "estimated_rows",
                        summary.estimated_rows.to_string().as_str(),
                    );
                    for c in summary.columns {
                        let metric1 = format!("pg_stats:{}:null_frac", c.column);
                        let metric2 = format!("pg_stats:{}:n_distinct", c.column);
                        let _ = repo.insert_object_stat(
                            object_id_owned.as_str(),
                            metric1.as_str(),
                            format!("{}", c.null_frac).as_str(),
                        );
                        let _ = repo.insert_object_stat(
                            object_id_owned.as_str(),
                            metric2.as_str(),
                            format!("{}", c.n_distinct).as_str(),
                        );
                    }
                }
                if cfg.sample_rows > 0 {
                    if let Ok(sample) = stats::sample_table_with_masking(
                        &pool,
                        schema,
                        table,
                        cfg.sample_rows,
                        cfg.masking_rules.as_deref(),
                    )
                    .await
                    {
                        let cols_json =
                            serde_json::to_string(&sample.column_names).unwrap_or("[]".to_string());
                        let rows_json =
                            serde_json::to_string(&sample.rows).unwrap_or("[]".to_string());
                        let _ = repo.upsert_samples(
                            o.id.to_string().as_str(),
                            cols_json.as_str(),
                            rows_json.as_str(),
                        );
                    }
                }
            }
        }
    }

    info!(version, merkle_root=%root, "bootstrap complete");
    Ok(version)
}

#[derive(Debug, Clone)]
pub struct RefreshResult {
    pub version: i64,
    pub changed_ids: Vec<Uuid>,
}

pub async fn execute_refresh(cfg: &Config) -> anyhow::Result<RefreshResult> {
    let pool = get_pg_pool(cfg).await?;
    let provider = PgCatalogProvider::new(pool);
    let repo = SqliteRepo::open(&cfg.sidecar_db).await?;
    repo.migrate().await?;

    let prior = repo.fetch_latest_hashes().await?; // qualified_name -> hash
    let now_objects = provider.snapshot().await?;
    let mut changed = Vec::new();
    let mut leaves = Vec::new();
    for o in &now_objects {
        leaves.push((o.qualified_name.clone(), o.hash.clone()));
        if prior
            .get(&o.qualified_name)
            .map(|h| h != &o.hash)
            .unwrap_or(true)
        {
            changed.push(o.id);
        }
    }
    let root = merkle_root(leaves);
    let version = repo.get_next_version().await?;
    let rows: Vec<(&Uuid, &str, &str, &str, &str)> = now_objects
        .iter()
        .map(|o| {
            (
                &o.id,
                kind_str(o.kind),
                o.qualified_name.as_str(),
                o.canonical_ddl.as_str(),
                o.hash.as_str(),
            )
        })
        .collect();
    repo.upsert_objects(version, &rows).await?;
    repo.insert_schema_version(version, &root).await?;

    // Refresh embeddings (simple approach: recompute for all current objects)
    let provider = build_embedding_provider_from_config(cfg)?;
    let inputs: Vec<String> = now_objects
        .iter()
        .map(|o| build_input_text(&o.qualified_name, &o.canonical_ddl))
        .collect();
    let vectors = provider.embed_batch(&inputs).await?;
    for (o, vec) in now_objects.iter().zip(vectors.into_iter()) {
        let _ = repo
            .upsert_embedding(&o.id.to_string(), &vec, &cfg.embed_provider, cfg.embed_dim)
            .await;
    }

    // Collect stats and samples for tables (could optimize by changed set)
    let pool2 = get_pg_pool(cfg).await?;
    for o in &now_objects {
        if matches!(o.kind, crate::catalog::ObjectKind::Table) {
            if let Some((schema, table)) = o.qualified_name.split_once('.') {
                if let Ok(summary) = stats::collect_pg_stats(&pool2, schema, table).await {
                    let object_id_owned = o.id.to_string();
                    let _ = repo.insert_object_stat(
                        object_id_owned.as_str(),
                        "estimated_rows",
                        summary.estimated_rows.to_string().as_str(),
                    );
                    for c in summary.columns {
                        let metric1 = format!("pg_stats:{}:null_frac", c.column);
                        let metric2 = format!("pg_stats:{}:n_distinct", c.column);
                        let _ = repo.insert_object_stat(
                            object_id_owned.as_str(),
                            metric1.as_str(),
                            format!("{}", c.null_frac).as_str(),
                        );
                        let _ = repo.insert_object_stat(
                            object_id_owned.as_str(),
                            metric2.as_str(),
                            format!("{}", c.n_distinct).as_str(),
                        );
                    }
                }
                if cfg.sample_rows > 0 {
                    if let Ok(sample) = stats::sample_table_with_masking(
                        &pool2,
                        schema,
                        table,
                        cfg.sample_rows,
                        cfg.masking_rules.as_deref(),
                    )
                    .await
                    {
                        let cols_json =
                            serde_json::to_string(&sample.column_names).unwrap_or("[]".to_string());
                        let rows_json =
                            serde_json::to_string(&sample.rows).unwrap_or("[]".to_string());
                        let _ = repo.upsert_samples(
                            o.id.to_string().as_str(),
                            cols_json.as_str(),
                            rows_json.as_str(),
                        );
                    }
                }
            }
        }
    }

    info!(version, merkle_root=%root, changed=%changed.len(), "refresh complete");
    Ok(RefreshResult {
        version,
        changed_ids: changed,
    })
}

fn kind_str(k: crate::catalog::ObjectKind) -> &'static str {
    match k {
        crate::catalog::ObjectKind::Table => "table",
        crate::catalog::ObjectKind::View => "view",
        crate::catalog::ObjectKind::Index => "index",
        crate::catalog::ObjectKind::MaterializedView => "materialized_view",
        crate::catalog::ObjectKind::Function => "function",
        crate::catalog::ObjectKind::Schema => "schema",
    }
}

static WATCHERS_ONCE: OnceCell<()> = OnceCell::new();

/// Spawn polling and optional NOTIFY listeners for change detection (once per process).
pub async fn run_background_watchers(cfg: &Config) {
    if WATCHERS_ONCE.get().is_some() {
        return;
    }
    let _ = WATCHERS_ONCE.set(());

    if cfg.bootstrap_on_start {
        let cfg_clone = cfg.clone();
        tokio::spawn(async move {
            if let Err(err) = execute_bootstrap(&cfg_clone).await {
                tracing::warn!(error=%format!("{:#}", err), "bootstrap on start failed");
            }
        });
    }

    let poll_interval_secs = cfg.poll_interval_secs;
    if poll_interval_secs > 0 {
        let cfg_clone = cfg.clone();
        tokio::spawn(async move {
            loop {
                if let Err(err) = execute_refresh(&cfg_clone).await {
                    tracing::warn!(error=%format!("{:#}", err), "polling refresh failed");
                }
                sleep(Duration::from_secs(poll_interval_secs)).await;
            }
        });
    }

    if cfg.enable_notify {
        let cfg_clone = cfg.clone();
        tokio::spawn(async move {
            if cfg_clone.install_pg_event_trigger {
                if let Err(err) = install_event_trigger(&cfg_clone).await {
                    tracing::warn!(error=%format!("{:#}", err), "failed to install event trigger (needs superuser)");
                }
            }
            loop {
                match PgListener::connect(&cfg_clone.database_url).await {
                    Ok(mut listener) => {
                        if let Err(err) = listener.listen(&cfg_clone.notify_channel).await {
                            tracing::warn!(error=%format!("{:#}", err), channel=%cfg_clone.notify_channel, "listen failed");
                            sleep(Duration::from_secs(5)).await;
                            continue;
                        }
                        tracing::info!(channel=%cfg_clone.notify_channel, "listening for DDL notifications");
                        loop {
                            match listener.recv().await {
                                Ok(_notification) => {
                                    if let Err(err) = execute_refresh(&cfg_clone).await {
                                        tracing::warn!(error=%format!("{:#}", err), "notify-triggered refresh failed");
                                    }
                                    // Simple debounce to coalesce bursts
                                    sleep(Duration::from_millis(500)).await;
                                }
                                Err(err) => {
                                    tracing::warn!(error=%format!("{:#}", err), "listener error; reconnecting");
                                    break;
                                }
                            }
                        }
                    }
                    Err(err) => {
                        tracing::warn!(error=%format!("{:#}", err), "connect for listener failed; retrying");
                        sleep(Duration::from_secs(5)).await;
                    }
                }
            }
        });
    }
}

/// Attempt to install a DDL event trigger that emits NOTIFY on each DDL.
async fn install_event_trigger(cfg: &Config) -> anyhow::Result<()> {
    let pool = get_pg_pool(cfg).await?;
    let channel = sanitize_channel(&cfg.notify_channel);
    let sql = format!(
        r#"
        CREATE SCHEMA IF NOT EXISTS feytra;
        CREATE OR REPLACE FUNCTION feytra.ddl_notify() RETURNS event_trigger LANGUAGE plpgsql AS $$
        BEGIN
            PERFORM pg_notify('{channel}', json_build_object('event', TG_EVENT, 'tag', TG_TAG)::text);
        END; $$;
        DO $$ BEGIN
            CREATE EVENT TRIGGER feytra_ddl_event ON ddl_command_end EXECUTE PROCEDURE feytra.ddl_notify();
        EXCEPTION WHEN duplicate_object THEN NULL; END $$;
        "#
    );
    sqlx::query(&sql).execute(&pool).await?;
    Ok(())
}

fn sanitize_channel(input: &str) -> String {
    // Allow only [A-Za-z0-9_], else fallback to default
    static RE: OnceCell<Regex> = OnceCell::new();
    let re = RE.get_or_init(|| Regex::new(r"^[A-Za-z0-9_]+$").unwrap());
    if re.is_match(input) {
        input.to_string()
    } else {
        "feytra_ddl".to_string()
    }
}

// Shared Postgres pool (clone is cheap). Ensures one pool per process.
static PG_POOL: OnceCell<Pool<Postgres>> = OnceCell::new();
async fn get_pg_pool(cfg: &Config) -> anyhow::Result<Pool<Postgres>> {
    if let Some(pool) = PG_POOL.get() {
        return Ok(pool.clone());
    }
    let pool = Pool::<Postgres>::connect(&cfg.database_url).await?;
    let _ = PG_POOL.set(pool.clone());
    Ok(pool)
}
