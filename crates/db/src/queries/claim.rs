// =============================================================================
// Claim / Subsidy Record queries
// =============================================================================

use sqlx::PgPool;
use uuid::Uuid;

use bominal_types::{ClaimOrSubsidyRecord, ClaimStatus};

pub async fn create(
    pool: &PgPool,
    case_id: Uuid,
    amount: rust_decimal::Decimal,
    currency: &str,
    service_date: chrono::DateTime<chrono::Utc>,
    notes: Option<&str>,
) -> Result<ClaimOrSubsidyRecord, sqlx::Error> {
    let claim_number = format!("CLM-{}", Uuid::new_v4().to_string().split('-').next().unwrap_or("000"));
    sqlx::query_as::<_, ClaimOrSubsidyRecord>(
        "INSERT INTO claim_or_subsidy_records
         (case_id, claim_number, status, amount, currency, service_date, notes)
         VALUES ($1, $2, 'submitted', $3, $4, $5, $6)
         RETURNING *"
    )
    .bind(case_id)
    .bind(&claim_number)
    .bind(amount)
    .bind(currency)
    .bind(service_date)
    .bind(notes)
    .fetch_one(pool)
    .await
}

pub async fn update_status(
    pool: &PgPool,
    id: Uuid,
    status: ClaimStatus,
    notes: Option<&str>,
) -> Result<ClaimOrSubsidyRecord, sqlx::Error> {
    sqlx::query_as::<_, ClaimOrSubsidyRecord>(
        "UPDATE claim_or_subsidy_records SET
           status = $2,
           notes = COALESCE($3, notes),
           processed_at = CASE WHEN $2 IN ('approved', 'denied') THEN NOW() ELSE processed_at END,
           updated_at = NOW()
         WHERE id = $1
         RETURNING *"
    )
    .bind(id)
    .bind(status)
    .bind(notes)
    .fetch_one(pool)
    .await
}

pub async fn list_for_case(
    pool: &PgPool,
    case_id: Uuid,
) -> Result<Vec<ClaimOrSubsidyRecord>, sqlx::Error> {
    sqlx::query_as::<_, ClaimOrSubsidyRecord>(
        "SELECT * FROM claim_or_subsidy_records
         WHERE case_id = $1
         ORDER BY service_date DESC"
    )
    .bind(case_id)
    .fetch_all(pool)
    .await
}
