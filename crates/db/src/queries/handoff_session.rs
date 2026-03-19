// =============================================================================
// Handoff Session queries — scoped temporary medical access
// =============================================================================

use sqlx::PgPool;
use uuid::Uuid;

use bominal_types::enums::HandoffLicenseType;
use bominal_types::medical::MedicalHandoffSession;

pub async fn start_session(
    pool: &PgPool,
    senior_person_id: Uuid,
    professional_user_id: Uuid,
    license_type: HandoffLicenseType,
    license_number: &str,
    institution_name: Option<&str>,
    institution_id: Option<Uuid>,
) -> Result<MedicalHandoffSession, sqlx::Error> {
    sqlx::query_as::<_, MedicalHandoffSession>(
        "INSERT INTO medical_handoff_sessions
           (senior_person_id, professional_user_id, license_type, license_number,
            institution_name, institution_id)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING *",
    )
    .bind(senior_person_id)
    .bind(professional_user_id)
    .bind(license_type)
    .bind(license_number)
    .bind(institution_name)
    .bind(institution_id)
    .fetch_one(pool)
    .await
}

pub async fn end_session(
    pool: &PgPool,
    id: Uuid,
) -> Result<MedicalHandoffSession, sqlx::Error> {
    sqlx::query_as::<_, MedicalHandoffSession>(
        "UPDATE medical_handoff_sessions
         SET ended_at = NOW(), is_active = FALSE
         WHERE id = $1
         RETURNING *",
    )
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn get_active_session(
    pool: &PgPool,
    professional_user_id: Uuid,
    senior_person_id: Uuid,
) -> Result<Option<MedicalHandoffSession>, sqlx::Error> {
    sqlx::query_as::<_, MedicalHandoffSession>(
        "SELECT * FROM medical_handoff_sessions
         WHERE professional_user_id = $1
           AND senior_person_id = $2
           AND is_active = TRUE
           AND expires_at > NOW()",
    )
    .bind(professional_user_id)
    .bind(senior_person_id)
    .fetch_optional(pool)
    .await
}

pub async fn list_for_senior(
    pool: &PgPool,
    senior_person_id: Uuid,
    limit: i64,
    offset: i64,
) -> Result<Vec<MedicalHandoffSession>, sqlx::Error> {
    sqlx::query_as::<_, MedicalHandoffSession>(
        "SELECT * FROM medical_handoff_sessions
         WHERE senior_person_id = $1
         ORDER BY started_at DESC
         LIMIT $2 OFFSET $3",
    )
    .bind(senior_person_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}
