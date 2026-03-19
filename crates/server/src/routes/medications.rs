// =============================================================================
// Medication Routes — CRUD for /medications
// =============================================================================

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use uuid::Uuid;

use crate::{auth::{extractor::AuthUser, permission::require_permission}, AppState};
use crate::middleware::validate::ValidatedJson;
use bominal_db::queries::{medication, platform_event, profile};
use bominal_types::inputs::UpdateMedicationInput;
use bominal_types::rbac::{Resource, Action};
use bominal_types::ApiResponse;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_medications))
        .route("/today", get(get_today_events))
        .route("/{id}", get(get_medication).patch(update_medication))
}

/// Resolve the person_id from the authenticated user's profile.
async fn resolve_person_id(
    pool: &sqlx::PgPool,
    user_id: Uuid,
) -> Result<Uuid, (StatusCode, Json<ApiResponse<()>>)> {
    match profile::get_person_profile_by_user_id(pool, user_id).await {
        Ok(Some(p)) => Ok(p.id),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<()>::error("Profile not found")),
        )),
        Err(e) => {
            tracing::error!("DB error resolving person_id: {e}");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("Internal server error")),
            ))
        }
    }
}

/// GET /api/medications
async fn list_medications(
    State(state): State<AppState>,
    user: AuthUser,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Medication, Action::List) {
        return e.into_response();
    }

    let person_id = match resolve_person_id(&state.pool, user.id).await {
        Ok(id) => id,
        Err(resp) => return resp.into_response(),
    };

    match medication::list_medications(&state.pool, person_id).await {
        Ok(data) => Json(ApiResponse::success(data)).into_response(),
        Err(e) => {
            tracing::error!("DB error listing medications: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("Internal server error")),
            )
                .into_response()
        }
    }
}

/// GET /api/medications/today
async fn get_today_events(
    State(state): State<AppState>,
    user: AuthUser,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Medication, Action::Read) {
        return e.into_response();
    }

    let person_id = match resolve_person_id(&state.pool, user.id).await {
        Ok(id) => id,
        Err(resp) => return resp.into_response(),
    };

    match medication::get_today_events(&state.pool, person_id).await {
        Ok(data) => Json(ApiResponse::success(data)).into_response(),
        Err(e) => {
            tracing::error!("DB error fetching today's med events: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("Internal server error")),
            )
                .into_response()
        }
    }
}

/// GET /api/medications/:id
async fn get_medication(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Medication, Action::Read) {
        return e.into_response();
    }

    match medication::get_medication(&state.pool, id).await {
        Ok(Some(data)) => Json(ApiResponse::success(data)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<()>::error("Medication not found")),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("DB error fetching medication: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("Internal server error")),
            )
                .into_response()
        }
    }
}

/// PATCH /api/medications/:id
async fn update_medication(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
    ValidatedJson(input): ValidatedJson<UpdateMedicationInput>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Medication, Action::Update) {
        return e.into_response();
    }

    let data = medication::UpdateMedicationData {
        name: input.name,
        dosage: input.dosage,
        form: input.form,
        frequency: input.frequency,
        prescribed_by: input.prescribed_by,
        prescribed_at: None,
        start_date: input.start_date,
        end_date: input.end_date,
        is_active: input.is_active,
        side_effects: input.side_effects,
        notes: input.notes,
        updated_by: None,
    };

    match medication::update_medication(&state.pool, id, &data).await {
        Ok(updated) => {
            let _ = platform_event::insert_event(
                &state.pool,
                Some(user.id),
                Some(&user.role.to_string()),
                None,
                "medication",
                id,
                "updated",
                "internal",
                "care_operations",
                None,
                None,
                None,
                None,
                None,
            )
            .await;
            Json(ApiResponse::success(updated)).into_response()
        }
        Err(e) => {
            tracing::error!("DB error updating medication: {e}");
            let status = match e {
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (
                status,
                Json(ApiResponse::<()>::error("Internal server error")),
            )
                .into_response()
        }
    }
}
