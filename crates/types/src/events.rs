// =============================================================================
// Domain Events — 13 event types + DomainEvent struct
// Ported from packages/events/src/types.ts
// =============================================================================

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// DomainEventType — 13 variants (12 from observability + 1 generic)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum DomainEventType {
    VisitCompleted,
    VisitMissed,
    MedicationTaken,
    MedicationMissed,
    MealDelivered,
    MealFailed,
    TransportCompleted,
    TransportFailed,
    SymptomReported,
    IncidentCreated,
    EligibilityStatusChanged,
    ReferralUpdated,
    /// Catch-all for internal plumbing
    Custom,
}

// ---------------------------------------------------------------------------
// DomainEvent — the envelope for all events
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainEvent {
    pub id: String,
    pub event_type: DomainEventType,
    pub subject_person_id: Option<String>,
    pub actor_user_id: Option<String>,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub message: String,
    pub metadata: Option<serde_json::Value>,
    pub occurred_at: DateTime<Utc>,
}

impl DomainEvent {
    pub fn new(
        event_type: DomainEventType,
        message: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            event_type,
            subject_person_id: None,
            actor_user_id: None,
            entity_type: None,
            entity_id: None,
            message: message.into(),
            metadata: None,
            occurred_at: Utc::now(),
        }
    }

    pub fn with_subject(mut self, person_id: impl Into<String>) -> Self {
        self.subject_person_id = Some(person_id.into());
        self
    }

    pub fn with_actor(mut self, user_id: impl Into<String>) -> Self {
        self.actor_user_id = Some(user_id.into());
        self
    }

    pub fn with_entity(
        mut self,
        entity_type: impl Into<String>,
        entity_id: impl Into<String>,
    ) -> Self {
        self.entity_type = Some(entity_type.into());
        self.entity_id = Some(entity_id.into());
        self
    }

    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

// ---------------------------------------------------------------------------
// Rule Engine types (ported from packages/events/src/rules/engine.ts)
// ---------------------------------------------------------------------------

/// Action to take when a rule fires.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RuleAction {
    Notify {
        user_id: String,
        title: String,
        message: String,
        notification_type: String,
    },
    Escalate {
        to_role: String,
        reason: String,
    },
    CreateTask {
        assigned_to: String,
        title: String,
        description: String,
    },
}

/// Result of evaluating a rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleResult {
    pub rule_name: String,
    pub triggered: bool,
    pub actions: Vec<RuleAction>,
}
