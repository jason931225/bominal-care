// =============================================================================
// Notification Routes — CRUD for /notifications
// =============================================================================

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch},
    Json, Router,
};
use uuid::Uuid;

use crate::{auth::{extractor::AuthUser, permission::require_permission}, AppState};
use crate::middleware::validate::ValidatedJson;
use bominal_db::queries::notification;
use bominal_types::inputs::NotificationInput;
use bominal_types::{ApiResponse, PaginationMeta, PaginationParams};
use bominal_types::rbac::{Resource, Action};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_notifications).post(create_notification))
        .route("/unread-count", get(get_unread_count))
        .route("/{id}/read", patch(mark_as_read))
}

/// GET /api/notifications?page=1&limit=20
async fn list_notifications(
    State(state): State<AppState>,
    user: AuthUser,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Notification, Action::List) {
        return e.into_response();
    }

    let params = PaginationParams::new(params.page, params.limit);

    match notification::list_notifications(&state.pool, user.id, params.limit, params.offset())
        .await
    {
        Ok(result) => {
            let meta = PaginationMeta::new(result.total, params.page, params.limit);
            Json(ApiResponse::success_with_meta(result.data, meta)).into_response()
        }
        Err(e) => {
            tracing::error!("DB error listing notifications: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}

/// GET /api/notifications/unread-count
async fn get_unread_count(
    State(state): State<AppState>,
    user: AuthUser,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Notification, Action::Read) {
        return e.into_response();
    }

    match notification::get_unread_count(&state.pool, user.id).await {
        Ok(count) => Json(ApiResponse::success(serde_json::json!({ "count": count }))).into_response(),
        Err(e) => {
            tracing::error!("DB error fetching unread count: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}

/// POST /api/notifications
async fn create_notification(
    State(state): State<AppState>,
    user: AuthUser,
    ValidatedJson(input): ValidatedJson<NotificationInput>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Notification, Action::Create) {
        return e.into_response();
    }

    let data = notification::CreateNotificationData {
        user_id: input.user_id,
        notification_type: Some(input.notification_type),
        title: input.title,
        message: input.message,
        link: input.link,
    };

    match notification::create_notification(&state.pool, &data).await {
        Ok(created) => (StatusCode::CREATED, Json(ApiResponse::success(created))).into_response(),
        Err(e) => {
            tracing::error!("DB error creating notification: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}

/// PATCH /api/notifications/:id/read
async fn mark_as_read(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Notification, Action::Update) {
        return e.into_response();
    }

    match notification::mark_as_read(&state.pool, id).await {
        Ok(data) => Json(ApiResponse::success(data)).into_response(),
        Err(e) => {
            tracing::error!("DB error marking notification as read: {e}");
            let (status, msg) = match e {
                sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "알림을 찾을 수 없습니다"),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, "서버 오류"),
            };
            (status, Json(ApiResponse::<()>::error(msg))).into_response()
        }
    }
}
