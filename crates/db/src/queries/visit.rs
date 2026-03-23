// =============================================================================
// Visit Queries
// Ported from packages/db/src/services/visit.service.ts (330 lines)
// =============================================================================

use chrono::{DateTime, Datelike, Duration, NaiveDate, NaiveTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use bominal_types::enums::{DayOfWeek, VisitStatus};
use bominal_types::models::{CarePlan, CaregiverApplication, Visit};
use bominal_types::state_machines::visit_machine;

use super::availability_slot;

// ---------------------------------------------------------------------------
// Input structs
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct ScheduleVisitData {
    pub care_plan_id: Uuid,
    pub caregiver_id: Uuid,
    pub scheduled_start: DateTime<Utc>,
    pub scheduled_end: DateTime<Utc>,
    pub tasks: Option<serde_json::Value>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CheckInData {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct CheckOutData {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct VisitFilters {
    pub caregiver_id: Option<Uuid>,
    pub care_plan_id: Option<Uuid>,
    pub status: Option<VisitStatus>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct VisitWithRelations {
    pub visit: Visit,
    pub care_plan: Option<CarePlan>,
    pub caregiver: Option<CaregiverApplication>,
}

#[derive(Debug, Clone)]
pub struct PaginatedVisits {
    pub data: Vec<Visit>,
    pub total: i64,
}

// ---------------------------------------------------------------------------
// schedule_visit
// ---------------------------------------------------------------------------

pub async fn schedule_visit(
    pool: &PgPool,
    data: &ScheduleVisitData,
) -> Result<Visit, sqlx::Error> {
    let id = Uuid::new_v4();
    let now = Utc::now();

    sqlx::query_as::<_, Visit>(
        "INSERT INTO visits (
           id, care_plan_id, caregiver_id, status, scheduled_start, scheduled_end,
           tasks, notes, created_at, updated_at
         ) VALUES ($1, $2, $3, 'SCHEDULED', $4, $5, $6, $7, $8, $8)
         RETURNING *",
    )
    .bind(id)
    .bind(data.care_plan_id)
    .bind(data.caregiver_id)
    .bind(data.scheduled_start)
    .bind(data.scheduled_end)
    .bind(&data.tasks)
    .bind(&data.notes)
    .bind(now)
    .fetch_one(pool)
    .await
}

// ---------------------------------------------------------------------------
// acknowledge_visit
// ---------------------------------------------------------------------------

pub async fn acknowledge_visit(
    pool: &PgPool,
    id: Uuid,
) -> Result<Visit, sqlx::Error> {
    let existing = sqlx::query_as::<_, Visit>(
        "SELECT * FROM visits WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| sqlx::Error::RowNotFound)?;

    let machine = visit_machine();
    if !machine.can_transition(existing.status, VisitStatus::CaregiverAcknowledged) {
        return Err(sqlx::Error::Protocol(format!(
            "Cannot acknowledge visit in status: {}",
            existing.status
        )));
    }

    let now = Utc::now();
    sqlx::query_as::<_, Visit>(
        "UPDATE visits SET status = 'CAREGIVER_ACKNOWLEDGED', updated_at = $1 WHERE id = $2 RETURNING *",
    )
    .bind(now)
    .bind(id)
    .fetch_one(pool)
    .await
}

// ---------------------------------------------------------------------------
// check_in
// ---------------------------------------------------------------------------

pub async fn check_in(
    pool: &PgPool,
    id: Uuid,
    data: &CheckInData,
) -> Result<Visit, sqlx::Error> {
    let existing = sqlx::query_as::<_, Visit>(
        "SELECT * FROM visits WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| sqlx::Error::RowNotFound)?;

    let machine = visit_machine();
    if !machine.can_transition(existing.status, VisitStatus::InProgress) {
        return Err(sqlx::Error::Protocol(format!(
            "Cannot check in on visit in status: {}",
            existing.status
        )));
    }

    let now = Utc::now();
    sqlx::query_as::<_, Visit>(
        "UPDATE visits
         SET status = 'IN_PROGRESS', actual_start = $1,
             check_in_latitude = $2, check_in_longitude = $3, updated_at = $1
         WHERE id = $4
         RETURNING *",
    )
    .bind(now)
    .bind(data.latitude)
    .bind(data.longitude)
    .bind(id)
    .fetch_one(pool)
    .await
}

// ---------------------------------------------------------------------------
// check_out
// ---------------------------------------------------------------------------

pub async fn check_out(
    pool: &PgPool,
    id: Uuid,
    data: &CheckOutData,
) -> Result<Visit, sqlx::Error> {
    let existing = sqlx::query_as::<_, Visit>(
        "SELECT * FROM visits WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| sqlx::Error::RowNotFound)?;

    let machine = visit_machine();
    if !machine.can_transition(existing.status, VisitStatus::Completed) {
        return Err(sqlx::Error::Protocol(format!(
            "Cannot check out on visit in status: {}",
            existing.status
        )));
    }

    let now = Utc::now();
    sqlx::query_as::<_, Visit>(
        "UPDATE visits
         SET status = 'COMPLETED', actual_end = $1,
             check_out_latitude = $2, check_out_longitude = $3,
             notes = COALESCE($4, notes), updated_at = $1
         WHERE id = $5
         RETURNING *",
    )
    .bind(now)
    .bind(data.latitude)
    .bind(data.longitude)
    .bind(&data.notes)
    .bind(id)
    .fetch_one(pool)
    .await
}

// ---------------------------------------------------------------------------
// mark_missed
// ---------------------------------------------------------------------------

pub async fn mark_missed(
    pool: &PgPool,
    id: Uuid,
) -> Result<Visit, sqlx::Error> {
    let existing = sqlx::query_as::<_, Visit>(
        "SELECT * FROM visits WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| sqlx::Error::RowNotFound)?;

    let machine = visit_machine();
    if !machine.can_transition(existing.status, VisitStatus::Missed) {
        return Err(sqlx::Error::Protocol(format!(
            "Cannot mark visit as missed in status: {}",
            existing.status
        )));
    }

    let now = Utc::now();
    sqlx::query_as::<_, Visit>(
        "UPDATE visits SET status = 'MISSED', updated_at = $1 WHERE id = $2 RETURNING *",
    )
    .bind(now)
    .bind(id)
    .fetch_one(pool)
    .await
}

// ---------------------------------------------------------------------------
// get_visit (with relations)
// ---------------------------------------------------------------------------

pub async fn get_visit(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<VisitWithRelations>, sqlx::Error> {
    let visit = sqlx::query_as::<_, Visit>(
        "SELECT * FROM visits WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    let visit = match visit {
        Some(v) => v,
        None => return Ok(None),
    };

    let care_plan = sqlx::query_as::<_, CarePlan>(
        "SELECT * FROM care_plans WHERE id = $1",
    )
    .bind(visit.care_plan_id)
    .fetch_optional(pool)
    .await?;

    let caregiver = sqlx::query_as::<_, CaregiverApplication>(
        "SELECT * FROM caregiver_applications WHERE id = $1",
    )
    .bind(visit.caregiver_id)
    .fetch_optional(pool)
    .await?;

    Ok(Some(VisitWithRelations {
        visit,
        care_plan,
        caregiver,
    }))
}

// ---------------------------------------------------------------------------
// list_visits (filtered + paginated)
// ---------------------------------------------------------------------------

pub async fn list_visits(
    pool: &PgPool,
    filters: &VisitFilters,
    limit: i64,
    offset: i64,
) -> Result<PaginatedVisits, sqlx::Error> {
    // Build dynamic WHERE clauses. We always bind all five filter values and use
    // ($N IS NULL OR column = $N) so the query plan stays stable.
    let data = sqlx::query_as::<_, Visit>(
        "SELECT * FROM visits
         WHERE ($1::uuid    IS NULL OR caregiver_id    = $1)
           AND ($2::uuid    IS NULL OR care_plan_id    = $2)
           AND ($3::visit_status IS NULL OR status     = $3)
           AND ($4::timestamptz  IS NULL OR scheduled_start >= $4)
           AND ($5::timestamptz  IS NULL OR scheduled_start <= $5)
         ORDER BY scheduled_start ASC
         LIMIT $6 OFFSET $7",
    )
    .bind(filters.caregiver_id)
    .bind(filters.care_plan_id)
    .bind(filters.status)
    .bind(filters.date_from)
    .bind(filters.date_to)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    let (total,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM visits
         WHERE ($1::uuid    IS NULL OR caregiver_id    = $1)
           AND ($2::uuid    IS NULL OR care_plan_id    = $2)
           AND ($3::visit_status IS NULL OR status     = $3)
           AND ($4::timestamptz  IS NULL OR scheduled_start >= $4)
           AND ($5::timestamptz  IS NULL OR scheduled_start <= $5)",
    )
    .bind(filters.caregiver_id)
    .bind(filters.care_plan_id)
    .bind(filters.status)
    .bind(filters.date_from)
    .bind(filters.date_to)
    .fetch_one(pool)
    .await?;

    Ok(PaginatedVisits { data, total })
}

// ---------------------------------------------------------------------------
// get_upcoming_visits
// ---------------------------------------------------------------------------

pub async fn get_upcoming_visits(
    pool: &PgPool,
    caregiver_id: Uuid,
    limit: i64,
) -> Result<Vec<Visit>, sqlx::Error> {
    let now = Utc::now();

    sqlx::query_as::<_, Visit>(
        "SELECT * FROM visits
         WHERE caregiver_id = $1
           AND status IN ('SCHEDULED', 'CAREGIVER_ACKNOWLEDGED', 'NEEDS_REASSIGNMENT')
           AND scheduled_start >= $2
         ORDER BY scheduled_start ASC
         LIMIT $3",
    )
    .bind(caregiver_id)
    .bind(now)
    .bind(limit)
    .fetch_all(pool)
    .await
}

// ---------------------------------------------------------------------------
// Recurring visit generation
// ---------------------------------------------------------------------------

/// Pattern for generating recurring visits over multiple weeks.
pub struct RecurringPattern {
    pub days: Vec<DayOfWeek>,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub service_type: String,
    pub weeks: u32,
    pub start_date: NaiveDate,
}

/// A date that was skipped during recurring generation, with a reason.
#[derive(Debug, Clone, Serialize)]
pub struct SkippedDate {
    pub date: NaiveDate,
    pub reason: String,
}

/// Result of bulk recurring visit generation.
#[derive(Debug, Clone)]
pub struct ScheduleResult {
    pub created: Vec<Visit>,
    pub skipped: Vec<SkippedDate>,
}

/// Convert a `DayOfWeek` enum to its `chrono::Weekday` equivalent.
fn to_chrono_weekday(day: DayOfWeek) -> chrono::Weekday {
    match day {
        DayOfWeek::Monday => chrono::Weekday::Mon,
        DayOfWeek::Tuesday => chrono::Weekday::Tue,
        DayOfWeek::Wednesday => chrono::Weekday::Wed,
        DayOfWeek::Thursday => chrono::Weekday::Thu,
        DayOfWeek::Friday => chrono::Weekday::Fri,
        DayOfWeek::Saturday => chrono::Weekday::Sat,
        DayOfWeek::Sunday => chrono::Weekday::Sun,
    }
}

/// Generate recurring visits for a caregiver over multiple weeks.
///
/// Iterates day-by-day from `pattern.start_date` for `pattern.weeks * 7` days.
/// For each date whose weekday is in `pattern.days`, checks availability and
/// existing visit conflicts, then inserts a visit or records a skip.
pub async fn generate_recurring_visits(
    pool: &PgPool,
    care_plan_id: Uuid,
    caregiver_id: Uuid,
    pattern: &RecurringPattern,
) -> Result<ScheduleResult, sqlx::Error> {
    // Validate inputs
    if pattern.end_time <= pattern.start_time {
        return Err(sqlx::Error::Protocol(
            "종료 시간은 시작 시간 이후여야 합니다".to_string(),
        ));
    }
    if pattern.weeks == 0 || pattern.weeks > 52 {
        return Err(sqlx::Error::Protocol(
            "주 수는 1~52 사이여야 합니다".to_string(),
        ));
    }
    if pattern.days.is_empty() {
        return Err(sqlx::Error::Protocol(
            "근무 요일을 선택하세요".to_string(),
        ));
    }

    // Collect requested weekdays for fast lookup
    let requested_weekdays: std::collections::HashSet<chrono::Weekday> = pattern
        .days
        .iter()
        .map(|d| to_chrono_weekday(*d))
        .collect();

    let total_days = (pattern.weeks * 7) as i64;
    let mut created = Vec::new();
    let mut skipped = Vec::new();

    let mut tx = pool.begin().await?;

    for day_offset in 0..total_days {
        let date = pattern.start_date + Duration::days(day_offset);
        if !requested_weekdays.contains(&date.weekday()) {
            continue;
        }

        // Check caregiver availability (weekly slots + exceptions)
        let available = availability_slot::check_availability(
            pool,
            caregiver_id,
            date,
            pattern.start_time,
            pattern.end_time,
        )
        .await?;

        if !available {
            skipped.push(SkippedDate {
                date,
                reason: "요양보호사 일정 차단".to_string(),
            });
            continue;
        }

        // Build scheduled_start / scheduled_end as UTC DateTime
        let scheduled_start = date
            .and_time(pattern.start_time)
            .and_utc();
        let scheduled_end = date
            .and_time(pattern.end_time)
            .and_utc();

        // Check for existing visit conflict
        let (conflict_count,): (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM visits \
             WHERE caregiver_id = $1 \
               AND scheduled_start < $2 \
               AND scheduled_end > $3 \
               AND status NOT IN ('CANCELLED', 'MISSED')",
        )
        .bind(caregiver_id)
        .bind(scheduled_end)
        .bind(scheduled_start)
        .fetch_one(&mut *tx)
        .await?;

        if conflict_count > 0 {
            skipped.push(SkippedDate {
                date,
                reason: "다른 방문 일정과 겹침".to_string(),
            });
            continue;
        }

        // Insert the visit
        let visit_id = Uuid::new_v4();
        let now = Utc::now();
        let tasks_json = serde_json::json!({ "service_type": pattern.service_type });

        let visit = sqlx::query_as::<_, Visit>(
            "INSERT INTO visits (
               id, care_plan_id, caregiver_id, status, scheduled_start, scheduled_end,
               tasks, created_at, updated_at
             ) VALUES ($1, $2, $3, 'SCHEDULED', $4, $5, $6, $7, $7)
             RETURNING *",
        )
        .bind(visit_id)
        .bind(care_plan_id)
        .bind(caregiver_id)
        .bind(scheduled_start)
        .bind(scheduled_end)
        .bind(tasks_json)
        .bind(now)
        .fetch_one(&mut *tx)
        .await?;

        created.push(visit);
    }

    tx.commit().await?;

    Ok(ScheduleResult { created, skipped })
}

// ---------------------------------------------------------------------------
// mark_needs_reassignment
// ---------------------------------------------------------------------------

/// Mark all scheduled/acknowledged visits for a caregiver on a specific date
/// as NEEDS_REASSIGNMENT. Uses row-level locks to prevent races.
pub async fn mark_needs_reassignment(
    pool: &PgPool,
    caregiver_id: Uuid,
    date: NaiveDate,
) -> Result<Vec<Visit>, sqlx::Error> {
    let mut tx = pool.begin().await?;

    // Find affected visits with row lock
    let visits = sqlx::query_as::<_, Visit>(
        "SELECT * FROM visits \
         WHERE caregiver_id = $1 \
           AND scheduled_start::DATE = $2 \
           AND status IN ('SCHEDULED', 'CAREGIVER_ACKNOWLEDGED') \
         FOR UPDATE",
    )
    .bind(caregiver_id)
    .bind(date)
    .fetch_all(&mut *tx)
    .await?;

    for visit in &visits {
        sqlx::query(
            "UPDATE visits SET status = 'NEEDS_REASSIGNMENT', updated_at = NOW() WHERE id = $1",
        )
        .bind(visit.id)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    Ok(visits)
}
