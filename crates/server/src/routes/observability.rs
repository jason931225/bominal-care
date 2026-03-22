// =============================================================================
// Observability Routes — GET /observability/signals, GET /observability/dashboard
// =============================================================================

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{auth::{extractor::AuthUser, permission::require_permission}, AppState};
use bominal_types::rbac::{Resource, Action};
use bominal_db::queries::observability;
use bominal_types::{
    ApiResponse, ObservabilityEventType, ObservabilitySignal, PaginationMeta, PaginationParams,
    SignalSeverity,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/signals", get(list_signals))
        .route("/dashboard", get(get_dashboard))
}

#[derive(Debug, Deserialize)]
struct SignalQueryParams {
    #[serde(flatten)]
    pagination: PaginationParams,
    event_type: Option<ObservabilityEventType>,
    severity: Option<SignalSeverity>,
    subject_person_id: Option<Uuid>,
    acknowledged: Option<bool>,
}

/// GET /api/observability/signals?page=1&limit=20
async fn list_signals(
    State(state): State<AppState>,
    user: AuthUser,
    Query(query): Query<SignalQueryParams>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Observability, Action::List) {
        return e.into_response();
    }

    let params = PaginationParams::new(query.pagination.page, query.pagination.limit);

    let filters = observability::SignalFilters {
        event_type: query.event_type,
        severity: query.severity,
        subject_person_id: query.subject_person_id,
        acknowledged: query.acknowledged,
    };

    match observability::list_signals(&state.pool, &filters, params.limit, params.offset()).await {
        Ok(result) => {
            let meta = PaginationMeta::new(result.total, params.page, params.limit);
            Json(ApiResponse::success_with_meta(result.data, meta)).into_response()
        }
        Err(e) => {
            tracing::error!("DB error listing observability signals: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}

/// Summary stats for the observability dashboard.
#[derive(Debug, Clone, Serialize)]
struct DashboardSummary {
    total_visits_today: i64,
    total_medications_today: i64,
    total_incidents: i64,
    recent_signals: Vec<ObservabilitySignal>,
}

/// GET /api/observability/dashboard
async fn get_dashboard(
    State(state): State<AppState>,
    user: AuthUser,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Observability, Action::Read) {
        return e.into_response();
    }

    // Count today's visit-related signals
    let total_visits_today = count_signals_by_type(
        &state.pool,
        &[
            ObservabilityEventType::VisitCompleted,
            ObservabilityEventType::VisitMissed,
        ],
    )
    .await;

    // Count today's medication-related signals
    let total_medications_today = count_signals_by_type(
        &state.pool,
        &[
            ObservabilityEventType::MedicationTaken,
            ObservabilityEventType::MedicationMissed,
        ],
    )
    .await;

    // Count total incidents
    let total_incidents =
        count_signals_by_type(&state.pool, &[ObservabilityEventType::IncidentCreated]).await;

    // Fetch recent signals (last 10)
    let recent_filters = observability::SignalFilters {
        event_type: None,
        severity: None,
        subject_person_id: None,
        acknowledged: None,
    };
    let recent_signals = observability::list_signals(&state.pool, &recent_filters, 10, 0)
        .await
        .map(|r| r.data)
        .unwrap_or_default();

    let summary = DashboardSummary {
        total_visits_today,
        total_medications_today,
        total_incidents,
        recent_signals,
    };

    Json(ApiResponse::success(summary)).into_response()
}

/// Helper: count signals matching any of the given event types.
async fn count_signals_by_type(
    pool: &sqlx::PgPool,
    event_types: &[ObservabilityEventType],
) -> i64 {
    if event_types.is_empty() {
        return 0;
    }

    // Build a simple OR filter using the first event type, then sum.
    // For simplicity, query each type and sum.
    let mut total: i64 = 0;
    for event_type in event_types {
        let filters = observability::SignalFilters {
            event_type: Some(*event_type),
            severity: None,
            subject_person_id: None,
            acknowledged: None,
        };
        if let Ok(result) = observability::list_signals(pool, &filters, 0, 0).await {
            total += result.total;
        }
    }
    total
}
