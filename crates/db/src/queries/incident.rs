// =============================================================================
// Incident queries — CRUD + severity escalation
// =============================================================================

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use bominal_types::{Incident, IncidentSeverity};

pub async fn create_incident(
    pool: &PgPool,
    visit_id: Option<Uuid>,
    reported_by: Uuid,
    severity: IncidentSeverity,
    title: &str,
    description: &str,
    occurred_at: DateTime<Utc>,
) -> Result<Incident, sqlx::Error> {
    sqlx::query_as::<_, Incident>(
        "INSERT INTO incidents (visit_id, reported_by, severity, title, description, occurred_at)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING *"
    )
    .bind(visit_id)
    .bind(reported_by)
    .bind(severity)
    .bind(title)
    .bind(description)
    .bind(occurred_at)
    .fetch_one(pool)
    .await
}

pub async fn get_incident(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<Incident>, sqlx::Error> {
    sqlx::query_as::<_, Incident>(
        "SELECT * FROM incidents WHERE id = $1 AND deleted_at IS NULL"
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn list_incidents(
    pool: &PgPool,
    visit_id: Option<Uuid>,
    limit: i64,
    offset: i64,
) -> Result<(Vec<Incident>, i64), sqlx::Error> {
    let total = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM incidents
         WHERE deleted_at IS NULL AND ($1::UUID IS NULL OR visit_id = $1)"
    )
    .bind(visit_id)
    .fetch_one(pool)
    .await?;

    let rows = sqlx::query_as::<_, Incident>(
        "SELECT * FROM incidents
         WHERE deleted_at IS NULL AND ($1::UUID IS NULL OR visit_id = $1)
         ORDER BY occurred_at DESC
         LIMIT $2 OFFSET $3"
    )
    .bind(visit_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok((rows, total))
}

pub async fn update_incident(
    pool: &PgPool,
    id: Uuid,
    severity: Option<IncidentSeverity>,
    resolution: Option<&str>,
    resolved_at: Option<DateTime<Utc>>,
) -> Result<Incident, sqlx::Error> {
    sqlx::query_as::<_, Incident>(
        "UPDATE incidents SET
           severity = COALESCE($2, severity),
           resolution = COALESCE($3, resolution),
           resolved_at = COALESCE($4, resolved_at),
           updated_at = NOW()
         WHERE id = $1 AND deleted_at IS NULL
         RETURNING *"
    )
    .bind(id)
    .bind(severity)
    .bind(resolution)
    .bind(resolved_at)
    .fetch_one(pool)
    .await
}
