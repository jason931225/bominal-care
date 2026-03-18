// =============================================================================
// Observability queries — ported from observability.service.ts
// =============================================================================

use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use bominal_types::enums::{ObservabilityEventType, SignalSeverity};
use bominal_types::models::ObservabilitySignal;

// ---------------------------------------------------------------------------
// Input / output types
// ---------------------------------------------------------------------------

pub struct CreateSignalData {
    pub event_type: ObservabilityEventType,
    pub severity: Option<SignalSeverity>,
    pub subject_person_id: Option<Uuid>,
    pub actor_user_id: Option<Uuid>,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub message: String,
    pub metadata: Option<serde_json::Value>,
}

pub struct SignalFilters {
    pub event_type: Option<ObservabilityEventType>,
    pub severity: Option<SignalSeverity>,
    pub subject_person_id: Option<Uuid>,
    pub acknowledged: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedSignals {
    pub data: Vec<ObservabilitySignal>,
    pub total: i64,
}

#[derive(Debug, Clone, FromRow)]
struct EventTypeCount {
    pub event_type: ObservabilityEventType,
    pub count: i64,
}

#[derive(Debug, Clone, FromRow)]
struct SeverityCount {
    pub severity: SignalSeverity,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTypeBreakdown {
    pub event_type: ObservabilityEventType,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeverityBreakdown {
    pub severity: SignalSeverity,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardStats {
    pub by_event_type: Vec<EventTypeBreakdown>,
    pub by_severity: Vec<SeverityBreakdown>,
    pub total: i64,
    pub unacknowledged: i64,
}

// ---------------------------------------------------------------------------
// Queries
// ---------------------------------------------------------------------------

pub async fn create_signal(
    pool: &PgPool,
    data: &CreateSignalData,
) -> Result<ObservabilitySignal, sqlx::Error> {
    let id = Uuid::new_v4();
    let now = Utc::now();
    let severity = data.severity.unwrap_or(SignalSeverity::Info);

    sqlx::query_as::<_, ObservabilitySignal>(
        "INSERT INTO observability_signals (
           id, event_type, severity, subject_person_id, actor_user_id,
           entity_type, entity_id, message, metadata, created_at
         ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)
         RETURNING *",
    )
    .bind(id)
    .bind(data.event_type)
    .bind(severity)
    .bind(data.subject_person_id)
    .bind(data.actor_user_id)
    .bind(&data.entity_type)
    .bind(&data.entity_id)
    .bind(&data.message)
    .bind(&data.metadata)
    .bind(now)
    .fetch_one(pool)
    .await
}

pub async fn acknowledge_signal(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
) -> Result<ObservabilitySignal, sqlx::Error> {
    // Verify existence
    sqlx::query_as::<_, ObservabilitySignal>(
        "SELECT * FROM observability_signals WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| sqlx::Error::RowNotFound)?;

    let now = Utc::now();

    sqlx::query_as::<_, ObservabilitySignal>(
        "UPDATE observability_signals
         SET acknowledged_at = $1, acknowledged_by = $2
         WHERE id = $3
         RETURNING *",
    )
    .bind(now)
    .bind(user_id)
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn list_signals(
    pool: &PgPool,
    filters: &SignalFilters,
    limit: i64,
    offset: i64,
) -> Result<PaginatedSignals, sqlx::Error> {
    // Use the NULL-means-no-filter pattern for typed filters.
    // The acknowledged flag needs special handling (IS NULL / IS NOT NULL).
    let ack_is_null = match filters.acknowledged {
        Some(false) => true,
        _ => false,
    };
    let ack_is_not_null = match filters.acknowledged {
        Some(true) => true,
        _ => false,
    };

    let data = sqlx::query_as::<_, ObservabilitySignal>(
        "SELECT * FROM observability_signals
         WHERE ($1::observability_event_type IS NULL OR event_type = $1)
           AND ($2::signal_severity IS NULL OR severity = $2)
           AND ($3::uuid IS NULL OR subject_person_id = $3)
           AND (NOT $4 OR acknowledged_at IS NULL)
           AND (NOT $5 OR acknowledged_at IS NOT NULL)
         ORDER BY created_at DESC
         LIMIT $6 OFFSET $7",
    )
    .bind(filters.event_type)
    .bind(filters.severity)
    .bind(filters.subject_person_id)
    .bind(ack_is_null)
    .bind(ack_is_not_null)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM observability_signals
         WHERE ($1::observability_event_type IS NULL OR event_type = $1)
           AND ($2::signal_severity IS NULL OR severity = $2)
           AND ($3::uuid IS NULL OR subject_person_id = $3)
           AND (NOT $4 OR acknowledged_at IS NULL)
           AND (NOT $5 OR acknowledged_at IS NOT NULL)",
    )
    .bind(filters.event_type)
    .bind(filters.severity)
    .bind(filters.subject_person_id)
    .bind(ack_is_null)
    .bind(ack_is_not_null)
    .fetch_one(pool)
    .await?;

    Ok(PaginatedSignals {
        data,
        total: total.0,
    })
}

pub async fn get_dashboard_stats(
    pool: &PgPool,
    subject_person_id: Option<Uuid>,
) -> Result<DashboardStats, sqlx::Error> {
    let by_event_type_rows = sqlx::query_as::<_, EventTypeCount>(
        "SELECT event_type, COUNT(*) AS count FROM observability_signals
         WHERE ($1::uuid IS NULL OR subject_person_id = $1)
         GROUP BY event_type ORDER BY count DESC",
    )
    .bind(subject_person_id)
    .fetch_all(pool)
    .await?;

    let by_severity_rows = sqlx::query_as::<_, SeverityCount>(
        "SELECT severity, COUNT(*) AS count FROM observability_signals
         WHERE ($1::uuid IS NULL OR subject_person_id = $1)
         GROUP BY severity ORDER BY count DESC",
    )
    .bind(subject_person_id)
    .fetch_all(pool)
    .await?;

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM observability_signals
         WHERE ($1::uuid IS NULL OR subject_person_id = $1)",
    )
    .bind(subject_person_id)
    .fetch_one(pool)
    .await?;

    let unacknowledged: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM observability_signals
         WHERE ($1::uuid IS NULL OR subject_person_id = $1)
           AND acknowledged_at IS NULL",
    )
    .bind(subject_person_id)
    .fetch_one(pool)
    .await?;

    let by_event_type = by_event_type_rows
        .into_iter()
        .map(|r| EventTypeBreakdown {
            event_type: r.event_type,
            count: r.count,
        })
        .collect();

    let by_severity = by_severity_rows
        .into_iter()
        .map(|r| SeverityBreakdown {
            severity: r.severity,
            count: r.count,
        })
        .collect();

    Ok(DashboardStats {
        by_event_type,
        by_severity,
        total: total.0,
        unacknowledged: unacknowledged.0,
    })
}
