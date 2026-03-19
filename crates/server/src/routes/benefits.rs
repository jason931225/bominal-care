// =============================================================================
// Benefits Routes — /api/benefits
// =============================================================================

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use uuid::Uuid;

use crate::{auth::{extractor::AuthUser, permission::require_permission}, AppState};
use bominal_types::ApiResponse;
use bominal_types::rbac::{Resource, Action};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/utilization", get(utilization))
}

#[derive(serde::Deserialize)]
struct BenefitQuery {
    senior_id: Option<Uuid>,
}

async fn utilization(
    State(state): State<AppState>,
    user: AuthUser,
    Query(q): Query<BenefitQuery>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::SeniorProfile, Action::Read) {
        return e.into_response();
    }
    let person_id = q.senior_id.or(user.person_id).unwrap_or(user.id);
    match sqlx::query_scalar::<_, serde_json::Value>(
        "SELECT COALESCE(json_agg(row_to_json(b)), '[]'::json)
         FROM benefit_utilization b WHERE senior_person_id = $1 ORDER BY period_start DESC"
    )
    .bind(person_id)
    .fetch_one(&state.pool)
    .await {
        Ok(data) => Json(ApiResponse::success(data)).into_response(),
        Err(e) => {
            tracing::error!("DB error: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::<()>::error("서버 오류"))).into_response()
        }
    }
}
