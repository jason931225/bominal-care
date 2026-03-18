-- =============================================================================
-- Korea Senior Care Portal — PostgreSQL DDL Schema
-- =============================================================================

CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- =============================================================================
-- ENUMS
-- =============================================================================

CREATE TYPE user_role AS ENUM (
  'SENIOR',
  'FAMILY',
  'CAREGIVER_APPLICANT',
  'CAREGIVER_APPROVED',
  'PROVIDER_ADMIN',
  'PROVIDER_STAFF',
  'MEDICAL_STAFF',
  'GOVERNMENT_REVIEWER',
  'PARTNER_OPERATOR',
  'PLATFORM_ADMIN'
);

CREATE TYPE gender AS ENUM (
  'MALE',
  'FEMALE',
  'OTHER',
  'PREFER_NOT_TO_SAY'
);

CREATE TYPE kyc_level AS ENUM (
  'NONE',
  'EMAIL_VERIFIED',
  'PHONE_VERIFIED',
  'IDENTITY_VERIFIED',
  'FULL_VERIFIED'
);

CREATE TYPE consent_purpose AS ENUM (
  'NO_SHARE',
  'MEDICAL_SHARE',
  'GOVERNMENT_SHARE',
  'BOTH_SHARE'
);

CREATE TYPE caregiver_application_status AS ENUM (
  'DRAFT',
  'SUBMITTED',
  'IDENTITY_VERIFIED',
  'CREDENTIAL_REVIEW',
  'APPROVED_PRIVATE_PAY',
  'APPROVED_UNDER_PROVIDER',
  'SUSPENDED',
  'REJECTED'
);

CREATE TYPE match_request_status AS ENUM (
  'CREATED',
  'SEARCHING',
  'RECOMMENDATIONS_READY',
  'SELECTED',
  'BOOKED',
  'FULFILLED',
  'CANCELLED'
);

CREATE TYPE visit_status AS ENUM (
  'SCHEDULED',
  'CAREGIVER_ACKNOWLEDGED',
  'IN_PROGRESS',
  'COMPLETED',
  'MISSED',
  'CANCELLED'
);

CREATE TYPE medication_event_status AS ENUM (
  'SCHEDULED',
  'REMINDER_SENT',
  'TAKEN',
  'MISSED',
  'HELD',
  'ESCALATED'
);

CREATE TYPE institution_referral_status AS ENUM (
  'CREATED',
  'ACCEPTED',
  'BOOKED',
  'ATTENDED',
  'DISCHARGED',
  'CLOSED'
);

CREATE TYPE eligibility_case_status AS ENUM (
  'NOT_STARTED',
  'SCREENING',
  'DOCS_MISSING',
  'UNDER_REVIEW',
  'APPROVED',
  'DENIED',
  'APPEALED',
  'FINAL'
);

CREATE TYPE incident_severity AS ENUM (
  'LOW',
  'MEDIUM',
  'HIGH',
  'CRITICAL'
);

CREATE TYPE service_category AS ENUM (
  'PERSONAL_CARE',
  'COMPANION',
  'NURSING',
  'REHABILITATION',
  'DEMENTIA_CARE',
  'RESPITE',
  'TRANSPORT',
  'MEAL_DELIVERY',
  'HOME_MODIFICATION',
  'CLEANING'
);

CREATE TYPE notification_type AS ENUM (
  'INFO',
  'WARNING',
  'ALERT',
  'ACTION_REQUIRED',
  'REMINDER'
);

CREATE TYPE observability_event_type AS ENUM (
  'VISIT_COMPLETED',
  'VISIT_MISSED',
  'MEDICATION_TAKEN',
  'MEDICATION_MISSED',
  'MEAL_DELIVERED',
  'MEAL_FAILED',
  'TRANSPORT_COMPLETED',
  'TRANSPORT_FAILED',
  'SYMPTOM_REPORTED',
  'INCIDENT_CREATED',
  'ELIGIBILITY_STATUS_CHANGED',
  'REFERRAL_UPDATED'
);

CREATE TYPE care_plan_status AS ENUM (
  'DRAFT',
  'ACTIVE',
  'PAUSED',
  'COMPLETED',
  'CANCELLED'
);

CREATE TYPE appointment_status AS ENUM (
  'SCHEDULED',
  'CONFIRMED',
  'IN_PROGRESS',
  'COMPLETED',
  'CANCELLED',
  'NO_SHOW'
);

CREATE TYPE day_of_week AS ENUM (
  'MONDAY',
  'TUESDAY',
  'WEDNESDAY',
  'THURSDAY',
  'FRIDAY',
  'SATURDAY',
  'SUNDAY'
);

CREATE TYPE credential_type AS ENUM (
  'CAREGIVER_CERTIFICATE',
  'NURSING_LICENSE',
  'SOCIAL_WORKER_LICENSE',
  'CPR_CERTIFICATION',
  'DEMENTIA_TRAINING',
  'FIRST_AID',
  'OTHER'
);

