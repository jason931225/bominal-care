// =============================================================================
// Korea Senior Care Portal — PostgreSQL Enum Types (27 total)
// Each enum maps 1:1 to a CREATE TYPE in schema.sql
// =============================================================================

use serde::{Deserialize, Serialize};
use strum::EnumString;

// ---------------------------------------------------------------------------
// 1. user_role
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "user_role", rename_all = "SCREAMING_SNAKE_CASE"))]
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
    PharmacyStaff,
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Senior => write!(f, "이용자"),
            Self::Family => write!(f, "보호자"),
            Self::CaregiverApplicant => write!(f, "신청자"),
            Self::CaregiverApproved => write!(f, "요양보호사"),
            Self::ProviderAdmin => write!(f, "기관관리자"),
            Self::ProviderStaff => write!(f, "기관직원"),
            Self::MedicalStaff => write!(f, "의료진"),
            Self::GovernmentReviewer => write!(f, "정부"),
            Self::PartnerOperator => write!(f, "파트너운영자"),
            Self::PlatformAdmin => write!(f, "관리자"),
            Self::PharmacyStaff => write!(f, "약사"),
        }
    }
}

// ---------------------------------------------------------------------------
// 2. gender
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "gender", rename_all = "SCREAMING_SNAKE_CASE"))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Gender {
    Male,
    Female,
    Other,
    PreferNotToSay,
}

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Male => write!(f, "남성"),
            Self::Female => write!(f, "여성"),
            Self::Other => write!(f, "기타"),
            Self::PreferNotToSay => write!(f, "미상"),
        }
    }
}

// ---------------------------------------------------------------------------
// 3. kyc_level
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "kyc_level", rename_all = "SCREAMING_SNAKE_CASE"))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum KycLevel {
    None,
    EmailVerified,
    PhoneVerified,
    IdentityVerified,
    FullVerified,
}

impl std::fmt::Display for KycLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "미인증"),
            Self::EmailVerified => write!(f, "이메일인증"),
            Self::PhoneVerified => write!(f, "전화인증"),
            Self::IdentityVerified => write!(f, "본인인증"),
            Self::FullVerified => write!(f, "완전인증"),
        }
    }
}

// ---------------------------------------------------------------------------
// 4. consent_purpose
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "consent_purpose", rename_all = "SCREAMING_SNAKE_CASE"))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum ConsentPurpose {
    NoShare,
    MedicalShare,
    GovernmentShare,
    BothShare,
}

impl std::fmt::Display for ConsentPurpose {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoShare => write!(f, "비공유"),
            Self::MedicalShare => write!(f, "의료정보공유"),
            Self::GovernmentShare => write!(f, "정부보고"),
            Self::BothShare => write!(f, "전체공유"),
        }
    }
}

// ---------------------------------------------------------------------------
// 5. caregiver_application_status
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "caregiver_application_status", rename_all = "SCREAMING_SNAKE_CASE"))]
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

impl std::fmt::Display for CaregiverApplicationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Draft => write!(f, "임시저장"),
            Self::Submitted => write!(f, "제출완료"),
            Self::IdentityVerified => write!(f, "본인인증완료"),
            Self::CredentialReview => write!(f, "자격심사중"),
            Self::ApprovedPrivatePay => write!(f, "승인(개인)"),
            Self::ApprovedUnderProvider => write!(f, "승인(기관)"),
            Self::Suspended => write!(f, "정지"),
            Self::Rejected => write!(f, "거부"),
        }
    }
}

// ---------------------------------------------------------------------------
// 6. match_request_status
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "match_request_status", rename_all = "SCREAMING_SNAKE_CASE"))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum MatchRequestStatus {
    Created,
    Searching,
    RecommendationsReady,
    Selected,
    Booked,
    Fulfilled,
    NoCandidates,
    Cancelled,
}

impl std::fmt::Display for MatchRequestStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Created => write!(f, "대기"),
            Self::Searching => write!(f, "매칭중"),
            Self::RecommendationsReady => write!(f, "추천완료"),
            Self::Selected => write!(f, "선택완료"),
            Self::Booked => write!(f, "예약완료"),
            Self::Fulfilled => write!(f, "매칭완료"),
            Self::NoCandidates => write!(f, "후보없음"),
            Self::Cancelled => write!(f, "취소"),
        }
    }
}

// ---------------------------------------------------------------------------
// 7. visit_status
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "visit_status", rename_all = "SCREAMING_SNAKE_CASE"))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum VisitStatus {
    Scheduled,
    CaregiverAcknowledged,
    InProgress,
    Completed,
    Missed,
    NeedsReassignment,
    Cancelled,
}

