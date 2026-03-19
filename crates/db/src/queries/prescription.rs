// =============================================================================
// Prescription queries — create, sign, retrieve
// =============================================================================

use sqlx::PgPool;
use uuid::Uuid;

use bominal_types::medical::Prescription;

pub async fn create(
    pool: &PgPool,
    senior_person_id: Uuid,
    prescribed_by: Uuid,
    institution_id: Option<Uuid>,
    medication_name: &str,
    dosage: &str,
    frequency: &str,
    duration_days: Option<i32>,
    instructions: Option<&str>,
) -> Result<Prescription, sqlx::Error> {
    sqlx::query_as::<_, Prescription>(
        "INSERT INTO prescriptions
           (senior_person_id, prescribed_by, institution_id, medication_name,
            dosage, frequency, duration_days, instructions)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
         RETURNING *",
    )
    .bind(senior_person_id)
    .bind(prescribed_by)
    .bind(institution_id)
    .bind(medication_name)
    .bind(dosage)
    .bind(frequency)
    .bind(duration_days)
    .bind(instructions)
    .fetch_one(pool)
    .await
}

pub async fn sign(
    pool: &PgPool,
    id: Uuid,
    signed_by: Uuid,
) -> Result<Prescription, sqlx::Error> {
    sqlx::query_as::<_, Prescription>(
        "UPDATE prescriptions
         SET is_signed = TRUE, signed_at = NOW(), signed_by = $2, updated_at = NOW()
         WHERE id = $1
         RETURNING *",
    )
    .bind(id)
    .bind(signed_by)
    .fetch_one(pool)
    .await
}

pub async fn get(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<Prescription>, sqlx::Error> {
    sqlx::query_as::<_, Prescription>(
        "SELECT * FROM prescriptions WHERE id = $1",
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
) -> Result<Vec<Prescription>, sqlx::Error> {
    sqlx::query_as::<_, Prescription>(
        "SELECT * FROM prescriptions
         WHERE senior_person_id = $1
         ORDER BY created_at DESC
         LIMIT $2 OFFSET $3",
    )
    .bind(senior_person_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}
