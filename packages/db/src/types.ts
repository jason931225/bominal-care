// =============================================================================
// Korea Senior Care Portal — Database Row Types
// Generated from schema.sql — use snake_case to match column names
// =============================================================================

// -----------------------------------------------------------------------------
// Enum string union types
// -----------------------------------------------------------------------------

export type UserRole =
  | 'SENIOR'
  | 'FAMILY'
  | 'CAREGIVER_APPLICANT'
  | 'CAREGIVER_APPROVED'
  | 'PROVIDER_ADMIN'
  | 'PROVIDER_STAFF'
  | 'MEDICAL_STAFF'
  | 'GOVERNMENT_REVIEWER'
  | 'PARTNER_OPERATOR'
  | 'PLATFORM_ADMIN';

export type Gender = 'MALE' | 'FEMALE' | 'OTHER' | 'PREFER_NOT_TO_SAY';

export type KycLevel =
  | 'NONE'
  | 'EMAIL_VERIFIED'
  | 'PHONE_VERIFIED'
  | 'IDENTITY_VERIFIED'
  | 'FULL_VERIFIED';

export type ConsentPurpose =
  | 'NO_SHARE'
  | 'MEDICAL_SHARE'
  | 'GOVERNMENT_SHARE'
  | 'BOTH_SHARE';

export type CaregiverApplicationStatus =
  | 'DRAFT'
  | 'SUBMITTED'
  | 'IDENTITY_VERIFIED'
  | 'CREDENTIAL_REVIEW'
  | 'APPROVED_PRIVATE_PAY'
  | 'APPROVED_UNDER_PROVIDER'
  | 'SUSPENDED'
  | 'REJECTED';

export type MatchRequestStatus =
  | 'CREATED'
  | 'SEARCHING'
  | 'RECOMMENDATIONS_READY'
  | 'SELECTED'
  | 'BOOKED'
  | 'FULFILLED'
  | 'CANCELLED';

export type VisitStatus =
  | 'SCHEDULED'
  | 'CAREGIVER_ACKNOWLEDGED'
  | 'IN_PROGRESS'
  | 'COMPLETED'
  | 'MISSED'
  | 'CANCELLED';

export type MedicationEventStatus =
  | 'SCHEDULED'
  | 'REMINDER_SENT'
  | 'TAKEN'
  | 'MISSED'
  | 'HELD'
  | 'ESCALATED';

export type InstitutionReferralStatus =
  | 'CREATED'
  | 'ACCEPTED'
  | 'BOOKED'
  | 'ATTENDED'
  | 'DISCHARGED'
  | 'CLOSED';

export type EligibilityCaseStatus =
  | 'NOT_STARTED'
  | 'SCREENING'
  | 'DOCS_MISSING'
  | 'UNDER_REVIEW'
  | 'APPROVED'
  | 'DENIED'
  | 'APPEALED'
  | 'FINAL';

export type IncidentSeverity = 'LOW' | 'MEDIUM' | 'HIGH' | 'CRITICAL';

export type ServiceCategory =
  | 'PERSONAL_CARE'
  | 'COMPANION'
  | 'NURSING'
  | 'REHABILITATION'
  | 'DEMENTIA_CARE'
  | 'RESPITE'
  | 'TRANSPORT'
  | 'MEAL_DELIVERY'
  | 'HOME_MODIFICATION'
  | 'CLEANING';

export type NotificationType =
  | 'INFO'
  | 'WARNING'
  | 'ALERT'
  | 'ACTION_REQUIRED'
  | 'REMINDER';

export type ObservabilityEventType =
  | 'VISIT_COMPLETED'
  | 'VISIT_MISSED'
  | 'MEDICATION_TAKEN'
  | 'MEDICATION_MISSED'
  | 'MEAL_DELIVERED'
  | 'MEAL_FAILED'
  | 'TRANSPORT_COMPLETED'
  | 'TRANSPORT_FAILED'
  | 'SYMPTOM_REPORTED'
  | 'INCIDENT_CREATED'
  | 'ELIGIBILITY_STATUS_CHANGED'
  | 'REFERRAL_UPDATED';

