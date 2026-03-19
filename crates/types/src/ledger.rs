// =============================================================================
// Ledger Types — append-only history entries
// =============================================================================

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LedgerAction {
    Created,
    Modified,
    Cancelled,
    Approved,
    Rejected,
}

impl std::fmt::Display for LedgerAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Created => write!(f, "created"),
            Self::Modified => write!(f, "modified"),
            Self::Cancelled => write!(f, "cancelled"),
            Self::Approved => write!(f, "approved"),
            Self::Rejected => write!(f, "rejected"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActorType {
    #[serde(rename = "self")]
    SelfActor,
    Family,
    Caregiver,
    MedicalProxy,
    PharmacistProxy,
    Provider,
    System,
}

impl std::fmt::Display for ActorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SelfActor => write!(f, "self"),
            Self::Family => write!(f, "family"),
            Self::Caregiver => write!(f, "caregiver"),
            Self::MedicalProxy => write!(f, "medical_proxy"),
            Self::PharmacistProxy => write!(f, "pharmacist_proxy"),
            Self::Provider => write!(f, "provider"),
            Self::System => write!(f, "system"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationLedgerEntry {
    pub id: Uuid,
    pub medication_id: Uuid,
    pub version: i32,
    pub action: String,
    pub actor_user_id: Uuid,
    pub actor_type: String,
    pub data: serde_json::Value,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppointmentLedgerEntry {
    pub id: Uuid,
    pub appointment_id: Uuid,
    pub version: i32,
    pub action: String,
    pub actor_user_id: Uuid,
    pub actor_type: String,
    pub data: serde_json::Value,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarePlanLedgerEntry {
    pub id: Uuid,
    pub care_plan_id: Uuid,
    pub version: i32,
    pub action: String,
    pub actor_user_id: Uuid,
    pub actor_type: String,
    pub data: serde_json::Value,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Input for creating a new ledger entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewLedgerEntry {
    pub action: LedgerAction,
    pub actor_user_id: Uuid,
    pub actor_type: ActorType,
    pub data: serde_json::Value,
    pub reason: Option<String>,
}
