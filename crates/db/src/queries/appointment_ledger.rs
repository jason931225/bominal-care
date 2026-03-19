// =============================================================================
// Appointment Ledger queries — append-only history
// =============================================================================

use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct AppointmentLedgerRow {
    pub id: Uuid,
    pub appointment_id: Uuid,
    pub version: i32,
    pub action: String,
    pub actor_user_id: Uuid,
    pub actor_type: String,
    pub data: serde_json::Value,
    pub reason: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub async fn create_entry(
    pool: &PgPool,
    appointment_id: Uuid,
    action: &str,
    actor_user_id: Uuid,
    actor_type: &str,
    data: serde_json::Value,
    reason: Option<&str>,
) -> Result<AppointmentLedgerRow, sqlx::Error> {
    sqlx::query_as::<_, AppointmentLedgerRow>(
        "INSERT INTO appointment_ledger (appointment_id, version, action, actor_user_id, actor_type, data, reason)
         VALUES ($1,
           COALESCE((SELECT MAX(version) FROM appointment_ledger WHERE appointment_id = $1), 0) + 1,
           $2, $3, $4, $5, $6)
         RETURNING *"
    )
    .bind(appointment_id)
    .bind(action)
    .bind(actor_user_id)
    .bind(actor_type)
    .bind(data)
    .bind(reason)
    .fetch_one(pool)
    .await
}

pub async fn get_current(
    pool: &PgPool,
    appointment_id: Uuid,
) -> Result<Option<AppointmentLedgerRow>, sqlx::Error> {
    sqlx::query_as::<_, AppointmentLedgerRow>(
        "SELECT * FROM appointment_ledger
         WHERE appointment_id = $1
         ORDER BY version DESC LIMIT 1"
    )
    .bind(appointment_id)
    .fetch_optional(pool)
    .await
}

pub async fn get_history(
    pool: &PgPool,
    appointment_id: Uuid,
) -> Result<Vec<AppointmentLedgerRow>, sqlx::Error> {
    sqlx::query_as::<_, AppointmentLedgerRow>(
        "SELECT * FROM appointment_ledger
         WHERE appointment_id = $1
         ORDER BY version ASC"
    )
    .bind(appointment_id)
    .fetch_all(pool)
    .await
}

pub async fn get_version(
    pool: &PgPool,
    appointment_id: Uuid,
    version: i32,
) -> Result<Option<AppointmentLedgerRow>, sqlx::Error> {
    sqlx::query_as::<_, AppointmentLedgerRow>(
        "SELECT * FROM appointment_ledger
         WHERE appointment_id = $1 AND version = $2"
    )
    .bind(appointment_id)
    .bind(version)
    .fetch_optional(pool)
    .await
}