CREATE TYPE credential_status AS ENUM (
  'PENDING',
  'VERIFIED',
  'EXPIRED',
  'REJECTED'
);

CREATE TYPE claim_status AS ENUM (
  'DRAFT',
  'SUBMITTED',
  'UNDER_REVIEW',
  'APPROVED',
  'DENIED',
  'PAID'
);

CREATE TYPE audit_action AS ENUM (
  'CREATE',
  'READ',
  'UPDATE',
  'DELETE',
  'LOGIN',
  'LOGOUT',
  'CONSENT_GRANT',
  'CONSENT_REVOKE',
  'STATUS_CHANGE'
);

CREATE TYPE relationship_type AS ENUM (
  'SPOUSE',
  'CHILD',
  'SIBLING',
  'PARENT',
  'GRANDCHILD',
  'OTHER_RELATIVE',
  'LEGAL_GUARDIAN',
  'FRIEND',
  'SOCIAL_WORKER'
);

CREATE TYPE provider_type AS ENUM (
  'HOME_CARE_AGENCY',
  'NURSING_HOSPITAL',
  'CLINIC',
  'PHARMACY',
  'REHABILITATION_CENTER',
  'DEMENTIA_CENTER',
  'SILVER_TOWN',
  'TRANSPORT_SERVICE',
  'MEAL_SERVICE',
  'COMMUNITY_CENTER'
);

CREATE TYPE signal_severity AS ENUM (
  'INFO',
  'WARNING',
  'ALERT',
  'CRITICAL'
);

CREATE TYPE medication_frequency AS ENUM (
  'ONCE_DAILY',
  'TWICE_DAILY',
  'THREE_TIMES_DAILY',
  'FOUR_TIMES_DAILY',
  'EVERY_OTHER_DAY',
  'WEEKLY',
  'AS_NEEDED',
  'CUSTOM'
);

CREATE TYPE observation_category AS ENUM (
  'VITAL_SIGNS',
  'MOOD',
  'APPETITE',
  'MOBILITY',
  'SLEEP',
  'PAIN',
  'COGNITIVE',
  'SKIN',
  'OTHER'
);

CREATE TYPE housing_type AS ENUM (
  'SILVER_TOWN',
  'SENIOR_APARTMENT',
  'GROUP_HOME',
  'ASSISTED_LIVING',
  'NURSING_FACILITY'
);

-- =============================================================================
-- TABLES
-- =============================================================================

-- -----------------------------------------------------------------------------
-- 1. users
-- -----------------------------------------------------------------------------
CREATE TABLE users (
  id             UUID          NOT NULL DEFAULT gen_random_uuid(),
  email          TEXT          UNIQUE,
  email_verified TIMESTAMPTZ,
  name           TEXT,
  image          TEXT,
  phone          TEXT          UNIQUE,
  role           user_role     NOT NULL DEFAULT 'SENIOR',
  kyc_level      kyc_level     NOT NULL DEFAULT 'NONE',
  locale         TEXT          NOT NULL DEFAULT 'ko',
  is_active      BOOLEAN       NOT NULL DEFAULT TRUE,
  created_at     TIMESTAMPTZ   NOT NULL DEFAULT NOW(),
  updated_at     TIMESTAMPTZ   NOT NULL DEFAULT NOW(),

  CONSTRAINT users_pkey PRIMARY KEY (id)
);

-- -----------------------------------------------------------------------------
-- 2. accounts
-- -----------------------------------------------------------------------------
CREATE TABLE accounts (
  id                  UUID  NOT NULL DEFAULT gen_random_uuid(),
  user_id             UUID  NOT NULL,
  type                TEXT  NOT NULL,
  provider            TEXT  NOT NULL,
  provider_account_id TEXT  NOT NULL,
  refresh_token       TEXT,
  access_token        TEXT,
  expires_at          INTEGER,
  token_type          TEXT,
  scope               TEXT,
  id_token            TEXT,
  session_state       TEXT,

  CONSTRAINT accounts_pkey PRIMARY KEY (id),
  CONSTRAINT accounts_user_id_fkey FOREIGN KEY (user_id)
    REFERENCES users (id) ON DELETE CASCADE,
  CONSTRAINT accounts_provider_provider_account_id_key UNIQUE (provider, provider_account_id)
);

-- -----------------------------------------------------------------------------
-- 3. sessions
-- -----------------------------------------------------------------------------
CREATE TABLE sessions (
  id            UUID        NOT NULL DEFAULT gen_random_uuid(),
  session_token TEXT        NOT NULL UNIQUE,
  user_id       UUID        NOT NULL,
  expires       TIMESTAMPTZ NOT NULL,

  CONSTRAINT sessions_pkey PRIMARY KEY (id),
  CONSTRAINT sessions_user_id_fkey FOREIGN KEY (user_id)
    REFERENCES users (id) ON DELETE CASCADE
);

