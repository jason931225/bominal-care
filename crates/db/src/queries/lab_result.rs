// =============================================================================
// Lab Result queries — create, retrieve, review
// =============================================================================

use sqlx::PgPool;
use uuid::Uuid;

use bominal_types::medical::LabResult;

pub async fn create(
    pool: &PgPool,
    senior_person_id: Uuid,
    ordered_by: Option<Uuid>,
    test_name: &str,
    test_code: Option<&str>,
    result_value: Option<&str>,
    result_unit: Option<&str>,
    reference_range: Option<&str>,
    is_critical: bool,
) -> Result<LabResult, sqlx::Error> {
    sqlx::query_as::<_, LabResult>(
        "INSERT INTO lab_results
           (senior_person_id, ordered_by, test_name, test_code,
            result_value, result_unit, reference_range, is_critical)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
         RETURNING *",
    )
    .bind(senior_person_id)
    .bind(ordered_by)
    .bind(test_name)
    .bind(test_code)
    .bind(result_value)
    .bind(result_unit)
    .bind(reference_range)
    .bind(is_critical)
    .fetch_one(pool)
    .await
}

pub async fn get(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<LabResult>, sqlx::Error> {
    sqlx::query_as::<_, LabResult>(
        "SELECT * FROM lab_results WHERE id = $1",
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
) -> Result<Vec<LabResult>, sqlx::Error> {
    sqlx::query_as::<_, LabResult>(
        "SELECT * FROM lab_results
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

pub async fn mark_reviewed(
    pool: &PgPool,
    id: Uuid,
    reviewed_by: Uuid,
) -> Result<LabResult, sqlx::Error> {
    sqlx::query_as::<_, LabResult>(
        "UPDATE lab_results
         SET reviewed_by = $2, reviewed_at = NOW(), updated_at = NOW()
         WHERE id = $1
         RETURNING *",
    )
    .bind(id)
    .bind(reviewed_by)
    .fetch_one(pool)
    .await
}
