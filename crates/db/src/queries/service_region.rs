// =============================================================================
// Service Region queries
// =============================================================================

use sqlx::PgPool;
use uuid::Uuid;

use bominal_types::ServiceRegion;

pub async fn add(
    pool: &PgPool,
    provider_id: Uuid,
    city: &str,
    district: &str,
) -> Result<ServiceRegion, sqlx::Error> {
    sqlx::query_as::<_, ServiceRegion>(
        "INSERT INTO service_regions (provider_id, city, district)
         VALUES ($1, $2, $3)
         RETURNING *"
    )
    .bind(provider_id)
    .bind(city)
    .bind(district)
    .fetch_one(pool)
    .await
}

pub async fn list_for_provider(
    pool: &PgPool,
    provider_id: Uuid,
) -> Result<Vec<ServiceRegion>, sqlx::Error> {
    sqlx::query_as::<_, ServiceRegion>(
        "SELECT * FROM service_regions
         WHERE provider_id = $1 AND is_active = TRUE
         ORDER BY city, district"
    )
    .bind(provider_id)
    .fetch_all(pool)
    .await
}
