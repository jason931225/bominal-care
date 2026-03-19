-- =============================================================================
-- Migration 0009: Identity Verification + Data Retention
-- =============================================================================

-- Identity verifications (replaces 주민등록번호 storage)
CREATE TABLE IF NOT EXISTS identity_verifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    method TEXT NOT NULL CHECK (method IN ('pass_auth', 'i_pin', 'mobile')),
    verified_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ,
    verification_hash TEXT NOT NULL,
    is_valid BOOL NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_identity_verifications_user ON identity_verifications(user_id);

-- Data retention policies
CREATE TABLE IF NOT EXISTS data_retention_policies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    data_category TEXT NOT NULL UNIQUE,
    retention_days INT NOT NULL,
    legal_basis TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Seed retention policies per Korean regulations
INSERT INTO data_retention_policies (data_category, retention_days, legal_basis, description)
VALUES
    ('medical_records', 3650, '의료법 제22조', '진료기록: 10년 보존'),
    ('prescription_records', 730, '약사법 제30조', '처방전: 2년 보존'),
    ('consent_records', 1825, '개인정보보호법 제39조의6', '동의기록: 5년 보존'),
    ('audit_logs', 1825, '개인정보보호법 제34조', '접근기록: 5년 보존'),
    ('session_data', 1, '내부정책', '세션: 1일 보존'),
    ('identity_verification', 90, '개인정보보호법 제15조', '본인인증: 90일 보존')
ON CONFLICT (data_category) DO NOTHING;
