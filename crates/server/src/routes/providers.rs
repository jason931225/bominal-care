// =============================================================================
// Provider Routes — /api/providers
// =============================================================================

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use uuid::Uuid;

use crate::{auth::{extractor::AuthUser, permission::require_permission}, AppState};
use bominal_db::queries::provider_organization;
use bominal_types::{ApiResponse, PaginationMeta, PaginationParams};
use bominal_types::rbac::{Resource, Action};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_providers))
        .route("/{id}", get(get_provider))
}

async fn list_providers(
    State(state): State<AppState>,
    user: AuthUser,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Provider, Action::List) {
        return e.into_response();
    }
    let params = PaginationParams::new(params.page, params.limit);
    match provider_organization::list(&state.pool, Some(true), params.limit, params.offset()).await {
        Ok((data, total)) => {
            let meta = PaginationMeta::new(total, params.page, params.limit);
            Json(ApiResponse::success_with_meta(data, meta)).into_response()
        }
        Err(e) => {
            tracing::error!("DB error listing providers: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::<()>::error("서버 오류"))).into_response()
        }
    }
}

async fn get_provider(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Provider, Action::Read) {
        return e.into_response();
    }
    match provider_organization::get(&state.pool, id).await {
        Ok(Some(data)) => Json(ApiResponse::success(data)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Json(ApiResponse::<()>::error("기관을 찾을 수 없습니다"))).into_response(),
        Err(e) => {
            tracing::error!("DB error: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::<()>::error("서버 오류"))).into_response()
        }
    }
}
