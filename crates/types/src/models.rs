// =============================================================================
// Korea Senior Care Portal — Database Row Structs (32 tables)
// Ported from packages/db/src/types.ts (693 lines)
// =============================================================================

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::enums::*;

// ---------------------------------------------------------------------------
// 1. User
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct User {
    pub id: Uuid,
    pub email: Option<String>,
    pub email_verified: Option<DateTime<Utc>>,
    pub name: Option<String>,
    pub image: Option<String>,
    pub phone: Option<String>,
    pub role: UserRole,
    pub kyc_level: KycLevel,
    pub locale: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// 2. Account
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Account {
    pub id: Uuid,
    pub user_id: Uuid,
    #[cfg_attr(feature = "ssr", sqlx(rename = "type"))]
    #[serde(rename = "type")]
    pub account_type: String,
    pub provider: String,
    pub provider_account_id: String,
    pub refresh_token: Option<String>,
    pub access_token: Option<String>,
    pub expires_at: Option<i32>,
    pub token_type: Option<String>,
    pub scope: Option<String>,
    pub id_token: Option<String>,
    pub session_state: Option<String>,
}

// ---------------------------------------------------------------------------
// 3. Session
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Session {
    pub id: Uuid,
    pub session_token: String,
    pub user_id: Uuid,
    pub expires: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// 4. VerificationToken
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct VerificationToken {
    pub identifier: String,
    pub token: String,
    pub expires: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// 5. PersonProfile
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct PersonProfile {
    pub id: Uuid,
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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

// ---------------------------------------------------------------------------
// 6. SeniorProfile
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct SeniorProfile {
    pub id: Uuid,
    pub person_id: Uuid,
    pub care_level: Option<CareLevelEnum>,
    pub copayment_tier: CopaymentTier,
    pub has_ltci_certification: bool,
    pub ltci_number: Option<String>,
    pub primary_diagnosis: Option<String>,
    pub mobility_level: Option<String>,
    pub cognitive_level: Option<String>,
    pub lives_alone: bool,
    pub preferred_language: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// 7. FamilyRelationship
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct FamilyRelationship {
    pub id: Uuid,
    pub senior_person_id: Uuid,
    pub family_person_id: Uuid,
    pub relationship_type: RelationshipType,
    pub is_primary_contact: bool,
    pub can_make_decisions: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// 8. ConsentRecord
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct ConsentRecord {
    pub id: Uuid,
    pub subject_person_id: Uuid,
    pub purpose: ConsentPurpose,
    pub granted_by: Uuid,
    pub is_active: bool,
    pub granted_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// 9. ProviderOrganization
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct ProviderOrganization {
    pub id: Uuid,
    pub name: String,
    #[cfg_attr(feature = "ssr", sqlx(rename = "type"))]
    #[serde(rename = "type")]
    pub provider_type: ProviderType,
    pub registration_number: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub postal_code: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
    pub license_number: Option<String>,
    pub license_expires_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub description: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub nhis_institution_code: Option<String>,
    pub medical_institution_code: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

// ---------------------------------------------------------------------------
// 10. CaregiverApplication
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct CaregiverApplication {
    pub id: Uuid,
    pub user_id: Uuid,
    pub provider_id: Option<Uuid>,
    pub status: CaregiverApplicationStatus,
    pub experience_years: Option<i32>,
    pub bio: Option<String>,
    pub specializations: Option<String>,
    pub has_dementia_experience: bool,
    pub has_overnight_availability: bool,
    pub smoking_status: bool,
    pub pet_friendly: bool,
    pub preferred_gender: Option<Gender>,
    pub languages_spoken: String,
    pub submitted_at: Option<DateTime<Utc>>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub reviewed_by: Option<Uuid>,
    pub rejection_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// 11. CaregiverCredential
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct CaregiverCredential {
    pub id: Uuid,
    pub application_id: Uuid,
    #[cfg_attr(feature = "ssr", sqlx(rename = "type"))]
    #[serde(rename = "type")]
    pub credential_type: CredentialType,
    pub status: CredentialStatus,
    pub issuer: Option<String>,
    pub issued_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub document_url: Option<String>,
    pub verified_at: Option<DateTime<Utc>>,
    pub verified_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// 12. ServiceRegion
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct ServiceRegion {
    pub id: Uuid,
    pub provider_id: Uuid,
    pub city: String,
    pub district: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// 13. AvailabilitySlot
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct AvailabilitySlot {
    pub id: Uuid,
    pub application_id: Uuid,
    pub day_of_week: DayOfWeek,
    pub start_time: String,
    pub end_time: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// 14. ServiceType
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct ServiceType {
    pub id: Uuid,
    pub application_id: Option<Uuid>,
    pub category: ServiceCategory,
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// 15. MatchRequest
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct MatchRequest {
    pub id: Uuid,
    pub senior_id: Uuid,
    pub requested_by: Uuid,
    pub status: MatchRequestStatus,
    pub service_category: ServiceCategory,
    pub region_city: String,
    pub region_district: String,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub schedule_notes: Option<String>,
    pub language_preference: Option<String>,
    pub gender_preference: Option<Gender>,
    pub requires_dementia_experience: bool,
    pub requires_overnight_care: bool,
    pub additional_notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// 16. MatchRecommendation
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct MatchRecommendation {
    pub id: Uuid,
    pub match_request_id: Uuid,
    pub caregiver_application_id: Uuid,
    pub score: f64,
    pub score_breakdown: Option<serde_json::Value>,
    pub rank: i32,
    pub is_selected: bool,
    pub selected_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// 17. CarePlan
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct CarePlan {
    pub id: Uuid,
    pub senior_id: Uuid,
    pub provider_id: Option<Uuid>,
    pub status: CarePlanStatus,
    pub title: String,
    pub description: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub goals: Option<serde_json::Value>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

// ---------------------------------------------------------------------------
// 18. Visit
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Visit {
    pub id: Uuid,
    pub care_plan_id: Uuid,
    pub caregiver_id: Uuid,
    pub status: VisitStatus,
    pub scheduled_start: DateTime<Utc>,
    pub scheduled_end: DateTime<Utc>,
    pub actual_start: Option<DateTime<Utc>>,
    pub actual_end: Option<DateTime<Utc>>,
    pub check_in_latitude: Option<f64>,
    pub check_in_longitude: Option<f64>,
    pub check_out_latitude: Option<f64>,
    pub check_out_longitude: Option<f64>,
    pub check_in_distance_meters: Option<f64>,
    pub tasks: Option<serde_json::Value>,
    pub notes: Option<String>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// 19. DailyObservation
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct DailyObservation {
    pub id: Uuid,
    pub care_plan_id: Uuid,
    pub observed_by: Uuid,
    pub category: ObservationCategory,
    pub date: DateTime<Utc>,
    pub value: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// 20. Incident
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Incident {
    pub id: Uuid,
    pub visit_id: Option<Uuid>,
    pub reported_by: Uuid,
    pub severity: IncidentSeverity,
    pub title: String,
    pub description: String,
    pub occurred_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolution: Option<String>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// 21. MedicalHistoryEntry
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct MedicalHistoryEntry {
    pub id: Uuid,
    pub person_id: Uuid,
    pub condition: String,
    pub diagnosed_at: Option<DateTime<Utc>>,
    pub treated_by: Option<String>,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

// ---------------------------------------------------------------------------
// 22. Medication
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Medication {
    pub id: Uuid,
    pub person_id: Uuid,
    pub name: String,
    pub dosage: String,
    pub form: String,
    pub frequency: MedicationFrequency,
    pub prescribed_by: Option<String>,
    pub prescribed_at: Option<DateTime<Utc>>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub side_effects: Option<String>,
    pub notes: Option<String>,
    pub approval_status: String,
    pub submitted_by: Option<Uuid>,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

// ---------------------------------------------------------------------------
// 23. MedicationSchedule
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct MedicationSchedule {
    pub id: Uuid,
    pub medication_id: Uuid,
    pub time_of_day: String,
    pub day_of_week: Option<DayOfWeek>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// 24. MedicationEvent
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct MedicationEvent {
    pub id: Uuid,
    pub medication_id: Uuid,
    pub scheduled_for: DateTime<Utc>,
    pub status: MedicationEventStatus,
    pub taken_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub recorded_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// 25. Appointment
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Appointment {
    pub id: Uuid,
    pub person_id: Uuid,
    pub institution_name: String,
    pub institution_type: Option<ProviderType>,
    pub appointment_date: DateTime<Utc>,
    pub status: AppointmentStatus,
    pub purpose: Option<String>,
    pub notes: Option<String>,
    pub address: Option<String>,
    pub approval_status: String,
    pub submitted_by: Option<Uuid>,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

// ---------------------------------------------------------------------------
// 26. InstitutionReferral
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct InstitutionReferral {
    pub id: Uuid,
    pub from_provider_id: Uuid,
    pub to_provider_id: Uuid,
    pub senior_person_id: Uuid,
    pub status: InstitutionReferralStatus,
    pub reason: Option<String>,
    pub notes: Option<String>,
    pub referred_at: DateTime<Utc>,
    pub accepted_at: Option<DateTime<Utc>>,
    pub discharged_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// 27. EligibilityCase
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct EligibilityCase {
    pub id: Uuid,
    pub senior_id: Uuid,
    pub status: EligibilityCaseStatus,
    pub program_name: String,
    pub application_date: Option<DateTime<Utc>>,
    pub determination_date: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub denial_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

// ---------------------------------------------------------------------------
// 28. ApprovalStep
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct ApprovalStep {
    pub id: Uuid,
    pub case_id: Uuid,
    pub step_name: String,
    pub step_order: i32,
    pub status: String,
    pub assigned_to: Option<Uuid>,
    pub completed_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// 29. ClaimOrSubsidyRecord
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct ClaimOrSubsidyRecord {
    pub id: Uuid,
    pub case_id: Uuid,
    pub claim_number: String,
    pub status: ClaimStatus,
    pub amount: Decimal,
    pub currency: String,
    pub service_date: DateTime<Utc>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub processed_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// 30. ObservabilitySignal
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct ObservabilitySignal {
    pub id: Uuid,
    pub event_type: ObservabilityEventType,
    pub severity: SignalSeverity,
    pub subject_person_id: Option<Uuid>,
    pub actor_user_id: Option<Uuid>,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub message: String,
    pub metadata: Option<serde_json::Value>,
    pub acknowledged_at: Option<DateTime<Utc>>,
    pub acknowledged_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// 31. Notification
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Notification {
    pub id: Uuid,
    pub user_id: Uuid,
    #[cfg_attr(feature = "ssr", sqlx(rename = "type"))]
    #[serde(rename = "type")]
    pub notification_type: NotificationType,
    pub title: String,
    pub message: String,
    pub link: Option<String>,
    pub is_read: bool,
    pub read_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// 32. AuditLog
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct AuditLog {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub action: AuditAction,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub old_value: Option<serde_json::Value>,
    pub new_value: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}
