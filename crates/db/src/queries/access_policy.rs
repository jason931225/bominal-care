// =============================================================================
// Access Policy queries — dynamic policy engine persistence
// =============================================================================

use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct AccessPolicyRow {
    pub id: Uuid,
    pub role: String,
    pub resource_type: String,
    pub action: String,
    pub scope: String,
    pub effect: String,
    pub conditions: Option<serde_json::Value>,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub async fn list_policies(
    pool: &PgPool,
    role: Option<&str>,
    resource_type: Option<&str>,
) -> Result<Vec<AccessPolicyRow>, sqlx::Error> {
    let rows = sqlx::query_as::<_, AccessPolicyRow>(
        "SELECT * FROM access_policies
         WHERE is_active = TRUE
           AND ($1::TEXT IS NULL OR role = $1)
           AND ($2::TEXT IS NULL OR resource_type = $2)
         ORDER BY role, resource_type, action"
    )
    .bind(role)
    .bind(resource_type)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn get_policy(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<AccessPolicyRow>, sqlx::Error> {
    sqlx::query_as::<_, AccessPolicyRow>(
        "SELECT * FROM access_policies WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn update_policy(
    pool: &PgPool,
    id: Uuid,
    new_effect: &str,
    changed_by: Uuid,
    reason: &str,
) -> Result<AccessPolicyRow, sqlx::Error> {
    // Get old effect for change log
    let old = sqlx::query_scalar::<_, String>(
        "SELECT effect FROM access_policies WHERE id = $1"
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    // Update policy
    let updated = sqlx::query_as::<_, AccessPolicyRow>(
        "UPDATE access_policies SET effect = $1, updated_at = NOW()
         WHERE id = $2 RETURNING *"
    )
    .bind(new_effect)
    .bind(id)
    .fetch_one(pool)
    .await?;

    // Log the change
    sqlx::query(
        "INSERT INTO policy_change_log (policy_id, changed_by, old_effect, new_effect, reason)
         VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(id)
    .bind(changed_by)
    .bind(&old)
    .bind(new_effect)
    .bind(reason)
    .execute(pool)
    .await?;

    Ok(updated)
}
