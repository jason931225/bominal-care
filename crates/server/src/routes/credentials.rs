// =============================================================================
// Credential Routes — /api/credentials
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
        .route("/expiring", get(expiring_credentials))
}

async fn expiring_credentials(
    State(state): State<AppState>,
    user: AuthUser,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::CaregiverApplication, Action::List) {
        return e.into_response();
    }
    match sqlx::query_scalar::<_, serde_json::Value>(
        "SELECT COALESCE(json_agg(row_to_json(cc)), '[]'::json)
         FROM caregiver_credentials cc
         WHERE cc.expires_at IS NOT NULL AND cc.expires_at < NOW() + INTERVAL '90 days'
         AND cc.status = 'VERIFIED'"
    ).fetch_one(&state.pool).await {
        Ok(data) => Json(ApiResponse::success(data)).into_response(),
        Err(e) => {
            tracing::error!("DB error: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::<()>::error("서버 오류"))).into_response()
        }
    }
}
