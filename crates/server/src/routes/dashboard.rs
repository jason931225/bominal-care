// =============================================================================
// Dashboard Routes — /api/dashboard (provider-level aggregations)
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
        .route("/provider", get(provider_dashboard))
}

async fn provider_dashboard(
    State(state): State<AppState>,
    user: AuthUser,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Provider, Action::Read) {
        return e.into_response();
    }
    let provider_id = user.provider_id;
    let result = sqlx::query_as::<_, (i64, i64, i64, i64)>(
        "SELECT
            (SELECT COUNT(*) FROM care_plans WHERE ($1::UUID IS NULL OR provider_id = $1) AND status = 'ACTIVE') AS active_plans,
            (SELECT COUNT(*) FROM visits v JOIN care_plans cp ON cp.id = v.care_plan_id WHERE ($1::UUID IS NULL OR cp.provider_id = $1) AND v.status = 'SCHEDULED' AND v.scheduled_start::date = CURRENT_DATE) AS todays_visits,
            (SELECT COUNT(*) FROM caregiver_applications WHERE ($1::UUID IS NULL OR provider_id = $1) AND status IN ('APPROVED_PRIVATE_PAY', 'APPROVED_UNDER_PROVIDER')) AS active_caregivers,
            (SELECT COUNT(*) FROM incidents WHERE resolved_at IS NULL AND deleted_at IS NULL) AS open_incidents"
    )
    .bind(provider_id)
    .fetch_one(&state.pool)
    .await;

    match result {
        Ok((plans, visits, caregivers, incidents)) => {
            let data = serde_json::json!({
                "active_care_plans": plans,
                "todays_visits": visits,
                "active_caregivers": caregivers,
                "open_incidents": incidents,
            });
            Json(ApiResponse::success(data)).into_response()
        }
        Err(e) => {
            tracing::error!("DB error: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::<()>::error("서버 오류"))).into_response()
        }
    }
}
