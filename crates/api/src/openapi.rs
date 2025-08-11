use utoipa::OpenApi;

use crate::routes::v1::{AskRequest, AskResponse, AdviceItem, DiffResponse, ObjectResponse, SchemaSummary, BootstrapResponse, RefreshResponse};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::v1::health,
        crate::routes::v1::bootstrap,
        crate::routes::v1::refresh,
        crate::routes::v1::get_schema,
        crate::routes::v1::get_object,
        crate::routes::v1::ask,
        crate::routes::v1::advice,
        crate::routes::v1::diff
    ),
    components(schemas(AskRequest, AskResponse, AdviceItem, DiffResponse, ObjectResponse, SchemaSummary, BootstrapResponse, RefreshResponse)),
    tags((name = "feytra", description = "Feytra Schema Intelligence API"))
)]
pub struct ApiDoc;


