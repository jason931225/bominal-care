// =============================================================================
// Care Plan Queries
// Ported from packages/db/src/services/care-plan.service.ts
// =============================================================================

use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use bominal_types::enums::CarePlanStatus;
use bominal_types::models::{CarePlan, DailyObservation, Visit};
use bominal_types::state_machines::care_plan_machine;

// ---------------------------------------------------------------------------
// Input structs
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct CreateCarePlanData {
    pub senior_id: Uuid,
    pub provider_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub start_date: Option<chrono::DateTime<Utc>>,
    pub end_date: Option<chrono::DateTime<Utc>>,
    pub goals: Option<serde_json::Value>,
    pub created_by: Option<Uuid>,
}

#[derive(Debug, Clone, Default)]
pub struct UpdateCarePlanData {
    pub title: Option<String>,
    pub description: Option<String>,
    pub start_date: Option<chrono::DateTime<Utc>>,
    pub end_date: Option<chrono::DateTime<Utc>>,
    pub goals: Option<serde_json::Value>,
    pub provider_id: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

// ---------------------------------------------------------------------------
// Result structs
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct PaginatedCarePlans {
    pub data: Vec<CarePlan>,
    pub total: i64,
}

#[derive(Debug, Clone)]
pub struct CarePlanWithDetails {
    pub care_plan: CarePlan,
    pub visits: Vec<Visit>,
    pub daily_observations: Vec<DailyObservation>,
}

// ---------------------------------------------------------------------------
// Queries
// ---------------------------------------------------------------------------

pub async fn create_care_plan(
    pool: &PgPool,
    data: &CreateCarePlanData,
) -> Result<CarePlan, sqlx::Error> {
    let id = Uuid::new_v4();
    let now = Utc::now();

    sqlx::query_as::<_, CarePlan>(
        r#"INSERT INTO care_plans (
             id, senior_id, provider_id, status, title, description,
             start_date, end_date, goals, created_by, created_at, updated_at
           ) VALUES ($1,$2,$3,'DRAFT',$4,$5,$6,$7,$8,$9,$10,$10)
           RETURNING *"#,
    )
    .bind(id)
    .bind(data.senior_id)
    .bind(data.provider_id)
    .bind(&data.title)
    .bind(&data.description)
    .bind(data.start_date)
    .bind(data.end_date)
    .bind(&data.goals)
    .bind(data.created_by)
    .bind(now)
    .fetch_one(pool)
    .await
}

pub async fn activate_care_plan(
    pool: &PgPool,
    id: Uuid,
) -> Result<CarePlan, sqlx::Error> {
    let existing = sqlx::query_as::<_, CarePlan>(
        "SELECT * FROM care_plans WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or(sqlx::Error::RowNotFound)?;

    let machine = care_plan_machine();
    if !machine.can_transition(existing.status, CarePlanStatus::Active) {
        return Err(sqlx::Error::Protocol(format!(
            "Cannot activate care plan in status: {}",
            existing.status
        )));
    }

    sqlx::query_as::<_, CarePlan>(
        r#"UPDATE care_plans
           SET status = 'ACTIVE', updated_at = NOW()
           WHERE id = $1
           RETURNING *"#,
    )
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn update_care_plan(
    pool: &PgPool,
    id: Uuid,
    data: &UpdateCarePlanData,
) -> Result<CarePlan, sqlx::Error> {
    // Verify existence
    let _existing = sqlx::query_as::<_, CarePlan>(
        "SELECT * FROM care_plans WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or(sqlx::Error::RowNotFound)?;

    sqlx::query_as::<_, CarePlan>(
        r#"UPDATE care_plans SET
             title       = COALESCE($1, title),
             description = COALESCE($2, description),
             start_date  = COALESCE($3, start_date),
             end_date    = COALESCE($4, end_date),
             goals       = COALESCE($5, goals),
             provider_id = COALESCE($6, provider_id),
             updated_by  = COALESCE($7, updated_by),
             updated_at  = NOW()
           WHERE id = $8
           RETURNING *"#,
    )
    .bind(&data.title)
    .bind(&data.description)
    .bind(data.start_date)
    .bind(data.end_date)
    .bind(&data.goals)
    .bind(data.provider_id)
    .bind(data.updated_by)
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn get_care_plan(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<CarePlanWithDetails>, sqlx::Error> {
    let care_plan = sqlx::query_as::<_, CarePlan>(
        "SELECT * FROM care_plans WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    let care_plan = match care_plan {
        Some(cp) => cp,
        None => return Ok(None),
    };

    let visits = sqlx::query_as::<_, Visit>(
        "SELECT * FROM visits WHERE care_plan_id = $1 ORDER BY scheduled_start ASC",
    )
    .bind(id)
    .fetch_all(pool)
    .await?;

    let daily_observations = sqlx::query_as::<_, DailyObservation>(
        "SELECT * FROM daily_observations WHERE care_plan_id = $1 ORDER BY date DESC",
    )
    .bind(id)
    .fetch_all(pool)
    .await?;

    Ok(Some(CarePlanWithDetails {
        care_plan,
        visits,
        daily_observations,
    }))
}

pub async fn list_care_plans(
    pool: &PgPool,
    senior_id: Uuid,
    limit: i64,
    offset: i64,
) -> Result<PaginatedCarePlans, sqlx::Error> {
    let data = sqlx::query_as::<_, CarePlan>(
        r#"SELECT * FROM care_plans
           WHERE senior_id = $1
           ORDER BY created_at DESC
           LIMIT $2 OFFSET $3"#,
    )
    .bind(senior_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    let row: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM care_plans WHERE senior_id = $1",
    )
    .bind(senior_id)
    .fetch_one(pool)
    .await?;

    Ok(PaginatedCarePlans {
        data,
        total: row.0,
    })
}