impl std::fmt::Display for VisitStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Scheduled => write!(f, "예정"),
            Self::CaregiverAcknowledged => write!(f, "체크인"),
            Self::InProgress => write!(f, "진행중"),
            Self::Completed => write!(f, "완료"),
            Self::Missed => write!(f, "미참석"),
            Self::NeedsReassignment => write!(f, "재배정필요"),
            Self::Cancelled => write!(f, "취소"),
        }
    }
}

// ---------------------------------------------------------------------------
// 8. medication_event_status
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "medication_event_status", rename_all = "SCREAMING_SNAKE_CASE"))]
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

impl std::fmt::Display for MedicationEventStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Scheduled => write!(f, "예정"),
            Self::ReminderSent => write!(f, "알림전송"),
            Self::Taken => write!(f, "복용완료"),
            Self::Missed => write!(f, "미복용"),
            Self::Held => write!(f, "보류"),
            Self::Escalated => write!(f, "상부보고"),
        }
    }
}

// ---------------------------------------------------------------------------
// 9. institution_referral_status
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "institution_referral_status", rename_all = "SCREAMING_SNAKE_CASE"))]
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

impl std::fmt::Display for InstitutionReferralStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Created => write!(f, "대기"),
            Self::Accepted => write!(f, "수락"),
            Self::Booked => write!(f, "예약완료"),
            Self::Attended => write!(f, "진행중"),
            Self::Discharged => write!(f, "퇴원"),
            Self::Closed => write!(f, "완료"),
        }
    }
}

// ---------------------------------------------------------------------------
// 10. eligibility_case_status
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "eligibility_case_status", rename_all = "SCREAMING_SNAKE_CASE"))]
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

impl std::fmt::Display for EligibilityCaseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotStarted => write!(f, "미신청"),
            Self::Screening => write!(f, "심사중"),
            Self::DocsMissing => write!(f, "서류미비"),
            Self::UnderReview => write!(f, "심사중"),
            Self::Approved => write!(f, "승인"),
            Self::Denied => write!(f, "거부"),
            Self::Appealed => write!(f, "이의신청"),
            Self::Final => write!(f, "만료"),
        }
    }
}

// ---------------------------------------------------------------------------
// 11. incident_severity
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "incident_severity", rename_all = "SCREAMING_SNAKE_CASE"))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum IncidentSeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl std::fmt::Display for IncidentSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Low => write!(f, "경미"),
            Self::Medium => write!(f, "보통"),
            Self::High => write!(f, "심각"),
            Self::Critical => write!(f, "위급"),
        }
    }
}

// ---------------------------------------------------------------------------
// 12. service_category
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "service_category", rename_all = "SCREAMING_SNAKE_CASE"))]
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

impl std::fmt::Display for ServiceCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PersonalCare => write!(f, "방문요양"),
            Self::Companion => write!(f, "정서지원"),
            Self::Nursing => write!(f, "방문간호"),
            Self::Rehabilitation => write!(f, "재활치료"),
            Self::DementiaCare => write!(f, "인지활동"),
            Self::Respite => write!(f, "단기보호"),
            Self::Transport => write!(f, "이동지원"),
            Self::MealDelivery => write!(f, "식사배달"),
            Self::HomeModification => write!(f, "주거개선"),
            Self::Cleaning => write!(f, "청소지원"),
        }
    }
}

// ---------------------------------------------------------------------------
// 13. notification_type
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "notification_type", rename_all = "SCREAMING_SNAKE_CASE"))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum NotificationType {
    Info,
    Warning,
    Alert,
    ActionRequired,
    Reminder,
    Emergency,
}

impl std::fmt::Display for NotificationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Info => write!(f, "시스템"),
            Self::Warning => write!(f, "주의"),
            Self::Alert => write!(f, "긴급"),
            Self::ActionRequired => write!(f, "조치필요"),
            Self::Reminder => write!(f, "알림"),
            Self::Emergency => write!(f, "긴급SOS"),
        }
    }
}

// ---------------------------------------------------------------------------
// 14. observability_event_type
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "observability_event_type", rename_all = "SCREAMING_SNAKE_CASE"))]
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

