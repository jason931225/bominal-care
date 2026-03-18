// =============================================================================
// Visit / Schedule Server Functions
// Wraps bominal_db::queries::visit for Leptos SSR
// =============================================================================

use leptos::prelude::*;
use leptos::server;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use bominal_types::enums::VisitStatus;
use bominal_types::inputs::VisitInput;
use bominal_types::models::Visit;

// ---------------------------------------------------------------------------
// Response types (serializable across the wire)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisitListResponse {
    pub data: Vec<Visit>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisitDetailResponse {
    pub visit: Visit,
    pub care_plan: Option<bominal_types::models::CarePlan>,
    pub caregiver: Option<bominal_types::models::CaregiverApplication>,
}

// ---------------------------------------------------------------------------
// Server functions
// ---------------------------------------------------------------------------

/// Paginated list of visits with optional filters.
#[server]
pub async fn list_visits(
    caregiver_id: Option<Uuid>,
    care_plan_id: Option<Uuid>,
    status: Option<String>,
    page: i64,
    limit: i64,
) -> Result<VisitListResponse, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let parsed_status = match &status {
        Some(s) => {
            let parsed: VisitStatus = s
                .parse()
                .map_err(|_| ServerFnError::new(format!("Invalid visit status: {s}")))?;
            Some(parsed)
        }
        None => None,
    };

    let filters = bominal_db::queries::visit::VisitFilters {
        caregiver_id,
        care_plan_id,
        status: parsed_status,
        date_from: None,
        date_to: None,
    };
    let params = bominal_types::common::PaginationParams::new(page, limit);

    let result = bominal_db::queries::visit::list_visits(
        &pool,
        &filters,
        params.limit,
        params.offset(),
    )
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(VisitListResponse {
        data: result.data,
        total: result.total,
    })
}

/// Schedule a new visit.
#[server]
pub async fn schedule_visit(
    input: VisitInput,
) -> Result<Visit, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let data = bominal_db::queries::visit::ScheduleVisitData {
        care_plan_id: input.care_plan_id,
        caregiver_id: input.caregiver_id,
        scheduled_start: input.scheduled_start,
        scheduled_end: input.scheduled_end,
        tasks: input.tasks,
        notes: None,
    };

    bominal_db::queries::visit::schedule_visit(&pool, &data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Get a single visit with related care plan and caregiver.
#[server]
pub async fn get_visit(
    id: Uuid,
) -> Result<Option<VisitDetailResponse>, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let result = bominal_db::queries::visit::get_visit(&pool, id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(result.map(|r| VisitDetailResponse {
        visit: r.visit,
        care_plan: r.care_plan,
        caregiver: r.caregiver,
    }))
}

/// Caregiver check-in for a visit.
/// Transitions the visit to IN_PROGRESS and records location.
#[server]
pub async fn check_in(
    visit_id: Uuid,
    latitude: Option<f64>,
    longitude: Option<f64>,
) -> Result<Visit, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let data = bominal_db::queries::visit::CheckInData {
        latitude,
        longitude,
    };

    bominal_db::queries::visit::check_in(&pool, visit_id, &data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Caregiver check-out for a visit.
/// Transitions the visit to COMPLETED and records location and notes.
#[server]
pub async fn check_out(
    visit_id: Uuid,
    latitude: Option<f64>,
    longitude: Option<f64>,
    notes: Option<String>,
) -> Result<Visit, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let data = bominal_db::queries::visit::CheckOutData {
        latitude,
        longitude,
        notes,
    };

    bominal_db::queries::visit::check_out(&pool, visit_id, &data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}