export type CarePlanStatus =
  | 'DRAFT'
  | 'ACTIVE'
  | 'PAUSED'
  | 'COMPLETED'
  | 'CANCELLED';

export type AppointmentStatus =
  | 'SCHEDULED'
  | 'CONFIRMED'
  | 'IN_PROGRESS'
  | 'COMPLETED'
  | 'CANCELLED'
  | 'NO_SHOW';

export type DayOfWeek =
  | 'MONDAY'
  | 'TUESDAY'
  | 'WEDNESDAY'
  | 'THURSDAY'
  | 'FRIDAY'
  | 'SATURDAY'
  | 'SUNDAY';

export type CredentialType =
  | 'CAREGIVER_CERTIFICATE'
  | 'NURSING_LICENSE'
  | 'SOCIAL_WORKER_LICENSE'
  | 'CPR_CERTIFICATION'
  | 'DEMENTIA_TRAINING'
  | 'FIRST_AID'
  | 'OTHER';

export type CredentialStatus = 'PENDING' | 'VERIFIED' | 'EXPIRED' | 'REJECTED';

export type ClaimStatus =
  | 'DRAFT'
  | 'SUBMITTED'
  | 'UNDER_REVIEW'
  | 'APPROVED'
  | 'DENIED'
  | 'PAID';

export type AuditAction =
  | 'CREATE'
  | 'READ'
  | 'UPDATE'
  | 'DELETE'
  | 'LOGIN'
  | 'LOGOUT'
  | 'CONSENT_GRANT'
  | 'CONSENT_REVOKE'
  | 'STATUS_CHANGE';

export type RelationshipType =
  | 'SPOUSE'
  | 'CHILD'
  | 'SIBLING'
  | 'PARENT'
  | 'GRANDCHILD'
  | 'OTHER_RELATIVE'
  | 'LEGAL_GUARDIAN'
  | 'FRIEND'
  | 'SOCIAL_WORKER';

export type ProviderType =
  | 'HOME_CARE_AGENCY'
  | 'NURSING_HOSPITAL'
  | 'CLINIC'
  | 'PHARMACY'
  | 'REHABILITATION_CENTER'
  | 'DEMENTIA_CENTER'
  | 'SILVER_TOWN'
  | 'TRANSPORT_SERVICE'
  | 'MEAL_SERVICE'
  | 'COMMUNITY_CENTER';

export type SignalSeverity = 'INFO' | 'WARNING' | 'ALERT' | 'CRITICAL';

export type MedicationFrequency =
  | 'ONCE_DAILY'
  | 'TWICE_DAILY'
  | 'THREE_TIMES_DAILY'
  | 'FOUR_TIMES_DAILY'
  | 'EVERY_OTHER_DAY'
  | 'WEEKLY'
  | 'AS_NEEDED'
  | 'CUSTOM';

export type ObservationCategory =
  | 'VITAL_SIGNS'
  | 'MOOD'
  | 'APPETITE'
  | 'MOBILITY'
  | 'SLEEP'
  | 'PAIN'
  | 'COGNITIVE'
  | 'SKIN'
  | 'OTHER';

export type HousingType =
  | 'SILVER_TOWN'
  | 'SENIOR_APARTMENT'
  | 'GROUP_HOME'
  | 'ASSISTED_LIVING'
  | 'NURSING_FACILITY';

// -----------------------------------------------------------------------------
// Row interfaces
// -----------------------------------------------------------------------------

export interface User {
  readonly id: string;
  readonly email: string | null;
  readonly email_verified: Date | null;
  readonly name: string | null;
  readonly image: string | null;
  readonly phone: string | null;
  readonly role: UserRole;
  readonly kyc_level: KycLevel;
  readonly locale: string;
  readonly is_active: boolean;
  readonly created_at: Date;
  readonly updated_at: Date;
}

export interface Account {
  readonly id: string;
  readonly user_id: string;
  readonly type: string;
  readonly provider: string;
  readonly provider_account_id: string;
  readonly refresh_token: string | null;
  readonly access_token: string | null;
  readonly expires_at: number | null;
  readonly token_type: string | null;
  readonly scope: string | null;
  readonly id_token: string | null;
  readonly session_state: string | null;
}

