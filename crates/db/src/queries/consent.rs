// =============================================================================
// Consent queries — ConsentRecord management
// Ported from packages/db/src/services/consent.service.ts
// =============================================================================

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use bominal_types::ConsentPurpose;

// -----------------------------------------------------------------------------
// Input types
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantConsentData {
    pub subject_person_id: Uuid,
    pub purpose: ConsentPurpose,
    pub granted_by: Uuid,
    pub expires_at: Option<DateTime<Utc>>,
}

// -----------------------------------------------------------------------------
// Row type for consent queries
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ConsentRecordRow {
    pub id: Uuid,
    pub subject_person_id: Uuid,
    pub purpose: ConsentPurpose,
    pub granted_by: Uuid,
    pub is_active: bool,
    pub granted_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
}

// -----------------------------------------------------------------------------
// Service functions
// -----------------------------------------------------------------------------

pub async fn grant_consent(
    pool: &PgPool,
    data: &GrantConsentData,
) -> Result<ConsentRecordRow, sqlx::Error> {
    let id = Uuid::new_v4();
    let now = chrono::Utc::now();

    sqlx::query_as::<_, ConsentRecordRow>(
        "INSERT INTO consent_records (
           id, subject_person_id, purpose, granted_by, expires_at, is_active, granted_at
         ) VALUES ($1, $2, $3, $4, $5, true, $6)
         RETURNING *",
    )
    .bind(id)
    .bind(data.subject_person_id)
    .bind(data.purpose)
    .bind(data.granted_by)
    .bind(data.expires_at)
    .bind(now)
    .fetch_one(pool)
    .await
}

/// Marks a consent record as inactive. The `revoked_by` identity should be
/// recorded in an AuditLog entry by the caller -- the ConsentRecord schema has
/// no revoked_by column, so we surface the value only via the audit trail.
pub async fn revoke_consent(
    pool: &PgPool,
    id: Uuid,
    _revoked_by: Uuid,
) -> Result<ConsentRecordRow, sqlx::Error> {
    // Verify the record exists
    let exists: Option<(Uuid,)> =
        sqlx::query_as("SELECT id FROM consent_records WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await?;

    if exists.is_none() {
        return Err(sqlx::Error::RowNotFound);
    }

    sqlx::query_as::<_, ConsentRecordRow>(
        "UPDATE consent_records
         SET is_active = false, revoked_at = NOW()
         WHERE id = $1
         RETURNING *",
    )
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn get_active_consent(
    pool: &PgPool,
    subject_person_id: Uuid,
    purpose: ConsentPurpose,
) -> Result<Option<ConsentRecordRow>, sqlx::Error> {
    sqlx::query_as::<_, ConsentRecordRow>(
        "SELECT * FROM consent_records
         WHERE subject_person_id = $1
           AND purpose = $2
           AND is_active = true
           AND (expires_at IS NULL OR expires_at > NOW())
         ORDER BY granted_at DESC
         LIMIT 1",
    )
    .bind(subject_person_id)
    .bind(purpose)
    .fetch_optional(pool)
    .await
}

pub async fn has_consent(
    pool: &PgPool,
    subject_person_id: Uuid,
    purpose: ConsentPurpose,
) -> Result<bool, sqlx::Error> {
    let record = get_active_consent(pool, subject_person_id, purpose).await?;
    Ok(record.is_some())
}

pub async fn get_consents_for_person(
    pool: &PgPool,
    subject_person_id: Uuid,
) -> Result<Vec<ConsentRecordRow>, sqlx::Error> {
    sqlx::query_as::<_, ConsentRecordRow>(
        "SELECT * FROM consent_records
         WHERE subject_person_id = $1
         ORDER BY granted_at DESC",
    )
    .bind(subject_person_id)
    .fetch_all(pool)
    .await
}
