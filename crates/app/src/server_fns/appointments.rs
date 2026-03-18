// =============================================================================
// Appointment server functions — Appointment CRUD
// =============================================================================

use chrono::{DateTime, Utc};
use leptos::prelude::*;
use leptos::server;
use uuid::Uuid;

use bominal_db::queries::appointment::{
    CreateAppointmentData, PaginatedAppointments, UpdateAppointmentData,
};
use bominal_types::enums::ProviderType;
use bominal_types::models::Appointment;

// -----------------------------------------------------------------------------
// list_appointments — paginated list of appointments for a person
// -----------------------------------------------------------------------------

#[server]
pub async fn list_appointments(
    person_id: Uuid,
    page: i64,
    limit: i64,
) -> Result<PaginatedAppointments, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let offset = (page - 1) * limit;

    bominal_db::queries::appointment::list_appointments(&pool, person_id, limit, offset)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

// -----------------------------------------------------------------------------
// create_appointment — create a new appointment
// -----------------------------------------------------------------------------

#[server]
pub async fn create_appointment(
    person_id: Uuid,
    institution_name: String,
    institution_type: Option<ProviderType>,
    appointment_date: DateTime<Utc>,
    purpose: Option<String>,
    notes: Option<String>,
    address: Option<String>,
    created_by: Option<Uuid>,
) -> Result<Appointment, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let data = CreateAppointmentData {
        person_id,
        institution_name,
        institution_type,
        appointment_date,
        purpose,
        notes,
        address,
        created_by,
    };

    bominal_db::queries::appointment::create_appointment(&pool, &data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

// -----------------------------------------------------------------------------
// get_appointment — get a single appointment
// -----------------------------------------------------------------------------

#[server]
pub async fn get_appointment(
    id: Uuid,
) -> Result<Option<Appointment>, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    bominal_db::queries::appointment::get_appointment(&pool, id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

// -----------------------------------------------------------------------------
// update_appointment — update an existing appointment
// -----------------------------------------------------------------------------

#[server]
pub async fn update_appointment(
    id: Uuid,
    institution_name: Option<String>,
    institution_type: Option<ProviderType>,
    appointment_date: Option<DateTime<Utc>>,
    purpose: Option<String>,
    notes: Option<String>,
    address: Option<String>,
    updated_by: Option<Uuid>,
) -> Result<Appointment, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let data = UpdateAppointmentData {
        institution_name,
        institution_type,
        appointment_date,
        purpose,
        notes,
        address,
        updated_by,
    };

    bominal_db::queries::appointment::update_appointment(&pool, id, &data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}
