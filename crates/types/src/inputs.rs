// =============================================================================
// Input / Validation structs — replace Zod schemas
// Ported from packages/types/src/*.ts
// =============================================================================

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::enums::*;

// ---------------------------------------------------------------------------
// Auth inputs
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct RegisterInput {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    #[validate(length(min = 1))]
    pub name: String,
    pub phone: Option<String>,
    pub role: UserRole,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct LoginInput {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionUser {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub role: UserRole,
    pub kyc_level: KycLevel,
    pub tenant_id: Option<Uuid>,
}

// ---------------------------------------------------------------------------
// Profile inputs
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct PersonProfileInput {
    pub user_id: Uuid,
    pub korean_name: Option<String>,
    pub english_name: Option<String>,
    pub date_of_birth: Option<DateTime<Utc>>,
    pub gender: Option<Gender>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub postal_code: Option<String>,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SeniorProfileInput {
    pub person_id: Uuid,
    pub care_level: Option<CareLevelEnum>,
    pub copayment_tier: Option<CopaymentTier>,
    pub has_ltci_certification: bool,
    pub ltci_number: Option<String>,
    pub primary_diagnosis: Option<String>,
    pub mobility_level: Option<String>,
    pub cognitive_level: Option<String>,
    pub lives_alone: bool,
    #[serde(default = "default_ko")]
    pub preferred_language: String,
}

fn default_ko() -> String {
    "ko".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct FamilyRelationshipInput {
    pub senior_person_id: Uuid,
    pub family_person_id: Uuid,
    pub relationship_type: RelationshipType,
    pub is_primary_contact: bool,
    pub can_make_decisions: bool,
}

// ---------------------------------------------------------------------------
// Consent inputs
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ConsentInput {
    pub subject_person_id: Uuid,
    pub purpose: ConsentPurpose,
    pub expires_at: Option<DateTime<Utc>>,
}

// ---------------------------------------------------------------------------
// Care inputs
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CarePlanInput {
    pub senior_id: Uuid,
    pub provider_id: Option<Uuid>,
    #[validate(length(min = 1))]
    pub title: String,
    pub description: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub goals: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateCarePlanInput {
    #[validate(length(min = 1))]
    pub title: Option<String>,
    pub description: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub goals: Option<serde_json::Value>,
    pub provider_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct VisitInput {
    pub care_plan_id: Uuid,
    pub caregiver_id: Uuid,
    pub scheduled_start: DateTime<Utc>,
    pub scheduled_end: DateTime<Utc>,
    pub tasks: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct VisitCheckIn {
    #[validate(range(min = -90.0, max = 90.0))]
    pub latitude: Option<f64>,
    #[validate(range(min = -180.0, max = 180.0))]
    pub longitude: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct VisitCheckOut {
    #[validate(range(min = -90.0, max = 90.0))]
    pub latitude: Option<f64>,
    #[validate(range(min = -180.0, max = 180.0))]
    pub longitude: Option<f64>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct DailyObservationInput {
    pub care_plan_id: Uuid,
    pub category: ObservationCategory,
    pub date: DateTime<Utc>,
    #[validate(length(min = 1))]
    pub value: String,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct IncidentInput {
    pub visit_id: Option<Uuid>,
    pub severity: IncidentSeverity,
    #[validate(length(min = 1))]
    pub title: String,
    #[validate(length(min = 1))]
    pub description: String,
    pub occurred_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// Medical inputs
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct MedicalHistoryInput {
    pub person_id: Uuid,
    #[validate(length(min = 1))]
    pub condition: String,
    pub diagnosed_at: Option<DateTime<Utc>>,
    pub treated_by: Option<String>,
    pub status: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct MedicationInput {
    pub person_id: Uuid,
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
    pub dosage: String,
    #[validate(length(min = 1))]
    pub form: String,
    pub frequency: MedicationFrequency,
    pub prescribed_by: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub side_effects: Option<String>,
    pub notes: Option<String>,
    pub instruction_timing: Option<InstructionTiming>,
    pub instruction_minutes: Option<i32>,
    pub instruction_text: Option<String>,
    pub total_quantity: Option<i32>,
    pub doses_per_intake: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateMedicationInput {
    #[validate(length(min = 1))]
    pub name: Option<String>,
    #[validate(length(min = 1))]
    pub dosage: Option<String>,
    #[validate(length(min = 1))]
    pub form: Option<String>,
    pub frequency: Option<MedicationFrequency>,
    pub prescribed_by: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub is_active: Option<bool>,
    pub side_effects: Option<String>,
    pub notes: Option<String>,
    pub instruction_timing: Option<InstructionTiming>,
    pub instruction_minutes: Option<i32>,
    pub instruction_text: Option<String>,
    pub total_quantity: Option<i32>,
    pub doses_per_intake: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct MedicationScheduleInput {
    pub medication_id: Uuid,
    #[validate(length(min = 1))]
    pub time_of_day: String,
    pub day_of_week: Option<DayOfWeek>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct MedicationEventUpdate {
    pub status: MedicationEventStatus,
    pub taken_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AppointmentInput {
    pub person_id: Uuid,
    #[validate(length(min = 1))]
    pub institution_name: String,
    pub institution_type: Option<ProviderType>,
    pub appointment_date: DateTime<Utc>,
    pub purpose: Option<String>,
    pub notes: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateAppointmentInput {
    #[validate(length(min = 1))]
    pub institution_name: Option<String>,
    pub institution_type: Option<ProviderType>,
    pub appointment_date: Option<DateTime<Utc>>,
    pub purpose: Option<String>,
    pub notes: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ReferralInput {
    pub from_provider_id: Uuid,
    pub to_provider_id: Uuid,
    pub senior_person_id: Uuid,
    #[validate(length(min = 1))]
    pub reason: String,
    pub notes: Option<String>,
}

// ---------------------------------------------------------------------------
// Matching inputs
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct MatchRequestInput {
    pub senior_id: Uuid,
    pub service_category: ServiceCategory,
    #[validate(length(min = 1))]
    pub region_city: String,
    #[validate(length(min = 1))]
    pub region_district: String,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub schedule_notes: Option<String>,
    pub language_preference: Option<String>,
    pub gender_preference: Option<Gender>,
    pub requires_dementia_experience: Option<bool>,
    pub requires_overnight_care: Option<bool>,
    pub additional_notes: Option<String>,
    pub requested_schedule: Option<Vec<ScheduleSlotInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct MatchCriteria {
    #[validate(length(min = 1))]
    pub region_city: String,
    #[validate(length(min = 1))]
    pub region_district: String,
    pub service_category: ServiceCategory,
    pub schedule_overlap: Option<bool>,
    pub language: Option<String>,
    pub gender_preference: Option<Gender>,
    pub dementia_experience: Option<bool>,
    pub overnight_available: Option<bool>,
}

// ---------------------------------------------------------------------------
// Provider inputs
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ProviderOrgInput {
    #[validate(length(min = 1))]
    pub name: String,
    pub provider_type: ProviderType,
    #[validate(length(min = 1))]
    pub registration_number: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub postal_code: Option<String>,
    pub phone: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(url)]
    pub website: Option<String>,
    pub license_number: Option<String>,
    pub license_expires_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub description: Option<String>,
    #[validate(range(min = -90.0, max = 90.0))]
    pub latitude: Option<f64>,
    #[validate(range(min = -180.0, max = 180.0))]
    pub longitude: Option<f64>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CaregiverApplicationInput {
    pub user_id: Uuid,
    pub provider_id: Option<Uuid>,
    #[validate(range(min = 0))]
    pub experience_years: Option<i32>,
    pub bio: Option<String>,
    pub specializations: Option<String>,
    pub has_dementia_experience: bool,
    pub has_overnight_availability: bool,
    pub smoking_status: bool,
    pub pet_friendly: bool,
    pub preferred_gender: Option<Gender>,
    #[serde(default = "default_ko")]
    pub languages_spoken: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CaregiverApplicationUpdate {
    pub status: CaregiverApplicationStatus,
    pub rejection_reason: Option<String>,
    pub reviewed_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CaregiverCredentialInput {
    pub credential_type: CredentialType,
    pub issuer: Option<String>,
    pub issued_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    #[validate(url)]
    pub document_url: Option<String>,
}

// ---------------------------------------------------------------------------
// Observability inputs
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ObservabilitySignalInput {
    pub event_type: ObservabilityEventType,
    pub severity: Option<SignalSeverity>,
    pub subject_person_id: Option<Uuid>,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    #[validate(length(min = 1))]
    pub message: String,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct NotificationInput {
    pub user_id: Uuid,
    pub notification_type: NotificationType,
    #[validate(length(min = 1))]
    pub title: String,
    #[validate(length(min = 1))]
    pub message: String,
    #[validate(url)]
    pub link: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct EligibilityCaseInput {
    pub senior_id: Uuid,
    #[validate(length(min = 1))]
    pub program_name: String,
    pub application_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ClaimInput {
    pub case_id: Uuid,
    pub amount: f64,
    #[serde(default = "default_krw")]
    pub currency: String,
    pub service_date: DateTime<Utc>,
    pub notes: Option<String>,
}

fn default_krw() -> String {
    "KRW".to_string()
}

// ---------------------------------------------------------------------------
// Schedule inputs
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleSlotInput {
    pub day_of_week: DayOfWeek,
    pub start_time: String,
    pub end_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct RecurringVisitInput {
    pub care_plan_id: Uuid,
    pub caregiver_id: Uuid,
    pub days: Vec<DayOfWeek>,
    #[validate(length(min = 1))]
    pub start_time: String,
    #[validate(length(min = 1))]
    pub end_time: String,
    #[validate(length(min = 1))]
    pub service_type: String,
    pub weeks: u32,
    #[validate(length(min = 1))]
    pub start_date: String,
}