-- -----------------------------------------------------------------------------
-- 4. verification_tokens
-- -----------------------------------------------------------------------------
CREATE TABLE verification_tokens (
  identifier TEXT        NOT NULL,
  token      TEXT        NOT NULL UNIQUE,
  expires    TIMESTAMPTZ NOT NULL,

  CONSTRAINT verification_tokens_identifier_token_key UNIQUE (identifier, token)
);

-- -----------------------------------------------------------------------------
-- 5. person_profiles
-- -----------------------------------------------------------------------------
CREATE TABLE person_profiles (
  id                      UUID        NOT NULL DEFAULT gen_random_uuid(),
  user_id                 UUID        NOT NULL UNIQUE,
  korean_name             TEXT,
  english_name            TEXT,
  date_of_birth           TIMESTAMPTZ,
  gender                  gender,
  phone                   TEXT,
  address                 TEXT,
  city                    TEXT,
  district                TEXT,
  postal_code             TEXT,
  emergency_contact_name  TEXT,
  emergency_contact_phone TEXT,
  created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  created_by              UUID,
  updated_by              UUID,

  CONSTRAINT person_profiles_pkey PRIMARY KEY (id),
  CONSTRAINT person_profiles_user_id_fkey FOREIGN KEY (user_id)
    REFERENCES users (id) ON DELETE CASCADE
);

-- -----------------------------------------------------------------------------
-- 6. senior_profiles
-- -----------------------------------------------------------------------------
CREATE TABLE senior_profiles (
  id                    UUID        NOT NULL DEFAULT gen_random_uuid(),
  person_id             UUID        NOT NULL UNIQUE,
  care_level            INTEGER,
  has_ltci_certification BOOLEAN    NOT NULL DEFAULT FALSE,
  ltci_number           TEXT,
  primary_diagnosis     TEXT,
  mobility_level        TEXT,
  cognitive_level       TEXT,
  lives_alone           BOOLEAN     NOT NULL DEFAULT FALSE,
  preferred_language    TEXT        NOT NULL DEFAULT 'ko',
  created_at            TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at            TIMESTAMPTZ NOT NULL DEFAULT NOW(),

  CONSTRAINT senior_profiles_pkey PRIMARY KEY (id),
  CONSTRAINT senior_profiles_person_id_fkey FOREIGN KEY (person_id)
    REFERENCES person_profiles (id) ON DELETE CASCADE
);

-- -----------------------------------------------------------------------------
-- 7. family_relationships
-- -----------------------------------------------------------------------------
CREATE TABLE family_relationships (
  id                  UUID              NOT NULL DEFAULT gen_random_uuid(),
  senior_person_id    UUID              NOT NULL,
  family_person_id    UUID              NOT NULL,
  relationship_type   relationship_type NOT NULL,
  is_primary_contact  BOOLEAN           NOT NULL DEFAULT FALSE,
  can_make_decisions  BOOLEAN           NOT NULL DEFAULT FALSE,
  created_at          TIMESTAMPTZ       NOT NULL DEFAULT NOW(),
  updated_at          TIMESTAMPTZ       NOT NULL DEFAULT NOW(),

  CONSTRAINT family_relationships_pkey PRIMARY KEY (id),
  CONSTRAINT family_relationships_senior_person_id_family_person_id_key UNIQUE (senior_person_id, family_person_id),
  CONSTRAINT family_relationships_senior_person_id_fkey FOREIGN KEY (senior_person_id)
    REFERENCES person_profiles (id) ON DELETE CASCADE,
  CONSTRAINT family_relationships_family_person_id_fkey FOREIGN KEY (family_person_id)
    REFERENCES person_profiles (id) ON DELETE CASCADE
);

-- -----------------------------------------------------------------------------
-- 8. consent_records
-- -----------------------------------------------------------------------------
CREATE TABLE consent_records (
  id                UUID           NOT NULL DEFAULT gen_random_uuid(),
  subject_person_id UUID           NOT NULL,
  purpose           consent_purpose NOT NULL,
  granted_by        UUID           NOT NULL,
  is_active         BOOLEAN        NOT NULL DEFAULT TRUE,
  granted_at        TIMESTAMPTZ    NOT NULL DEFAULT NOW(),
  revoked_at        TIMESTAMPTZ,
  expires_at        TIMESTAMPTZ,
  created_at        TIMESTAMPTZ    NOT NULL DEFAULT NOW(),
  updated_at        TIMESTAMPTZ    NOT NULL DEFAULT NOW(),

  CONSTRAINT consent_records_pkey PRIMARY KEY (id),
  CONSTRAINT consent_records_subject_person_id_fkey FOREIGN KEY (subject_person_id)
    REFERENCES person_profiles (id) ON DELETE CASCADE
);

CREATE INDEX consent_records_subject_person_id_purpose_idx
  ON consent_records (subject_person_id, purpose);

