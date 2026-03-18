// =============================================================================
// Notification queries — ported from notification.service.ts
// =============================================================================

use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use bominal_types::enums::NotificationType;
use bominal_types::models::Notification;

// ---------------------------------------------------------------------------
// Input / output types
// ---------------------------------------------------------------------------

pub struct CreateNotificationData {
    pub user_id: Uuid,
    pub notification_type: Option<NotificationType>,
    pub title: String,
    pub message: String,
    pub link: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedNotifications {
    pub data: Vec<Notification>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkAllReadResult {
    pub count: u64,
}

// ---------------------------------------------------------------------------
// Queries
// ---------------------------------------------------------------------------

pub async fn create_notification(
    pool: &PgPool,
    data: &CreateNotificationData,
) -> Result<Notification, sqlx::Error> {
    let id = Uuid::new_v4();
    let now = Utc::now();
    let notification_type = data.notification_type.unwrap_or(NotificationType::Info);

    sqlx::query_as::<_, Notification>(
        "INSERT INTO notifications (
           id, user_id, type, title, message, link, is_read, created_at
         ) VALUES ($1,$2,$3,$4,$5,$6,false,$7)
         RETURNING *",
    )
    .bind(id)
    .bind(data.user_id)
    .bind(notification_type)
    .bind(&data.title)
    .bind(&data.message)
    .bind(&data.link)
    .bind(now)
    .fetch_one(pool)
    .await
}

pub async fn mark_as_read(
    pool: &PgPool,
    id: Uuid,
) -> Result<Notification, sqlx::Error> {
    // Verify existence
    sqlx::query_as::<_, Notification>(
        "SELECT * FROM notifications WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| sqlx::Error::RowNotFound)?;

    let now = Utc::now();

    sqlx::query_as::<_, Notification>(
        "UPDATE notifications
         SET is_read = true, read_at = $1
         WHERE id = $2
         RETURNING *",
    )
    .bind(now)
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn mark_all_as_read(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<MarkAllReadResult, sqlx::Error> {
    let now = Utc::now();

    let result = sqlx::query(
        "UPDATE notifications
         SET is_read = true, read_at = $1
         WHERE user_id = $2 AND is_read = false",
    )
    .bind(now)
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(MarkAllReadResult {
        count: result.rows_affected(),
    })
}

pub async fn get_unread_count(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<i64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM notifications WHERE user_id = $1 AND is_read = false",
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(row.0)
}

pub async fn list_notifications(
    pool: &PgPool,
    user_id: Uuid,
    limit: i64,
    offset: i64,
) -> Result<PaginatedNotifications, sqlx::Error> {
    let data = sqlx::query_as::<_, Notification>(
        "SELECT * FROM notifications WHERE user_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
    )
    .bind(user_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM notifications WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(PaginatedNotifications {
        data,
        total: total.0,
    })
}
