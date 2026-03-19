// =============================================================================
// Identity Verification queries
// =============================================================================

use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct IdentityVerificationRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub method: String,
    pub verified_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub verification_hash: String,
    pub is_valid: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub async fn create(
    pool: &PgPool,
    user_id: Uuid,
    method: &str,
    verification_hash: &str,
    expires_at: Option<chrono::DateTime<chrono::Utc>>,
) -> Result<IdentityVerificationRow, sqlx::Error> {
    sqlx::query_as::<_, IdentityVerificationRow>(
        "INSERT INTO identity_verifications (user_id, method, verification_hash, expires_at)
         VALUES ($1, $2, $3, $4)
         RETURNING *",
    )
    .bind(user_id)
    .bind(method)
    .bind(verification_hash)
    .bind(expires_at)
    .fetch_one(pool)
    .await
}

pub async fn get_latest(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Option<IdentityVerificationRow>, sqlx::Error> {
    sqlx::query_as::<_, IdentityVerificationRow>(
        "SELECT * FROM identity_verifications
         WHERE user_id = $1 AND is_valid = TRUE
         ORDER BY verified_at DESC LIMIT 1",
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn check_expired(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<bool, sqlx::Error> {
    let has_valid = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(
            SELECT 1 FROM identity_verifications
            WHERE user_id = $1 AND is_valid = TRUE
              AND (expires_at IS NULL OR expires_at > NOW())
        )",
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;
    Ok(!has_valid)
}
