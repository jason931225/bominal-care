// =============================================================================
// Availability Routes — GET/PUT /availability, GET/POST/DELETE /availability/exceptions
// =============================================================================

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get},
    Json, Router,
};
use chrono::{NaiveDate, NaiveTime};
use uuid::Uuid;

use crate::{
    auth::{extractor::AuthUser, permission::require_permission},
    AppState,
};
use bominal_db::queries::availability_slot;
use bominal_types::rbac::{Action, Resource};
use bominal_types::{ApiResponse, DayOfWeek};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_slots).put(replace_slots))
        .route("/exceptions", get(list_exceptions).post(create_exception))
        .route("/exceptions/{id}", delete(delete_exception))
}

// ---------------------------------------------------------------------------
// Input structs
// ---------------------------------------------------------------------------

#[derive(serde::Deserialize)]
struct SlotInput {
    day_of_week: DayOfWeek,
    start_time: String, // "09:00"
    end_time: String,   // "17:00"
}

#[derive(serde::Deserialize)]
struct ReplaceSlotBody {
    slots: Vec<SlotInput>,
}

#[derive(serde::Deserialize)]
struct CreateExceptionInput {
    exception_date: String, // "2026-04-02"
    is_available: bool,
    start_time: Option<String>,
    end_time: Option<String>,
    reason: Option<String>,
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// GET /api/availability — list the authenticated caregiver's weekly slots
async fn list_slots(
    State(state): State<AppState>,
    user: AuthUser,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::CaregiverApplication, Action::Read) {
        return e.into_response();
    }

    match availability_slot::list_for_user(&state.pool, user.id).await {
        Ok(data) => Json(ApiResponse::success(data)).into_response(),
        Err(e) => {
            tracing::error!("DB error listing availability slots: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}

/// PUT /api/availability — replace all weekly slots for the authenticated caregiver
async fn replace_slots(
    State(state): State<AppState>,
    user: AuthUser,
    Json(body): Json<ReplaceSlotBody>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::CaregiverApplication, Action::Update) {
        return e.into_response();
    }

    // Parse time strings to NaiveTime
    let mut slot_data = Vec::with_capacity(body.slots.len());
    for input in &body.slots {
        let start = match NaiveTime::parse_from_str(&input.start_time, "%H:%M") {
            Ok(t) => t,
            Err(_) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse::<()>::error("시작 시간 형식이 올바르지 않습니다 (예: 09:00)")),
                )
                    .into_response();
            }
        };
        let end = match NaiveTime::parse_from_str(&input.end_time, "%H:%M") {
            Ok(t) => t,
            Err(_) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse::<()>::error("종료 시간 형식이 올바르지 않습니다 (예: 17:00)")),
                )
                    .into_response();
            }
        };
        if start >= end {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::<()>::error("시작 시간은 종료 시간보다 이전이어야 합니다")),
            )
                .into_response();
        }
        slot_data.push(availability_slot::SlotData {
            day_of_week: input.day_of_week,
            start_time: start,
            end_time: end,
        });
    }

    match availability_slot::replace_slots(&state.pool, user.id, &slot_data).await {
        Ok(data) => Json(ApiResponse::success(data)).into_response(),
        Err(e) => {
            tracing::error!("DB error replacing availability slots: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}

/// GET /api/availability/exceptions — list the authenticated caregiver's exceptions
async fn list_exceptions(
    State(state): State<AppState>,
    user: AuthUser,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::CaregiverApplication, Action::Read) {
        return e.into_response();
    }

    match availability_slot::list_exceptions(&state.pool, user.id).await {
        Ok(data) => Json(ApiResponse::success(data)).into_response(),
        Err(e) => {
            tracing::error!("DB error listing availability exceptions: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}

/// POST /api/availability/exceptions — create a date-based exception
async fn create_exception(
    State(state): State<AppState>,
    user: AuthUser,
    Json(input): Json<CreateExceptionInput>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::CaregiverApplication, Action::Update) {
        return e.into_response();
    }

    let exception_date = match NaiveDate::parse_from_str(&input.exception_date, "%Y-%m-%d") {
        Ok(d) => d,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::<()>::error("날짜 형식이 올바르지 않습니다 (예: 2026-04-02)")),
            )
                .into_response();
        }
    };

    let start_time = match input.start_time.as_deref() {
        Some(s) => match NaiveTime::parse_from_str(s, "%H:%M") {
            Ok(t) => Some(t),
            Err(_) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse::<()>::error("시작 시간 형식이 올바르지 않습니다 (예: 09:00)")),
                )
                    .into_response();
            }
        },
        None => None,
    };

    let end_time = match input.end_time.as_deref() {
        Some(s) => match NaiveTime::parse_from_str(s, "%H:%M") {
            Ok(t) => Some(t),
            Err(_) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse::<()>::error("종료 시간 형식이 올바르지 않습니다 (예: 17:00)")),
                )
                    .into_response();
            }
        },
        None => None,
    };

    match availability_slot::create_exception(
        &state.pool,
        user.id,
        exception_date,
        input.is_available,
        start_time,
        end_time,
        input.reason.as_deref(),
    )
    .await
    {
        Ok(data) => (StatusCode::CREATED, Json(ApiResponse::success(data))).into_response(),
        Err(e) => {
            tracing::error!("DB error creating availability exception: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}

/// DELETE /api/availability/exceptions/:id — remove an exception
async fn delete_exception(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::CaregiverApplication, Action::Delete) {
        return e.into_response();
    }

    match availability_slot::delete_exception(&state.pool, id, user.id).await {
        Ok(true) => Json(ApiResponse::success(serde_json::json!({"deleted": true}))).into_response(),
        Ok(false) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<()>::error("예외를 찾을 수 없거나 이미 삭제되었습니다")),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("DB error deleting availability exception: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}
