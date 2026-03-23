// =============================================================================
// Clinical Encounter Routes — /api/clinical
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
use bominal_db::queries::platform_event;
use bominal_types::medical::CreateEncounterInput;
use bominal_types::rbac::{Action, Resource};
use bominal_types::ApiResponse;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_encounters).post(create_encounter))
        .route("/{id}", get(get_encounter))
        .route("/{id}/sign", post(sign_encounter))
        .route("/{id}/addendum", post(add_addendum))
}

#[derive(serde::Deserialize)]
struct EncounterQuery {
    senior_id: Option<Uuid>,
}

/// POST /api/clinical
async fn create_encounter(
    State(state): State<AppState>,
    user: AuthUser,
    ValidatedJson(input): ValidatedJson<CreateEncounterInput>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::ClinicalEncounter, Action::Create) {
        return e.into_response();
    }

    match sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO clinical_encounters \
         (senior_person_id, provider_user_id, institution_id, subjective, objective, assessment, plan) \
         VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id",
    )
    .bind(input.senior_person_id)
    .bind(user.id)
    .bind(user.provider_id)
    .bind(&input.subjective)
    .bind(&input.objective)
    .bind(&input.assessment)
    .bind(&input.plan)
    .fetch_one(&state.pool)
    .await
    {
        Ok(id) => {
            let _ = platform_event::insert_event(
                &state.pool,
                Some(user.id),
                Some(&user.role.to_string()),
                None,
                "clinical_encounter",
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
            tracing::error!("DB error creating encounter: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}

/// POST /api/clinical/:id/sign
async fn sign_encounter(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::ClinicalEncounter, Action::Sign) {
        return e.into_response();
    }

    match sqlx::query(
        "UPDATE clinical_encounters SET is_signed = TRUE, signed_at = NOW() \
         WHERE id = $1 AND provider_user_id = $2 AND is_signed = FALSE",
    )
    .bind(id)
    .bind(user.id)
    .execute(&state.pool)
    .await
    {
        Ok(r) if r.rows_affected() > 0 => {
            let _ = platform_event::insert_event(
                &state.pool,
                Some(user.id),
                Some(&user.role.to_string()),
                None,
                "clinical_encounter",
                id,
                "signed",
                "phi",
                "care_operations",
                None, None, None, None, None,
            )
            .await;
            Json(ApiResponse::success(serde_json::json!({"signed": true}))).into_response()
        }
        Ok(_) => (
            StatusCode::CONFLICT,
            Json(ApiResponse::<()>::error(
                "진료 기록이 이미 서명되었거나 찾을 수 없습니다",
            )),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("DB error signing encounter: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}

/// Addendum input for inline deserialization.
#[derive(serde::Deserialize, validator::Validate)]
struct AddAddendumInput {
    #[validate(length(min = 1))]
    content: String,
}

/// POST /api/clinical/:id/addendum
async fn add_addendum(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
    ValidatedJson(input): ValidatedJson<AddAddendumInput>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::ClinicalEncounter, Action::Update) {
        return e.into_response();
    }

    match sqlx::query(
        "UPDATE clinical_encounters \
         SET addendum = $2, addendum_at = NOW() \
         WHERE id = $1 AND provider_user_id = $3 AND is_signed = TRUE",
    )
    .bind(id)
    .bind(&input.content)
    .bind(user.id)
    .execute(&state.pool)
    .await
    {
        Ok(r) if r.rows_affected() > 0 => {
            let _ = platform_event::insert_event(
                &state.pool,
                Some(user.id),
                Some(&user.role.to_string()),
                None,
                "clinical_encounter",
                id,
                "addendum_added",
                "phi",
                "care_operations",
                None, None, None, None, None,
            )
            .await;
            Json(ApiResponse::success(serde_json::json!({"addendum_added": true}))).into_response()
        }
        Ok(_) => (
            StatusCode::CONFLICT,
            Json(ApiResponse::<()>::error(
                "서명된 진료 기록만 추가 기록을 작성할 수 있습니다",
            )),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("DB error adding addendum: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}

/// GET /api/clinical?senior_id=...
async fn list_encounters(
    State(state): State<AppState>,
    user: AuthUser,
    Query(q): Query<EncounterQuery>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::ClinicalEncounter, Action::List) {
        return e.into_response();
    }

    let senior_id = q.senior_id.or(user.person_id);

    match sqlx::query_scalar::<_, serde_json::Value>(
        "SELECT COALESCE(json_agg(row_to_json(c)), '[]'::json) \
         FROM (SELECT * FROM clinical_encounters \
               WHERE ($1::UUID IS NULL OR senior_person_id = $1) \
               ORDER BY encounter_date DESC LIMIT 50) c",
    )
    .bind(senior_id)
    .fetch_one(&state.pool)
    .await
    {
        Ok(data) => Json(ApiResponse::success(data)).into_response(),
        Err(e) => {
            tracing::error!("DB error listing encounters: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}

/// GET /api/clinical/:id
async fn get_encounter(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::ClinicalEncounter, Action::Read) {
        return e.into_response();
    }

    match sqlx::query_scalar::<_, serde_json::Value>(
        "SELECT row_to_json(c) FROM clinical_encounters c WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await
    {
        Ok(Some(data)) => Json(ApiResponse::success(data)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<()>::error("진료 기록을 찾을 수 없습니다")),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("DB error fetching encounter: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}
