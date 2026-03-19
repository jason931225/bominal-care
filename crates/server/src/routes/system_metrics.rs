// =============================================================================
// System Metrics Routes — /api/system (platform admin only)
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
        .route("/metrics", get(metrics))
}

async fn metrics(
    State(state): State<AppState>,
    user: AuthUser,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::AccessPolicy, Action::Read) {
        return e.into_response();
    }
    let result = sqlx::query_as::<_, (i64, i64, i64, i64, i64)>(
        "SELECT
            (SELECT COUNT(*) FROM users WHERE is_active = TRUE) AS active_users,
            (SELECT COUNT(*) FROM visits WHERE status = 'IN_PROGRESS') AS active_visits,
            (SELECT COUNT(*) FROM platform_events WHERE created_at > NOW() - INTERVAL '1 hour') AS events_last_hour,
            (SELECT COUNT(*) FROM emergency_events WHERE status = 'triggered') AS active_emergencies,
            (SELECT COUNT(*) FROM medical_handoff_sessions WHERE is_active = TRUE AND expires_at > NOW()) AS active_handoffs"
    )
    .fetch_one(&state.pool)
    .await;

    match result {
        Ok((users, visits, events, emergencies, handoffs)) => {
            let data = serde_json::json!({
                "active_users": users,
                "active_visits": visits,
                "events_last_hour": events,
                "active_emergencies": emergencies,
                "active_handoffs": handoffs,
            });
            Json(ApiResponse::success(data)).into_response()
        }
        Err(e) => {
            tracing::error!("DB error: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::<()>::error("서버 오류"))).into_response()
        }
    }
}
