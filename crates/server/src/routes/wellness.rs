// =============================================================================
// Wellness Routes — /api/wellness
// =============================================================================

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use uuid::Uuid;

use crate::{auth::{extractor::AuthUser, permission::require_permission}, AppState};
use bominal_types::ApiResponse;
use bominal_types::rbac::{Resource, Action};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/check-in", post(check_in))
        .route("/today", get(today))
        .route("/history/{person_id}", get(history))
}

#[derive(serde::Deserialize)]
struct WellnessCheckInInput {
    mood: String,
    pain_level: Option<i32>,
    notes: Option<String>,
}

async fn check_in(
    State(state): State<AppState>,
    user: AuthUser,
    Json(input): Json<WellnessCheckInInput>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::WellnessCheckin, Action::Create) {
        return e.into_response();
    }
    let person_id = user.person_id.unwrap_or(user.id);
    match sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO wellness_checkins (person_id, mood, pain_level, notes, checked_in_by)
         VALUES ($1, $2::wellness_mood, $3, $4, $5) RETURNING id"
    )
    .bind(person_id)
    .bind(&input.mood)
    .bind(input.pain_level)
    .bind(&input.notes)
    .bind(user.id)
    .fetch_one(&state.pool)
    .await {
        Ok(id) => (StatusCode::CREATED, Json(ApiResponse::success(serde_json::json!({"id": id})))).into_response(),
        Err(e) => {
            tracing::error!("DB error: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::<()>::error("서버 오류"))).into_response()
        }
    }
}

async fn today(
    State(state): State<AppState>,
    user: AuthUser,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::WellnessCheckin, Action::Read) {
        return e.into_response();
    }
    let person_id = user.person_id.unwrap_or(user.id);
    match sqlx::query_scalar::<_, serde_json::Value>(
        "SELECT row_to_json(w) FROM wellness_checkins w
         WHERE person_id = $1 AND created_at::date = CURRENT_DATE
         ORDER BY created_at DESC LIMIT 1"
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

async fn history(
    State(state): State<AppState>,
    user: AuthUser,
    Path(person_id): Path<Uuid>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::WellnessCheckin, Action::List) {
        return e.into_response();
    }
    match sqlx::query_scalar::<_, serde_json::Value>(
        "SELECT COALESCE(json_agg(row_to_json(w)), '[]'::json)
         FROM (SELECT * FROM wellness_checkins WHERE person_id = $1 ORDER BY created_at DESC LIMIT 30) w"
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
