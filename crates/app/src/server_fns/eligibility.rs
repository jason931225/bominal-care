// =============================================================================
// Eligibility server functions — web-government portal
// =============================================================================

use leptos::prelude::*;
use leptos::server;
use uuid::Uuid;

use bominal_types::enums::EligibilityCaseStatus;
use bominal_types::models::{ApprovalStep, EligibilityCase};
use bominal_types::state_machines::eligibility_case_machine;

/// Paginated list of eligibility cases with optional status and program filters.
#[server]
pub async fn list_eligibility_cases(
    status: Option<String>,
    program: Option<String>,
    page: i64,
    limit: i64,
) -> Result<PaginatedEligibilityCases, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let parsed_status = status
        .as_deref()
        .map(|s| {
            s.parse::<EligibilityCaseStatus>()
                .map_err(|_| ServerFnError::new(format!("Invalid status: {s}")))
        })
        .transpose()?;

    let offset = (page.max(1) - 1) * limit.clamp(1, 100);
    let clamped_limit = limit.clamp(1, 100);

    let data = sqlx::query_as::<_, EligibilityCase>(
        "SELECT * FROM eligibility_cases
         WHERE ($1::eligibility_case_status IS NULL OR status = $1)
           AND ($2::text IS NULL OR program_name = $2)
         ORDER BY created_at DESC
         LIMIT $3 OFFSET $4",
    )
    .bind(parsed_status)
    .bind(&program)
    .bind(clamped_limit)
    .bind(offset)
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM eligibility_cases
         WHERE ($1::eligibility_case_status IS NULL OR status = $1)
           AND ($2::text IS NULL OR program_name = $2)",
    )
    .bind(parsed_status)
    .bind(&program)
    .fetch_one(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(PaginatedEligibilityCases {
        data,
        total: total.0,
    })
}

/// Get a single eligibility case with its approval steps.
#[server]
pub async fn get_eligibility_case(
    id: Uuid,
) -> Result<Option<EligibilityCaseWithSteps>, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let case = sqlx::query_as::<_, EligibilityCase>(
        "SELECT * FROM eligibility_cases WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    let case = match case {
        Some(c) => c,
        None => return Ok(None),
    };

    let steps = sqlx::query_as::<_, ApprovalStep>(
        "SELECT * FROM approval_steps WHERE case_id = $1 ORDER BY step_order ASC",
    )
    .bind(id)
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(Some(EligibilityCaseWithSteps { case, steps }))
}

/// Transition an eligibility case to a new status with optional notes.
#[server]
pub async fn update_eligibility_status(
    id: Uuid,
    status: EligibilityCaseStatus,
    notes: Option<String>,
) -> Result<EligibilityCase, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let existing = sqlx::query_as::<_, EligibilityCase>(
        "SELECT * FROM eligibility_cases WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?
    .ok_or_else(|| ServerFnError::new("Eligibility case not found"))?;

    let machine = eligibility_case_machine();
    if !machine.can_transition(existing.status, status) {
        return Err(ServerFnError::new(format!(
            "Invalid status transition: {} -> {}",
            existing.status, status,
        )));
    }

    let now = chrono::Utc::now();

    let determination_date = if matches!(
        status,
        EligibilityCaseStatus::Approved | EligibilityCaseStatus::Denied
    ) {
        Some(now)
    } else {
        existing.determination_date
    };

    let denial_reason = if status == EligibilityCaseStatus::Denied {
        notes.clone().or(existing.denial_reason)
    } else {
        existing.denial_reason
    };

    let final_notes = notes.or(existing.notes);

    sqlx::query_as::<_, EligibilityCase>(
        "UPDATE eligibility_cases
         SET status = $1,
             notes = $2,
             denial_reason = $3,
             determination_date = $4,
             updated_at = $5
         WHERE id = $6
         RETURNING *",
    )
    .bind(status)
    .bind(&final_notes)
    .bind(&denial_reason)
    .bind(determination_date)
    .bind(now)
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))
}

// ---------------------------------------------------------------------------
// Response types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PaginatedEligibilityCases {
    pub data: Vec<EligibilityCase>,
    pub total: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EligibilityCaseWithSteps {
    pub case: EligibilityCase,
    pub steps: Vec<ApprovalStep>,
}
