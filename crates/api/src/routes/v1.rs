use axum::{routing::{get, post}, Json, Router};
use feytra-core::{jobs, Config};
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
    let version = jobs::execute_refresh(&state.cfg).await.unwrap_or(1);
    Json(RefreshResponse { version, changed_ids: vec![] })
}

#[utoipa::path(get, path = "/v1/schema/{version}")]
pub async fn get_schema(axum::extract::Path(version): axum::extract::Path<i64>) -> Json<SchemaSummary> {
    Json(SchemaSummary { version, objects: vec![] })
}

#[utoipa::path(get, path = "/v1/objects/{id}")]
pub async fn get_object(axum::extract::Path(_id): axum::extract::Path<Uuid>) -> Json<ObjectResponse> {
    Json(ObjectResponse { id: Uuid::new_v4(), kind: "table".into(), qualified_name: "public.example".into(), canonical_ddl: "CREATE TABLE public.example(id int);".into() })
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


