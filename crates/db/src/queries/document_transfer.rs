// =============================================================================
// Document Transfer Request queries — inter-institution document exchange
// =============================================================================

use sqlx::PgPool;
use uuid::Uuid;

use bominal_types::medical::DocumentTransferRequest;

pub async fn create_request(
    pool: &PgPool,
    from_institution_id: Uuid,
    to_institution_id: Uuid,
    senior_person_id: Uuid,
    document_type: &str,
    requested_by: Uuid,
    notes: Option<&str>,
) -> Result<DocumentTransferRequest, sqlx::Error> {
    sqlx::query_as::<_, DocumentTransferRequest>(
        "INSERT INTO document_transfer_requests
           (from_institution_id, to_institution_id, senior_person_id,
            document_type, requested_by, notes)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING *",
    )
    .bind(from_institution_id)
    .bind(to_institution_id)
    .bind(senior_person_id)
    .bind(document_type)
    .bind(requested_by)
    .bind(notes)
    .fetch_one(pool)
    .await
}

pub async fn approve(
    pool: &PgPool,
    id: Uuid,
    approved_by: Uuid,
) -> Result<DocumentTransferRequest, sqlx::Error> {
    sqlx::query_as::<_, DocumentTransferRequest>(
        "UPDATE document_transfer_requests
         SET status = 'approved', approved_by = $2, approved_at = NOW(), updated_at = NOW()
         WHERE id = $1
         RETURNING *",
    )
    .bind(id)
    .bind(approved_by)
    .fetch_one(pool)
    .await
}

pub async fn get(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<DocumentTransferRequest>, sqlx::Error> {
    sqlx::query_as::<_, DocumentTransferRequest>(
        "SELECT * FROM document_transfer_requests WHERE id = $1",
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
) -> Result<Vec<DocumentTransferRequest>, sqlx::Error> {
    sqlx::query_as::<_, DocumentTransferRequest>(
        "SELECT * FROM document_transfer_requests
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