-- -----------------------------------------------------------------------------
-- 9. provider_organizations
-- -----------------------------------------------------------------------------
CREATE TABLE provider_organizations (
  id                  UUID          NOT NULL DEFAULT gen_random_uuid(),
  name                TEXT          NOT NULL,
  type                provider_type NOT NULL,
  registration_number TEXT          NOT NULL UNIQUE,
  address             TEXT,
  city                TEXT,
  district            TEXT,
  postal_code         TEXT,
  phone               TEXT,
  email               TEXT,
  website             TEXT,
  license_number      TEXT,
  license_expires_at  TIMESTAMPTZ,
  is_active           BOOLEAN       NOT NULL DEFAULT TRUE,
  description         TEXT,
  latitude            DOUBLE PRECISION,
  longitude           DOUBLE PRECISION,
  created_at          TIMESTAMPTZ   NOT NULL DEFAULT NOW(),
  updated_at          TIMESTAMPTZ   NOT NULL DEFAULT NOW(),
  created_by          UUID,
  updated_by          UUID,

  CONSTRAINT provider_organizations_pkey PRIMARY KEY (id)
);

-- -----------------------------------------------------------------------------
-- 10. caregiver_applications
-- -----------------------------------------------------------------------------
CREATE TABLE caregiver_applications (
  id                        UUID                       NOT NULL DEFAULT gen_random_uuid(),
  user_id                   UUID                       NOT NULL,
  provider_id               UUID,
  status                    caregiver_application_status NOT NULL DEFAULT 'DRAFT',
  experience_years          INTEGER,
  bio                       TEXT,
  specializations           TEXT,
  has_dementia_experience   BOOLEAN                    NOT NULL DEFAULT FALSE,
  has_overnight_availability BOOLEAN                   NOT NULL DEFAULT FALSE,
  smoking_status            BOOLEAN                    NOT NULL DEFAULT FALSE,
  pet_friendly              BOOLEAN                    NOT NULL DEFAULT TRUE,
  preferred_gender          gender,
  languages_spoken          TEXT                       NOT NULL DEFAULT 'ko',
  submitted_at              TIMESTAMPTZ,
  reviewed_at               TIMESTAMPTZ,
  reviewed_by               UUID,
  rejection_reason          TEXT,
  created_at                TIMESTAMPTZ                NOT NULL DEFAULT NOW(),
  updated_at                TIMESTAMPTZ                NOT NULL DEFAULT NOW(),

  CONSTRAINT caregiver_applications_pkey PRIMARY KEY (id),
  CONSTRAINT caregiver_applications_user_id_fkey FOREIGN KEY (user_id)
    REFERENCES users (id) ON DELETE RESTRICT,
  CONSTRAINT caregiver_applications_provider_id_fkey FOREIGN KEY (provider_id)
    REFERENCES provider_organizations (id) ON DELETE SET NULL
);

-- -----------------------------------------------------------------------------
-- 11. caregiver_credentials
-- -----------------------------------------------------------------------------
CREATE TABLE caregiver_credentials (
  id             UUID              NOT NULL DEFAULT gen_random_uuid(),
  application_id UUID              NOT NULL,
  type           credential_type   NOT NULL,
  status         credential_status NOT NULL DEFAULT 'PENDING',
  issuer         TEXT,
  issued_at      TIMESTAMPTZ,
  expires_at     TIMESTAMPTZ,
  document_url   TEXT,
  verified_at    TIMESTAMPTZ,
  verified_by    UUID,
  created_at     TIMESTAMPTZ       NOT NULL DEFAULT NOW(),
  updated_at     TIMESTAMPTZ       NOT NULL DEFAULT NOW(),

  CONSTRAINT caregiver_credentials_pkey PRIMARY KEY (id),
  CONSTRAINT caregiver_credentials_application_id_fkey FOREIGN KEY (application_id)
    REFERENCES caregiver_applications (id) ON DELETE CASCADE
);

-- -----------------------------------------------------------------------------
-- 12. service_regions
-- -----------------------------------------------------------------------------
CREATE TABLE service_regions (
  id          UUID        NOT NULL DEFAULT gen_random_uuid(),
  provider_id UUID        NOT NULL,
  city        TEXT        NOT NULL,
  district    TEXT        NOT NULL,
  is_active   BOOLEAN     NOT NULL DEFAULT TRUE,
  created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),

  CONSTRAINT service_regions_pkey PRIMARY KEY (id),
  CONSTRAINT service_regions_provider_id_city_district_key UNIQUE (provider_id, city, district),
  CONSTRAINT service_regions_provider_id_fkey FOREIGN KEY (provider_id)
    REFERENCES provider_organizations (id) ON DELETE CASCADE
);

-- -----------------------------------------------------------------------------
-- 13. availability_slots
-- -----------------------------------------------------------------------------
CREATE TABLE availability_slots (
  id             UUID        NOT NULL DEFAULT gen_random_uuid(),
  application_id UUID        NOT NULL,
  day_of_week    day_of_week NOT NULL,
  start_time     TEXT        NOT NULL,
  end_time       TEXT        NOT NULL,
  is_active      BOOLEAN     NOT NULL DEFAULT TRUE,
  created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),

  CONSTRAINT availability_slots_pkey PRIMARY KEY (id),
  CONSTRAINT availability_slots_application_id_fkey FOREIGN KEY (application_id)
    REFERENCES caregiver_applications (id) ON DELETE CASCADE
);