export interface Session {
  readonly id: string;
  readonly session_token: string;
  readonly user_id: string;
  readonly expires: Date;
}

export interface VerificationToken {
  readonly identifier: string;
  readonly token: string;
  readonly expires: Date;
}

export interface PersonProfile {
  readonly id: string;
  readonly user_id: string;
  readonly korean_name: string | null;
  readonly english_name: string | null;
  readonly date_of_birth: Date | null;
  readonly gender: Gender | null;
  readonly phone: string | null;
  readonly address: string | null;
  readonly city: string | null;
  readonly district: string | null;
  readonly postal_code: string | null;
  readonly emergency_contact_name: string | null;
  readonly emergency_contact_phone: string | null;
  readonly created_at: Date;
  readonly updated_at: Date;
  readonly created_by: string | null;
  readonly updated_by: string | null;
}

export interface SeniorProfile {
  readonly id: string;
  readonly person_id: string;
  readonly care_level: number | null;
  readonly has_ltci_certification: boolean;
  readonly ltci_number: string | null;
  readonly primary_diagnosis: string | null;
  readonly mobility_level: string | null;
  readonly cognitive_level: string | null;
  readonly lives_alone: boolean;
  readonly preferred_language: string;
  readonly created_at: Date;
  readonly updated_at: Date;
}

export interface FamilyRelationship {
  readonly id: string;
  readonly senior_person_id: string;
  readonly family_person_id: string;
  readonly relationship_type: RelationshipType;
  readonly is_primary_contact: boolean;
  readonly can_make_decisions: boolean;
  readonly created_at: Date;
  readonly updated_at: Date;
}

export interface ConsentRecord {
  readonly id: string;
  readonly subject_person_id: string;
  readonly purpose: ConsentPurpose;
  readonly granted_by: string;
  readonly is_active: boolean;
  readonly granted_at: Date;
  readonly revoked_at: Date | null;
  readonly expires_at: Date | null;
  readonly created_at: Date;
  readonly updated_at: Date;
}

export interface ProviderOrganization {
  readonly id: string;
  readonly name: string;
  readonly type: ProviderType;
  readonly registration_number: string;
  readonly address: string | null;
  readonly city: string | null;
  readonly district: string | null;
  readonly postal_code: string | null;
  readonly phone: string | null;
  readonly email: string | null;
  readonly website: string | null;
  readonly license_number: string | null;
  readonly license_expires_at: Date | null;
  readonly is_active: boolean;
  readonly description: string | null;
  readonly latitude: number | null;
  readonly longitude: number | null;
  readonly created_at: Date;
  readonly updated_at: Date;
  readonly created_by: string | null;
  readonly updated_by: string | null;
}

export interface CaregiverApplication {
  readonly id: string;
  readonly user_id: string;
  readonly provider_id: string | null;
  readonly status: CaregiverApplicationStatus;
  readonly experience_years: number | null;
  readonly bio: string | null;
  readonly specializations: string | null;
  readonly has_dementia_experience: boolean;
  readonly has_overnight_availability: boolean;
  readonly smoking_status: boolean;
  readonly pet_friendly: boolean;
  readonly preferred_gender: Gender | null;
  readonly languages_spoken: string;
  readonly submitted_at: Date | null;
  readonly reviewed_at: Date | null;
  readonly reviewed_by: string | null;
  readonly rejection_reason: string | null;
  readonly created_at: Date;
  readonly updated_at: Date;
}

export interface CaregiverCredential {
  readonly id: string;
  readonly application_id: string;
  readonly type: CredentialType;
  readonly status: CredentialStatus;
  readonly issuer: string | null;
  readonly issued_at: Date | null;
  readonly expires_at: Date | null;
  readonly document_url: string | null;
  readonly verified_at: Date | null;
  readonly verified_by: string | null;
  readonly created_at: Date;
  readonly updated_at: Date;
}

export interface ServiceRegion {
  readonly id: string;
  readonly provider_id: string;
  readonly city: string;
  readonly district: string;
  readonly is_active: boolean;
  readonly created_at: Date;
  readonly updated_at: Date;
}

