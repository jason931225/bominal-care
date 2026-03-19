// =============================================================================
// Handoff Routes — /api/handoff (medical professional scoped access)
// =============================================================================

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use uuid::Uuid;

use crate::{
    auth::{extractor::AuthUser, permission::require_permission},
    middleware::validate::ValidatedJson,
    AppState,
};
use bominal_types::medical::StartHandoffInput;
use bominal_types::rbac::{Action, Resource};
use bominal_types::ApiResponse;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/start", post(start_handoff))
        .route("/end/{id}", post(end_handoff))
        .route("/active", get(active_session))
}

/// POST /api/handoff/start
async fn start_handoff(
    State(state): State<AppState>,
    user: AuthUser,
    ValidatedJson(input): ValidatedJson<StartHandoffInput>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Handoff, Action::Create) {
        return e.into_response();
    }

    match sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO medical_handoff_sessions \
         (senior_person_id, professional_user_id, license_type, license_number, institution_name, institution_id) \
         VALUES ($1, $2, $3, $4, $5, $6) RETURNING id",
    )
    .bind(input.senior_person_id)
    .bind(user.id)
    .bind(&input.license_type)
    .bind(&input.license_number)
    .bind(&input.institution_name)
    .bind(input.institution_id)
    .fetch_one(&state.pool)
    .await
    {
        Ok(id) => (
            StatusCode::CREATED,
            Json(ApiResponse::success(serde_json::json!({"id": id}))),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("DB error starting handoff: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}

/// POST /api/handoff/end/:id
async fn end_handoff(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Handoff, Action::Update) {
        return e.into_response();
    }

    match sqlx::query(
        "UPDATE medical_handoff_sessions \
         SET ended_at = NOW(), is_active = FALSE \
         WHERE id = $1 AND professional_user_id = $2",
    )
    .bind(id)
    .bind(user.id)
    .execute(&state.pool)
    .await
    {
        Ok(r) if r.rows_affected() > 0 => {
            Json(ApiResponse::success(serde_json::json!({"ended": true}))).into_response()
        }
        Ok(_) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<()>::error("핸드오프 세션을 찾을 수 없습니다")),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("DB error ending handoff: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}

/// GET /api/handoff/active
async fn active_session(
    State(state): State<AppState>,
    user: AuthUser,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Handoff, Action::Read) {
        return e.into_response();
    }

    match sqlx::query_scalar::<_, serde_json::Value>(
        "SELECT row_to_json(h) FROM medical_handoff_sessions h \
         WHERE professional_user_id = $1 AND is_active = TRUE AND expires_at > NOW() \
         ORDER BY started_at DESC LIMIT 1",
    )
    .bind(user.id)
    .fetch_optional(&state.pool)
    .await
    {
        Ok(data) => Json(ApiResponse::success(data)).into_response(),
        Err(e) => {
            tracing::error!("DB error fetching active handoff: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}