impl std::fmt::Display for ObservabilityEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::VisitCompleted => write!(f, "방문완료"),
            Self::VisitMissed => write!(f, "방문미참석"),
            Self::MedicationTaken => write!(f, "복약완료"),
            Self::MedicationMissed => write!(f, "복약누락"),
            Self::MealDelivered => write!(f, "식사배달완료"),
            Self::MealFailed => write!(f, "식사배달실패"),
            Self::TransportCompleted => write!(f, "이동완료"),
            Self::TransportFailed => write!(f, "이동실패"),
            Self::SymptomReported => write!(f, "증상보고"),
            Self::IncidentCreated => write!(f, "사고접수"),
            Self::EligibilityStatusChanged => write!(f, "자격변경"),
            Self::ReferralUpdated => write!(f, "의뢰갱신"),
        }
    }
}

// ---------------------------------------------------------------------------
// 15. care_plan_status
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "care_plan_status", rename_all = "SCREAMING_SNAKE_CASE"))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum CarePlanStatus {
    Draft,
    Active,
    Paused,
    Completed,
    Cancelled,
}

impl std::fmt::Display for CarePlanStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Draft => write!(f, "임시저장"),
            Self::Active => write!(f, "활성"),
            Self::Paused => write!(f, "일시중지"),
            Self::Completed => write!(f, "완료"),
            Self::Cancelled => write!(f, "취소"),
        }
    }
}

// ---------------------------------------------------------------------------
// 16. appointment_status
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "appointment_status", rename_all = "SCREAMING_SNAKE_CASE"))]
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

impl std::fmt::Display for AppointmentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Scheduled => write!(f, "예정"),
            Self::Confirmed => write!(f, "확인"),
            Self::InProgress => write!(f, "진행중"),
            Self::Completed => write!(f, "완료"),
            Self::Cancelled => write!(f, "취소"),
            Self::NoShow => write!(f, "미참석"),
        }
    }
}

// ---------------------------------------------------------------------------
// 17. day_of_week
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "day_of_week", rename_all = "SCREAMING_SNAKE_CASE"))]
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

impl std::fmt::Display for DayOfWeek {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Monday => write!(f, "월요일"),
            Self::Tuesday => write!(f, "화요일"),
            Self::Wednesday => write!(f, "수요일"),
            Self::Thursday => write!(f, "목요일"),
            Self::Friday => write!(f, "금요일"),
            Self::Saturday => write!(f, "토요일"),
            Self::Sunday => write!(f, "일요일"),
        }
    }
}

// ---------------------------------------------------------------------------
// 18. credential_type
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "credential_type", rename_all = "SCREAMING_SNAKE_CASE"))]
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

impl std::fmt::Display for CredentialType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CaregiverCertificate => write!(f, "요양보호사자격증"),
            Self::NursingLicense => write!(f, "간호사면허"),
            Self::SocialWorkerLicense => write!(f, "사회복지사자격증"),
            Self::CprCertification => write!(f, "심폐소생술자격"),
            Self::DementiaTraining => write!(f, "치매교육이수"),
            Self::FirstAid => write!(f, "응급처치자격"),
            Self::Other => write!(f, "기타"),
        }
    }
}

// ---------------------------------------------------------------------------
// 19. credential_status
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "credential_status", rename_all = "SCREAMING_SNAKE_CASE"))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum CredentialStatus {
    Pending,
    Verified,
    Expired,
    Rejected,
}

impl std::fmt::Display for CredentialStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "심사중"),
            Self::Verified => write!(f, "인증완료"),
            Self::Expired => write!(f, "만료"),
            Self::Rejected => write!(f, "거부"),
        }
    }
}

// ---------------------------------------------------------------------------
// 20. claim_status
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "claim_status", rename_all = "SCREAMING_SNAKE_CASE"))]
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

impl std::fmt::Display for ClaimStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Draft => write!(f, "임시저장"),
            Self::Submitted => write!(f, "제출완료"),
            Self::UnderReview => write!(f, "심사중"),
            Self::Approved => write!(f, "승인"),
            Self::Denied => write!(f, "거부"),
            Self::Paid => write!(f, "지급완료"),
        }
    }
}

// ---------------------------------------------------------------------------
// 21. audit_action
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "audit_action", rename_all = "SCREAMING_SNAKE_CASE"))]
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

impl std::fmt::Display for AuditAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Create => write!(f, "생성"),
            Self::Read => write!(f, "조회"),
            Self::Update => write!(f, "수정"),
            Self::Delete => write!(f, "삭제"),
            Self::Login => write!(f, "로그인"),
            Self::Logout => write!(f, "로그아웃"),
            Self::ConsentGrant => write!(f, "동의부여"),
            Self::ConsentRevoke => write!(f, "동의철회"),
            Self::StatusChange => write!(f, "상태변경"),
        }
    }
}

// ---------------------------------------------------------------------------
// 22. relationship_type
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "relationship_type", rename_all = "SCREAMING_SNAKE_CASE"))]
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

