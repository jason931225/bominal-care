// =============================================================================
// Government Routes — /api/gov (aggregated, de-identified data only)
// =============================================================================

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};

use crate::{auth::{extractor::AuthUser, permission::require_permission}, AppState};
use bominal_types::ApiResponse;
use bominal_types::rbac::{Resource, Action};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/dashboard", get(dashboard))
        .route("/beneficiary-stats", get(beneficiary_stats))
        .route("/visit-stats", get(visit_stats))
        .route("/provider-compliance", get(provider_compliance))
}

async fn dashboard(
    State(state): State<AppState>,
    user: AuthUser,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::EligibilityCase, Action::List) {
        return e.into_response();
    }
    let result = sqlx::query_as::<_, (i64, i64, i64)>(
        "SELECT
            (SELECT COUNT(*) FROM senior_profiles) AS total_seniors,
            (SELECT COUNT(*) FROM visits WHERE status = 'IN_PROGRESS') AS active_visits,
            (SELECT COUNT(*) FROM eligibility_cases WHERE status NOT IN ('APPROVED', 'DENIED', 'FINAL')) AS pending_cases"
    )
    .fetch_one(&state.pool)
    .await;

    match result {
        Ok((seniors, visits, cases)) => {
            let data = serde_json::json!({
                "total_seniors": seniors,
                "active_visits": visits,
                "pending_cases": cases,
            });
            Json(ApiResponse::success(data)).into_response()
        }
        Err(e) => {
            tracing::error!("DB error: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::<()>::error("서버 오류"))).into_response()
        }
    }
}

async fn beneficiary_stats(
    State(state): State<AppState>,
    user: AuthUser,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::EligibilityCase, Action::List) {
        return e.into_response();
    }
    match sqlx::query_scalar::<_, serde_json::Value>(
        "SELECT COALESCE(json_agg(row_to_json(g)), '[]'::json) FROM gov_beneficiary_stats g"
    ).fetch_one(&state.pool).await {
        Ok(data) => Json(ApiResponse::success(data)).into_response(),
        Err(e) => {
            tracing::error!("DB error: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::<()>::error("서버 오류"))).into_response()
        }
    }
}

async fn visit_stats(
    State(state): State<AppState>,
    user: AuthUser,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::EligibilityCase, Action::List) {
        return e.into_response();
    }
    match sqlx::query_scalar::<_, serde_json::Value>(
        "SELECT COALESCE(json_agg(row_to_json(g)), '[]'::json) FROM gov_visit_stats g"
    ).fetch_one(&state.pool).await {
        Ok(data) => Json(ApiResponse::success(data)).into_response(),
        Err(e) => {
            tracing::error!("DB error: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::<()>::error("서버 오류"))).into_response()
        }
    }
}

async fn provider_compliance(
    State(state): State<AppState>,
    user: AuthUser,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Provider, Action::List) {
        return e.into_response();
    }
    match sqlx::query_scalar::<_, serde_json::Value>(
        "SELECT COALESCE(json_agg(row_to_json(g)), '[]'::json) FROM gov_provider_compliance g"
    ).fetch_one(&state.pool).await {
        Ok(data) => Json(ApiResponse::success(data)).into_response(),
        Err(e) => {
            tracing::error!("DB error: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::<()>::error("서버 오류"))).into_response()
        }
    }
}
