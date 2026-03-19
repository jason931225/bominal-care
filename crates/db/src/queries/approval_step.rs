// =============================================================================
// Approval Step queries
// =============================================================================

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use bominal_types::ApprovalStep;

pub async fn create(
    pool: &PgPool,
    case_id: Uuid,
    step_name: &str,
    step_order: i32,
    assigned_to: Option<Uuid>,
) -> Result<ApprovalStep, sqlx::Error> {
    sqlx::query_as::<_, ApprovalStep>(
        "INSERT INTO approval_steps (case_id, step_name, step_order, status, assigned_to)
         VALUES ($1, $2, $3, 'pending', $4)
         RETURNING *"
    )
    .bind(case_id)
    .bind(step_name)
    .bind(step_order)
    .bind(assigned_to)
    .fetch_one(pool)
    .await
}

pub async fn update(
    pool: &PgPool,
    id: Uuid,
    status: &str,
    notes: Option<&str>,
    completed_at: Option<DateTime<Utc>>,
) -> Result<ApprovalStep, sqlx::Error> {
    sqlx::query_as::<_, ApprovalStep>(
        "UPDATE approval_steps SET
           status = $2,
           notes = COALESCE($3, notes),
           completed_at = COALESCE($4, completed_at),
           updated_at = NOW()
         WHERE id = $1
         RETURNING *"
    )
    .bind(id)
    .bind(status)
    .bind(notes)
    .bind(completed_at)
    .fetch_one(pool)
    .await
}

pub async fn list_for_case(
    pool: &PgPool,
    case_id: Uuid,
) -> Result<Vec<ApprovalStep>, sqlx::Error> {
    sqlx::query_as::<_, ApprovalStep>(
        "SELECT * FROM approval_steps
         WHERE case_id = $1
         ORDER BY step_order"
    )
    .bind(case_id)
    .fetch_all(pool)
    .await
}
