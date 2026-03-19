// =============================================================================
// Care Summary Routes — /api/care-summaries
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
        .route("/", get(list_summaries))
        .route("/today", get(today_summary))
}

#[derive(serde::Deserialize)]
struct SummaryQuery {
    senior_id: Option<Uuid>,
}

async fn list_summaries(
    State(state): State<AppState>,
    user: AuthUser,
    Query(query): Query<SummaryQuery>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::CarePlan, Action::Read) {
        return e.into_response();
    }
    let person_id = query.senior_id.or(user.person_id).unwrap_or(user.id);
    match sqlx::query_scalar::<_, serde_json::Value>(
        "SELECT COALESCE(json_agg(row_to_json(d)), '[]'::json)
         FROM (SELECT * FROM daily_care_summaries WHERE person_id = $1 ORDER BY summary_date DESC LIMIT 30) d"
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

async fn today_summary(
    State(state): State<AppState>,
    user: AuthUser,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::CarePlan, Action::Read) {
        return e.into_response();
    }
    let person_id = user.person_id.unwrap_or(user.id);
    match sqlx::query_scalar::<_, serde_json::Value>(
        "SELECT row_to_json(d) FROM daily_care_summaries d
         WHERE person_id = $1 AND summary_date = CURRENT_DATE"
    )
    .bind(person_id)
    .fetch_optional(&state.pool)
    .await {
        Ok(data) => Json(ApiResponse::success(data)).into_response(),
        Err(e) => {
            tracing::error!("DB error: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::<()>::error("서버 오류"))).into_response()
        }
    }
}
