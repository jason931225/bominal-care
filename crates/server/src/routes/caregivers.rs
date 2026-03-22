// =============================================================================
// Caregiver Application Routes — GET /caregiver-applications
// =============================================================================

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{auth::{extractor::AuthUser, permission::require_permission}, AppState};
use bominal_types::rbac::{Resource, Action};
use bominal_db::queries::caregiver_application;
use bominal_types::{ApiResponse, CaregiverApplicationStatus, PaginationMeta, PaginationParams};

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(list_applications))
}

#[derive(Debug, Deserialize)]
struct ApplicationQueryParams {
    #[serde(flatten)]
    pagination: PaginationParams,
    status: Option<CaregiverApplicationStatus>,
    provider_id: Option<Uuid>,
}

/// GET /api/caregiver-applications?page=1&limit=20&status=...
async fn list_applications(
    State(state): State<AppState>,
    user: AuthUser,
    Query(query): Query<ApplicationQueryParams>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::CaregiverApplication, Action::List) {
        return e.into_response();
    }

    let params = PaginationParams::new(query.pagination.page, query.pagination.limit);

    let filters = caregiver_application::ApplicationFilters {
        status: query.status,
        provider_id: query.provider_id,
    };

    match caregiver_application::list_applications(
        &state.pool,
        &filters,
        params.limit,
        params.offset(),
    )
    .await
    {
        Ok(result) => {
            let meta = PaginationMeta::new(result.total, params.page, params.limit);
            Json(ApiResponse::success_with_meta(result.data, meta)).into_response()
        }
        Err(e) => {
            tracing::error!("DB error listing caregiver applications: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}