export interface AvailabilitySlot {
  readonly id: string;
  readonly application_id: string;
  readonly day_of_week: DayOfWeek;
  readonly start_time: string;
  readonly end_time: string;
  readonly is_active: boolean;
  readonly created_at: Date;
  readonly updated_at: Date;
}

export interface ServiceType {
  readonly id: string;
  readonly application_id: string | null;
  readonly category: ServiceCategory;
  readonly name: string;
  readonly description: string | null;
  readonly is_active: boolean;
  readonly created_at: Date;
  readonly updated_at: Date;
}

export interface MatchRequest {
  readonly id: string;
  readonly senior_id: string;
  readonly requested_by: string;
  readonly status: MatchRequestStatus;
  readonly service_category: ServiceCategory;
  readonly region_city: string;
  readonly region_district: string;
  readonly start_date: Date | null;
  readonly end_date: Date | null;
  readonly schedule_notes: string | null;
  readonly language_preference: string | null;
  readonly gender_preference: Gender | null;
  readonly requires_dementia_experience: boolean;
  readonly requires_overnight_care: boolean;
  readonly additional_notes: string | null;
  readonly created_at: Date;
  readonly updated_at: Date;
}

export interface MatchRecommendation {
  readonly id: string;
  readonly match_request_id: string;
  readonly caregiver_application_id: string;
  readonly score: number;
  readonly score_breakdown: Record<string, unknown> | null;
  readonly rank: number;
  readonly is_selected: boolean;
  readonly selected_at: Date | null;
  readonly created_at: Date;
}

export interface CarePlan {
  readonly id: string;
  readonly senior_id: string;
  readonly provider_id: string | null;
  readonly status: CarePlanStatus;
  readonly title: string;
  readonly description: string | null;
  readonly start_date: Date | null;
  readonly end_date: Date | null;
  readonly goals: Record<string, unknown> | null;
  readonly created_at: Date;
  readonly updated_at: Date;
  readonly created_by: string | null;
  readonly updated_by: string | null;
}

export interface Visit {
  readonly id: string;
  readonly care_plan_id: string;
  readonly caregiver_id: string;
  readonly status: VisitStatus;
  readonly scheduled_start: Date;
  readonly scheduled_end: Date;
  readonly actual_start: Date | null;
  readonly actual_end: Date | null;
  readonly check_in_latitude: number | null;
  readonly check_in_longitude: number | null;
  readonly check_out_latitude: number | null;
  readonly check_out_longitude: number | null;
  readonly tasks: Record<string, unknown> | null;
  readonly notes: string | null;
  readonly created_at: Date;
  readonly updated_at: Date;
}

export interface DailyObservation {
  readonly id: string;
  readonly care_plan_id: string;
  readonly observed_by: string;
  readonly category: ObservationCategory;
  readonly date: Date;
  readonly value: string;
  readonly notes: string | null;
  readonly created_at: Date;
  readonly updated_at: Date;
}

export interface Incident {
  readonly id: string;
  readonly visit_id: string | null;
  readonly reported_by: string;
  readonly severity: IncidentSeverity;
  readonly title: string;
  readonly description: string;
  readonly occurred_at: Date;
  readonly resolved_at: Date | null;
  readonly resolution: string | null;
  readonly created_at: Date;
  readonly updated_at: Date;
}

export interface MedicalHistoryEntry {
  readonly id: string;
  readonly person_id: string;
  readonly condition: string;
  readonly diagnosed_at: Date | null;
  readonly treated_by: string | null;
  readonly status: string;
  readonly notes: string | null;
  readonly created_at: Date;
  readonly updated_at: Date;
  readonly created_by: string | null;
  readonly updated_by: string | null;
}

export interface Medication {
  readonly id: string;
  readonly person_id: string;
  readonly name: string;
  readonly dosage: string;
  readonly form: string;
  readonly frequency: MedicationFrequency;
  readonly prescribed_by: string | null;
  readonly prescribed_at: Date | null;
  readonly start_date: Date | null;
  readonly end_date: Date | null;
  readonly is_active: boolean;
  readonly side_effects: string | null;
  readonly notes: string | null;
  readonly created_at: Date;
  readonly updated_at: Date;
  readonly created_by: string | null;
  readonly updated_by: string | null;
}

