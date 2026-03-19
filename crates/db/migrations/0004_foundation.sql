-- =============================================================================
-- Migration 0004: Foundation — indexes, schema fixes, new enums, care_level migration
-- =============================================================================

-- ---------------------------------------------------------------------------
-- Part A: Missing indexes (16)
-- ---------------------------------------------------------------------------

CREATE INDEX IF NOT EXISTS idx_appointments_person_id ON appointments(person_id);
CREATE INDEX IF NOT EXISTS idx_medications_person_id ON medications(person_id);
CREATE INDEX IF NOT EXISTS idx_care_plans_senior_id ON care_plans(senior_id);
CREATE INDEX IF NOT EXISTS idx_visits_care_plan_id ON visits(care_plan_id);
CREATE INDEX IF NOT EXISTS idx_visits_caregiver_id ON visits(caregiver_id);
CREATE INDEX IF NOT EXISTS idx_daily_observations_care_plan_id ON daily_observations(care_plan_id);
CREATE INDEX IF NOT EXISTS idx_medical_history_person_id ON medical_history_entries(person_id);
CREATE INDEX IF NOT EXISTS idx_incidents_visit_id ON incidents(visit_id);
CREATE INDEX IF NOT EXISTS idx_match_recs_request_id ON match_recommendations(match_request_id);
CREATE INDEX IF NOT EXISTS idx_match_recs_caregiver_id ON match_recommendations(caregiver_application_id);
CREATE INDEX IF NOT EXISTS idx_family_rels_senior_id ON family_relationships(senior_person_id);
CREATE INDEX IF NOT EXISTS idx_creds_application_id ON caregiver_credentials(application_id);
CREATE INDEX IF NOT EXISTS idx_avail_application_id ON availability_slots(application_id);
CREATE INDEX IF NOT EXISTS idx_service_types_app_id ON service_types(application_id);
CREATE INDEX IF NOT EXISTS idx_approval_steps_case_id ON approval_steps(case_id);
CREATE INDEX IF NOT EXISTS idx_claims_case_id ON claim_or_subsidy_records(case_id);

-- ---------------------------------------------------------------------------
-- Part B: Schema fixes
-- ---------------------------------------------------------------------------

-- Soft-delete support for core tables
ALTER TABLE appointments ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ;
ALTER TABLE medications ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ;
ALTER TABLE care_plans ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ;
ALTER TABLE visits ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ;
ALTER TABLE incidents ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ;
ALTER TABLE consent_records ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ;

-- Approval workflow columns for appointments
ALTER TABLE appointments ADD COLUMN IF NOT EXISTS approval_status TEXT NOT NULL DEFAULT 'approved';
ALTER TABLE appointments ADD COLUMN IF NOT EXISTS submitted_by UUID REFERENCES users(id);
ALTER TABLE appointments ADD COLUMN IF NOT EXISTS approved_by UUID REFERENCES users(id);
ALTER TABLE appointments ADD COLUMN IF NOT EXISTS approved_at TIMESTAMPTZ;

-- Approval workflow columns for medications
ALTER TABLE medications ADD COLUMN IF NOT EXISTS approval_status TEXT NOT NULL DEFAULT 'approved';
ALTER TABLE medications ADD COLUMN IF NOT EXISTS submitted_by UUID REFERENCES users(id);
ALTER TABLE medications ADD COLUMN IF NOT EXISTS approved_by UUID REFERENCES users(id);
ALTER TABLE medications ADD COLUMN IF NOT EXISTS approved_at TIMESTAMPTZ;

-- GPS distance tracking for visits
ALTER TABLE visits ADD COLUMN IF NOT EXISTS check_in_distance_meters DOUBLE PRECISION;

-- Institution codes for provider organizations
ALTER TABLE provider_organizations ADD COLUMN IF NOT EXISTS nhis_institution_code TEXT UNIQUE;
ALTER TABLE provider_organizations ADD COLUMN IF NOT EXISTS medical_institution_code TEXT UNIQUE;

-- ---------------------------------------------------------------------------
-- Part C: New enums (8 new + 2 modified)
-- ---------------------------------------------------------------------------

-- 1. handoff_license_type
DO $$ BEGIN
    CREATE TYPE handoff_license_type AS ENUM ('doctor', 'nurse', 'pharmacist');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

-- 2. consent_purpose_v2
DO $$ BEGIN
    CREATE TYPE consent_purpose_v2 AS ENUM (
        'personal_info_collection',
        'sensitive_info_processing',
        'third_party_medical',
        'third_party_government',
        'third_party_provider',
        'third_party_family',
        'third_party_caregiver',
        'marketing'
    );
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

-- 3. copayment_tier
DO $$ BEGIN
    CREATE TYPE copayment_tier AS ENUM ('exempt', 'reduction_60', 'reduction_40', 'standard');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

-- 4. credential_classification
DO $$ BEGIN
    CREATE TYPE credential_classification AS ENUM ('license', 'qualification');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

-- 5. care_level_enum
DO $$ BEGIN
    CREATE TYPE care_level_enum AS ENUM ('level_1', 'level_2', 'level_3', 'level_4', 'level_5', 'cognitive');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

-- 6. internal_permission_level
DO $$ BEGIN
    CREATE TYPE internal_permission_level AS ENUM ('staff', 'manager', 'security_admin', 'org_admin');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

-- 7. wellness_mood
DO $$ BEGIN
    CREATE TYPE wellness_mood AS ENUM ('good', 'okay', 'not_great', 'need_help');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

-- 8. emergency_event_status
DO $$ BEGIN
    CREATE TYPE emergency_event_status AS ENUM ('triggered', 'responders_notified', 'resolved', 'false_alarm');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

-- Modify existing enums: add new values
ALTER TYPE user_role ADD VALUE IF NOT EXISTS 'PHARMACY_STAFF';
ALTER TYPE notification_type ADD VALUE IF NOT EXISTS 'EMERGENCY';

-- ---------------------------------------------------------------------------
-- Part D: care_level INTEGER → care_level_enum migration
-- ---------------------------------------------------------------------------

-- Add copayment_tier to senior_profiles
ALTER TABLE senior_profiles ADD COLUMN IF NOT EXISTS copayment_tier copayment_tier NOT NULL DEFAULT 'standard';

-- Add new care_level column as enum
ALTER TABLE senior_profiles ADD COLUMN IF NOT EXISTS care_level_new care_level_enum;

-- Migrate existing integer data
UPDATE senior_profiles SET care_level_new = CASE
    WHEN care_level = 1 THEN 'level_1'::care_level_enum
    WHEN care_level = 2 THEN 'level_2'::care_level_enum
    WHEN care_level = 3 THEN 'level_3'::care_level_enum
    WHEN care_level = 4 THEN 'level_4'::care_level_enum
    WHEN care_level = 5 THEN 'level_5'::care_level_enum
    ELSE NULL
END
WHERE care_level IS NOT NULL AND care_level_new IS NULL;

-- Drop old column and rename new one
ALTER TABLE senior_profiles DROP COLUMN IF EXISTS care_level;
ALTER TABLE senior_profiles RENAME COLUMN care_level_new TO care_level;
