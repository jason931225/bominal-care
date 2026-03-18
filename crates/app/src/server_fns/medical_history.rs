// =============================================================================
// Medical history server functions — MedicalHistoryEntry CRUD
// =============================================================================

use chrono::{DateTime, Utc};
use leptos::prelude::*;
use leptos::server;
use uuid::Uuid;

use bominal_db::queries::medical_history::{
    CreateMedicalHistoryEntryData, PaginatedEntries,
};
use bominal_types::models::MedicalHistoryEntry;

// -----------------------------------------------------------------------------
// list_medical_history — paginated list of medical history entries
// -----------------------------------------------------------------------------

#[server]
pub async fn list_medical_history(
    person_id: Uuid,
    page: i64,
    limit: i64,
) -> Result<PaginatedEntries, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let offset = (page - 1) * limit;

    bominal_db::queries::medical_history::list_entries(&pool, person_id, limit, offset)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

// -----------------------------------------------------------------------------
// create_medical_history — create a new medical history entry
// -----------------------------------------------------------------------------

#[server]
pub async fn create_medical_history(
    person_id: Uuid,
    condition: String,
    diagnosed_at: Option<DateTime<Utc>>,
    treated_by: Option<String>,
    status: Option<String>,
    notes: Option<String>,
    created_by: Option<Uuid>,
) -> Result<MedicalHistoryEntry, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let data = CreateMedicalHistoryEntryData {
        person_id,
        condition,
        diagnosed_at,
        treated_by,
        status,
        notes,
        created_by,
    };

    bominal_db::queries::medical_history::create_entry(&pool, &data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

// -----------------------------------------------------------------------------
// get_medical_history_entry — fetch a single medical history entry
// -----------------------------------------------------------------------------

#[server]
pub async fn get_medical_history_entry(
    id: Uuid,
) -> Result<Option<MedicalHistoryEntry>, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    bominal_db::queries::medical_history::get_entry(&pool, id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}