-- -----------------------------------------------------------------------------
-- 14. service_types
-- -----------------------------------------------------------------------------
CREATE TABLE service_types (
  id             UUID             NOT NULL DEFAULT gen_random_uuid(),
  application_id UUID,
  category       service_category NOT NULL,
  name           TEXT             NOT NULL,
  description    TEXT,
  is_active      BOOLEAN          NOT NULL DEFAULT TRUE,
  created_at     TIMESTAMPTZ      NOT NULL DEFAULT NOW(),
  updated_at     TIMESTAMPTZ      NOT NULL DEFAULT NOW(),

  CONSTRAINT service_types_pkey PRIMARY KEY (id),
  CONSTRAINT service_types_application_id_fkey FOREIGN KEY (application_id)
    REFERENCES caregiver_applications (id) ON DELETE SET NULL
);

-- -----------------------------------------------------------------------------
-- 15. match_requests
-- -----------------------------------------------------------------------------
CREATE TABLE match_requests (
  id                          UUID               NOT NULL DEFAULT gen_random_uuid(),
  senior_id                   UUID               NOT NULL,
  requested_by                UUID               NOT NULL,
  status                      match_request_status NOT NULL DEFAULT 'CREATED',
  service_category            service_category   NOT NULL,
  region_city                 TEXT               NOT NULL,
  region_district             TEXT               NOT NULL,
  start_date                  TIMESTAMPTZ,
  end_date                    TIMESTAMPTZ,
  schedule_notes              TEXT,
  language_preference         TEXT,
  gender_preference           gender,
  requires_dementia_experience BOOLEAN           NOT NULL DEFAULT FALSE,
  requires_overnight_care     BOOLEAN            NOT NULL DEFAULT FALSE,
  additional_notes            TEXT,
  created_at                  TIMESTAMPTZ        NOT NULL DEFAULT NOW(),
  updated_at                  TIMESTAMPTZ        NOT NULL DEFAULT NOW(),

  CONSTRAINT match_requests_pkey PRIMARY KEY (id),
  CONSTRAINT match_requests_senior_id_fkey FOREIGN KEY (senior_id)
    REFERENCES senior_profiles (id) ON DELETE RESTRICT
);

-- -----------------------------------------------------------------------------
-- 16. match_recommendations
-- -----------------------------------------------------------------------------
CREATE TABLE match_recommendations (
  id                      UUID        NOT NULL DEFAULT gen_random_uuid(),
  match_request_id        UUID        NOT NULL,
  caregiver_application_id UUID       NOT NULL,
  score                   DOUBLE PRECISION NOT NULL,
  score_breakdown         JSONB,
  rank                    INTEGER     NOT NULL,
  is_selected             BOOLEAN     NOT NULL DEFAULT FALSE,
  selected_at             TIMESTAMPTZ,
  created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),

  CONSTRAINT match_recommendations_pkey PRIMARY KEY (id),
  CONSTRAINT match_recommendations_match_request_id_fkey FOREIGN KEY (match_request_id)
    REFERENCES match_requests (id) ON DELETE CASCADE,
  CONSTRAINT match_recommendations_caregiver_application_id_fkey FOREIGN KEY (caregiver_application_id)
    REFERENCES caregiver_applications (id) ON DELETE RESTRICT
);

-- -----------------------------------------------------------------------------
-- 17. care_plans
-- -----------------------------------------------------------------------------
CREATE TABLE care_plans (
  id          UUID             NOT NULL DEFAULT gen_random_uuid(),
  senior_id   UUID             NOT NULL,
  provider_id UUID,
  status      care_plan_status NOT NULL DEFAULT 'DRAFT',
  title       TEXT             NOT NULL,
  description TEXT,
  start_date  TIMESTAMPTZ,
  end_date    TIMESTAMPTZ,
  goals       JSONB,
  created_at  TIMESTAMPTZ      NOT NULL DEFAULT NOW(),
  updated_at  TIMESTAMPTZ      NOT NULL DEFAULT NOW(),
  created_by  UUID,
  updated_by  UUID,

  CONSTRAINT care_plans_pkey PRIMARY KEY (id),
  CONSTRAINT care_plans_senior_id_fkey FOREIGN KEY (senior_id)
    REFERENCES senior_profiles (id) ON DELETE RESTRICT,
  CONSTRAINT care_plans_provider_id_fkey FOREIGN KEY (provider_id)
    REFERENCES provider_organizations (id) ON DELETE SET NULL
);

