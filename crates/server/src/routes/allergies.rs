// =============================================================================
// Allergy Routes — /api/allergies (falls under MedicalHistory resource)
// =============================================================================

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch},
    Json, Router,
};
use uuid::Uuid;

use crate::{
    auth::{extractor::AuthUser, permission::require_permission},
    middleware::validate::ValidatedJson,
    AppState,
};
use bominal_db::queries::platform_event;
use bominal_types::medical::CreateAllergyInput;
use bominal_types::rbac::{Action, Resource};
use bominal_types::ApiResponse;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_allergies).post(create_allergy))
        .route("/{id}/deactivate", patch(deactivate_allergy))
}

#[derive(serde::Deserialize)]
struct AllergyQuery {
    senior_id: Option<Uuid>,
}

/// POST /api/allergies
async fn create_allergy(
    State(state): State<AppState>,
    user: AuthUser,
    ValidatedJson(input): ValidatedJson<CreateAllergyInput>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::MedicalHistory, Action::Create) {
        return e.into_response();
    }

    match sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO patient_allergies \
         (senior_person_id, allergen, reaction, severity, reported_by) \
         VALUES ($1, $2, $3, $4, $5) RETURNING id",
    )
    .bind(input.senior_person_id)
    .bind(&input.allergen)
    .bind(&input.reaction)
    .bind(&input.severity)
    .bind(user.id)
    .fetch_one(&state.pool)
    .await
    {
        Ok(id) => {
            let _ = platform_event::insert_event(
                &state.pool,
                Some(user.id),
                Some(&user.role.to_string()),
                None,
                "allergy",
                id,
                "created",
                "phi",
                "care_operations",
                None, None, None, None, None,
            )
            .await;
            (
                StatusCode::CREATED,
                Json(ApiResponse::success(serde_json::json!({"id": id}))),
            )
                .into_response()
        }
        Err(e) => {
            tracing::error!("DB error creating allergy: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}

/// GET /api/allergies?senior_id=... — returns active allergies only
async fn list_allergies(
    State(state): State<AppState>,
    user: AuthUser,
    Query(q): Query<AllergyQuery>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::MedicalHistory, Action::List) {
        return e.into_response();
    }

    let senior_id = q.senior_id.or(user.person_id);

    match sqlx::query_scalar::<_, serde_json::Value>(
        "SELECT COALESCE(json_agg(row_to_json(a)), '[]'::json) \
         FROM (SELECT * FROM patient_allergies \
               WHERE ($1::UUID IS NULL OR senior_person_id = $1) \
               AND is_active = TRUE \
               ORDER BY created_at DESC LIMIT 100) a",
    )
    .bind(senior_id)
    .fetch_one(&state.pool)
    .await
    {
        Ok(data) => Json(ApiResponse::success(data)).into_response(),
        Err(e) => {
            tracing::error!("DB error listing allergies: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}

/// PATCH /api/allergies/:id/deactivate
async fn deactivate_allergy(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::MedicalHistory, Action::Update) {
        return e.into_response();
    }

    match sqlx::query(
        "UPDATE patient_allergies SET is_active = FALSE, updated_at = NOW() \
         WHERE id = $1 AND is_active = TRUE",
    )
    .bind(id)
    .execute(&state.pool)
    .await
    {
        Ok(r) if r.rows_affected() > 0 => {
            let _ = platform_event::insert_event(
                &state.pool,
                Some(user.id),
                Some(&user.role.to_string()),
                None,
                "allergy",
                id,
                "deactivated",
                "phi",
                "care_operations",
                None, None, None, None, None,
            )
            .await;
            Json(ApiResponse::success(serde_json::json!({"deactivated": true}))).into_response()
        }
        Ok(_) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<()>::error(
                "알레르기를 찾을 수 없거나 이미 비활성화되었습니다",
            )),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("DB error deactivating allergy: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}
