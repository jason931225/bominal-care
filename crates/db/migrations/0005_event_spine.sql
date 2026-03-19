-- =============================================================================
-- Migration 0005: Event Spine — platform events, alerts, metrics, integrity
-- =============================================================================

-- platform_events: immutable audit trail for all significant actions
CREATE TABLE IF NOT EXISTS platform_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    actor_user_id UUID REFERENCES users(id),
    actor_role TEXT,
    proxy_user_id UUID REFERENCES users(id),
    entity_type TEXT NOT NULL,
    entity_id UUID NOT NULL,
    action TEXT NOT NULL,
    sensitivity TEXT NOT NULL DEFAULT 'internal'
        CHECK (sensitivity IN ('public', 'internal', 'confidential', 'restricted')),
    category TEXT NOT NULL DEFAULT 'administrative'
        CHECK (category IN ('clinical', 'care_operations', 'senior_safety', 'access_identity', 'consent_policy', 'financial', 'administrative', 'system')),
    before_state JSONB,
    after_state JSONB,
    metadata JSONB,
    ip_address TEXT,
    user_agent TEXT,
    hash TEXT,
    previous_hash TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_pe_entity ON platform_events(entity_type, entity_id);
CREATE INDEX idx_pe_actor ON platform_events(actor_user_id);
CREATE INDEX idx_pe_category ON platform_events(category);
CREATE INDEX idx_pe_sensitivity ON platform_events(sensitivity);
CREATE INDEX idx_pe_created_at ON platform_events(created_at);
CREATE INDEX idx_pe_hash ON platform_events(hash);

-- Append-only trigger: prevent UPDATE and DELETE on platform_events
CREATE OR REPLACE FUNCTION prevent_event_mutation() RETURNS TRIGGER AS $$
BEGIN
    RAISE EXCEPTION 'platform_events is append-only: % not allowed', TG_OP;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_platform_events_immutable
    BEFORE UPDATE OR DELETE ON platform_events
    FOR EACH ROW EXECUTE FUNCTION prevent_event_mutation();

-- alert_rules: configurable rules that fire on events
CREATE TABLE IF NOT EXISTS alert_rules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    category TEXT NOT NULL,
    condition JSONB NOT NULL,
    actions JSONB NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    cooldown_minutes INTEGER NOT NULL DEFAULT 60,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- alert_firings: log of when rules fired
CREATE TABLE IF NOT EXISTS alert_firings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    rule_id UUID NOT NULL REFERENCES alert_rules(id),
    event_id UUID NOT NULL REFERENCES platform_events(id),
    actions_taken JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_af_rule_id ON alert_firings(rule_id);
CREATE INDEX idx_af_created_at ON alert_firings(created_at);

-- system_metrics: platform health snapshots
CREATE TABLE IF NOT EXISTS system_metrics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    metric_name TEXT NOT NULL,
    metric_value DOUBLE PRECISION NOT NULL,
    labels JSONB,
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_sm_name ON system_metrics(metric_name);
CREATE INDEX idx_sm_recorded_at ON system_metrics(recorded_at);

-- integrity_verifications: hash chain verification results
CREATE TABLE IF NOT EXISTS integrity_verifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    entity_type TEXT NOT NULL,
    entity_id UUID NOT NULL,
    chain_length INTEGER NOT NULL,
    is_valid BOOLEAN NOT NULL,
    broken_at_event_id UUID REFERENCES platform_events(id),
    verified_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- compliance_reports: generated regulatory reports
CREATE TABLE IF NOT EXISTS compliance_reports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    report_type TEXT NOT NULL,
    period_start TIMESTAMPTZ NOT NULL,
    period_end TIMESTAMPTZ NOT NULL,
    generated_by UUID REFERENCES users(id),
    data JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
