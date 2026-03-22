// =============================================================================
// Visit Routes — GET /visits, POST /visits/check-in, POST /visits/check-out
// =============================================================================

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

use crate::{auth::{extractor::AuthUser, permission::require_permission}, AppState};
use crate::middleware::validate::ValidatedJson;
use bominal_db::queries::{platform_event, visit};
use bominal_types::rbac::{Resource, Action};
use bominal_types::{ApiResponse, PaginationMeta, PaginationParams, VisitStatus};
use bominal_types::inputs::{VisitCheckIn, VisitCheckOut};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_visits))
        .route("/{id}/check-in", post(check_in))
        .route("/{id}/check-out", post(check_out))
}

/// Query parameters for visit filtering.
#[derive(Debug, Deserialize)]
struct VisitQueryParams {
    #[serde(flatten)]
    pagination: PaginationParams,
    caregiver_id: Option<Uuid>,
    care_plan_id: Option<Uuid>,
    status: Option<VisitStatus>,
    date_from: Option<DateTime<Utc>>,
    date_to: Option<DateTime<Utc>>,
}

/// GET /api/visits?page=1&limit=20&caregiver_id=...&status=...
async fn list_visits(
    State(state): State<AppState>,
    user: AuthUser,
    Query(query): Query<VisitQueryParams>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Visit, Action::List) {
        return e.into_response();
    }

    let params = PaginationParams::new(query.pagination.page, query.pagination.limit);

    let filters = visit::VisitFilters {
        caregiver_id: query.caregiver_id,
        care_plan_id: query.care_plan_id,
        status: query.status,
        date_from: query.date_from,
        date_to: query.date_to,
    };

    match visit::list_visits(&state.pool, &filters, params.limit, params.offset()).await {
        Ok(result) => {
            let meta = PaginationMeta::new(result.total, params.page, params.limit);
            Json(ApiResponse::success_with_meta(result.data, meta)).into_response()
        }
        Err(e) => {
            tracing::error!("DB error listing visits: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}

/// POST /api/visits/:id/check-in
async fn check_in(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
    ValidatedJson(input): ValidatedJson<VisitCheckIn>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Visit, Action::Update) {
        return e.into_response();
    }

    let data = visit::CheckInData {
        latitude: input.latitude,
        longitude: input.longitude,
    };

    match visit::check_in(&state.pool, id, &data).await {
        Ok(updated) => {
            let _ = platform_event::insert_event(
                &state.pool,
                Some(user.id),
                Some(&user.role.to_string()),
                None,
                "visit",
                id,
                "checked_in",
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
            tracing::error!("DB error checking in: {e}");
            let (status, msg) = match e {
                sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "방문을 찾을 수 없습니다"),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, "서버 오류"),
            };
            (status, Json(ApiResponse::<()>::error(msg))).into_response()
        }
    }
}

/// POST /api/visits/:id/check-out
async fn check_out(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
    ValidatedJson(input): ValidatedJson<VisitCheckOut>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Visit, Action::Update) {
        return e.into_response();
    }

    let data = visit::CheckOutData {
        latitude: input.latitude,
        longitude: input.longitude,
        notes: input.notes,
    };

    match visit::check_out(&state.pool, id, &data).await {
        Ok(updated) => {
            let _ = platform_event::insert_event(
                &state.pool,
                Some(user.id),
                Some(&user.role.to_string()),
                None,
                "visit",
                id,
                "checked_out",
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
            tracing::error!("DB error checking out: {e}");
            let (status, msg) = match e {
                sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "방문을 찾을 수 없습니다"),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, "서버 오류"),
            };
            (status, Json(ApiResponse::<()>::error(msg))).into_response()
        }
    }
}
