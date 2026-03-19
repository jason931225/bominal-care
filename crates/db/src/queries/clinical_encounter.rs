// =============================================================================
// Clinical Encounter queries — SOAP notes CRUD
// =============================================================================

use sqlx::PgPool;
use uuid::Uuid;

use bominal_types::medical::ClinicalEncounter;

pub async fn create(
    pool: &PgPool,
    senior_person_id: Uuid,
    provider_user_id: Uuid,
    institution_id: Option<Uuid>,
    subjective: Option<&str>,
    objective: Option<&str>,
    assessment: Option<&str>,
    plan: Option<&str>,
) -> Result<ClinicalEncounter, sqlx::Error> {
    sqlx::query_as::<_, ClinicalEncounter>(
        "INSERT INTO clinical_encounters
           (senior_person_id, provider_user_id, institution_id,
            subjective, objective, assessment, plan)
         VALUES ($1, $2, $3, $4, $5, $6, $7)
         RETURNING *",
    )
    .bind(senior_person_id)
    .bind(provider_user_id)
    .bind(institution_id)
    .bind(subjective)
    .bind(objective)
    .bind(assessment)
    .bind(plan)
    .fetch_one(pool)
    .await
}

pub async fn sign(
    pool: &PgPool,
    id: Uuid,
) -> Result<ClinicalEncounter, sqlx::Error> {
    sqlx::query_as::<_, ClinicalEncounter>(
        "UPDATE clinical_encounters
         SET is_signed = TRUE, signed_at = NOW(), updated_at = NOW()
         WHERE id = $1
         RETURNING *",
    )
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn add_addendum(
    pool: &PgPool,
    id: Uuid,
    addendum: &str,
) -> Result<ClinicalEncounter, sqlx::Error> {
    sqlx::query_as::<_, ClinicalEncounter>(
        "UPDATE clinical_encounters
         SET addendum = $2, addendum_at = NOW(), updated_at = NOW()
         WHERE id = $1
         RETURNING *",
    )
    .bind(id)
    .bind(addendum)
    .fetch_one(pool)
    .await
}

pub async fn get(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<ClinicalEncounter>, sqlx::Error> {
    sqlx::query_as::<_, ClinicalEncounter>(
        "SELECT * FROM clinical_encounters WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn list_for_senior(
    pool: &PgPool,
    senior_person_id: Uuid,
    limit: i64,
    offset: i64,
) -> Result<Vec<ClinicalEncounter>, sqlx::Error> {
    sqlx::query_as::<_, ClinicalEncounter>(
        "SELECT * FROM clinical_encounters
         WHERE senior_person_id = $1
         ORDER BY encounter_date DESC
         LIMIT $2 OFFSET $3",
    )
    .bind(senior_person_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}
