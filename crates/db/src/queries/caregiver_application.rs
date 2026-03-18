// =============================================================================
// Caregiver Application Queries
// Ported from packages/db/src/services/caregiver-application.service.ts
// =============================================================================

use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use bominal_types::enums::{CaregiverApplicationStatus, CredentialType, Gender};
use bominal_types::models::{
    AvailabilitySlot, CaregiverApplication, CaregiverCredential, ServiceType,
};
use bominal_types::state_machines::caregiver_application_machine;

// ---------------------------------------------------------------------------
// Input structs
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct CreateApplicationData {
    pub provider_id: Option<Uuid>,
    pub experience_years: Option<i32>,
    pub bio: Option<String>,
    pub specializations: Option<String>,
    pub has_dementia_experience: bool,
    pub has_overnight_availability: bool,
    pub smoking_status: bool,
    pub pet_friendly: bool,
    pub preferred_gender: Option<Gender>,
    pub languages_spoken: String,
}

impl Default for CreateApplicationData {
    fn default() -> Self {
        Self {
            provider_id: None,
            experience_years: None,
            bio: None,
            specializations: None,
            has_dementia_experience: false,
            has_overnight_availability: false,
            smoking_status: false,
            pet_friendly: true,
            preferred_gender: None,
            languages_spoken: "ko".to_string(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct UpdateApplicationData {
    pub provider_id: Option<Uuid>,
    pub experience_years: Option<i32>,
    pub bio: Option<String>,
    pub specializations: Option<String>,
    pub has_dementia_experience: Option<bool>,
    pub has_overnight_availability: Option<bool>,
    pub smoking_status: Option<bool>,
    pub pet_friendly: Option<bool>,
    pub preferred_gender: Option<Option<Gender>>,
    pub languages_spoken: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ApplicationFilters {
    pub status: Option<CaregiverApplicationStatus>,
    pub provider_id: Option<Uuid>,
}

#[derive(Debug, Clone)]
pub struct AddCredentialData {
    pub credential_type: CredentialType,
    pub issuer: Option<String>,
    pub issued_at: Option<chrono::DateTime<Utc>>,
    pub expires_at: Option<chrono::DateTime<Utc>>,
    pub document_url: Option<String>,
}

// ---------------------------------------------------------------------------
// Result structs
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct PaginatedApplications {
    pub data: Vec<CaregiverApplication>,
    pub total: i64,
}

#[derive(Debug, Clone)]
pub struct ApplicationWithRelations {
    pub application: CaregiverApplication,
    pub credentials: Vec<CaregiverCredential>,
    pub availability_slots: Vec<AvailabilitySlot>,
    pub service_types: Vec<ServiceType>,
}

// ---------------------------------------------------------------------------
// Queries
// ---------------------------------------------------------------------------

pub async fn create_application(
    pool: &PgPool,
    user_id: Uuid,
    data: &CreateApplicationData,
) -> Result<CaregiverApplication, sqlx::Error> {
    let id = Uuid::new_v4();
    let now = Utc::now();

    sqlx::query_as::<_, CaregiverApplication>(
        r#"INSERT INTO caregiver_applications (
             id, user_id, status, provider_id, experience_years, bio, specializations,
             has_dementia_experience, has_overnight_availability, smoking_status,
             pet_friendly, preferred_gender, languages_spoken, created_at, updated_at
           ) VALUES ($1,$2,'DRAFT',$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$13)
           RETURNING *"#,
    )
    .bind(id)
    .bind(user_id)
    .bind(data.provider_id)
    .bind(data.experience_years)
    .bind(&data.bio)
    .bind(&data.specializations)
    .bind(data.has_dementia_experience)
    .bind(data.has_overnight_availability)
    .bind(data.smoking_status)
    .bind(data.pet_friendly)
    .bind(data.preferred_gender)
    .bind(&data.languages_spoken)
    .bind(now)
    .fetch_one(pool)
    .await
}

pub async fn update_application(
    pool: &PgPool,
    id: Uuid,
    data: &UpdateApplicationData,
) -> Result<CaregiverApplication, sqlx::Error> {
    // Verify existence
    let _existing = sqlx::query_as::<_, CaregiverApplication>(
        "SELECT * FROM caregiver_applications WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or(sqlx::Error::RowNotFound)?;

    // Fixed-shape UPDATE with COALESCE: each field is only overwritten when
    // the caller supplies a Some value. For preferred_gender (Option<Option<Gender>>),
    // a boolean flag ($9) distinguishes "not provided" from "set to NULL".
    sqlx::query_as::<_, CaregiverApplication>(
        r#"UPDATE caregiver_applications SET
             provider_id       = COALESCE($1, provider_id),
             experience_years  = COALESCE($2, experience_years),
             bio               = COALESCE($3, bio),
             specializations   = COALESCE($4, specializations),
             has_dementia_experience   = COALESCE($5, has_dementia_experience),
             has_overnight_availability = COALESCE($6, has_overnight_availability),
             smoking_status    = COALESCE($7, smoking_status),
             pet_friendly      = COALESCE($8, pet_friendly),
             preferred_gender  = CASE WHEN $9 THEN $10 ELSE preferred_gender END,
             languages_spoken  = COALESCE($11, languages_spoken),
             updated_at        = NOW()
           WHERE id = $12
           RETURNING *"#,
    )
    .bind(data.provider_id)
    .bind(data.experience_years)
    .bind(&data.bio)
    .bind(&data.specializations)
    .bind(data.has_dementia_experience)
    .bind(data.has_overnight_availability)
    .bind(data.smoking_status)
    .bind(data.pet_friendly)
    .bind(data.preferred_gender.is_some()) // $9: flag whether to update preferred_gender
    .bind(data.preferred_gender.flatten())  // $10: new value (may be NULL)
    .bind(&data.languages_spoken)
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn submit_application(
    pool: &PgPool,
    id: Uuid,
) -> Result<CaregiverApplication, sqlx::Error> {
    let existing = sqlx::query_as::<_, CaregiverApplication>(
        "SELECT * FROM caregiver_applications WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or(sqlx::Error::RowNotFound)?;

    let machine = caregiver_application_machine();
    if !machine.can_transition(existing.status, CaregiverApplicationStatus::Submitted) {
        return Err(sqlx::Error::Protocol(format!(
            "Cannot transition application from {} to SUBMITTED",
            existing.status
        )));
    }

    sqlx::query_as::<_, CaregiverApplication>(
        r#"UPDATE caregiver_applications
           SET status = 'SUBMITTED', submitted_at = NOW(), updated_at = NOW()
           WHERE id = $1
           RETURNING *"#,
    )
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn transition_status(
    pool: &PgPool,
    id: Uuid,
    new_status: CaregiverApplicationStatus,
    reviewed_by: Option<Uuid>,
    rejection_reason: Option<&str>,
) -> Result<CaregiverApplication, sqlx::Error> {
    let existing = sqlx::query_as::<_, CaregiverApplication>(
        "SELECT * FROM caregiver_applications WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or(sqlx::Error::RowNotFound)?;

    let machine = caregiver_application_machine();
    if !machine.can_transition(existing.status, new_status) {
        return Err(sqlx::Error::Protocol(format!(
            "Invalid transition for caregiver application: {} -> {}",
            existing.status, new_status
        )));
    }

    sqlx::query_as::<_, CaregiverApplication>(
        r#"UPDATE caregiver_applications SET
             status           = $1,
             reviewed_by      = COALESCE($2, reviewed_by),
             reviewed_at      = CASE WHEN $2 IS NOT NULL THEN NOW() ELSE reviewed_at END,
             rejection_reason = COALESCE($3, rejection_reason),
             updated_at       = NOW()
           WHERE id = $4
           RETURNING *"#,
    )
    .bind(new_status)
    .bind(reviewed_by)
    .bind(rejection_reason)
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn get_application(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<ApplicationWithRelations>, sqlx::Error> {
    let application = sqlx::query_as::<_, CaregiverApplication>(
        "SELECT * FROM caregiver_applications WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    let application = match application {
        Some(app) => app,
        None => return Ok(None),
    };

    let credentials = sqlx::query_as::<_, CaregiverCredential>(
        "SELECT * FROM caregiver_credentials WHERE application_id = $1",
    )
    .bind(id)
    .fetch_all(pool)
    .await?;

    let availability_slots = sqlx::query_as::<_, AvailabilitySlot>(
        "SELECT * FROM availability_slots WHERE application_id = $1",
    )
    .bind(id)
    .fetch_all(pool)
    .await?;

    let service_types = sqlx::query_as::<_, ServiceType>(
        "SELECT * FROM service_types WHERE application_id = $1",
    )
    .bind(id)
    .fetch_all(pool)
    .await?;

    Ok(Some(ApplicationWithRelations {
        application,
        credentials,
        availability_slots,
        service_types,
    }))
}

pub async fn list_applications(
    pool: &PgPool,
    filters: &ApplicationFilters,
    limit: i64,
    offset: i64,
) -> Result<PaginatedApplications, sqlx::Error> {
    // Use a fixed query with optional filter params via COALESCE pattern
    let data = sqlx::query_as::<_, CaregiverApplication>(
        r#"SELECT * FROM caregiver_applications
           WHERE ($1::caregiver_application_status IS NULL OR status = $1)
             AND ($2::uuid IS NULL OR provider_id = $2)
           ORDER BY created_at DESC
           LIMIT $3 OFFSET $4"#,
    )
    .bind(filters.status)
    .bind(filters.provider_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    let row: (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM caregiver_applications
           WHERE ($1::caregiver_application_status IS NULL OR status = $1)
             AND ($2::uuid IS NULL OR provider_id = $2)"#,
    )
    .bind(filters.status)
    .bind(filters.provider_id)
    .fetch_one(pool)
    .await?;

    Ok(PaginatedApplications {
        data,
        total: row.0,
    })
}

pub async fn add_credential(
    pool: &PgPool,
    application_id: Uuid,
    data: &AddCredentialData,
) -> Result<CaregiverCredential, sqlx::Error> {
    let id = Uuid::new_v4();
    let now = Utc::now();

    sqlx::query_as::<_, CaregiverCredential>(
        r#"INSERT INTO caregiver_credentials (
             id, application_id, type, status, issuer, issued_at, expires_at,
             document_url, created_at, updated_at
           ) VALUES ($1,$2,$3,'PENDING',$4,$5,$6,$7,$8,$8)
           RETURNING *"#,
    )
    .bind(id)
    .bind(application_id)
    .bind(data.credential_type)
    .bind(&data.issuer)
    .bind(data.issued_at)
    .bind(data.expires_at)
    .bind(&data.document_url)
    .bind(now)
    .fetch_one(pool)
    .await
}

pub async fn verify_credential(
    pool: &PgPool,
    credential_id: Uuid,
    verified_by: Uuid,
) -> Result<CaregiverCredential, sqlx::Error> {
    // Verify existence
    let _existing = sqlx::query_as::<_, CaregiverCredential>(
        "SELECT * FROM caregiver_credentials WHERE id = $1",
    )
    .bind(credential_id)
    .fetch_optional(pool)
    .await?
    .ok_or(sqlx::Error::RowNotFound)?;

    sqlx::query_as::<_, CaregiverCredential>(
        r#"UPDATE caregiver_credentials
           SET status = 'VERIFIED', verified_at = NOW(), verified_by = $1, updated_at = NOW()
           WHERE id = $2
           RETURNING *"#,
    )
    .bind(verified_by)
    .bind(credential_id)
    .fetch_one(pool)
    .await
}
