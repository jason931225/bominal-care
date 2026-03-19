// =============================================================================
// Provider Organization queries
// =============================================================================

use sqlx::PgPool;
use uuid::Uuid;

use bominal_types::ProviderOrganization;

pub async fn create(
    pool: &PgPool,
    name: &str,
    provider_type: bominal_types::ProviderType,
    registration_number: &str,
    created_by: Option<Uuid>,
) -> Result<ProviderOrganization, sqlx::Error> {
    sqlx::query_as::<_, ProviderOrganization>(
        "INSERT INTO provider_organizations (name, type, registration_number, created_by)
         VALUES ($1, $2, $3, $4)
         RETURNING *"
    )
    .bind(name)
    .bind(provider_type)
    .bind(registration_number)
    .bind(created_by)
    .fetch_one(pool)
    .await
}

pub async fn get(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<ProviderOrganization>, sqlx::Error> {
    sqlx::query_as::<_, ProviderOrganization>(
        "SELECT * FROM provider_organizations WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn list(
    pool: &PgPool,
    is_active: Option<bool>,
    limit: i64,
    offset: i64,
) -> Result<(Vec<ProviderOrganization>, i64), sqlx::Error> {
    let total = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM provider_organizations
         WHERE ($1::BOOLEAN IS NULL OR is_active = $1)"
    )
    .bind(is_active)
    .fetch_one(pool)
    .await?;

    let rows = sqlx::query_as::<_, ProviderOrganization>(
        "SELECT * FROM provider_organizations
         WHERE ($1::BOOLEAN IS NULL OR is_active = $1)
         ORDER BY name
         LIMIT $2 OFFSET $3"
    )
    .bind(is_active)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok((rows, total))
}

pub async fn update(
    pool: &PgPool,
    id: Uuid,
    name: Option<&str>,
    phone: Option<&str>,
    email: Option<&str>,
    is_active: Option<bool>,
    updated_by: Option<Uuid>,
) -> Result<ProviderOrganization, sqlx::Error> {
    sqlx::query_as::<_, ProviderOrganization>(
        "UPDATE provider_organizations SET
           name = COALESCE($2, name),
           phone = COALESCE($3, phone),
           email = COALESCE($4, email),
           is_active = COALESCE($5, is_active),
           updated_by = $6,
           updated_at = NOW()
         WHERE id = $1
         RETURNING *"
    )
    .bind(id)
    .bind(name)
    .bind(phone)
    .bind(email)
    .bind(is_active)
    .bind(updated_by)
    .fetch_one(pool)
    .await
}
