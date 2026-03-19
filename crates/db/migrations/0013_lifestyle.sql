-- =============================================================================
-- Migration 0013: Korean Lifestyle — benefits, holidays, alerts, community
-- =============================================================================

-- Benefit utilization tracking
CREATE TABLE IF NOT EXISTS benefit_utilization (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    senior_person_id UUID NOT NULL REFERENCES person_profiles(id),
    benefit_type TEXT NOT NULL,
    period_start DATE NOT NULL,
    period_end DATE NOT NULL,
    monthly_cap_krw BIGINT NOT NULL DEFAULT 0,
    used_krw BIGINT NOT NULL DEFAULT 0,
    remaining_krw BIGINT GENERATED ALWAYS AS (monthly_cap_krw - used_krw) STORED,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(senior_person_id, benefit_type, period_start)
);
CREATE INDEX IF NOT EXISTS idx_benefit_util_senior ON benefit_utilization(senior_person_id);

-- Korean holidays (for scheduling awareness)
CREATE TABLE IF NOT EXISTS korean_holidays (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    holiday_date DATE NOT NULL UNIQUE,
    name_ko TEXT NOT NULL,
    name_en TEXT,
    is_lunar BOOL NOT NULL DEFAULT FALSE
);

-- Seed 2026 Korean holidays
INSERT INTO korean_holidays (holiday_date, name_ko, name_en, is_lunar) VALUES
    ('2026-01-01', '신정', 'New Year', FALSE),
    ('2026-02-16', '설날 전날', 'Lunar New Year Eve', TRUE),
    ('2026-02-17', '설날', 'Lunar New Year', TRUE),
    ('2026-02-18', '설날 다음날', 'Lunar New Year +1', TRUE),
    ('2026-03-01', '삼일절', 'Independence Movement Day', FALSE),
    ('2026-05-05', '어린이날', 'Children''s Day', FALSE),
    ('2026-05-24', '부처님오신날', 'Buddha''s Birthday', TRUE),
    ('2026-06-06', '현충일', 'Memorial Day', FALSE),
    ('2026-08-15', '광복절', 'Liberation Day', FALSE),
    ('2026-09-24', '추석 전날', 'Chuseok Eve', TRUE),
    ('2026-09-25', '추석', 'Chuseok', TRUE),
    ('2026-09-26', '추석 다음날', 'Chuseok +1', TRUE),
    ('2026-10-03', '개천절', 'National Foundation Day', FALSE),
    ('2026-10-09', '한글날', 'Hangul Day', FALSE),
    ('2026-12-25', '크리스마스', 'Christmas', FALSE)
ON CONFLICT (holiday_date) DO NOTHING;

-- Community activities
CREATE TABLE IF NOT EXISTS community_activities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT NOT NULL,
    description TEXT,
    location TEXT,
    latitude DOUBLE PRECISION,
    longitude DOUBLE PRECISION,
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ,
    category TEXT NOT NULL,
    is_active BOOL NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_community_activities_start ON community_activities(start_time);

-- Seasonal alerts
CREATE TABLE IF NOT EXISTS seasonal_alerts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT NOT NULL,
    message TEXT NOT NULL,
    severity TEXT NOT NULL DEFAULT 'info' CHECK (severity IN ('info', 'warning', 'critical')),
    active_from DATE NOT NULL,
    active_until DATE NOT NULL,
    region TEXT,
    is_active BOOL NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
