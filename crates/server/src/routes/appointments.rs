// =============================================================================
// Appointment Routes — CRUD for /appointments
// =============================================================================

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use uuid::Uuid;

use crate::{auth::{extractor::AuthUser, permission::require_permission}, AppState};
use crate::middleware::validate::ValidatedJson;
use bominal_db::queries::{appointment, platform_event, profile};
use bominal_types::inputs::{AppointmentInput, UpdateAppointmentInput};
use bominal_types::rbac::{Resource, Action};
use bominal_types::{ApiResponse, PaginationMeta, PaginationParams};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_appointments).post(create_appointment))
        .route(
            "/{id}",
            get(get_appointment)
                .patch(update_appointment)
                .delete(delete_appointment),
        )
}

/// GET /api/appointments?page=1&limit=20
async fn list_appointments(
    State(state): State<AppState>,
    user: AuthUser,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Appointment, Action::List) {
        return e.into_response();
    }

    let person_id = match resolve_person_id(&state.pool, user.id).await {
        Ok(id) => id,
        Err(resp) => return resp.into_response(),
    };

    let params = PaginationParams::new(params.page, params.limit);

    match appointment::list_appointments(&state.pool, person_id, params.limit, params.offset())
        .await
    {
        Ok(result) => {
            let meta = PaginationMeta::new(result.total, params.page, params.limit);
            Json(ApiResponse::success_with_meta(result.data, meta)).into_response()
        }
        Err(e) => {
            tracing::error!("DB error listing appointments: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}

/// POST /api/appointments
async fn create_appointment(
    State(state): State<AppState>,
    user: AuthUser,
    ValidatedJson(input): ValidatedJson<AppointmentInput>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Appointment, Action::Create) {
        return e.into_response();
    }

    let data = appointment::CreateAppointmentData {
        person_id: input.person_id,
        institution_name: input.institution_name,
        institution_type: input.institution_type,
        appointment_date: input.appointment_date,
        purpose: input.purpose,
        notes: input.notes,
        address: input.address,
        created_by: Some(user.id),
    };

    match appointment::create_appointment(&state.pool, &data).await {
        Ok(created) => {
            let _ = platform_event::insert_event(
                &state.pool,
                Some(user.id),
                Some(&user.role.to_string()),
                None,
                "appointment",
                created.id,
                "created",
                "internal",
                "care_operations",
                None,
                None,
                None,
                None,
                None,
            )
            .await;
            (StatusCode::CREATED, Json(ApiResponse::success(created))).into_response()
        }
        Err(e) => {
            tracing::error!("DB error creating appointment: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}

/// GET /api/appointments/:id
async fn get_appointment(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Appointment, Action::Read) {
        return e.into_response();
    }

    match appointment::get_appointment(&state.pool, id).await {
        Ok(Some(data)) => Json(ApiResponse::success(data)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<()>::error("예약을 찾을 수 없습니다")),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("DB error fetching appointment: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}

/// PATCH /api/appointments/:id
async fn update_appointment(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
    ValidatedJson(input): ValidatedJson<UpdateAppointmentInput>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Appointment, Action::Update) {
        return e.into_response();
    }

    let data = appointment::UpdateAppointmentData {
        institution_name: input.institution_name,
        institution_type: input.institution_type,
        appointment_date: input.appointment_date,
        purpose: input.purpose,
        notes: input.notes,
        address: input.address,
        updated_by: None,
    };

    match appointment::update_appointment(&state.pool, id, &data).await {
        Ok(updated) => {
            let _ = platform_event::insert_event(
                &state.pool,
                Some(user.id),
                Some(&user.role.to_string()),
                None,
                "appointment",
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
            tracing::error!("DB error updating appointment: {e}");
            let (status, msg) = match e {
                sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "예약을 찾을 수 없습니다"),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, "서버 오류"),
            };
            (
                status,
                Json(ApiResponse::<()>::error(msg)),
            )
                .into_response()
        }
    }
}

/// DELETE /api/appointments/:id — cancels (soft-delete via status)
async fn delete_appointment(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Appointment, Action::Delete) {
        return e.into_response();
    }

    match appointment::cancel_appointment(&state.pool, id).await {
        Ok(data) => {
            let _ = platform_event::insert_event(
                &state.pool,
                Some(user.id),
                Some(&user.role.to_string()),
                None,
                "appointment",
                id,
                "cancelled",
                "internal",
                "care_operations",
                None,
                None,
                None,
                None,
                None,
            )
            .await;
            Json(ApiResponse::success(data)).into_response()
        }
        Err(e) => {
            tracing::error!("DB error cancelling appointment: {e}");
            let (status, msg) = match e {
                sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "예약을 찾을 수 없습니다"),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, "서버 오류"),
            };
            (
                status,
                Json(ApiResponse::<()>::error(msg)),
            )
                .into_response()
        }
    }
}

async fn resolve_person_id(
    pool: &sqlx::PgPool,
    user_id: Uuid,
) -> Result<Uuid, (StatusCode, Json<ApiResponse<()>>)> {
    match profile::get_person_profile_by_user_id(pool, user_id).await {
        Ok(Some(p)) => Ok(p.id),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<()>::error("프로필을 찾을 수 없습니다")),
        )),
        Err(e) => {
            tracing::error!("DB error resolving person_id: {e}");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            ))
        }
    }
}
