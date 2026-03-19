// =============================================================================
// Dispensing Routes — /api/dispensing (pharmacy workflow)
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
use bominal_types::rbac::{Action, Resource};
use bominal_types::ApiResponse;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/queue", get(dispensing_queue))
        .route("/{prescription_id}/confirm", post(confirm_dispensing))
}

/// GET /api/dispensing/queue — list unsigned prescriptions for pharmacy staff
async fn dispensing_queue(
    State(state): State<AppState>,
    user: AuthUser,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Dispensing, Action::List) {
        return e.into_response();
    }

    match sqlx::query_scalar::<_, serde_json::Value>(
        "SELECT COALESCE(json_agg(row_to_json(p)), '[]'::json) \
         FROM (SELECT p.*, pp.korean_name AS senior_name \
               FROM prescriptions p \
               LEFT JOIN person_profiles pp ON pp.id = p.senior_person_id \
               WHERE p.is_signed = TRUE \
               AND NOT EXISTS ( \
                   SELECT 1 FROM dispensing_records dr \
                   WHERE dr.prescription_id = p.id \
               ) \
               ORDER BY p.created_at ASC LIMIT 50) p",
    )
    .fetch_one(&state.pool)
    .await
    {
        Ok(data) => Json(ApiResponse::success(data)).into_response(),
        Err(e) => {
            tracing::error!("DB error fetching dispensing queue: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}

/// Input for confirming a dispensing event.
#[derive(serde::Deserialize, validator::Validate)]
struct ConfirmDispensingInput {
    notes: Option<String>,
    quantity_dispensed: Option<i32>,
}

/// POST /api/dispensing/:prescription_id/confirm
async fn confirm_dispensing(
    State(state): State<AppState>,
    user: AuthUser,
    Path(prescription_id): Path<Uuid>,
    ValidatedJson(input): ValidatedJson<ConfirmDispensingInput>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Dispensing, Action::Create) {
        return e.into_response();
    }

    // Verify the prescription exists and is signed before dispensing
    let prescription_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM prescriptions WHERE id = $1 AND is_signed = TRUE)",
    )
    .bind(prescription_id)
    .fetch_one(&state.pool)
    .await;

    match prescription_exists {
        Ok(false) => {
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(ApiResponse::<()>::error(
                    "서명된 처방전만 조제할 수 있습니다",
                )),
            )
                .into_response();
        }
        Err(e) => {
            tracing::error!("DB error checking prescription: {e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response();
        }
        Ok(true) => {}
    }

    match sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO dispensing_records \
         (prescription_id, dispensed_by, notes, quantity_dispensed) \
         VALUES ($1, $2, $3, $4) RETURNING id",
    )
    .bind(prescription_id)
    .bind(user.id)
    .bind(&input.notes)
    .bind(input.quantity_dispensed)
    .fetch_one(&state.pool)
    .await
    {
        Ok(id) => (
            StatusCode::CREATED,
            Json(ApiResponse::success(serde_json::json!({"id": id, "dispensed": true}))),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("DB error confirming dispensing: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}
