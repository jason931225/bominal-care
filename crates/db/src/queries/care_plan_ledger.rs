// =============================================================================
// Care Plan Ledger queries — append-only history
// =============================================================================

use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct CarePlanLedgerRow {
    pub id: Uuid,
    pub care_plan_id: Uuid,
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
    care_plan_id: Uuid,
    action: &str,
    actor_user_id: Uuid,
    actor_type: &str,
    data: serde_json::Value,
    reason: Option<&str>,
) -> Result<CarePlanLedgerRow, sqlx::Error> {
    sqlx::query_as::<_, CarePlanLedgerRow>(
        "INSERT INTO care_plan_ledger (care_plan_id, version, action, actor_user_id, actor_type, data, reason)
         VALUES ($1,
           COALESCE((SELECT MAX(version) FROM care_plan_ledger WHERE care_plan_id = $1), 0) + 1,
           $2, $3, $4, $5, $6)
         RETURNING *"
    )
    .bind(care_plan_id)
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
    care_plan_id: Uuid,
) -> Result<Option<CarePlanLedgerRow>, sqlx::Error> {
    sqlx::query_as::<_, CarePlanLedgerRow>(
        "SELECT * FROM care_plan_ledger
         WHERE care_plan_id = $1
         ORDER BY version DESC LIMIT 1"
    )
    .bind(care_plan_id)
    .fetch_optional(pool)
    .await
}

pub async fn get_history(
    pool: &PgPool,
    care_plan_id: Uuid,
) -> Result<Vec<CarePlanLedgerRow>, sqlx::Error> {
    sqlx::query_as::<_, CarePlanLedgerRow>(
        "SELECT * FROM care_plan_ledger
         WHERE care_plan_id = $1
         ORDER BY version ASC"
    )
    .bind(care_plan_id)
    .fetch_all(pool)
    .await
}

pub async fn get_version(
    pool: &PgPool,
    care_plan_id: Uuid,
    version: i32,
) -> Result<Option<CarePlanLedgerRow>, sqlx::Error> {
    sqlx::query_as::<_, CarePlanLedgerRow>(
        "SELECT * FROM care_plan_ledger
         WHERE care_plan_id = $1 AND version = $2"
    )
    .bind(care_plan_id)
    .bind(version)
    .fetch_optional(pool)
    .await
}
