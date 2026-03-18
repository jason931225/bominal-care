// =============================================================================
// Caregiver Application Server Functions
// Wraps bominal_db::queries::caregiver_application for Leptos SSR
// =============================================================================

use leptos::prelude::*;
use leptos::server;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use bominal_types::enums::CaregiverApplicationStatus;
use bominal_types::inputs::{CaregiverApplicationInput, CaregiverCredentialInput};
use bominal_types::models::{CaregiverApplication, CaregiverCredential};

// ---------------------------------------------------------------------------
// Response types (serializable across the wire)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaregiverListResponse {
    pub data: Vec<CaregiverApplication>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationDetailResponse {
    pub application: CaregiverApplication,
    pub credentials: Vec<CaregiverCredential>,
    pub availability_slots: Vec<bominal_types::models::AvailabilitySlot>,
    pub service_types: Vec<bominal_types::models::ServiceType>,
}

// ---------------------------------------------------------------------------
// Server functions
// ---------------------------------------------------------------------------

/// Paginated list of caregiver applications with optional status filter.
#[server]
pub async fn list_caregivers(
    status: Option<String>,
    page: i64,
    limit: i64,
) -> Result<CaregiverListResponse, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let parsed_status = match &status {
        Some(s) => {
            let parsed: CaregiverApplicationStatus = s
                .parse()
                .map_err(|_| ServerFnError::new(format!("Invalid status: {s}")))?;
            Some(parsed)
        }
        None => None,
    };

    let filters = bominal_db::queries::caregiver_application::ApplicationFilters {
        status: parsed_status,
        provider_id: None,
    };
    let params = bominal_types::common::PaginationParams::new(page, limit);

    let result = bominal_db::queries::caregiver_application::list_applications(
        &pool,
        &filters,
        params.limit,
        params.offset(),
    )
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(CaregiverListResponse {
        data: result.data,
        total: result.total,
    })
}

/// Get a single application with its credentials, slots, and service types.
#[server]
pub async fn get_application(
    id: Uuid,
) -> Result<Option<ApplicationDetailResponse>, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let result = bominal_db::queries::caregiver_application::get_application(&pool, id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(result.map(|r| ApplicationDetailResponse {
        application: r.application,
        credentials: r.credentials,
        availability_slots: r.availability_slots,
        service_types: r.service_types,
    }))
}

/// Create a new draft caregiver application.
#[server]
pub async fn create_application(
    input: CaregiverApplicationInput,
) -> Result<CaregiverApplication, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let data = bominal_db::queries::caregiver_application::CreateApplicationData {
        provider_id: input.provider_id,
        experience_years: input.experience_years,
        bio: input.bio,
        specializations: input.specializations,
        has_dementia_experience: input.has_dementia_experience,
        has_overnight_availability: input.has_overnight_availability,
        smoking_status: input.smoking_status,
        pet_friendly: input.pet_friendly,
        preferred_gender: input.preferred_gender,
        languages_spoken: input.languages_spoken,
    };

    bominal_db::queries::caregiver_application::create_application(&pool, input.user_id, &data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Update a draft caregiver application.
#[server]
pub async fn update_application(
    id: Uuid,
    provider_id: Option<Uuid>,
    experience_years: Option<i32>,
    bio: Option<String>,
    specializations: Option<String>,
    has_dementia_experience: Option<bool>,
    has_overnight_availability: Option<bool>,
    smoking_status: Option<bool>,
    pet_friendly: Option<bool>,
    languages_spoken: Option<String>,
) -> Result<CaregiverApplication, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let data = bominal_db::queries::caregiver_application::UpdateApplicationData {
        provider_id,
        experience_years,
        bio,
        specializations,
        has_dementia_experience,
        has_overnight_availability,
        smoking_status,
        pet_friendly,
        preferred_gender: None,
        languages_spoken,
    };

    bominal_db::queries::caregiver_application::update_application(&pool, id, &data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Submit a draft application for review.
/// Transitions the application from DRAFT to SUBMITTED.
#[server]
pub async fn submit_application(
    id: Uuid,
) -> Result<CaregiverApplication, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    bominal_db::queries::caregiver_application::submit_application(&pool, id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Review an application: approve, reject, or transition its status.
#[server]
pub async fn review_application(
    id: Uuid,
    status: CaregiverApplicationStatus,
    rejection_reason: Option<String>,
    reviewed_by: Uuid,
) -> Result<CaregiverApplication, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    bominal_db::queries::caregiver_application::transition_status(
        &pool,
        id,
        status,
        Some(reviewed_by),
        rejection_reason.as_deref(),
    )
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))
}

/// List credentials for a specific application.
#[server]
pub async fn list_credentials(
    application_id: Uuid,
) -> Result<Vec<CaregiverCredential>, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    // Fetch via get_application and extract credentials
    let result = bominal_db::queries::caregiver_application::get_application(&pool, application_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    match result {
        Some(detail) => Ok(detail.credentials),
        None => Err(ServerFnError::new("Application not found")),
    }
}

/// Add a credential to a caregiver application.
#[server]
pub async fn add_credential(
    application_id: Uuid,
    input: CaregiverCredentialInput,
) -> Result<CaregiverCredential, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let data = bominal_db::queries::caregiver_application::AddCredentialData {
        credential_type: input.credential_type,
        issuer: input.issuer,
        issued_at: input.issued_at,
        expires_at: input.expires_at,
        document_url: input.document_url,
    };

    bominal_db::queries::caregiver_application::add_credential(&pool, application_id, &data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}
