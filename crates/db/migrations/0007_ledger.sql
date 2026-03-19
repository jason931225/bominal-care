-- =============================================================================
-- Migration 0007: Immutable Ledger — append-only history for medications, appointments, care plans
-- =============================================================================

-- Medication ledger
CREATE TABLE IF NOT EXISTS medication_ledger (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    medication_id UUID NOT NULL REFERENCES medications(id),
    version INT NOT NULL,
    action TEXT NOT NULL CHECK (action IN ('created', 'modified', 'cancelled', 'approved', 'rejected')),
    actor_user_id UUID NOT NULL REFERENCES users(id),
    actor_type TEXT NOT NULL CHECK (actor_type IN ('self', 'family', 'caregiver', 'medical_proxy', 'pharmacist_proxy', 'provider', 'system')),
    data JSONB NOT NULL,
    reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(medication_id, version)
);
CREATE INDEX IF NOT EXISTS idx_medication_ledger_med_id ON medication_ledger(medication_id);

-- Appointment ledger
CREATE TABLE IF NOT EXISTS appointment_ledger (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    appointment_id UUID NOT NULL REFERENCES appointments(id),
    version INT NOT NULL,
    action TEXT NOT NULL CHECK (action IN ('created', 'modified', 'cancelled', 'approved', 'rejected')),
    actor_user_id UUID NOT NULL REFERENCES users(id),
    actor_type TEXT NOT NULL CHECK (actor_type IN ('self', 'family', 'caregiver', 'medical_proxy', 'pharmacist_proxy', 'provider', 'system')),
    data JSONB NOT NULL,
    reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(appointment_id, version)
);
CREATE INDEX IF NOT EXISTS idx_appointment_ledger_appt_id ON appointment_ledger(appointment_id);

-- Care plan ledger
CREATE TABLE IF NOT EXISTS care_plan_ledger (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    care_plan_id UUID NOT NULL REFERENCES care_plans(id),
    version INT NOT NULL,
    action TEXT NOT NULL CHECK (action IN ('created', 'modified', 'cancelled', 'approved', 'rejected')),
    actor_user_id UUID NOT NULL REFERENCES users(id),
    actor_type TEXT NOT NULL CHECK (actor_type IN ('self', 'family', 'caregiver', 'medical_proxy', 'pharmacist_proxy', 'provider', 'system')),
    data JSONB NOT NULL,
    reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(care_plan_id, version)
);
CREATE INDEX IF NOT EXISTS idx_care_plan_ledger_cp_id ON care_plan_ledger(care_plan_id);

-- Current-state views (latest version per entity)
CREATE OR REPLACE VIEW medication_current AS
SELECT DISTINCT ON (medication_id) *
FROM medication_ledger
ORDER BY medication_id, version DESC;

CREATE OR REPLACE VIEW appointment_current AS
SELECT DISTINCT ON (appointment_id) *
FROM appointment_ledger
ORDER BY appointment_id, version DESC;

CREATE OR REPLACE VIEW care_plan_current AS
SELECT DISTINCT ON (care_plan_id) *
FROM care_plan_ledger
ORDER BY care_plan_id, version DESC;
