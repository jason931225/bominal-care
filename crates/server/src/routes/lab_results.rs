// =============================================================================
// Lab Result Routes — /api/lab-results
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
use bominal_types::medical::CreateLabResultInput;
use bominal_types::rbac::{Action, Resource};
use bominal_types::ApiResponse;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_lab_results).post(create_lab_result))
        .route("/{id}", get(get_lab_result))
        .route("/{id}/review", patch(review_lab_result))
}

#[derive(serde::Deserialize)]
struct LabResultQuery {
    senior_id: Option<Uuid>,
}

/// POST /api/lab-results
async fn create_lab_result(
    State(state): State<AppState>,
    user: AuthUser,
    ValidatedJson(input): ValidatedJson<CreateLabResultInput>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::LabResult, Action::Create) {
        return e.into_response();
    }

    match sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO lab_results \
         (senior_person_id, ordered_by, test_name, test_code, result_value, result_unit, reference_range, is_critical) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, COALESCE($8, FALSE)) RETURNING id",
    )
    .bind(input.senior_person_id)
    .bind(user.id)
    .bind(&input.test_name)
    .bind(&input.test_code)
    .bind(&input.result_value)
    .bind(&input.result_unit)
    .bind(&input.reference_range)
    .bind(input.is_critical)
    .fetch_one(&state.pool)
    .await
    {
        Ok(id) => (
            StatusCode::CREATED,
            Json(ApiResponse::success(serde_json::json!({"id": id}))),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("DB error creating lab result: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}

/// GET /api/lab-results?senior_id=...
async fn list_lab_results(
    State(state): State<AppState>,
    user: AuthUser,
    Query(q): Query<LabResultQuery>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::LabResult, Action::List) {
        return e.into_response();
    }

    let senior_id = q.senior_id.or(user.person_id);

    match sqlx::query_scalar::<_, serde_json::Value>(
        "SELECT COALESCE(json_agg(row_to_json(lr)), '[]'::json) \
         FROM (SELECT * FROM lab_results \
               WHERE ($1::UUID IS NULL OR senior_person_id = $1) \
               ORDER BY created_at DESC LIMIT 50) lr",
    )
    .bind(senior_id)
    .fetch_one(&state.pool)
    .await
    {
        Ok(data) => Json(ApiResponse::success(data)).into_response(),
        Err(e) => {
            tracing::error!("DB error listing lab results: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}

/// GET /api/lab-results/:id
async fn get_lab_result(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::LabResult, Action::Read) {
        return e.into_response();
    }

    match sqlx::query_scalar::<_, serde_json::Value>(
        "SELECT row_to_json(lr) FROM lab_results lr WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await
    {
        Ok(Some(data)) => Json(ApiResponse::success(data)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<()>::error("검사 결과를 찾을 수 없습니다")),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("DB error fetching lab result: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}

/// Review input for marking a lab result as reviewed.
#[derive(serde::Deserialize, validator::Validate)]
struct ReviewInput {
    notes: Option<String>,
}

/// PATCH /api/lab-results/:id/review
async fn review_lab_result(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
    ValidatedJson(input): ValidatedJson<ReviewInput>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::LabResult, Action::Update) {
        return e.into_response();
    }

    // We store the reviewer's notes as a side effect of marking reviewed;
    // if a notes column doesn't exist, we simply ignore it in the query.
    match sqlx::query(
        "UPDATE lab_results SET reviewed_by = $2, reviewed_at = NOW() \
         WHERE id = $1 AND reviewed_by IS NULL",
    )
    .bind(id)
    .bind(user.id)
    .execute(&state.pool)
    .await
    {
        Ok(r) if r.rows_affected() > 0 => {
            // Log review notes if provided
            if let Some(ref notes) = input.notes {
                tracing::info!(lab_result_id = %id, reviewer = %user.id, notes = %notes, "Lab result reviewed");
            }
            Json(ApiResponse::success(serde_json::json!({"reviewed": true}))).into_response()
        }
        Ok(_) => (
            StatusCode::CONFLICT,
            Json(ApiResponse::<()>::error(
                "검사 결과가 이미 검토되었거나 찾을 수 없습니다",
            )),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("DB error reviewing lab result: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}