export interface MedicationSchedule {
  readonly id: string;
  readonly medication_id: string;
  readonly time_of_day: string;
  readonly day_of_week: DayOfWeek | null;
  readonly is_active: boolean;
  readonly created_at: Date;
  readonly updated_at: Date;
}

export interface MedicationEvent {
  readonly id: string;
  readonly medication_id: string;
  readonly scheduled_for: Date;
  readonly status: MedicationEventStatus;
  readonly taken_at: Date | null;
  readonly notes: string | null;
  readonly recorded_by: string | null;
  readonly created_at: Date;
  readonly updated_at: Date;
}

export interface Appointment {
  readonly id: string;
  readonly person_id: string;
  readonly institution_name: string;
  readonly institution_type: ProviderType | null;
  readonly appointment_date: Date;
  readonly status: AppointmentStatus;
  readonly purpose: string | null;
  readonly notes: string | null;
  readonly address: string | null;
  readonly created_at: Date;
  readonly updated_at: Date;
  readonly created_by: string | null;
  readonly updated_by: string | null;
}

export interface InstitutionReferral {
  readonly id: string;
  readonly from_provider_id: string;
  readonly to_provider_id: string;
  readonly senior_person_id: string;
  readonly status: InstitutionReferralStatus;
  readonly reason: string | null;
  readonly notes: string | null;
  readonly referred_at: Date;
  readonly accepted_at: Date | null;
  readonly discharged_at: Date | null;
  readonly created_at: Date;
  readonly updated_at: Date;
}

export interface EligibilityCase {
  readonly id: string;
  readonly senior_id: string;
  readonly status: EligibilityCaseStatus;
  readonly program_name: string;
  readonly application_date: Date | null;
  readonly determination_date: Date | null;
  readonly notes: string | null;
  readonly denial_reason: string | null;
  readonly created_at: Date;
  readonly updated_at: Date;
  readonly created_by: string | null;
  readonly updated_by: string | null;
}

export interface ApprovalStep {
  readonly id: string;
  readonly case_id: string;
  readonly step_name: string;
  readonly step_order: number;
  readonly status: string;
  readonly assigned_to: string | null;
  readonly completed_at: Date | null;
  readonly notes: string | null;
  readonly created_at: Date;
  readonly updated_at: Date;
}

export interface ClaimOrSubsidyRecord {
  readonly id: string;
  readonly case_id: string;
  readonly claim_number: string;
  readonly status: ClaimStatus;
  readonly amount: string; // DECIMAL is returned as string by pg driver
  readonly currency: string;
  readonly service_date: Date;
  readonly submitted_at: Date | null;
  readonly processed_at: Date | null;
  readonly notes: string | null;
  readonly created_at: Date;
  readonly updated_at: Date;
}

export interface ObservabilitySignal {
  readonly id: string;
  readonly event_type: ObservabilityEventType;
  readonly severity: SignalSeverity;
  readonly subject_person_id: string | null;
  readonly actor_user_id: string | null;
  readonly entity_type: string | null;
  readonly entity_id: string | null;
  readonly message: string;
  readonly metadata: Record<string, unknown> | null;
  readonly acknowledged_at: Date | null;
  readonly acknowledged_by: string | null;
  readonly created_at: Date;
}

export interface Notification {
  readonly id: string;
  readonly user_id: string;
  readonly type: NotificationType;
  readonly title: string;
  readonly message: string;
  readonly link: string | null;
  readonly is_read: boolean;
  readonly read_at: Date | null;
  readonly created_at: Date;
}

export interface AuditLog {
  readonly id: string;
  readonly user_id: string | null;
  readonly action: AuditAction;
  readonly entity_type: string | null;
  readonly entity_id: string | null;
  readonly old_value: Record<string, unknown> | null;
  readonly new_value: Record<string, unknown> | null;
  readonly ip_address: string | null;
  readonly user_agent: string | null;
  readonly created_at: Date;
}
