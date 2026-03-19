// =============================================================================
// Patient Allergy queries — create, retrieve, deactivate
// =============================================================================

use sqlx::PgPool;
use uuid::Uuid;

use bominal_types::medical::PatientAllergy;

pub async fn create(
    pool: &PgPool,
    senior_person_id: Uuid,
    allergen: &str,
    reaction: Option<&str>,
    severity: Option<&str>,
    reported_by: Option<Uuid>,
) -> Result<PatientAllergy, sqlx::Error> {
    sqlx::query_as::<_, PatientAllergy>(
        "INSERT INTO patient_allergies
           (senior_person_id, allergen, reaction, severity, reported_by)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING *",
    )
    .bind(senior_person_id)
    .bind(allergen)
    .bind(reaction)
    .bind(severity)
    .bind(reported_by)
    .fetch_one(pool)
    .await
}

pub async fn get(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<PatientAllergy>, sqlx::Error> {
    sqlx::query_as::<_, PatientAllergy>(
        "SELECT * FROM patient_allergies WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn list_for_senior(
    pool: &PgPool,
    senior_person_id: Uuid,
    active_only: bool,
) -> Result<Vec<PatientAllergy>, sqlx::Error> {
    if active_only {
        sqlx::query_as::<_, PatientAllergy>(
            "SELECT * FROM patient_allergies
             WHERE senior_person_id = $1 AND is_active = TRUE
             ORDER BY created_at DESC",
        )
        .bind(senior_person_id)
        .fetch_all(pool)
        .await
    } else {
        sqlx::query_as::<_, PatientAllergy>(
            "SELECT * FROM patient_allergies
             WHERE senior_person_id = $1
             ORDER BY created_at DESC",
        )
        .bind(senior_person_id)
        .fetch_all(pool)
        .await
    }
}

pub async fn deactivate(
    pool: &PgPool,
    id: Uuid,
) -> Result<PatientAllergy, sqlx::Error> {
    sqlx::query_as::<_, PatientAllergy>(
        "UPDATE patient_allergies
         SET is_active = FALSE, updated_at = NOW()
         WHERE id = $1
         RETURNING *",
    )
    .bind(id)
    .fetch_one(pool)
    .await
}
