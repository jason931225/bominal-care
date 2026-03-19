// =============================================================================
// Platform Event — immutable event spine for all significant actions
// =============================================================================

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Sensitivity levels
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Sensitivity {
    Public,
    Internal,
    Confidential,
    Restricted,
}

impl std::fmt::Display for Sensitivity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Public => write!(f, "public"),
            Self::Internal => write!(f, "internal"),
            Self::Confidential => write!(f, "confidential"),
            Self::Restricted => write!(f, "restricted"),
        }
    }
}

// ---------------------------------------------------------------------------
// Event categories
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventCategory {
    Clinical,
    CareOperations,
    SeniorSafety,
    AccessIdentity,
    ConsentPolicy,
    Financial,
    Administrative,
    System,
}

impl std::fmt::Display for EventCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Clinical => write!(f, "clinical"),
            Self::CareOperations => write!(f, "care_operations"),
            Self::SeniorSafety => write!(f, "senior_safety"),
            Self::AccessIdentity => write!(f, "access_identity"),
            Self::ConsentPolicy => write!(f, "consent_policy"),
            Self::Financial => write!(f, "financial"),
            Self::Administrative => write!(f, "administrative"),
            Self::System => write!(f, "system"),
        }
    }
}

// ---------------------------------------------------------------------------
// PlatformEvent struct
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformEvent {
    pub id: Uuid,
    pub actor_user_id: Option<Uuid>,
    pub actor_role: Option<String>,
    pub proxy_user_id: Option<Uuid>,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub action: String,
    pub sensitivity: Sensitivity,
    pub category: EventCategory,
    pub before_state: Option<serde_json::Value>,
    pub after_state: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub hash: Option<String>,
    pub previous_hash: Option<String>,
    pub created_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// EventBuilder — fluent API for constructing events
// ---------------------------------------------------------------------------
pub struct EventBuilder {
    actor_user_id: Option<Uuid>,
    actor_role: Option<String>,
    proxy_user_id: Option<Uuid>,
    entity_type: String,
    entity_id: Uuid,
    action: String,
    sensitivity: Sensitivity,
    category: EventCategory,
    before_state: Option<serde_json::Value>,
    after_state: Option<serde_json::Value>,
    metadata: Option<serde_json::Value>,
    ip_address: Option<String>,
    user_agent: Option<String>,
}

impl EventBuilder {
    pub fn new(entity_type: impl Into<String>, entity_id: Uuid, action: impl Into<String>) -> Self {
        Self {
            actor_user_id: None,
            actor_role: None,
            proxy_user_id: None,
            entity_type: entity_type.into(),
            entity_id,
            action: action.into(),
            sensitivity: Sensitivity::Internal,
            category: EventCategory::Administrative,
            before_state: None,
            after_state: None,
            metadata: None,
            ip_address: None,
            user_agent: None,
        }
    }

    pub fn actor(mut self, user_id: Uuid, role: impl Into<String>) -> Self {
        self.actor_user_id = Some(user_id);
        self.actor_role = Some(role.into());
        self
    }

    pub fn proxy(mut self, proxy_id: Uuid) -> Self {
        self.proxy_user_id = Some(proxy_id);
        self
    }

    pub fn sensitivity(mut self, s: Sensitivity) -> Self {
        self.sensitivity = s;
        self
    }

    pub fn category(mut self, c: EventCategory) -> Self {
        self.category = c;
        self
    }

    pub fn before(mut self, state: serde_json::Value) -> Self {
        self.before_state = Some(state);
        self
    }

    pub fn after(mut self, state: serde_json::Value) -> Self {
        self.after_state = Some(state);
        self
    }

    pub fn metadata(mut self, meta: serde_json::Value) -> Self {
        self.metadata = Some(meta);
        self
    }

    pub fn ip(mut self, ip: impl Into<String>) -> Self {
        self.ip_address = Some(ip.into());
        self
    }

    pub fn user_agent(mut self, ua: impl Into<String>) -> Self {
        self.user_agent = Some(ua.into());
        self
    }

    /// Finalize: produces the insert-ready data (hash computed on insert).
    pub fn build(self) -> NewPlatformEvent {
        NewPlatformEvent {
            actor_user_id: self.actor_user_id,
            actor_role: self.actor_role,
            proxy_user_id: self.proxy_user_id,
            entity_type: self.entity_type,
            entity_id: self.entity_id,
            action: self.action,
            sensitivity: self.sensitivity.to_string(),
            category: self.category.to_string(),
            before_state: self.before_state,
            after_state: self.after_state,
            metadata: self.metadata,
            ip_address: self.ip_address,
            user_agent: self.user_agent,
        }
    }
}

/// Data ready for insertion (hash + previous_hash computed during insert).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewPlatformEvent {
    pub actor_user_id: Option<Uuid>,
    pub actor_role: Option<String>,
    pub proxy_user_id: Option<Uuid>,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub action: String,
    pub sensitivity: String,
    pub category: String,
    pub before_state: Option<serde_json::Value>,
    pub after_state: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

// ---------------------------------------------------------------------------
// Alert types
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub condition: serde_json::Value,
    pub actions: serde_json::Value,
    pub is_active: bool,
    pub cooldown_minutes: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertFiring {
    pub id: Uuid,
    pub rule_id: Uuid,
    pub event_id: Uuid,
    pub actions_taken: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetric {
    pub id: Uuid,
    pub metric_name: String,
    pub metric_value: f64,
    pub labels: Option<serde_json::Value>,
    pub recorded_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrityVerification {
    pub id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub chain_length: i32,
    pub is_valid: bool,
    pub broken_at_event_id: Option<Uuid>,
    pub verified_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    pub id: Uuid,
    pub report_type: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub generated_by: Option<Uuid>,
    pub data: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// Access Policy types
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyEffect {
    Allow,
    Deny,
    RequiresApproval,
    AllowAnonymized,
}

impl std::fmt::Display for PolicyEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Allow => write!(f, "allow"),
            Self::Deny => write!(f, "deny"),
            Self::RequiresApproval => write!(f, "requires_approval"),
            Self::AllowAnonymized => write!(f, "allow_anonymized"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPolicy {
    pub id: Uuid,
    pub role: String,
    pub resource_type: String,
    pub action: String,
    pub scope: String,
    pub effect: String,
    pub conditions: Option<serde_json::Value>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyChangeLog {
    pub id: Uuid,
    pub policy_id: Uuid,
    pub changed_by: Uuid,
    pub old_effect: Option<String>,
    pub new_effect: String,
    pub reason: String,
    pub created_at: DateTime<Utc>,
}
