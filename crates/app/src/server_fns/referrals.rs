// =============================================================================
// Referral server functions — web-internal portal
// =============================================================================

use leptos::prelude::*;
use leptos::server;
use uuid::Uuid;

use bominal_db::queries::referral::{
    self, CreateReferralData, PaginatedReferrals, ReferralFilters, ReferralWithProviders,
};
use bominal_types::enums::InstitutionReferralStatus;
use bominal_types::models::InstitutionReferral;
use bominal_types::ReferralInput;

/// Paginated list of referrals with optional provider filters.
#[server]
pub async fn list_referrals(
    from_provider_id: Option<Uuid>,
    to_provider_id: Option<Uuid>,
    page: i64,
    limit: i64,
) -> Result<PaginatedReferrals, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let filters = ReferralFilters {
        from_provider_id,
        to_provider_id,
        senior_person_id: None,
    };

    let offset = (page.max(1) - 1) * limit.clamp(1, 100);

    referral::list_referrals(&pool, &filters, limit.clamp(1, 100), offset)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Create a new institution referral.
#[server]
pub async fn create_referral(
    input: ReferralInput,
) -> Result<InstitutionReferral, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let data = CreateReferralData {
        from_provider_id: input.from_provider_id,
        to_provider_id: input.to_provider_id,
        senior_person_id: input.senior_person_id,
        reason: Some(input.reason),
        notes: input.notes,
    };

    referral::create_referral(&pool, &data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Get a single referral with its associated provider details.
#[server]
pub async fn get_referral(
    id: Uuid,
) -> Result<Option<ReferralWithProviders>, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    referral::get_referral(&pool, id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Transition a referral to a new status with optional notes.
#[server]
pub async fn update_referral_status(
    id: Uuid,
    status: InstitutionReferralStatus,
    notes: Option<String>,
) -> Result<InstitutionReferral, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    referral::update_status(&pool, id, status, notes.as_deref())
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}
