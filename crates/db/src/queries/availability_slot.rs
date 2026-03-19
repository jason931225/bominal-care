// =============================================================================
// Availability Slot queries
// =============================================================================

use sqlx::PgPool;
use uuid::Uuid;

use bominal_types::{AvailabilitySlot, DayOfWeek};

pub async fn add(
    pool: &PgPool,
    application_id: Uuid,
    day_of_week: DayOfWeek,
    start_time: &str,
    end_time: &str,
) -> Result<AvailabilitySlot, sqlx::Error> {
    sqlx::query_as::<_, AvailabilitySlot>(
        "INSERT INTO availability_slots (application_id, day_of_week, start_time, end_time)
         VALUES ($1, $2, $3, $4)
         RETURNING *"
    )
    .bind(application_id)
    .bind(day_of_week)
    .bind(start_time)
    .bind(end_time)
    .fetch_one(pool)
    .await
}

pub async fn list_for_application(
    pool: &PgPool,
    application_id: Uuid,
) -> Result<Vec<AvailabilitySlot>, sqlx::Error> {
    sqlx::query_as::<_, AvailabilitySlot>(
        "SELECT * FROM availability_slots
         WHERE application_id = $1 AND is_active = TRUE
         ORDER BY day_of_week, start_time"
    )
    .bind(application_id)
    .fetch_all(pool)
    .await
}
