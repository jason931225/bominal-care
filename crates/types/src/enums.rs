// =============================================================================
// Korea Senior Care Portal — PostgreSQL Enum Types (27 total)
// Each enum maps 1:1 to a CREATE TYPE in schema.sql
// =============================================================================

use serde::{Deserialize, Serialize};
use sqlx::Type;
use strum::{Display, EnumString};

// ---------------------------------------------------------------------------
// 1. user_role
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type, Display, EnumString)]
#[sqlx(type_name = "user_role", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum UserRole {
    Senior,
    Family,
    CaregiverApplicant,
    CaregiverApproved,
    ProviderAdmin,
    ProviderStaff,
    MedicalStaff,
    GovernmentReviewer,
    PartnerOperator,
    PlatformAdmin,
}

// ---------------------------------------------------------------------------
// 2. gender
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type, Display, EnumString)]
#[sqlx(type_name = "gender", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Gender {
    Male,
    Female,
    Other,
    PreferNotToSay,
}

// ---------------------------------------------------------------------------
// 3. kyc_level
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type, Display, EnumString)]
#[sqlx(type_name = "kyc_level", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum KycLevel {
    None,
    EmailVerified,
    PhoneVerified,
    IdentityVerified,
    FullVerified,
}

// ---------------------------------------------------------------------------
// 4. consent_purpose
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type, Display, EnumString)]
#[sqlx(type_name = "consent_purpose", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum ConsentPurpose {
    NoShare,
    MedicalShare,
    GovernmentShare,
    BothShare,
}

// ---------------------------------------------------------------------------
// 5. caregiver_application_status
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type, Display, EnumString)]
#[sqlx(type_name = "caregiver_application_status", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum CaregiverApplicationStatus {
    Draft,
    Submitted,
    IdentityVerified,
    CredentialReview,
    ApprovedPrivatePay,
    ApprovedUnderProvider,
    Suspended,
    Rejected,
}

// ---------------------------------------------------------------------------
// 6. match_request_status
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type, Display, EnumString)]
#[sqlx(type_name = "match_request_status", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum MatchRequestStatus {
    Created,
    Searching,
    RecommendationsReady,
    Selected,
    Booked,
    Fulfilled,
    Cancelled,
}

// ---------------------------------------------------------------------------
// 7. visit_status
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type, Display, EnumString)]
#[sqlx(type_name = "visit_status", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum VisitStatus {
    Scheduled,
    CaregiverAcknowledged,
    InProgress,
    Completed,
    Missed,
    Cancelled,
}

// ---------------------------------------------------------------------------
// 8. medication_event_status
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type, Display, EnumString)]
#[sqlx(type_name = "medication_event_status", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum MedicationEventStatus {
    Scheduled,
    ReminderSent,
    Taken,
    Missed,
    Held,
    Escalated,
}

// ---------------------------------------------------------------------------
// 9. institution_referral_status
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type, Display, EnumString)]
#[sqlx(type_name = "institution_referral_status", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum InstitutionReferralStatus {
    Created,
    Accepted,
    Booked,
    Attended,
    Discharged,
    Closed,
}

// ---------------------------------------------------------------------------
// 10. eligibility_case_status
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type, Display, EnumString)]
#[sqlx(type_name = "eligibility_case_status", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum EligibilityCaseStatus {
    NotStarted,
    Screening,
    DocsMissing,
    UnderReview,
    Approved,
    Denied,
    Appealed,
    Final,
}

// ---------------------------------------------------------------------------
// 11. incident_severity
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type, Display, EnumString)]
#[sqlx(type_name = "incident_severity", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum IncidentSeverity {
    Low,
    Medium,
    High,
    Critical,
}

// ---------------------------------------------------------------------------
// 12. service_category
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type, Display, EnumString)]
#[sqlx(type_name = "service_category", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum ServiceCategory {
    PersonalCare,
    Companion,
    Nursing,
    Rehabilitation,
    DementiaCare,
    Respite,
    Transport,
    MealDelivery,
    HomeModification,
    Cleaning,
}

