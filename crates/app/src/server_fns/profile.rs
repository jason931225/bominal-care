// =============================================================================
// Profile server functions — PersonProfile and SeniorProfile
// =============================================================================

use chrono::{DateTime, Utc};
use leptos::prelude::*;
use leptos::server;
use uuid::Uuid;

use bominal_db::queries::profile::{
    PersonProfileRow, PersonProfileWithUser, SeniorProfileWithPerson, UpdatePersonProfileData,
};

// -----------------------------------------------------------------------------
// get_profile — fetch person profile by user_id
// -----------------------------------------------------------------------------

#[server]
pub async fn get_profile(
    user_id: Uuid,
) -> Result<Option<PersonProfileWithUser>, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    bominal_db::queries::profile::get_person_profile_by_user_id(&pool, user_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

// -----------------------------------------------------------------------------
// update_profile — update person profile fields
// -----------------------------------------------------------------------------

#[server]
pub async fn update_profile(
    profile_id: Uuid,
    first_name: Option<String>,
    last_name: Option<String>,
    date_of_birth: Option<DateTime<Utc>>,
    gender: Option<String>,
    national_id: Option<String>,
    phone: Option<String>,
    address: Option<String>,
    city: Option<String>,
    district: Option<String>,
    emergency_contact_name: Option<String>,
    emergency_contact_phone: Option<String>,
) -> Result<PersonProfileRow, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let data = UpdatePersonProfileData {
        first_name,
        last_name,
        date_of_birth,
        gender,
        national_id,
        phone,
        address,
        city,
        district,
        emergency_contact_name,
        emergency_contact_phone,
    };

    bominal_db::queries::profile::update_person_profile(&pool, profile_id, &data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

// -----------------------------------------------------------------------------
// get_senior_profile — fetch senior profile with person data
// -----------------------------------------------------------------------------

#[server]
pub async fn get_senior_profile(
    person_id: Uuid,
) -> Result<Option<SeniorProfileWithPerson>, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    bominal_db::queries::profile::get_senior_profile(&pool, person_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}
