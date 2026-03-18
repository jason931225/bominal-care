// =============================================================================
// Consent server functions — ConsentRecord management
// =============================================================================

use chrono::{DateTime, Utc};
use leptos::prelude::*;
use leptos::server;
use uuid::Uuid;

use bominal_db::queries::consent::{ConsentRecordRow, GrantConsentData};
use bominal_types::ConsentPurpose;

// -----------------------------------------------------------------------------
// list_consents — list all consents for a person
// -----------------------------------------------------------------------------

#[server]
pub async fn list_consents(
    person_id: Uuid,
) -> Result<Vec<ConsentRecordRow>, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    bominal_db::queries::consent::get_consents_for_person(&pool, person_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

// -----------------------------------------------------------------------------
// grant_consent — grant a new consent
// -----------------------------------------------------------------------------

#[server]
pub async fn grant_consent(
    subject_person_id: Uuid,
    purpose: ConsentPurpose,
    granted_by: Uuid,
    expires_at: Option<DateTime<Utc>>,
) -> Result<ConsentRecordRow, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let data = GrantConsentData {
        subject_person_id,
        purpose,
        granted_by,
        expires_at,
    };

    bominal_db::queries::consent::grant_consent(&pool, &data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

// -----------------------------------------------------------------------------
// revoke_consent — revoke an existing consent
// -----------------------------------------------------------------------------

#[server]
pub async fn revoke_consent(
    consent_id: Uuid,
    revoked_by: Uuid,
) -> Result<ConsentRecordRow, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    bominal_db::queries::consent::revoke_consent(&pool, consent_id, revoked_by)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}
