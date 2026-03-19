-- =============================================================================
-- Migration 0012: HIS-lite — Medical Professional Portal tables
-- =============================================================================

-- Medical professional profiles (linked to users with medical roles)
CREATE TABLE IF NOT EXISTS medical_professional_profiles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL UNIQUE REFERENCES users(id),
    license_type handoff_license_type NOT NULL,
    license_number TEXT NOT NULL,
    institution_id UUID REFERENCES provider_organizations(id),
    specialty TEXT,
    is_verified BOOL NOT NULL DEFAULT FALSE,
    verified_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_medical_prof_user ON medical_professional_profiles(user_id);
CREATE INDEX IF NOT EXISTS idx_medical_prof_license ON medical_professional_profiles(license_number);

-- Medical handoff sessions (scoped temporary access)
CREATE TABLE IF NOT EXISTS medical_handoff_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    senior_person_id UUID NOT NULL REFERENCES person_profiles(id),
    professional_user_id UUID NOT NULL REFERENCES users(id),
    license_type handoff_license_type NOT NULL,
    license_number TEXT NOT NULL,
    institution_name TEXT,
    institution_id UUID REFERENCES provider_organizations(id),
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ended_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ NOT NULL DEFAULT (NOW() + INTERVAL '30 minutes'),
    is_active BOOL NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_handoff_senior ON medical_handoff_sessions(senior_person_id);
CREATE INDEX IF NOT EXISTS idx_handoff_professional ON medical_handoff_sessions(professional_user_id);

-- Prescriptions (linked to medications via ledger)
CREATE TABLE IF NOT EXISTS prescriptions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    senior_person_id UUID NOT NULL REFERENCES person_profiles(id),
    prescribed_by UUID NOT NULL REFERENCES users(id),
    institution_id UUID REFERENCES provider_organizations(id),
    medication_name TEXT NOT NULL,
    dosage TEXT NOT NULL,
    frequency TEXT NOT NULL,
    duration_days INT,
    instructions TEXT,
    is_signed BOOL NOT NULL DEFAULT FALSE,
    signed_at TIMESTAMPTZ,
    signed_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_prescriptions_senior ON prescriptions(senior_person_id);
CREATE INDEX IF NOT EXISTS idx_prescriptions_prescriber ON prescriptions(prescribed_by);

-- Immutability trigger for signed prescriptions
CREATE OR REPLACE FUNCTION prevent_signed_prescription_update() RETURNS TRIGGER AS $$
BEGIN
    IF OLD.is_signed = TRUE THEN
        RAISE EXCEPTION 'Cannot modify a signed prescription';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_prevent_signed_rx_update ON prescriptions;
CREATE TRIGGER trg_prevent_signed_rx_update
    BEFORE UPDATE ON prescriptions
    FOR EACH ROW
    EXECUTE FUNCTION prevent_signed_prescription_update();

-- Clinical encounters (SOAP notes)
CREATE TABLE IF NOT EXISTS clinical_encounters (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    senior_person_id UUID NOT NULL REFERENCES person_profiles(id),
    provider_user_id UUID NOT NULL REFERENCES users(id),
    institution_id UUID REFERENCES provider_organizations(id),
    encounter_date TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    subjective TEXT,
    objective TEXT,
    assessment TEXT,
    plan TEXT,
    is_signed BOOL NOT NULL DEFAULT FALSE,
    signed_at TIMESTAMPTZ,
    addendum TEXT,
    addendum_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_clinical_encounters_senior ON clinical_encounters(senior_person_id);

-- Immutability trigger for signed encounters
CREATE OR REPLACE FUNCTION prevent_signed_encounter_update() RETURNS TRIGGER AS $$
BEGIN
    IF OLD.is_signed = TRUE AND NEW.addendum IS NULL THEN
        RAISE EXCEPTION 'Cannot modify a signed encounter except to add an addendum';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_prevent_signed_enc_update ON clinical_encounters;
CREATE TRIGGER trg_prevent_signed_enc_update
    BEFORE UPDATE ON clinical_encounters
    FOR EACH ROW
    EXECUTE FUNCTION prevent_signed_encounter_update();

-- Lab results
CREATE TABLE IF NOT EXISTS lab_results (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    senior_person_id UUID NOT NULL REFERENCES person_profiles(id),
    ordered_by UUID REFERENCES users(id),
    test_name TEXT NOT NULL,
    test_code TEXT,
    result_value TEXT,
    result_unit TEXT,
    reference_range TEXT,
    is_critical BOOL NOT NULL DEFAULT FALSE,
    reviewed_by UUID REFERENCES users(id),
    reviewed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_lab_results_senior ON lab_results(senior_person_id);

-- Patient allergies
CREATE TABLE IF NOT EXISTS patient_allergies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    senior_person_id UUID NOT NULL REFERENCES person_profiles(id),
    allergen TEXT NOT NULL,
    reaction TEXT,
    severity TEXT CHECK (severity IN ('mild', 'moderate', 'severe', 'life_threatening')),
    is_active BOOL NOT NULL DEFAULT TRUE,
    reported_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_allergies_senior ON patient_allergies(senior_person_id);

-- Document transfer requests
CREATE TABLE IF NOT EXISTS document_transfer_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    from_institution_id UUID NOT NULL REFERENCES provider_organizations(id),
    to_institution_id UUID NOT NULL REFERENCES provider_organizations(id),
    senior_person_id UUID NOT NULL REFERENCES person_profiles(id),
    document_type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'requested' CHECK (status IN ('requested', 'approved', 'transferred', 'rejected')),
    requested_by UUID NOT NULL REFERENCES users(id),
    approved_by UUID REFERENCES users(id),
    approved_at TIMESTAMPTZ,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_doc_transfer_senior ON document_transfer_requests(senior_person_id);

-- Generic substitution records (pharmacy)
CREATE TABLE IF NOT EXISTS generic_substitution_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    prescription_id UUID NOT NULL REFERENCES prescriptions(id),
    original_medication TEXT NOT NULL,
    substituted_medication TEXT NOT NULL,
    reason TEXT,
    pharmacist_user_id UUID NOT NULL REFERENCES users(id),
    prescriber_notified BOOL NOT NULL DEFAULT FALSE,
    prescriber_notified_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_generic_sub_rx ON generic_substitution_records(prescription_id);
