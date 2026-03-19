// =============================================================================
// Medical Types — HIS-lite structs for medical portal
// =============================================================================

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::enums::HandoffLicenseType;

// -- Medical Professional Profile --

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct MedicalProfessionalProfile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub license_type: HandoffLicenseType,
    pub license_number: String,
    pub institution_id: Option<Uuid>,
    pub specialty: Option<String>,
    pub is_verified: bool,
    pub verified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// -- Handoff Session --

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct MedicalHandoffSession {
    pub id: Uuid,
    pub senior_person_id: Uuid,
    pub professional_user_id: Uuid,
    pub license_type: HandoffLicenseType,
    pub license_number: String,
    pub institution_name: Option<String>,
    pub institution_id: Option<Uuid>,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub expires_at: DateTime<Utc>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

// -- Prescription --

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Prescription {
    pub id: Uuid,
    pub senior_person_id: Uuid,
    pub prescribed_by: Uuid,
    pub institution_id: Option<Uuid>,
    pub medication_name: String,
    pub dosage: String,
    pub frequency: String,
    pub duration_days: Option<i32>,
    pub instructions: Option<String>,
    pub is_signed: bool,
    pub signed_at: Option<DateTime<Utc>>,
    pub signed_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// -- Clinical Encounter --

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct ClinicalEncounter {
    pub id: Uuid,
    pub senior_person_id: Uuid,
    pub provider_user_id: Uuid,
    pub institution_id: Option<Uuid>,
    pub encounter_date: DateTime<Utc>,
    pub subjective: Option<String>,
    pub objective: Option<String>,
    pub assessment: Option<String>,
    pub plan: Option<String>,
    pub is_signed: bool,
    pub signed_at: Option<DateTime<Utc>>,
    pub addendum: Option<String>,
    pub addendum_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// -- Lab Result --

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct LabResult {
    pub id: Uuid,
    pub senior_person_id: Uuid,
    pub ordered_by: Option<Uuid>,
    pub test_name: String,
    pub test_code: Option<String>,
    pub result_value: Option<String>,
    pub result_unit: Option<String>,
    pub reference_range: Option<String>,
    pub is_critical: bool,
    pub reviewed_by: Option<Uuid>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// -- Patient Allergy --

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct PatientAllergy {
    pub id: Uuid,
    pub senior_person_id: Uuid,
    pub allergen: String,
    pub reaction: Option<String>,
    pub severity: Option<String>,
    pub is_active: bool,
    pub reported_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// -- Document Transfer Request --

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct DocumentTransferRequest {
    pub id: Uuid,
    pub from_institution_id: Uuid,
    pub to_institution_id: Uuid,
    pub senior_person_id: Uuid,
    pub document_type: String,
    pub status: String,
    pub requested_by: Uuid,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// -- Generic Substitution --

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct GenericSubstitutionRecord {
    pub id: Uuid,
    pub prescription_id: Uuid,
    pub original_medication: String,
    pub substituted_medication: String,
    pub reason: Option<String>,
    pub pharmacist_user_id: Uuid,
    pub prescriber_notified: bool,
    pub prescriber_notified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

// -- Input DTOs --

#[derive(Debug, Clone, Serialize, Deserialize, validator::Validate)]
pub struct StartHandoffInput {
    pub senior_person_id: Uuid,
    #[validate(length(min = 1))]
    pub license_number: String,
    pub license_type: HandoffLicenseType,
    pub institution_name: Option<String>,
    pub institution_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, validator::Validate)]
pub struct CreatePrescriptionInput {
    pub senior_person_id: Uuid,
    #[validate(length(min = 1))]
    pub medication_name: String,
    #[validate(length(min = 1))]
    pub dosage: String,
    #[validate(length(min = 1))]
    pub frequency: String,
    pub duration_days: Option<i32>,
    pub instructions: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, validator::Validate)]
pub struct CreateEncounterInput {
    pub senior_person_id: Uuid,
    pub subjective: Option<String>,
    pub objective: Option<String>,
    pub assessment: Option<String>,
    pub plan: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, validator::Validate)]
pub struct CreateLabResultInput {
    pub senior_person_id: Uuid,
    #[validate(length(min = 1))]
    pub test_name: String,
    pub test_code: Option<String>,
    pub result_value: Option<String>,
    pub result_unit: Option<String>,
    pub reference_range: Option<String>,
    pub is_critical: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, validator::Validate)]
pub struct CreateAllergyInput {
    pub senior_person_id: Uuid,
    #[validate(length(min = 1))]
    pub allergen: String,
    pub reaction: Option<String>,
    pub severity: Option<String>,
}