-- -----------------------------------------------------------------------------
-- 18. visits
-- -----------------------------------------------------------------------------
CREATE TABLE visits (
  id                  UUID         NOT NULL DEFAULT gen_random_uuid(),
  care_plan_id        UUID         NOT NULL,
  caregiver_id        UUID         NOT NULL,
  status              visit_status NOT NULL DEFAULT 'SCHEDULED',
  scheduled_start     TIMESTAMPTZ  NOT NULL,
  scheduled_end       TIMESTAMPTZ  NOT NULL,
  actual_start        TIMESTAMPTZ,
  actual_end          TIMESTAMPTZ,
  check_in_latitude   DOUBLE PRECISION,
  check_in_longitude  DOUBLE PRECISION,
  check_out_latitude  DOUBLE PRECISION,
  check_out_longitude DOUBLE PRECISION,
  tasks               JSONB,
  notes               TEXT,
  created_at          TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
  updated_at          TIMESTAMPTZ  NOT NULL DEFAULT NOW(),

  CONSTRAINT visits_pkey PRIMARY KEY (id),
  CONSTRAINT visits_care_plan_id_fkey FOREIGN KEY (care_plan_id)
    REFERENCES care_plans (id) ON DELETE CASCADE,
  CONSTRAINT visits_caregiver_id_fkey FOREIGN KEY (caregiver_id)
    REFERENCES caregiver_applications (id) ON DELETE RESTRICT
);

-- -----------------------------------------------------------------------------
-- 19. daily_observations
-- -----------------------------------------------------------------------------
CREATE TABLE daily_observations (
  id           UUID                 NOT NULL DEFAULT gen_random_uuid(),
  care_plan_id UUID                 NOT NULL,
  observed_by  UUID                 NOT NULL,
  category     observation_category NOT NULL,
  date         TIMESTAMPTZ          NOT NULL,
  value        TEXT                 NOT NULL,
  notes        TEXT,
  created_at   TIMESTAMPTZ          NOT NULL DEFAULT NOW(),
  updated_at   TIMESTAMPTZ          NOT NULL DEFAULT NOW(),

  CONSTRAINT daily_observations_pkey PRIMARY KEY (id),
  CONSTRAINT daily_observations_care_plan_id_fkey FOREIGN KEY (care_plan_id)
    REFERENCES care_plans (id) ON DELETE CASCADE
);

-- -----------------------------------------------------------------------------
-- 20. incidents
-- -----------------------------------------------------------------------------
CREATE TABLE incidents (
  id          UUID              NOT NULL DEFAULT gen_random_uuid(),
  visit_id    UUID,
  reported_by UUID              NOT NULL,
  severity    incident_severity NOT NULL,
  title       TEXT              NOT NULL,
  description TEXT              NOT NULL,
  occurred_at TIMESTAMPTZ       NOT NULL,
  resolved_at TIMESTAMPTZ,
  resolution  TEXT,
  created_at  TIMESTAMPTZ       NOT NULL DEFAULT NOW(),
  updated_at  TIMESTAMPTZ       NOT NULL DEFAULT NOW(),

  CONSTRAINT incidents_pkey PRIMARY KEY (id),
  CONSTRAINT incidents_visit_id_fkey FOREIGN KEY (visit_id)
    REFERENCES visits (id) ON DELETE SET NULL
);

-- -----------------------------------------------------------------------------
-- 21. medical_history_entries
-- -----------------------------------------------------------------------------
CREATE TABLE medical_history_entries (
  id           UUID        NOT NULL DEFAULT gen_random_uuid(),
  person_id    UUID        NOT NULL,
  condition    TEXT        NOT NULL,
  diagnosed_at TIMESTAMPTZ,
  treated_by   TEXT,
  status       TEXT        NOT NULL DEFAULT 'active',
  notes        TEXT,
  created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  created_by   UUID,
  updated_by   UUID,

  CONSTRAINT medical_history_entries_pkey PRIMARY KEY (id),
  CONSTRAINT medical_history_entries_person_id_fkey FOREIGN KEY (person_id)
    REFERENCES person_profiles (id) ON DELETE CASCADE
);

-- -----------------------------------------------------------------------------
-- 22. medications
-- -----------------------------------------------------------------------------
CREATE TABLE medications (
  id             UUID                 NOT NULL DEFAULT gen_random_uuid(),
  person_id      UUID                 NOT NULL,
  name           TEXT                 NOT NULL,
  dosage         TEXT                 NOT NULL,
  form           TEXT                 NOT NULL,
  frequency      medication_frequency NOT NULL,
  prescribed_by  TEXT,
  prescribed_at  TIMESTAMPTZ,
  start_date     TIMESTAMPTZ,
  end_date       TIMESTAMPTZ,
  is_active      BOOLEAN              NOT NULL DEFAULT TRUE,
  side_effects   TEXT,
  notes          TEXT,
  created_at     TIMESTAMPTZ          NOT NULL DEFAULT NOW(),
  updated_at     TIMESTAMPTZ          NOT NULL DEFAULT NOW(),
  created_by     UUID,
  updated_by     UUID,

  CONSTRAINT medications_pkey PRIMARY KEY (id),
  CONSTRAINT medications_person_id_fkey FOREIGN KEY (person_id)
    REFERENCES person_profiles (id) ON DELETE CASCADE
);

