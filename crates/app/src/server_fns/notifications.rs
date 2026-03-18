// =============================================================================
// Notification server functions — shared across all portals
// =============================================================================

use leptos::prelude::*;
use leptos::server;
use uuid::Uuid;

use bominal_db::queries::notification::{self, MarkAllReadResult, PaginatedNotifications};
use bominal_types::models::Notification;

/// Paginated list of notifications for a user.
#[server]
pub async fn list_notifications(
    user_id: Uuid,
    page: i64,
    limit: i64,
) -> Result<PaginatedNotifications, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let offset = (page.max(1) - 1) * limit.clamp(1, 100);

    notification::list_notifications(&pool, user_id, limit.clamp(1, 100), offset)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Get the count of unread notifications for a user.
#[server]
pub async fn get_unread_count(user_id: Uuid) -> Result<i64, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    notification::get_unread_count(&pool, user_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Mark a single notification as read.
#[server]
pub async fn mark_as_read(notification_id: Uuid) -> Result<Notification, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    notification::mark_as_read(&pool, notification_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Mark all notifications as read for a user.
#[server]
pub async fn mark_all_as_read(user_id: Uuid) -> Result<MarkAllReadResult, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    notification::mark_all_as_read(&pool, user_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}
