// =============================================================================
// Notification Handler — evaluates rules and creates DB notifications
// Ported from packages/events/src/handlers/notification.ts
// =============================================================================

use sqlx::PgPool;
use tokio::sync::broadcast;
use uuid::Uuid;

use bominal_types::events::{DomainEvent, RuleAction};

use crate::events::rules::{RuleContextDyn, RuleEngine};

/// Maps rule action severity to the `NotificationType` enum stored in the DB.
/// Schema defines: INFO | WARNING | ALERT | ACTION_REQUIRED | REMINDER
fn severity_to_notification_type(action: &RuleAction) -> &'static str {
    match action {
        RuleAction::Notify { notification_type, .. } => match notification_type.as_str() {
            "INFO" => "INFO",
            "WARNING" => "WARNING",
            "ALERT" => "ALERT",
            "ACTION_REQUIRED" => "ACTION_REQUIRED",
            _ => "INFO",
        },
        RuleAction::Escalate { .. } => "ACTION_REQUIRED",
        RuleAction::CreateTask { .. } => "INFO",
    }
}

/// Builds a human-readable title from the rule action type.
fn build_title(action: &RuleAction) -> &'static str {
    match action {
        RuleAction::Notify { .. } => "Notification",
        RuleAction::Escalate { .. } => "Escalation",
        RuleAction::CreateTask { .. } => "Task",
    }
}

/// Extracts the target user/role and message from a RuleAction.
fn action_targets(action: &RuleAction) -> Vec<(&str, &str)> {
    match action {
        RuleAction::Notify { user_id, message, .. } => {
            vec![(user_id.as_str(), message.as_str())]
        }
        RuleAction::Escalate { to_role, reason } => {
            vec![(to_role.as_str(), reason.as_str())]
        }
        RuleAction::CreateTask {
            assigned_to,
            description,
            ..
        } => {
            vec![(assigned_to.as_str(), description.as_str())]
        }
    }
}

/// Spawns a background task that listens on the event bus, evaluates rules for
/// each incoming event, and inserts notification rows for triggered actions.
pub async fn run_notification_handler(
    mut receiver: broadcast::Receiver<DomainEvent>,
    pool: PgPool,
    rule_engine: &RuleEngine,
    rule_context: &dyn RuleContextDyn,
) {
    loop {
        let event = match receiver.recv().await {
            Ok(event) => event,
            Err(broadcast::error::RecvError::Lagged(skipped)) => {
                tracing::warn!(
                    skipped,
                    "Notification handler lagged behind event bus"
                );
                continue;
            }
            Err(broadcast::error::RecvError::Closed) => {
                tracing::info!("Event bus closed, notification handler shutting down");
                break;
            }
        };

        let results = rule_engine.evaluate(&event, rule_context).await;

        let triggered_actions: Vec<&RuleAction> = results
            .iter()
            .filter(|r| r.triggered)
            .flat_map(|r| &r.actions)
            .collect();

        if triggered_actions.is_empty() {
            continue;
        }

        for action in triggered_actions {
            let notification_type = severity_to_notification_type(action);
            let title = build_title(action);

            for (target, message) in action_targets(action) {
                // Role-based targets (e.g. "role:family") need a separate
                // role-fan-out service to resolve to concrete user IDs.
                if target.starts_with("role:") {
                    tracing::info!(
                        target,
                        event_type = %event.event_type,
                        "Skipping role-based target; role-fan-out service should handle this"
                    );
                    continue;
                }

                let id = Uuid::new_v4();
                let result = sqlx::query(
                    "INSERT INTO notifications (id, user_id, type, title, message, is_read) \
                     VALUES ($1, $2, $3, $4, $5, $6)",
                )
                .bind(id)
                .bind(target)
                .bind(notification_type)
                .bind(title)
                .bind(message)
                .bind(false)
                .execute(&pool)
                .await;

                if let Err(err) = result {
                    tracing::error!(
                        %err,
                        target,
                        event_type = %event.event_type,
                        "Failed to insert notification"
                    );
                }
            }
        }
    }
}
