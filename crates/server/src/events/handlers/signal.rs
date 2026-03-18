// =============================================================================
// Signal Handler — records observability signals for every domain event
// Ported from packages/events/src/handlers/signal.ts
// =============================================================================

use sqlx::PgPool;
use tokio::sync::broadcast;
use uuid::Uuid;

use bominal_types::events::{DomainEvent, DomainEventType};

/// Maps every `DomainEventType` to the observability_event_type string stored
/// in the database.
fn to_observability_event_type(event_type: DomainEventType) -> &'static str {
    match event_type {
        DomainEventType::VisitCompleted => "VISIT_COMPLETED",
        DomainEventType::VisitMissed => "VISIT_MISSED",
        DomainEventType::MedicationTaken => "MEDICATION_TAKEN",
        DomainEventType::MedicationMissed => "MEDICATION_MISSED",
        DomainEventType::MealDelivered => "MEAL_DELIVERED",
        DomainEventType::MealFailed => "MEAL_FAILED",
        DomainEventType::TransportCompleted => "TRANSPORT_COMPLETED",
        DomainEventType::TransportFailed => "TRANSPORT_FAILED",
        DomainEventType::SymptomReported => "SYMPTOM_REPORTED",
        DomainEventType::IncidentCreated => "INCIDENT_CREATED",
        DomainEventType::EligibilityStatusChanged => "ELIGIBILITY_STATUS_CHANGED",
        DomainEventType::ReferralUpdated => "REFERRAL_UPDATED",
        DomainEventType::Custom => "CUSTOM",
    }
}

/// Determines signal severity from event metadata when present, otherwise
/// derives a sensible default based on the event type.
fn resolve_signal_severity(event: &DomainEvent) -> &'static str {
    // Check metadata for an explicit severity value.
    if let Some(severity_val) = event.metadata.as_ref().and_then(|m| m.get("severity")) {
        if let Some(s) = severity_val.as_str() {
            let upper = s.to_uppercase();
            if upper == "CRITICAL" {
                return "CRITICAL";
            }
            if upper == "ALERT" || upper == "HIGH" {
                return "ALERT";
            }
            if upper == "WARNING" || upper == "MEDIUM" {
                return "WARNING";
            }
        }
    }

    // Default by event type: failures and misses are warnings.
    match event.event_type {
        DomainEventType::VisitMissed
        | DomainEventType::MedicationMissed
        | DomainEventType::MealFailed
        | DomainEventType::TransportFailed => "WARNING",
        _ => "INFO",
    }
}

/// Builds a human-readable signal message from the event.
fn build_signal_message(event: &DomainEvent) -> String {
    let base = format!("Event \"{}\" recorded", event.event_type);
    match &event.subject_person_id {
        Some(person_id) => format!("{} for personId={}", base, person_id),
        None => base,
    }
}

/// Spawns a background task that listens on the event bus and inserts an
/// `observability_signals` row for every domain event received.
pub async fn run_signal_handler(
    mut receiver: broadcast::Receiver<DomainEvent>,
    pool: PgPool,
) {
    loop {
        let event = match receiver.recv().await {
            Ok(event) => event,
            Err(broadcast::error::RecvError::Lagged(skipped)) => {
                tracing::warn!(skipped, "Signal handler lagged behind event bus");
                continue;
            }
            Err(broadcast::error::RecvError::Closed) => {
                tracing::info!("Event bus closed, signal handler shutting down");
                break;
            }
        };

        let observability_type = to_observability_event_type(event.event_type);
        let severity = resolve_signal_severity(&event);
        let message = build_signal_message(&event);
        let metadata_json = event
            .metadata
            .as_ref()
            .and_then(|m| serde_json::to_string(m).ok());

        let id = Uuid::new_v4();
        let result = sqlx::query(
            "INSERT INTO observability_signals \
               (id, event_type, severity, subject_person_id, actor_user_id, \
                entity_type, entity_id, message, metadata) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
        )
        .bind(id)
        .bind(observability_type)
        .bind(severity)
        .bind(event.subject_person_id.as_deref())
        .bind(event.actor_user_id.as_deref())
        .bind(event.entity_type.as_deref())
        .bind(event.entity_id.as_deref())
        .bind(&message)
        .bind(metadata_json.as_deref())
        .execute(&pool)
        .await;

        if let Err(err) = result {
            tracing::error!(
                %err,
                event_type = %event.event_type,
                "Failed to insert observability signal"
            );
        }
    }
}
