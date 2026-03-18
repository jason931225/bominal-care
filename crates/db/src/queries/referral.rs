// =============================================================================
// Referral queries — ported from referral.service.ts
// =============================================================================

use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use bominal_types::enums::InstitutionReferralStatus;
use bominal_types::models::{InstitutionReferral, ProviderOrganization};
use bominal_types::state_machines::referral_machine;

// ---------------------------------------------------------------------------
// Input / output types
// ---------------------------------------------------------------------------

pub struct CreateReferralData {
    pub from_provider_id: Uuid,
    pub to_provider_id: Uuid,
    pub senior_person_id: Uuid,
    pub reason: Option<String>,
    pub notes: Option<String>,
}

pub struct ReferralFilters {
    pub from_provider_id: Option<Uuid>,
    pub to_provider_id: Option<Uuid>,
    pub senior_person_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralWithProviders {
    pub referral: InstitutionReferral,
    pub from_provider: Option<ProviderOrganization>,
    pub to_provider: Option<ProviderOrganization>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedReferrals {
    pub data: Vec<InstitutionReferral>,
    pub total: i64,
}

// ---------------------------------------------------------------------------
// Queries
// ---------------------------------------------------------------------------

pub async fn create_referral(
    pool: &PgPool,
    data: &CreateReferralData,
) -> Result<InstitutionReferral, sqlx::Error> {
    let id = Uuid::new_v4();
    let now = Utc::now();

    sqlx::query_as::<_, InstitutionReferral>(
        "INSERT INTO institution_referrals (
           id, from_provider_id, to_provider_id, senior_person_id,
           status, reason, notes, referred_at, created_at, updated_at
         ) VALUES ($1,$2,$3,$4,'CREATED',$5,$6,$7,$7,$7)
         RETURNING *",
    )
    .bind(id)
    .bind(data.from_provider_id)
    .bind(data.to_provider_id)
    .bind(data.senior_person_id)
    .bind(&data.reason)
    .bind(&data.notes)
    .bind(now)
    .fetch_one(pool)
    .await
}

pub async fn update_status(
    pool: &PgPool,
    id: Uuid,
    status: InstitutionReferralStatus,
    notes: Option<&str>,
) -> Result<InstitutionReferral, sqlx::Error> {
    let existing = sqlx::query_as::<_, InstitutionReferral>(
        "SELECT * FROM institution_referrals WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| sqlx::Error::RowNotFound)?;

    let machine = referral_machine();
    if !machine.can_transition(existing.status, status) {
        return Err(sqlx::Error::Protocol(format!(
            "Invalid referral status transition: {} -> {}",
            existing.status, status,
        )));
    }

    let now = Utc::now();

    // Determine timestamp columns to set based on the target status.
    let accepted_at = if status == InstitutionReferralStatus::Accepted {
        Some(now)
    } else {
        existing.accepted_at
    };
    let discharged_at = if status == InstitutionReferralStatus::Discharged {
        Some(now)
    } else {
        existing.discharged_at
    };
    let final_notes = notes.map(|n| n.to_string()).or(existing.notes);

    sqlx::query_as::<_, InstitutionReferral>(
        "UPDATE institution_referrals
         SET status = $1,
             notes = $2,
             accepted_at = $3,
             discharged_at = $4,
             updated_at = $5
         WHERE id = $6
         RETURNING *",
    )
    .bind(status)
    .bind(&final_notes)
    .bind(accepted_at)
    .bind(discharged_at)
    .bind(now)
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn get_referral(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<ReferralWithProviders>, sqlx::Error> {
    let referral = match sqlx::query_as::<_, InstitutionReferral>(
        "SELECT * FROM institution_referrals WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    {
        Some(r) => r,
        None => return Ok(None),
    };

    let from_provider = sqlx::query_as::<_, ProviderOrganization>(
        "SELECT * FROM provider_organizations WHERE id = $1",
    )
    .bind(referral.from_provider_id)
    .fetch_optional(pool)
    .await?;

    let to_provider = sqlx::query_as::<_, ProviderOrganization>(
        "SELECT * FROM provider_organizations WHERE id = $1",
    )
    .bind(referral.to_provider_id)
    .fetch_optional(pool)
    .await?;

    Ok(Some(ReferralWithProviders {
        referral,
        from_provider,
        to_provider,
    }))
}

pub async fn list_referrals(
    pool: &PgPool,
    filters: &ReferralFilters,
    limit: i64,
    offset: i64,
) -> Result<PaginatedReferrals, sqlx::Error> {
    // Build dynamic WHERE clause.
    // We always bind all three filter values and use a pattern where NULL
    // means "no filter" via ($N IS NULL OR column = $N).
    let data = sqlx::query_as::<_, InstitutionReferral>(
        "SELECT * FROM institution_referrals
         WHERE ($1::uuid IS NULL OR from_provider_id = $1)
           AND ($2::uuid IS NULL OR to_provider_id = $2)
           AND ($3::uuid IS NULL OR senior_person_id = $3)
         ORDER BY referred_at DESC
         LIMIT $4 OFFSET $5",
    )
    .bind(filters.from_provider_id)
    .bind(filters.to_provider_id)
    .bind(filters.senior_person_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM institution_referrals
         WHERE ($1::uuid IS NULL OR from_provider_id = $1)
           AND ($2::uuid IS NULL OR to_provider_id = $2)
           AND ($3::uuid IS NULL OR senior_person_id = $3)",
    )
    .bind(filters.from_provider_id)
    .bind(filters.to_provider_id)
    .bind(filters.senior_person_id)
    .fetch_one(pool)
    .await?;

    Ok(PaginatedReferrals {
        data,
        total: total.0,
    })
}