impl std::fmt::Display for RelationshipType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Spouse => write!(f, "배우자"),
            Self::Child => write!(f, "자녀"),
            Self::Sibling => write!(f, "형제자매"),
            Self::Parent => write!(f, "부모"),
            Self::Grandchild => write!(f, "손자녀"),
            Self::OtherRelative => write!(f, "기타친족"),
            Self::LegalGuardian => write!(f, "법정후견인"),
            Self::Friend => write!(f, "지인"),
            Self::SocialWorker => write!(f, "사회복지사"),
        }
    }
}

// ---------------------------------------------------------------------------
// 23. provider_type
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "provider_type", rename_all = "SCREAMING_SNAKE_CASE"))]
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

impl std::fmt::Display for ProviderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HomeCareAgency => write!(f, "방문요양기관"),
            Self::NursingHospital => write!(f, "요양병원"),
            Self::Clinic => write!(f, "의원"),
            Self::Pharmacy => write!(f, "약국"),
            Self::RehabilitationCenter => write!(f, "재활센터"),
            Self::DementiaCenter => write!(f, "치매센터"),
            Self::SilverTown => write!(f, "실버타운"),
            Self::TransportService => write!(f, "이동지원서비스"),
            Self::MealService => write!(f, "식사서비스"),
            Self::CommunityCenter => write!(f, "복지관"),
        }
    }
}

// ---------------------------------------------------------------------------
// 24. signal_severity
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "signal_severity", rename_all = "SCREAMING_SNAKE_CASE"))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum SignalSeverity {
    Info,
    Warning,
    Alert,
    Critical,
}

impl std::fmt::Display for SignalSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Info => write!(f, "정보"),
            Self::Warning => write!(f, "주의"),
            Self::Alert => write!(f, "경고"),
            Self::Critical => write!(f, "위급"),
        }
    }
}

// ---------------------------------------------------------------------------
// 25. medication_frequency
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "medication_frequency", rename_all = "SCREAMING_SNAKE_CASE"))]
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

impl std::fmt::Display for MedicationFrequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OnceDaily => write!(f, "하루 1회"),
            Self::TwiceDaily => write!(f, "하루 2회"),
            Self::ThreeTimesDaily => write!(f, "하루 3회"),
            Self::FourTimesDaily => write!(f, "하루 4회"),
            Self::EveryOtherDay => write!(f, "격일"),
            Self::Weekly => write!(f, "주 1회"),
            Self::AsNeeded => write!(f, "필요시"),
            Self::Custom => write!(f, "맞춤"),
        }
    }
}

// ---------------------------------------------------------------------------
// 26. instruction_timing
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "instruction_timing", rename_all = "SCREAMING_SNAKE_CASE"))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum InstructionTiming {
    BeforeMeal,
    WithMeal,
    AfterMeal,
    EmptyStomach,
    Bedtime,
    Anytime,
}

impl std::fmt::Display for InstructionTiming {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BeforeMeal => write!(f, "식전"),
            Self::WithMeal => write!(f, "식사 중"),
            Self::AfterMeal => write!(f, "식후"),
            Self::EmptyStomach => write!(f, "공복"),
            Self::Bedtime => write!(f, "취침 전"),
            Self::Anytime => write!(f, "시간 무관"),
        }
    }
}

// ---------------------------------------------------------------------------
// 27. observation_category
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "observation_category", rename_all = "SCREAMING_SNAKE_CASE"))]
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

impl std::fmt::Display for ObservationCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::VitalSigns => write!(f, "활력징후"),
            Self::Mood => write!(f, "기분"),
            Self::Appetite => write!(f, "식욕"),
            Self::Mobility => write!(f, "이동"),
            Self::Sleep => write!(f, "수면"),
            Self::Pain => write!(f, "통증"),
            Self::Cognitive => write!(f, "인지"),
            Self::Skin => write!(f, "피부"),
            Self::Other => write!(f, "기타"),
        }
    }
}

// ---------------------------------------------------------------------------
// 27. housing_type
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "housing_type", rename_all = "SCREAMING_SNAKE_CASE"))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum HousingType {
    SilverTown,
    SeniorApartment,
    GroupHome,
    AssistedLiving,
    NursingFacility,
}

impl std::fmt::Display for HousingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SilverTown => write!(f, "실버타운"),
            Self::SeniorApartment => write!(f, "시니어아파트"),
            Self::GroupHome => write!(f, "공동생활가정"),
            Self::AssistedLiving => write!(f, "주거지원시설"),
            Self::NursingFacility => write!(f, "요양시설"),
        }
    }
}

