// =============================================================================
// Incident Routes — /api/incidents
// =============================================================================

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use uuid::Uuid;

use crate::{auth::{extractor::AuthUser, permission::require_permission}, middleware::validate::ValidatedJson, AppState};
use bominal_db::queries::incident;
use bominal_types::{ApiResponse, PaginationMeta, PaginationParams};
use bominal_types::inputs::IncidentInput;
use bominal_types::rbac::{Resource, Action};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_incidents).post(create_incident))
        .route("/{id}", get(get_incident))
}

async fn list_incidents(
    State(state): State<AppState>,
    user: AuthUser,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Observability, Action::List) {
        return e.into_response();
    }
    let params = PaginationParams::new(params.page, params.limit);
    match incident::list_incidents(&state.pool, None, params.limit, params.offset()).await {
        Ok((data, total)) => {
            let meta = PaginationMeta::new(total, params.page, params.limit);
            Json(ApiResponse::success_with_meta(data, meta)).into_response()
        }
        Err(e) => {
            tracing::error!("DB error: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::<()>::error("서버 오류"))).into_response()
        }
    }
}

async fn create_incident(
    State(state): State<AppState>,
    user: AuthUser,
    ValidatedJson(input): ValidatedJson<IncidentInput>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Observability, Action::Create) {
        return e.into_response();
    }
    match incident::create_incident(&state.pool, input.visit_id, user.id, input.severity, &input.title, &input.description, input.occurred_at).await {
        Ok(data) => (StatusCode::CREATED, Json(ApiResponse::success(data))).into_response(),
        Err(e) => {
            tracing::error!("DB error: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::<()>::error("서버 오류"))).into_response()
        }
    }
}

async fn get_incident(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Observability, Action::Read) {
        return e.into_response();
    }
    match incident::get_incident(&state.pool, id).await {
        Ok(Some(data)) => Json(ApiResponse::success(data)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Json(ApiResponse::<()>::error("사고보고를 찾을 수 없습니다"))).into_response(),
        Err(e) => {
            tracing::error!("DB error: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::<()>::error("서버 오류"))).into_response()
        }
    }
}
