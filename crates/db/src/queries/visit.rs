// =============================================================================
// Visit Queries
// Ported from packages/db/src/services/visit.service.ts (330 lines)
// =============================================================================

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use bominal_types::enums::VisitStatus;
use bominal_types::models::{CarePlan, CaregiverApplication, Visit};
use bominal_types::state_machines::visit_machine;

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
           AND status IN ('SCHEDULED', 'CAREGIVER_ACKNOWLEDGED')
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
