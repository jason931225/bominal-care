// =============================================================================
// Medication server functions — Medication CRUD, schedules, and events
// =============================================================================

use chrono::{DateTime, Utc};
use leptos::prelude::*;
use leptos::server;
use uuid::Uuid;

use bominal_db::queries::medication::{
    CreateMedicationData, MedicationWithSchedules, UpdateMedicationData,
};
use bominal_types::enums::{MedicationEventStatus, MedicationFrequency};
use bominal_types::models::{Medication, MedicationEvent};

// -----------------------------------------------------------------------------
// list_medications — active medications with schedules for a person
// -----------------------------------------------------------------------------

#[server]
pub async fn list_medications(
    person_id: Uuid,
) -> Result<Vec<MedicationWithSchedules>, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    bominal_db::queries::medication::list_medications(&pool, person_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

// -----------------------------------------------------------------------------
// create_medication — create a new medication
// -----------------------------------------------------------------------------

#[server]
pub async fn create_medication(
    person_id: Uuid,
    name: String,
    dosage: String,
    form: String,
    frequency: MedicationFrequency,
    prescribed_by: Option<String>,
    prescribed_at: Option<DateTime<Utc>>,
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
    side_effects: Option<String>,
    notes: Option<String>,
    created_by: Option<Uuid>,
) -> Result<Medication, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let data = CreateMedicationData {
        person_id,
        name,
        dosage,
        form,
        frequency,
        prescribed_by,
        prescribed_at,
        start_date,
        end_date,
        side_effects,
        notes,
        created_by,
    };

    bominal_db::queries::medication::create_medication(&pool, &data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

// -----------------------------------------------------------------------------
// get_medication — get single medication with schedules
// -----------------------------------------------------------------------------

#[server]
pub async fn get_medication(
    person_id: Uuid,
    medication_id: Uuid,
) -> Result<Option<MedicationWithSchedules>, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    // list_medications returns all active medications; filter to the requested one
    let all = bominal_db::queries::medication::list_medications(&pool, person_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let found = all
        .into_iter()
        .find(|m| m.medication.id == medication_id);

    Ok(found)
}

// -----------------------------------------------------------------------------
// update_medication — update an existing medication
// -----------------------------------------------------------------------------

#[server]
pub async fn update_medication(
    id: Uuid,
    name: Option<String>,
    dosage: Option<String>,
    form: Option<String>,
    frequency: Option<MedicationFrequency>,
    prescribed_by: Option<String>,
    prescribed_at: Option<DateTime<Utc>>,
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
    is_active: Option<bool>,
    side_effects: Option<String>,
    notes: Option<String>,
    updated_by: Option<Uuid>,
) -> Result<Medication, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let data = UpdateMedicationData {
        name,
        dosage,
        form,
        frequency,
        prescribed_by,
        prescribed_at,
        start_date,
        end_date,
        is_active,
        side_effects,
        notes,
        updated_by,
    };

    bominal_db::queries::medication::update_medication(&pool, id, &data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

// -----------------------------------------------------------------------------
// get_today_events — today's medication events for a person
// -----------------------------------------------------------------------------

#[server]
pub async fn get_today_events(
    person_id: Uuid,
) -> Result<Vec<MedicationEvent>, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    bominal_db::queries::medication::get_today_events(&pool, person_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

// -----------------------------------------------------------------------------
// update_event_status — update a medication event's status
// -----------------------------------------------------------------------------

#[server]
pub async fn update_event_status(
    event_id: Uuid,
    status: MedicationEventStatus,
    recorded_by: Option<Uuid>,
    notes: Option<String>,
) -> Result<MedicationEvent, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    bominal_db::queries::medication::update_event_status(
        &pool,
        event_id,
        status,
        recorded_by,
        notes.as_deref(),
    )
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))
}
