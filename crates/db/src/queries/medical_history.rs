// =============================================================================
// Medical History Queries
// Ported from packages/db/src/services/medical-history.service.ts (165 lines)
// =============================================================================

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use bominal_types::models::MedicalHistoryEntry;

// ---------------------------------------------------------------------------
// Input structs
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct CreateMedicalHistoryEntryData {
    pub person_id: Uuid,
    pub condition: String,
    pub diagnosed_at: Option<DateTime<Utc>>,
    pub treated_by: Option<String>,
    pub status: Option<String>,
    pub notes: Option<String>,
    pub created_by: Option<Uuid>,
}

#[derive(Debug, Clone, Default)]
pub struct UpdateMedicalHistoryEntryData {
    pub condition: Option<String>,
    pub diagnosed_at: Option<DateTime<Utc>>,
    pub treated_by: Option<String>,
    pub status: Option<String>,
    pub notes: Option<String>,
    pub updated_by: Option<Uuid>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PaginatedEntries {
    pub data: Vec<MedicalHistoryEntry>,
    pub total: i64,
}

// ---------------------------------------------------------------------------
// create_entry
// ---------------------------------------------------------------------------

pub async fn create_entry(
    pool: &PgPool,
    data: &CreateMedicalHistoryEntryData,
) -> Result<MedicalHistoryEntry, sqlx::Error> {
    let id = Uuid::new_v4();
    let now = Utc::now();
    let status = data.status.as_deref().unwrap_or("active");

    sqlx::query_as::<_, MedicalHistoryEntry>(
        "INSERT INTO medical_history_entries (
           id, person_id, condition, diagnosed_at, treated_by,
           status, notes, created_by, created_at, updated_at
         ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $9)
         RETURNING *",
    )
    .bind(id)
    .bind(data.person_id)
    .bind(&data.condition)
    .bind(data.diagnosed_at)
    .bind(&data.treated_by)
    .bind(status)
    .bind(&data.notes)
    .bind(data.created_by)
    .bind(now)
    .fetch_one(pool)
    .await
}

// ---------------------------------------------------------------------------
// update_entry
// ---------------------------------------------------------------------------

pub async fn update_entry(
    pool: &PgPool,
    id: Uuid,
    data: &UpdateMedicalHistoryEntryData,
) -> Result<MedicalHistoryEntry, sqlx::Error> {
    // Verify existence
    sqlx::query_as::<_, MedicalHistoryEntry>(
        "SELECT * FROM medical_history_entries WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| sqlx::Error::RowNotFound)?;

    let now = Utc::now();

    sqlx::query_as::<_, MedicalHistoryEntry>(
        "UPDATE medical_history_entries SET
           condition    = COALESCE($1, condition),
           diagnosed_at = COALESCE($2, diagnosed_at),
           treated_by   = COALESCE($3, treated_by),
           status       = COALESCE($4, status),
           notes        = COALESCE($5, notes),
           updated_by   = COALESCE($6, updated_by),
           updated_at   = $7
         WHERE id = $8
         RETURNING *",
    )
    .bind(&data.condition)
    .bind(data.diagnosed_at)
    .bind(&data.treated_by)
    .bind(&data.status)
    .bind(&data.notes)
    .bind(data.updated_by)
    .bind(now)
    .bind(id)
    .fetch_one(pool)
    .await
}

// ---------------------------------------------------------------------------
// get_entry
// ---------------------------------------------------------------------------

pub async fn get_entry(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<MedicalHistoryEntry>, sqlx::Error> {
    sqlx::query_as::<_, MedicalHistoryEntry>(
        "SELECT * FROM medical_history_entries WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

// ---------------------------------------------------------------------------
// list_entries (paginated)
// ---------------------------------------------------------------------------

pub async fn list_entries(
    pool: &PgPool,
    person_id: Uuid,
    limit: i64,
    offset: i64,
) -> Result<PaginatedEntries, sqlx::Error> {
    let data = sqlx::query_as::<_, MedicalHistoryEntry>(
        "SELECT * FROM medical_history_entries
         WHERE person_id = $1
         ORDER BY diagnosed_at DESC
         LIMIT $2 OFFSET $3",
    )
    .bind(person_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    let (total,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM medical_history_entries WHERE person_id = $1",
    )
    .bind(person_id)
    .fetch_one(pool)
    .await?;

    Ok(PaginatedEntries { data, total })
}

// ---------------------------------------------------------------------------
// get_active_conditions
// ---------------------------------------------------------------------------

pub async fn get_active_conditions(
    pool: &PgPool,
    person_id: Uuid,
) -> Result<Vec<MedicalHistoryEntry>, sqlx::Error> {
    sqlx::query_as::<_, MedicalHistoryEntry>(
        "SELECT * FROM medical_history_entries
         WHERE person_id = $1
           AND status IN ('active', 'chronic')
         ORDER BY diagnosed_at DESC",
    )
    .bind(person_id)
    .fetch_all(pool)
    .await
}
