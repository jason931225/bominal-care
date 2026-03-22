// =============================================================================
// Audit Log Routes — GET /audit-logs (government only)
// =============================================================================

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};

use crate::{auth::{extractor::AuthUser, permission::require_permission}, AppState};
use bominal_types::rbac::{Resource, Action};
use bominal_db::queries::audit;
use bominal_types::{ApiResponse, PaginationMeta, PaginationParams};

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(list_audit_logs))
}

/// GET /api/audit-logs?page=1&limit=20
/// Restricted to government reviewers, provider admins, and platform admins.
async fn list_audit_logs(
    State(state): State<AppState>,
    user: AuthUser,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::AuditLog, Action::List) {
        return e.into_response();
    }

    let params = PaginationParams::new(params.page, params.limit);

    let filters = audit::AuditLogFilters::default();
    let pagination = audit::Pagination {
        page: params.page,
        limit: params.limit,
    };

    match audit::list_audit_logs(&state.pool, &filters, &pagination).await {
        Ok(result) => {
            let meta = PaginationMeta::new(result.total, params.page, params.limit);
            Json(ApiResponse::success_with_meta(result.data, meta)).into_response()
        }
        Err(e) => {
            tracing::error!("DB error listing audit logs: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}