// ---------------------------------------------------------------------------
// 13. notification_type
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type, Display, EnumString)]
#[sqlx(type_name = "notification_type", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum NotificationType {
    Info,
    Warning,
    Alert,
    ActionRequired,
    Reminder,
}

// ---------------------------------------------------------------------------
// 14. observability_event_type
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type, Display, EnumString)]
#[sqlx(type_name = "observability_event_type", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum ObservabilityEventType {
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
}

// ---------------------------------------------------------------------------
// 15. care_plan_status
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type, Display, EnumString)]
#[sqlx(type_name = "care_plan_status", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum CarePlanStatus {
    Draft,
    Active,
    Paused,
    Completed,
    Cancelled,
}

// ---------------------------------------------------------------------------
// 16. appointment_status
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type, Display, EnumString)]
#[sqlx(type_name = "appointment_status", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum AppointmentStatus {
    Scheduled,
    Confirmed,
    InProgress,
    Completed,
    Cancelled,
    NoShow,
}

// ---------------------------------------------------------------------------
// 17. day_of_week
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type, Display, EnumString)]
#[sqlx(type_name = "day_of_week", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

// ---------------------------------------------------------------------------
// 18. credential_type
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type, Display, EnumString)]
#[sqlx(type_name = "credential_type", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum CredentialType {
    CaregiverCertificate,
    NursingLicense,
    SocialWorkerLicense,
    CprCertification,
    DementiaTraining,
    FirstAid,
    Other,
}

// ---------------------------------------------------------------------------
// 19. credential_status
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type, Display, EnumString)]
#[sqlx(type_name = "credential_status", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum CredentialStatus {
    Pending,
    Verified,
    Expired,
    Rejected,
}

// ---------------------------------------------------------------------------
// 20. claim_status
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type, Display, EnumString)]
#[sqlx(type_name = "claim_status", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum ClaimStatus {
    Draft,
    Submitted,
    UnderReview,
    Approved,
    Denied,
    Paid,
}

// ---------------------------------------------------------------------------
// 21. audit_action
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type, Display, EnumString)]
#[sqlx(type_name = "audit_action", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum AuditAction {
    Create,
    Read,
    Update,
    Delete,
    Login,
    Logout,
    ConsentGrant,
    ConsentRevoke,
    StatusChange,
}

// ---------------------------------------------------------------------------
// 22. relationship_type
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type, Display, EnumString)]
#[sqlx(type_name = "relationship_type", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum RelationshipType {
    Spouse,
    Child,
    Sibling,
    Parent,
    Grandchild,
    OtherRelative,
    LegalGuardian,
    Friend,
    SocialWorker,
}

// ---------------------------------------------------------------------------
// 23. provider_type
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type, Display, EnumString)]
#[sqlx(type_name = "provider_type", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum ProviderType {
    HomeCareAgency,
    NursingHospital,
    Clinic,
    Pharmacy,
    RehabilitationCenter,
    DementiaCenter,
    SilverTown,
    TransportService,
    MealService,
    CommunityCenter,
}

// ---------------------------------------------------------------------------
// 24. signal_severity
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type, Display, EnumString)]
#[sqlx(type_name = "signal_severity", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum SignalSeverity {
    Info,
    Warning,
    Alert,
    Critical,
}

// ---------------------------------------------------------------------------
// 25. medication_frequency
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type, Display, EnumString)]
#[sqlx(type_name = "medication_frequency", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum MedicationFrequency {
    OnceDaily,
    TwiceDaily,
    ThreeTimesDaily,
    FourTimesDaily,
    EveryOtherDay,
    Weekly,
    AsNeeded,
    Custom,
}

// ---------------------------------------------------------------------------
// 26. observation_category
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type, Display, EnumString)]
#[sqlx(type_name = "observation_category", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum ObservationCategory {
    VitalSigns,
    Mood,
    Appetite,
    Mobility,
    Sleep,
    Pain,
    Cognitive,
    Skin,
    Other,
}

// ---------------------------------------------------------------------------
// 27. housing_type
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type, Display, EnumString)]
#[sqlx(type_name = "housing_type", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum HousingType {
    SilverTown,
    SeniorApartment,
    GroupHome,
    AssistedLiving,
    NursingFacility,
}
