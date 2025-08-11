use axum::{routing::{get, post}, Json, Router};
use feytra_core::{jobs, Config};
use feytra_core::repo::sqlite::SqliteRepo;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AppState {
    pub cfg: Config,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BootstrapResponse { pub version: i64 }

#[derive(Debug, Serialize, ToSchema)]
pub struct RefreshResponse { pub version: i64, pub changed_ids: Vec<Uuid> }

#[derive(Debug, Serialize, ToSchema)]
pub struct SchemaSummary { pub version: i64, pub objects: Vec<ObjectResponse> }

#[derive(Debug, Serialize, ToSchema)]
pub struct ObjectResponse { pub id: Uuid, pub kind: String, pub qualified_name: String, pub canonical_ddl: String }

#[derive(Debug, Deserialize, ToSchema)]
pub struct AskRequest { pub question: String, pub role: Option<String> }

#[derive(Debug, Serialize, ToSchema)]
pub struct AskResponse { pub context_objects: Vec<String>, pub sql: String, pub rationale: String }

#[derive(Debug, Serialize, ToSchema)]
pub struct AdviceItem { pub kind: String, pub message: String, pub target: Option<String> }

pub fn router(cfg: Config) -> Router {
    let state = AppState { cfg };
    Router::new()
        .route("/health", get(health))
        .route("/bootstrap", post(bootstrap))
        .route("/refresh", post(refresh))
        .route("/schema/:version", get(get_schema))
        .route("/objects/:id", get(get_object))
        .route("/objects/:id/stats", get(get_object_stats))
        .route("/objects/:id/sample", get(get_object_sample))
        .route("/ask", post(ask))
        .route("/advice", post(advice))
        .route("/diff", get(diff))
        .with_state(state)
}

#[utoipa::path(get, path = "/health")]
pub async fn health() -> Json<serde_json::Value> { Json(serde_json::json!({"status":"ok"})) }

#[utoipa::path(post, path = "/v1/bootstrap")]
pub async fn bootstrap(axum::extract::State(state): axum::extract::State<AppState>) -> Json<BootstrapResponse> {
    let version = jobs::execute_bootstrap(&state.cfg).await.unwrap_or(1);
    Json(BootstrapResponse { version })
}

#[utoipa::path(post, path = "/v1/refresh")]
pub async fn refresh(axum::extract::State(state): axum::extract::State<AppState>) -> Json<RefreshResponse> {
    // For now, the job returns only version; TODO return changed ids from persistent diff table if stored
    let res = jobs::execute_refresh(&state.cfg).await;
    match res {
        Ok(r) => Json(RefreshResponse { version: r.version, changed_ids: r.changed_ids }),
        Err(_) => Json(RefreshResponse { version: 0, changed_ids: vec![] }),
    }
}

#[utoipa::path(get, path = "/v1/schema/{version}")]
pub async fn get_schema(axum::extract::State(state): axum::extract::State<AppState>, axum::extract::Path(version): axum::extract::Path<i64>) -> Json<SchemaSummary> {
    let repo = SqliteRepo::open(&state.cfg.sidecar_db).await.unwrap();
    let rows = repo.fetch_objects_for_version(version).await.unwrap_or_default();
    let objects: Vec<ObjectResponse> = rows.into_iter().map(|(id, kind, qualified_name, canonical_ddl)| {
        ObjectResponse { id: Uuid::parse_str(&id).unwrap_or_else(|_| Uuid::new_v4()), kind, qualified_name, canonical_ddl }
    }).collect();
    Json(SchemaSummary { version, objects })
}

#[utoipa::path(get, path = "/v1/objects/{id}")]
pub async fn get_object(axum::extract::State(state): axum::extract::State<AppState>, axum::extract::Path(id): axum::extract::Path<Uuid>) -> Json<ObjectResponse> {
    let repo = SqliteRepo::open(&state.cfg.sidecar_db).await.unwrap();
    if let Some((kind, qualified_name, canonical_ddl)) = repo.fetch_object_by_id(&id.to_string()).await.unwrap_or(None) {
        Json(ObjectResponse { id, kind, qualified_name, canonical_ddl })
    } else {
        Json(ObjectResponse { id, kind: "unknown".into(), qualified_name: "".into(), canonical_ddl: "".into() })
    }
}

#[utoipa::path(get, path = "/v1/objects/{id}/stats")]
pub async fn get_object_stats(axum::extract::State(state): axum::extract::State<AppState>, axum::extract::Path(id): axum::extract::Path<Uuid>) -> Json<serde_json::Value> {
    let repo = SqliteRepo::open(&state.cfg.sidecar_db).await.unwrap();
    let stats = repo.fetch_stats_for_object(&id.to_string(), 500).await.unwrap_or_default();
    let arr: Vec<serde_json::Value> = stats.into_iter().map(|(metric, value, ts)| serde_json::json!({"metric": metric, "value": value, "collected_at": ts})).collect();
    Json(serde_json::json!({"object_id": id, "stats": arr}))
}

#[utoipa::path(get, path = "/v1/objects/{id}/sample")]
pub async fn get_object_sample(axum::extract::State(state): axum::extract::State<AppState>, axum::extract::Path(id): axum::extract::Path<Uuid>) -> Json<serde_json::Value> {
    let repo = SqliteRepo::open(&state.cfg.sidecar_db).await.unwrap();
    if let Some((cols, rows, when)) = repo.fetch_latest_samples(&id.to_string()).await.unwrap_or(None) {
        let column_names: Vec<String> = serde_json::from_str(&cols).unwrap_or_default();
        let rows_json: serde_json::Value = serde_json::from_str(&rows).unwrap_or(serde_json::json!([]));
        Json(serde_json::json!({"object_id": id, "sampled_at": when, "column_names": column_names, "rows": rows_json}))
    } else {
        Json(serde_json::json!({"object_id": id, "column_names": [], "rows": []}))
    }
}

#[utoipa::path(post, path = "/v1/ask")]
pub async fn ask(Json(req): Json<AskRequest>) -> Json<AskResponse> {
    let sql = "SELECT * FROM public.example LIMIT 10;".to_string();
    Json(AskResponse { context_objects: vec!["public.example".into()], sql, rationale: format!("Heuristic based on question: {}", req.question) })
}

#[utoipa::path(post, path = "/v1/advice")]
pub async fn advice() -> Json<Vec<AdviceItem>> {
    Json(vec![AdviceItem { kind: "naming".into(), message: "Table name 'Users' should be snake_case".into(), target: Some("public.Users".into()) }])
}

#[utoipa::path(get, path = "/v1/diff")]
pub async fn diff(axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>) -> Json<super::v1::DiffResponse> {
    let from = params.get("from").cloned().unwrap_or_default();
    let to = params.get("to").cloned().unwrap_or_default();
    Json(DiffResponse { from, to, summary: "No changes".into(), diffs: vec![] })
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DiffEntry { pub qualified_name: String, pub old: String, pub new: String }

#[derive(Debug, Serialize, ToSchema)]
pub struct DiffResponse { pub from: String, pub to: String, pub summary: String, pub diffs: Vec<DiffEntry> }


