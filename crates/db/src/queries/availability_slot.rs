// =============================================================================
// Availability Slot queries — weekly recurring slots + date-based exceptions
// =============================================================================

use chrono::{Datelike, NaiveDate, NaiveTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use bominal_types::enums::DayOfWeek;
use bominal_types::models::{AvailabilityException, AvailabilitySlot};

// ---------------------------------------------------------------------------
// Input struct for slot creation
// ---------------------------------------------------------------------------

pub struct SlotData {
    pub day_of_week: DayOfWeek,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
}

// ---------------------------------------------------------------------------
// Weekly slots
// ---------------------------------------------------------------------------

/// List all active availability slots for a user, ordered by day and time.
pub async fn list_for_user(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<AvailabilitySlot>, sqlx::Error> {
    sqlx::query_as::<_, AvailabilitySlot>(
        "SELECT * FROM availability_slots \
         WHERE user_id = $1 AND is_active = TRUE \
         ORDER BY day_of_week, start_time",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

/// Replace all active slots for a user with new ones (idempotent PUT).
/// Deactivates existing slots, inserts new ones, and returns the result.
pub async fn replace_slots(
    pool: &PgPool,
    user_id: Uuid,
    slots: &[SlotData],
) -> Result<Vec<AvailabilitySlot>, sqlx::Error> {
    let mut tx = pool.begin().await?;

    // Deactivate existing slots
    sqlx::query(
        "UPDATE availability_slots \
         SET is_active = FALSE, updated_at = NOW() \
         WHERE user_id = $1 AND is_active = TRUE",
    )
    .bind(user_id)
    .execute(&mut *tx)
    .await?;

    // Insert new slots
    let now = Utc::now();
    for slot in slots {
        let id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO availability_slots \
             (id, user_id, day_of_week, start_time, end_time, is_active, created_at, updated_at) \
             VALUES ($1, $2, $3, $4, $5, TRUE, $6, $6)",
        )
        .bind(id)
        .bind(user_id)
        .bind(slot.day_of_week)
        .bind(slot.start_time)
        .bind(slot.end_time)
        .bind(now)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    // Return the newly created slots
    list_for_user(pool, user_id).await
}

// ---------------------------------------------------------------------------
// Backward compatibility — application-based queries
// ---------------------------------------------------------------------------

/// List slots associated with a caregiver application (legacy flow).
pub async fn list_for_application(
    pool: &PgPool,
    application_id: Uuid,
) -> Result<Vec<AvailabilitySlot>, sqlx::Error> {
    sqlx::query_as::<_, AvailabilitySlot>(
        "SELECT * FROM availability_slots \
         WHERE application_id = $1 AND is_active = TRUE \
         ORDER BY day_of_week, start_time",
    )
    .bind(application_id)
    .fetch_all(pool)
    .await
}

// ---------------------------------------------------------------------------
// Exceptions — date-based overrides
// ---------------------------------------------------------------------------

/// List all exceptions for a user, ordered by date.
pub async fn list_exceptions(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<AvailabilityException>, sqlx::Error> {
    sqlx::query_as::<_, AvailabilityException>(
        "SELECT * FROM availability_exceptions \
         WHERE user_id = $1 \
         ORDER BY exception_date",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

/// Create a single date-based exception (day off or extra availability).
pub async fn create_exception(
    pool: &PgPool,
    user_id: Uuid,
    exception_date: NaiveDate,
    is_available: bool,
    start_time: Option<NaiveTime>,
    end_time: Option<NaiveTime>,
    reason: Option<&str>,
) -> Result<AvailabilityException, sqlx::Error> {
    sqlx::query_as::<_, AvailabilityException>(
        "INSERT INTO availability_exceptions \
         (user_id, exception_date, is_available, start_time, end_time, reason) \
         VALUES ($1, $2, $3, $4, $5, $6) \
         RETURNING *",
    )
    .bind(user_id)
    .bind(exception_date)
    .bind(is_available)
    .bind(start_time)
    .bind(end_time)
    .bind(reason)
    .fetch_one(pool)
    .await
}

/// Delete an exception owned by the given user. Returns true if a row was deleted.
pub async fn delete_exception(
    pool: &PgPool,
    exception_id: Uuid,
    user_id: Uuid,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "DELETE FROM availability_exceptions \
         WHERE id = $1 AND user_id = $2",
    )
    .bind(exception_id)
    .bind(user_id)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}

// ---------------------------------------------------------------------------
// Availability check
// ---------------------------------------------------------------------------

/// Check if a caregiver is available on a specific date and time range.
/// Returns false if a blocking exception exists, or no weekly slot covers the window.
pub async fn check_availability(
    pool: &PgPool,
    user_id: Uuid,
    date: NaiveDate,
    start_time: NaiveTime,
    end_time: NaiveTime,
) -> Result<bool, sqlx::Error> {
    // Check for blocking exception on this date
    let blocked: Option<(bool,)> = sqlx::query_as(
        "SELECT is_available FROM availability_exceptions \
         WHERE user_id = $1 AND exception_date = $2",
    )
    .bind(user_id)
    .bind(date)
    .fetch_optional(pool)
    .await?;

    if let Some((is_available,)) = blocked {
        if !is_available {
            return Ok(false); // Explicitly blocked
        }
        // is_available=true with time range means extra availability — check separately
    }

    // Convert date to DayOfWeek
    let day = match date.weekday() {
        chrono::Weekday::Mon => DayOfWeek::Monday,
        chrono::Weekday::Tue => DayOfWeek::Tuesday,
        chrono::Weekday::Wed => DayOfWeek::Wednesday,
        chrono::Weekday::Thu => DayOfWeek::Thursday,
        chrono::Weekday::Fri => DayOfWeek::Friday,
        chrono::Weekday::Sat => DayOfWeek::Saturday,
        chrono::Weekday::Sun => DayOfWeek::Sunday,
    };

    // Check weekly slot covers the requested time
    let slot_exists: Option<(i64,)> = sqlx::query_as(
        "SELECT COUNT(*) FROM availability_slots \
         WHERE user_id = $1 AND day_of_week = $2 \
         AND start_time <= $3 AND end_time >= $4 \
         AND is_active = TRUE",
    )
    .bind(user_id)
    .bind(day)
    .bind(start_time)
    .bind(end_time)
    .fetch_one(pool)
    .await
    .ok();

    Ok(slot_exists.map(|(c,)| c > 0).unwrap_or(false))
}
