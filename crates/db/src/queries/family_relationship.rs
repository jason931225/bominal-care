// =============================================================================
// Family Relationship queries
// =============================================================================

use sqlx::PgPool;
use uuid::Uuid;

use bominal_types::{FamilyRelationship, RelationshipType};

pub async fn add_member(
    pool: &PgPool,
    senior_person_id: Uuid,
    family_person_id: Uuid,
    relationship_type: RelationshipType,
    is_primary_contact: bool,
    can_make_decisions: bool,
) -> Result<FamilyRelationship, sqlx::Error> {
    sqlx::query_as::<_, FamilyRelationship>(
        "INSERT INTO family_relationships
         (senior_person_id, family_person_id, relationship_type, is_primary_contact, can_make_decisions)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING *"
    )
    .bind(senior_person_id)
    .bind(family_person_id)
    .bind(relationship_type)
    .bind(is_primary_contact)
    .bind(can_make_decisions)
    .fetch_one(pool)
    .await
}

pub async fn list_for_senior(
    pool: &PgPool,
    senior_person_id: Uuid,
) -> Result<Vec<FamilyRelationship>, sqlx::Error> {
    sqlx::query_as::<_, FamilyRelationship>(
        "SELECT * FROM family_relationships
         WHERE senior_person_id = $1
         ORDER BY is_primary_contact DESC, created_at"
    )
    .bind(senior_person_id)
    .fetch_all(pool)
    .await
}

pub async fn verify_link(
    pool: &PgPool,
    family_person_id: Uuid,
    senior_person_id: Uuid,
) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(
            SELECT 1 FROM family_relationships
            WHERE family_person_id = $1 AND senior_person_id = $2
        )"
    )
    .bind(family_person_id)
    .bind(senior_person_id)
    .fetch_one(pool)
    .await
}