-- -----------------------------------------------------------------------------
-- 23. medication_schedules
-- -----------------------------------------------------------------------------
CREATE TABLE medication_schedules (
  id            UUID        NOT NULL DEFAULT gen_random_uuid(),
  medication_id UUID        NOT NULL,
  time_of_day   TEXT        NOT NULL,
  day_of_week   day_of_week,
  is_active     BOOLEAN     NOT NULL DEFAULT TRUE,
  created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),

  CONSTRAINT medication_schedules_pkey PRIMARY KEY (id),
  CONSTRAINT medication_schedules_medication_id_fkey FOREIGN KEY (medication_id)
    REFERENCES medications (id) ON DELETE CASCADE
);

-- -----------------------------------------------------------------------------
-- 24. medication_events
-- -----------------------------------------------------------------------------
CREATE TABLE medication_events (
  id            UUID                   NOT NULL DEFAULT gen_random_uuid(),
  medication_id UUID                   NOT NULL,
  scheduled_for TIMESTAMPTZ            NOT NULL,
  status        medication_event_status NOT NULL DEFAULT 'SCHEDULED',
  taken_at      TIMESTAMPTZ,
  notes         TEXT,
  recorded_by   UUID,
  created_at    TIMESTAMPTZ            NOT NULL DEFAULT NOW(),
  updated_at    TIMESTAMPTZ            NOT NULL DEFAULT NOW(),

  CONSTRAINT medication_events_pkey PRIMARY KEY (id),
  CONSTRAINT medication_events_medication_id_fkey FOREIGN KEY (medication_id)
    REFERENCES medications (id) ON DELETE CASCADE
);

CREATE INDEX medication_events_medication_id_scheduled_for_idx
  ON medication_events (medication_id, scheduled_for);

-- -----------------------------------------------------------------------------
-- 25. appointments
-- -----------------------------------------------------------------------------
CREATE TABLE appointments (
  id               UUID               NOT NULL DEFAULT gen_random_uuid(),
  person_id        UUID               NOT NULL,
  institution_name TEXT               NOT NULL,
  institution_type provider_type,
  appointment_date TIMESTAMPTZ        NOT NULL,
  status           appointment_status NOT NULL DEFAULT 'SCHEDULED',
  purpose          TEXT,
  notes            TEXT,
  address          TEXT,
  created_at       TIMESTAMPTZ        NOT NULL DEFAULT NOW(),
  updated_at       TIMESTAMPTZ        NOT NULL DEFAULT NOW(),
  created_by       UUID,
  updated_by       UUID,

  CONSTRAINT appointments_pkey PRIMARY KEY (id),
  CONSTRAINT appointments_person_id_fkey FOREIGN KEY (person_id)
    REFERENCES person_profiles (id) ON DELETE CASCADE
);

-- -----------------------------------------------------------------------------
-- 26. institution_referrals
-- -----------------------------------------------------------------------------
CREATE TABLE institution_referrals (
  id               UUID                      NOT NULL DEFAULT gen_random_uuid(),
  from_provider_id UUID                      NOT NULL,
  to_provider_id   UUID                      NOT NULL,
  senior_person_id UUID                      NOT NULL,
  status           institution_referral_status NOT NULL DEFAULT 'CREATED',
  reason           TEXT,
  notes            TEXT,
  referred_at      TIMESTAMPTZ               NOT NULL DEFAULT NOW(),
  accepted_at      TIMESTAMPTZ,
  discharged_at    TIMESTAMPTZ,
  created_at       TIMESTAMPTZ               NOT NULL DEFAULT NOW(),
  updated_at       TIMESTAMPTZ               NOT NULL DEFAULT NOW(),

  CONSTRAINT institution_referrals_pkey PRIMARY KEY (id),
  CONSTRAINT institution_referrals_from_provider_id_fkey FOREIGN KEY (from_provider_id)
    REFERENCES provider_organizations (id) ON DELETE RESTRICT,
  CONSTRAINT institution_referrals_to_provider_id_fkey FOREIGN KEY (to_provider_id)
    REFERENCES provider_organizations (id) ON DELETE RESTRICT
);

-- -----------------------------------------------------------------------------
-- 27. eligibility_cases
-- -----------------------------------------------------------------------------
CREATE TABLE eligibility_cases (
  id                  UUID                   NOT NULL DEFAULT gen_random_uuid(),
  senior_id           UUID                   NOT NULL,
  status              eligibility_case_status NOT NULL DEFAULT 'NOT_STARTED',
  program_name        TEXT                   NOT NULL,
  application_date    TIMESTAMPTZ,
  determination_date  TIMESTAMPTZ,
  notes               TEXT,
  denial_reason       TEXT,
  created_at          TIMESTAMPTZ            NOT NULL DEFAULT NOW(),
  updated_at          TIMESTAMPTZ            NOT NULL DEFAULT NOW(),
  created_by          UUID,
  updated_by          UUID,

  CONSTRAINT eligibility_cases_pkey PRIMARY KEY (id),
  CONSTRAINT eligibility_cases_senior_id_fkey FOREIGN KEY (senior_id)
    REFERENCES senior_profiles (id) ON DELETE RESTRICT
);

