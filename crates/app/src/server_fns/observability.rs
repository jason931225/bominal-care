// =============================================================================
// Observability server functions — web-family + web-government + mobile-caregiver
// =============================================================================

use leptos::prelude::*;
use leptos::server;
use uuid::Uuid;

use bominal_db::queries::observability::{
    self, CreateSignalData, DashboardStats, PaginatedSignals, SignalFilters,
};
use bominal_types::enums::{ObservabilityEventType, SignalSeverity};
use bominal_types::models::ObservabilitySignal;
use bominal_types::ObservabilitySignalInput;

/// Paginated, filtered list of observability signals.
#[server]
pub async fn list_signals(
    event_type: Option<String>,
    severity: Option<String>,
    person_id: Option<Uuid>,
    acknowledged: Option<bool>,
    page: i64,
    limit: i64,
) -> Result<PaginatedSignals, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let parsed_event_type = event_type
        .as_deref()
        .map(|s| {
            s.parse::<ObservabilityEventType>()
                .map_err(|_| ServerFnError::new(format!("Invalid event_type: {s}")))
        })
        .transpose()?;

    let parsed_severity = severity
        .as_deref()
        .map(|s| {
            s.parse::<SignalSeverity>()
                .map_err(|_| ServerFnError::new(format!("Invalid severity: {s}")))
        })
        .transpose()?;

    let filters = SignalFilters {
        event_type: parsed_event_type,
        severity: parsed_severity,
        subject_person_id: person_id,
        acknowledged,
    };

    let offset = (page.max(1) - 1) * limit.clamp(1, 100);

    observability::list_signals(&pool, &filters, limit.clamp(1, 100), offset)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Create a new observability signal (observation / incident).
#[server]
pub async fn create_signal(
    input: ObservabilitySignalInput,
) -> Result<ObservabilitySignal, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let data = CreateSignalData {
        event_type: input.event_type,
        severity: input.severity,
        subject_person_id: input.subject_person_id,
        actor_user_id: None,
        entity_type: input.entity_type,
        entity_id: input.entity_id,
        message: input.message,
        metadata: input.metadata,
    };

    observability::create_signal(&pool, &data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Acknowledge an observability signal.
#[server]
pub async fn acknowledge_signal(
    signal_id: Uuid,
    acknowledged_by: Uuid,
) -> Result<ObservabilitySignal, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    observability::acknowledge_signal(&pool, signal_id, acknowledged_by)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Get aggregated dashboard statistics for observability signals.
#[server]
pub async fn get_dashboard_stats() -> Result<DashboardStats, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    observability::get_dashboard_stats(&pool, None)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}