// ---------------------------------------------------------------------------
// 28. handoff_license_type
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "handoff_license_type", rename_all = "snake_case"))]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum HandoffLicenseType {
    Doctor,
    Nurse,
    Pharmacist,
}

impl std::fmt::Display for HandoffLicenseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Doctor => write!(f, "의사"),
            Self::Nurse => write!(f, "간호사"),
            Self::Pharmacist => write!(f, "약사"),
        }
    }
}

// ---------------------------------------------------------------------------
// 29. consent_purpose_v2
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "consent_purpose_v2", rename_all = "snake_case"))]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ConsentPurposeV2 {
    PersonalInfoCollection,
    SensitiveInfoProcessing,
    ThirdPartyMedical,
    ThirdPartyGovernment,
    ThirdPartyProvider,
    ThirdPartyFamily,
    ThirdPartyCaregiver,
    Marketing,
}

impl std::fmt::Display for ConsentPurposeV2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PersonalInfoCollection => write!(f, "개인정보 수집"),
            Self::SensitiveInfoProcessing => write!(f, "민감정보 처리"),
            Self::ThirdPartyMedical => write!(f, "의료기관 제공"),
            Self::ThirdPartyGovernment => write!(f, "정부기관 제공"),
            Self::ThirdPartyProvider => write!(f, "서비스기관 제공"),
            Self::ThirdPartyFamily => write!(f, "보호자 제공"),
            Self::ThirdPartyCaregiver => write!(f, "요양보호사 제공"),
            Self::Marketing => write!(f, "마케팅"),
        }
    }
}

// ---------------------------------------------------------------------------
// 30. copayment_tier
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "copayment_tier", rename_all = "snake_case"))]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CopaymentTier {
    Exempt,
    Reduction60,
    Reduction40,
    Standard,
}

impl std::fmt::Display for CopaymentTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Exempt => write!(f, "전액면제"),
            Self::Reduction60 => write!(f, "60%감경"),
            Self::Reduction40 => write!(f, "40%감경"),
            Self::Standard => write!(f, "일반"),
        }
    }
}

// ---------------------------------------------------------------------------
// 31. credential_classification
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "credential_classification", rename_all = "snake_case"))]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CredentialClassification {
    License,
    Qualification,
}

impl std::fmt::Display for CredentialClassification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::License => write!(f, "면허"),
            Self::Qualification => write!(f, "자격"),
        }
    }
}

// ---------------------------------------------------------------------------
// 32. care_level_enum
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "care_level_enum", rename_all = "snake_case"))]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CareLevelEnum {
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Cognitive,
}

impl std::fmt::Display for CareLevelEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Level1 => write!(f, "1등급"),
            Self::Level2 => write!(f, "2등급"),
            Self::Level3 => write!(f, "3등급"),
            Self::Level4 => write!(f, "4등급"),
            Self::Level5 => write!(f, "5등급"),
            Self::Cognitive => write!(f, "인지지원등급"),
        }
    }
}

// ---------------------------------------------------------------------------
// 33. internal_permission_level
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "internal_permission_level", rename_all = "snake_case"))]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum InternalPermissionLevel {
    Staff,
    Manager,
    SecurityAdmin,
    OrgAdmin,
}

impl std::fmt::Display for InternalPermissionLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Staff => write!(f, "직원"),
            Self::Manager => write!(f, "관리자"),
            Self::SecurityAdmin => write!(f, "보안관리자"),
            Self::OrgAdmin => write!(f, "기관장"),
        }
    }
}

// ---------------------------------------------------------------------------
// 34. wellness_mood
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "wellness_mood", rename_all = "snake_case"))]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum WellnessMood {
    Good,
    Okay,
    NotGreat,
    NeedHelp,
}

impl std::fmt::Display for WellnessMood {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Good => write!(f, "좋아요"),
            Self::Okay => write!(f, "괜찮아요"),
            Self::NotGreat => write!(f, "별로예요"),
            Self::NeedHelp => write!(f, "도움필요"),
        }
    }
}

// ---------------------------------------------------------------------------
// 35. emergency_event_status
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "emergency_event_status", rename_all = "snake_case"))]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum EmergencyEventStatus {
    Triggered,
    RespondersNotified,
    Resolved,
    FalseAlarm,
}

impl std::fmt::Display for EmergencyEventStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Triggered => write!(f, "발생"),
            Self::RespondersNotified => write!(f, "응답자알림"),
            Self::Resolved => write!(f, "해결"),
            Self::FalseAlarm => write!(f, "오보"),
        }
    }
}