-- -----------------------------------------------------------------------------
-- 28. approval_steps
-- -----------------------------------------------------------------------------
CREATE TABLE approval_steps (
  id           UUID        NOT NULL DEFAULT gen_random_uuid(),
  case_id      UUID        NOT NULL,
  step_name    TEXT        NOT NULL,
  step_order   INTEGER     NOT NULL,
  status       TEXT        NOT NULL DEFAULT 'pending',
  assigned_to  UUID,
  completed_at TIMESTAMPTZ,
  notes        TEXT,
  created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),

  CONSTRAINT approval_steps_pkey PRIMARY KEY (id),
  CONSTRAINT approval_steps_case_id_fkey FOREIGN KEY (case_id)
    REFERENCES eligibility_cases (id) ON DELETE CASCADE
);

-- -----------------------------------------------------------------------------
-- 29. claim_or_subsidy_records
-- -----------------------------------------------------------------------------
CREATE TABLE claim_or_subsidy_records (
  id           UUID         NOT NULL DEFAULT gen_random_uuid(),
  case_id      UUID         NOT NULL,
  claim_number TEXT         NOT NULL UNIQUE,
  status       claim_status NOT NULL DEFAULT 'DRAFT',
  amount       DECIMAL(12,2) NOT NULL,
  currency     TEXT         NOT NULL DEFAULT 'KRW',
  service_date TIMESTAMPTZ  NOT NULL,
  submitted_at TIMESTAMPTZ,
  processed_at TIMESTAMPTZ,
  notes        TEXT,
  created_at   TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
  updated_at   TIMESTAMPTZ  NOT NULL DEFAULT NOW(),

  CONSTRAINT claim_or_subsidy_records_pkey PRIMARY KEY (id),
  CONSTRAINT claim_or_subsidy_records_case_id_fkey FOREIGN KEY (case_id)
    REFERENCES eligibility_cases (id) ON DELETE RESTRICT
);

-- -----------------------------------------------------------------------------
-- 30. observability_signals
-- -----------------------------------------------------------------------------
CREATE TABLE observability_signals (
  id                UUID                   NOT NULL DEFAULT gen_random_uuid(),
  event_type        observability_event_type NOT NULL,
  severity          signal_severity        NOT NULL DEFAULT 'INFO',
  subject_person_id UUID,
  actor_user_id     UUID,
  entity_type       TEXT,
  entity_id         TEXT,
  message           TEXT                   NOT NULL,
  metadata          JSONB,
  acknowledged_at   TIMESTAMPTZ,
  acknowledged_by   UUID,
  created_at        TIMESTAMPTZ            NOT NULL DEFAULT NOW(),

  CONSTRAINT observability_signals_pkey PRIMARY KEY (id)
);

CREATE INDEX observability_signals_event_type_created_at_idx
  ON observability_signals (event_type, created_at);

CREATE INDEX observability_signals_subject_person_id_idx
  ON observability_signals (subject_person_id);

-- -----------------------------------------------------------------------------
-- 31. notifications
-- -----------------------------------------------------------------------------
CREATE TABLE notifications (
  id         UUID              NOT NULL DEFAULT gen_random_uuid(),
  user_id    UUID              NOT NULL,
  type       notification_type NOT NULL DEFAULT 'INFO',
  title      TEXT              NOT NULL,
  message    TEXT              NOT NULL,
  link       TEXT,
  is_read    BOOLEAN           NOT NULL DEFAULT FALSE,
  read_at    TIMESTAMPTZ,
  created_at TIMESTAMPTZ       NOT NULL DEFAULT NOW(),

  CONSTRAINT notifications_pkey PRIMARY KEY (id),
  CONSTRAINT notifications_user_id_fkey FOREIGN KEY (user_id)
    REFERENCES users (id) ON DELETE CASCADE
);

CREATE INDEX notifications_user_id_is_read_idx
  ON notifications (user_id, is_read);

-- -----------------------------------------------------------------------------
-- 32. audit_logs
-- -----------------------------------------------------------------------------
CREATE TABLE audit_logs (
  id          UUID         NOT NULL DEFAULT gen_random_uuid(),
  user_id     UUID,
  action      audit_action NOT NULL,
  entity_type TEXT,
  entity_id   TEXT,
  old_value   JSONB,
  new_value   JSONB,
  ip_address  TEXT,
  user_agent  TEXT,
  created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),

  CONSTRAINT audit_logs_pkey PRIMARY KEY (id),
  CONSTRAINT audit_logs_user_id_fkey FOREIGN KEY (user_id)
    REFERENCES users (id) ON DELETE SET NULL
);

CREATE INDEX audit_logs_entity_type_entity_id_idx
  ON audit_logs (entity_type, entity_id);

CREATE INDEX audit_logs_user_id_created_at_idx
  ON audit_logs (user_id, created_at);
