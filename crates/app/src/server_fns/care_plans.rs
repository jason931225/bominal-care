// =============================================================================
// Care Plan Server Functions
// Wraps bominal_db::queries::care_plan for Leptos SSR
// =============================================================================

use leptos::prelude::*;
use leptos::server;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use bominal_types::inputs::CarePlanInput;
use bominal_types::models::{CarePlan, DailyObservation, Visit};

// ---------------------------------------------------------------------------
// Response types (serializable across the wire)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarePlanListResponse {
    pub data: Vec<CarePlan>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarePlanDetailResponse {
    pub care_plan: CarePlan,
    pub visits: Vec<Visit>,
    pub daily_observations: Vec<DailyObservation>,
}

// ---------------------------------------------------------------------------
// Server functions
// ---------------------------------------------------------------------------

/// Paginated list of care plans for a senior.
#[server]
pub async fn list_care_plans(
    senior_id: Uuid,
    page: i64,
    limit: i64,
) -> Result<CarePlanListResponse, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let params = bominal_types::common::PaginationParams::new(page, limit);

    let result = bominal_db::queries::care_plan::list_care_plans(
        &pool,
        senior_id,
        params.limit,
        params.offset(),
    )
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(CarePlanListResponse {
        data: result.data,
        total: result.total,
    })
}

/// Create a new draft care plan.
#[server]
pub async fn create_care_plan(
    input: CarePlanInput,
) -> Result<CarePlan, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let session = use_context::<bominal_types::inputs::SessionUser>()
        .ok_or_else(|| ServerFnError::new("Not authenticated"))?;

    let data = bominal_db::queries::care_plan::CreateCarePlanData {
        senior_id: input.senior_id,
        provider_id: input.provider_id,
        title: input.title,
        description: input.description,
        start_date: input.start_date,
        end_date: input.end_date,
        goals: input.goals,
        created_by: Some(session.id),
    };

    bominal_db::queries::care_plan::create_care_plan(&pool, &data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Get a single care plan with its visits and daily observations.
#[server]
pub async fn get_care_plan(
    id: Uuid,
) -> Result<Option<CarePlanDetailResponse>, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let result = bominal_db::queries::care_plan::get_care_plan(&pool, id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(result.map(|r| CarePlanDetailResponse {
        care_plan: r.care_plan,
        visits: r.visits,
        daily_observations: r.daily_observations,
    }))
}

/// Update a care plan (title, description, dates, goals, provider).
#[server]
pub async fn update_care_plan(
    id: Uuid,
    title: Option<String>,
    description: Option<String>,
    start_date: Option<chrono::DateTime<chrono::Utc>>,
    end_date: Option<chrono::DateTime<chrono::Utc>>,
    goals: Option<serde_json::Value>,
    provider_id: Option<Uuid>,
) -> Result<CarePlan, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let session = use_context::<bominal_types::inputs::SessionUser>()
        .ok_or_else(|| ServerFnError::new("Not authenticated"))?;

    let data = bominal_db::queries::care_plan::UpdateCarePlanData {
        title,
        description,
        start_date,
        end_date,
        goals,
        provider_id,
        updated_by: Some(session.id),
    };

    bominal_db::queries::care_plan::update_care_plan(&pool, id, &data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Activate a care plan. Transitions from DRAFT to ACTIVE.
#[server]
pub async fn activate_care_plan(
    id: Uuid,
) -> Result<CarePlan, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    bominal_db::queries::care_plan::activate_care_plan(&pool, id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}
