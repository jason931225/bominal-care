-- =============================================================================
-- Migration 0008: Consent Model — PIPA-compliant granular consent
-- =============================================================================

-- Add new columns to consent_records
ALTER TABLE consent_records ADD COLUMN IF NOT EXISTS purpose_v2 consent_purpose_v2;
ALTER TABLE consent_records ADD COLUMN IF NOT EXISTS policy_version TEXT;
ALTER TABLE consent_records ADD COLUMN IF NOT EXISTS consent_text_hash TEXT;
ALTER TABLE consent_records ADD COLUMN IF NOT EXISTS collection_items TEXT[];
ALTER TABLE consent_records ADD COLUMN IF NOT EXISTS retention_period_days INT;

-- Migrate existing data: map old purposes to new granular ones
-- NO_SHARE → leave null (requires re-consent)
-- MEDICAL_SHARE → personal_info_collection + third_party_medical
-- GOVERNMENT_SHARE → third_party_government

-- Only run data migration if there are existing records
UPDATE consent_records SET purpose_v2 = 'personal_info_collection'::consent_purpose_v2
WHERE purpose::text = 'MEDICAL_SHARE' AND purpose_v2 IS NULL;

UPDATE consent_records SET purpose_v2 = 'third_party_government'::consent_purpose_v2
WHERE purpose::text = 'GOVERNMENT_SHARE' AND purpose_v2 IS NULL;
