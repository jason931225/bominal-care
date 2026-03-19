-- =============================================================================
-- Migration 0006: Policy Engine — dynamic access policies
-- =============================================================================

CREATE TABLE IF NOT EXISTS access_policies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    role TEXT NOT NULL,
    resource_type TEXT NOT NULL
        CHECK (resource_type IN (
            'PROFILE', 'SENIOR_PROFILE', 'CARE_PLAN', 'VISIT', 'MEDICATION',
            'MEDICAL_HISTORY', 'MATCH_REQUEST', 'PROVIDER', 'CAREGIVER_APPLICATION',
            'ELIGIBILITY_CASE', 'OBSERVABILITY', 'NOTIFICATION', 'CONSENT',
            'AUDIT_LOG', 'REFERRAL', 'APPOINTMENT', 'PRESCRIPTION',
            'CLINICAL_ENCOUNTER', 'LAB_RESULT', 'DOCUMENT_TRANSFER',
            'DISPENSING', 'WELLNESS_CHECKIN', 'EMERGENCY_EVENT', 'HANDOFF', 'ACCESS_POLICY'
        )),
    action TEXT NOT NULL
        CHECK (action IN ('CREATE', 'READ', 'UPDATE', 'DELETE', 'LIST', 'APPROVE', 'REJECT', 'SIGN', 'TRANSFER')),
    scope TEXT NOT NULL
        CHECK (scope IN ('OWN', 'ASSIGNED', 'ORG', 'ALL', 'LINKED')),
    effect TEXT NOT NULL DEFAULT 'allow'
        CHECK (effect IN ('allow', 'deny', 'requires_approval', 'allow_anonymized')),
    conditions JSONB,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (role, resource_type, action, scope)
);

CREATE INDEX idx_ap_role ON access_policies(role);
CREATE INDEX idx_ap_resource ON access_policies(resource_type);

-- Policy change log for audit trail
CREATE TABLE IF NOT EXISTS policy_change_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    policy_id UUID NOT NULL REFERENCES access_policies(id),
    changed_by UUID NOT NULL REFERENCES users(id),
    old_effect TEXT,
    new_effect TEXT NOT NULL,
    reason TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_pcl_policy ON policy_change_log(policy_id);
