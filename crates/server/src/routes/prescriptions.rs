// =============================================================================
// Prescription Routes — /api/prescriptions
// =============================================================================

use axum::{
    extract::{Path, Query, State},
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
use bominal_types::medical::CreatePrescriptionInput;
use bominal_types::rbac::{Action, Resource};
use bominal_types::ApiResponse;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_prescriptions).post(create_prescription))
        .route("/{id}", get(get_prescription))
        .route("/{id}/sign", post(sign_prescription))
}

#[derive(serde::Deserialize)]
struct PrescriptionQuery {
    senior_id: Option<Uuid>,
}

/// GET /api/prescriptions?senior_id=...
async fn list_prescriptions(
    State(state): State<AppState>,
    user: AuthUser,
    Query(q): Query<PrescriptionQuery>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Prescription, Action::List) {
        return e.into_response();
    }

    let senior_id = q.senior_id.or(user.person_id);

    match sqlx::query_scalar::<_, serde_json::Value>(
        "SELECT COALESCE(json_agg(row_to_json(p)), '[]'::json) \
         FROM (SELECT * FROM prescriptions \
               WHERE ($1::UUID IS NULL OR senior_person_id = $1) \
               ORDER BY created_at DESC LIMIT 50) p",
    )
    .bind(senior_id)
    .fetch_one(&state.pool)
    .await
    {
        Ok(data) => Json(ApiResponse::success(data)).into_response(),
        Err(e) => {
            tracing::error!("DB error listing prescriptions: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}

/// POST /api/prescriptions
async fn create_prescription(
    State(state): State<AppState>,
    user: AuthUser,
    ValidatedJson(input): ValidatedJson<CreatePrescriptionInput>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Prescription, Action::Create) {
        return e.into_response();
    }

    match sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO prescriptions \
         (senior_person_id, prescribed_by, institution_id, medication_name, dosage, frequency, duration_days, instructions) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING id",
    )
    .bind(input.senior_person_id)
    .bind(user.id)
    .bind(user.provider_id)
    .bind(&input.medication_name)
    .bind(&input.dosage)
    .bind(&input.frequency)
    .bind(input.duration_days)
    .bind(&input.instructions)
    .fetch_one(&state.pool)
    .await
    {
        Ok(id) => (
            StatusCode::CREATED,
            Json(ApiResponse::success(serde_json::json!({"id": id}))),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("DB error creating prescription: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}

/// GET /api/prescriptions/:id
async fn get_prescription(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Prescription, Action::Read) {
        return e.into_response();
    }

    match sqlx::query_scalar::<_, serde_json::Value>(
        "SELECT row_to_json(p) FROM prescriptions p WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await
    {
        Ok(Some(data)) => Json(ApiResponse::success(data)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<()>::error("처방전을 찾을 수 없습니다")),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("DB error fetching prescription: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}

/// POST /api/prescriptions/:id/sign
async fn sign_prescription(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Prescription, Action::Sign) {
        return e.into_response();
    }

    match sqlx::query(
        "UPDATE prescriptions SET is_signed = TRUE, signed_at = NOW(), signed_by = $2 \
         WHERE id = $1 AND is_signed = FALSE",
    )
    .bind(id)
    .bind(user.id)
    .execute(&state.pool)
    .await
    {
        Ok(r) if r.rows_affected() > 0 => {
            Json(ApiResponse::success(serde_json::json!({"signed": true}))).into_response()
        }
        Ok(_) => (
            StatusCode::CONFLICT,
            Json(ApiResponse::<()>::error(
                "처방전이 이미 서명되었거나 찾을 수 없습니다",
            )),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("DB error signing prescription: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}
