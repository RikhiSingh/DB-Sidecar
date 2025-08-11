use axum::{routing::{get, post}, Router};
use feytra_core::Config;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod openapi;
pub mod routes;

pub fn build_router(cfg: Config) -> Router {
    let api = routes::v1::router(cfg.clone());
    // Background watchers (polling + optional NOTIFY)
    let cfg_clone = cfg.clone();
    tokio::spawn(async move { feytra_core::jobs::run_background_watchers(&cfg_clone).await });
    let health = axum::routing::get(|| async { axum::Json(serde_json::json!({"status":"ok"})) });

    let openapi = openapi::ApiDoc::openapi();

    Router::new()
        .route("/health", get(health))
        .nest("/v1", api)
        .merge(SwaggerUi::new("/docs").url("/api-doc/openapi.json", openapi))
}


