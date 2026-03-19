// =============================================================================
// Appointment queries — ported from appointment.service.ts
// =============================================================================

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use bominal_types::enums::{AppointmentStatus, ProviderType};
use bominal_types::models::Appointment;
use bominal_types::state_machines::appointment_machine;

// ---------------------------------------------------------------------------
// Input types
// ---------------------------------------------------------------------------

pub struct CreateAppointmentData {
    pub person_id: Uuid,
    pub institution_name: String,
    pub institution_type: Option<ProviderType>,
    pub appointment_date: DateTime<Utc>,
    pub purpose: Option<String>,
    pub notes: Option<String>,
    pub address: Option<String>,
    pub created_by: Option<Uuid>,
}

pub struct UpdateAppointmentData {
    pub institution_name: Option<String>,
    pub institution_type: Option<ProviderType>,
    pub appointment_date: Option<DateTime<Utc>>,
    pub purpose: Option<String>,
    pub notes: Option<String>,
    pub address: Option<String>,
    pub updated_by: Option<Uuid>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PaginatedAppointments {
    pub data: Vec<Appointment>,
    pub total: i64,
}

// ---------------------------------------------------------------------------
// Queries
// ---------------------------------------------------------------------------

pub async fn create_appointment(
    pool: &PgPool,
    data: &CreateAppointmentData,
) -> Result<Appointment, sqlx::Error> {
    let id = Uuid::new_v4();
    let now = Utc::now();

    sqlx::query_as::<_, Appointment>(
        "INSERT INTO appointments (
           id, person_id, institution_name, institution_type, appointment_date,
           status, purpose, notes, address, created_by, created_at, updated_at
         ) VALUES ($1,$2,$3,$4,$5,'SCHEDULED',$6,$7,$8,$9,$10,$10)
         RETURNING *",
    )
    .bind(id)
    .bind(data.person_id)
    .bind(&data.institution_name)
    .bind(data.institution_type)
    .bind(data.appointment_date)
    .bind(&data.purpose)
    .bind(&data.notes)
    .bind(&data.address)
    .bind(data.created_by)
    .bind(now)
    .fetch_one(pool)
    .await
}

pub async fn update_appointment(
    pool: &PgPool,
    id: Uuid,
    data: &UpdateAppointmentData,
) -> Result<Appointment, sqlx::Error> {
    // Verify existence
    let _existing = sqlx::query_as::<_, Appointment>(
        "SELECT * FROM appointments WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| {
        sqlx::Error::RowNotFound
    })?;

    let now = Utc::now();

    // Bind all optional fields; COALESCE keeps the existing value when NULL.
    sqlx::query_as::<_, Appointment>(
        "UPDATE appointments SET
           institution_name = COALESCE($1, institution_name),
           institution_type = COALESCE($2, institution_type),
           appointment_date = COALESCE($3, appointment_date),
           purpose = COALESCE($4, purpose),
           notes = COALESCE($5, notes),
           address = COALESCE($6, address),
           updated_by = COALESCE($7, updated_by),
           updated_at = $8
         WHERE id = $9
         RETURNING *",
    )
    .bind(&data.institution_name)
    .bind(data.institution_type)
    .bind(data.appointment_date)
    .bind(&data.purpose)
    .bind(&data.notes)
    .bind(&data.address)
    .bind(data.updated_by)
    .bind(now)
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn update_status(
    pool: &PgPool,
    id: Uuid,
    status: AppointmentStatus,
) -> Result<Appointment, sqlx::Error> {
    let existing = sqlx::query_as::<_, Appointment>(
        "SELECT * FROM appointments WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| sqlx::Error::RowNotFound)?;

    let machine = appointment_machine();
    if !machine.can_transition(existing.status, status) {
        return Err(sqlx::Error::Protocol(format!(
            "Invalid appointment status transition: {} -> {}",
            existing.status, status,
        )));
    }

    let now = Utc::now();

    sqlx::query_as::<_, Appointment>(
        "UPDATE appointments SET status = $1, updated_at = $2 WHERE id = $3 RETURNING *",
    )
    .bind(status)
    .bind(now)
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn cancel_appointment(
    pool: &PgPool,
    id: Uuid,
) -> Result<Appointment, sqlx::Error> {
    update_status(pool, id, AppointmentStatus::Cancelled).await
}

pub async fn get_appointment(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<Appointment>, sqlx::Error> {
    sqlx::query_as::<_, Appointment>(
        "SELECT * FROM appointments WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn list_appointments(
    pool: &PgPool,
    person_id: Uuid,
    limit: i64,
    offset: i64,
) -> Result<PaginatedAppointments, sqlx::Error> {
    let data = sqlx::query_as::<_, Appointment>(
        "SELECT * FROM appointments WHERE person_id = $1 ORDER BY appointment_date DESC LIMIT $2 OFFSET $3",
    )
    .bind(person_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM appointments WHERE person_id = $1",
    )
    .bind(person_id)
    .fetch_one(pool)
    .await?;

    Ok(PaginatedAppointments {
        data,
        total: total.0,
    })
}

pub async fn get_upcoming_appointments(
    pool: &PgPool,
    person_id: Uuid,
    limit: i64,
) -> Result<Vec<Appointment>, sqlx::Error> {
    let now = Utc::now();

    sqlx::query_as::<_, Appointment>(
        "SELECT * FROM appointments
         WHERE person_id = $1
           AND appointment_date >= $2
           AND status IN ('SCHEDULED', 'CONFIRMED')
         ORDER BY appointment_date ASC
         LIMIT $3",
    )
    .bind(person_id)
    .bind(now)
    .bind(limit)
    .fetch_all(pool)
    .await
}
