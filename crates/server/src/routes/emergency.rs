// =============================================================================
// Emergency Routes — /api/emergency
// =============================================================================

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, patch},
    Json, Router,
};
use uuid::Uuid;

use crate::{auth::{extractor::AuthUser, permission::require_permission}, AppState};
use bominal_types::ApiResponse;
use bominal_types::rbac::{Resource, Action};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/trigger", post(trigger))
        .route("/{id}/resolve", patch(resolve))
        .route("/recent", get(recent))
}

#[derive(serde::Deserialize)]
struct EmergencyTriggerInput {
    latitude: Option<f64>,
    longitude: Option<f64>,
}

async fn trigger(
    State(state): State<AppState>,
    user: AuthUser,
    Json(input): Json<EmergencyTriggerInput>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::EmergencyEvent, Action::Create) {
        return e.into_response();
    }
    let person_id = user.person_id.unwrap_or(user.id);
    match sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO emergency_events (person_id, latitude, longitude, triggered_by)
         VALUES ($1, $2, $3, $4) RETURNING id"
    )
    .bind(person_id)
    .bind(input.latitude)
    .bind(input.longitude)
    .bind(user.id)
    .fetch_one(&state.pool)
    .await {
        Ok(id) => {
            // Emit CRITICAL platform event
            let _ = bominal_db::queries::platform_event::insert_event(
                &state.pool, Some(user.id), Some(&user.role.to_string()), None,
                "emergency_event", id, "triggered", "restricted", "senior_safety",
                None, None,
                Some(serde_json::json!({"latitude": input.latitude, "longitude": input.longitude})),
                None, None
            ).await;
            (StatusCode::CREATED, Json(ApiResponse::success(serde_json::json!({"id": id})))).into_response()
        }
        Err(e) => {
            tracing::error!("DB error: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::<()>::error("서버 오류"))).into_response()
        }
    }
}

#[derive(serde::Deserialize)]
struct ResolveInput {
    resolution_notes: Option<String>,
    is_false_alarm: Option<bool>,
}

async fn resolve(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
    Json(input): Json<ResolveInput>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::EmergencyEvent, Action::Update) {
        return e.into_response();
    }
    let status = if input.is_false_alarm.unwrap_or(false) { "false_alarm" } else { "resolved" };
    match sqlx::query(
        "UPDATE emergency_events SET status = $2::emergency_event_status, resolved_by = $3, resolved_at = NOW(), resolution_notes = $4, updated_at = NOW() WHERE id = $1"
    )
    .bind(id)
    .bind(status)
    .bind(user.id)
    .bind(&input.resolution_notes)
    .execute(&state.pool)
    .await {
        Ok(_) => Json(ApiResponse::success(serde_json::json!({"status": status}))).into_response(),
        Err(e) => {
            tracing::error!("DB error: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::<()>::error("서버 오류"))).into_response()
        }
    }
}

async fn recent(
    State(state): State<AppState>,
    user: AuthUser,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::EmergencyEvent, Action::List) {
        return e.into_response();
    }
    match sqlx::query_scalar::<_, serde_json::Value>(
        "SELECT COALESCE(json_agg(row_to_json(e)), '[]'::json)
         FROM (SELECT * FROM emergency_events ORDER BY created_at DESC LIMIT 20) e"
    )
    .fetch_one(&state.pool)
    .await {
        Ok(data) => Json(ApiResponse::success(data)).into_response(),
        Err(e) => {
            tracing::error!("DB error: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::<()>::error("서버 오류"))).into_response()
        }
    }
}
