-- =============================================================================
-- Migration 0011: Safety — wellness, emergency, medication reminders, visit evidence
-- =============================================================================

-- Wellness check-ins (senior daily mood tracking)
CREATE TABLE IF NOT EXISTS wellness_checkins (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    person_id UUID NOT NULL REFERENCES person_profiles(id),
    mood wellness_mood NOT NULL,
    pain_level INT CHECK (pain_level BETWEEN 0 AND 10),
    notes TEXT,
    checked_in_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_wellness_checkins_person ON wellness_checkins(person_id);
CREATE INDEX IF NOT EXISTS idx_wellness_checkins_date ON wellness_checkins(created_at);

-- Wellness check config (notification schedule)
CREATE TABLE IF NOT EXISTS wellness_check_config (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    person_id UUID NOT NULL UNIQUE REFERENCES person_profiles(id),
    enabled BOOL NOT NULL DEFAULT TRUE,
    check_time TIME NOT NULL DEFAULT '09:00',
    notify_family_after_minutes INT NOT NULL DEFAULT 120,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Emergency events (SOS)
CREATE TABLE IF NOT EXISTS emergency_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    person_id UUID NOT NULL REFERENCES person_profiles(id),
    status emergency_event_status NOT NULL DEFAULT 'triggered',
    latitude DOUBLE PRECISION,
    longitude DOUBLE PRECISION,
    triggered_by UUID REFERENCES users(id),
    resolved_by UUID REFERENCES users(id),
    resolved_at TIMESTAMPTZ,
    resolution_notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_emergency_events_person ON emergency_events(person_id);

-- Medication reminders
CREATE TABLE IF NOT EXISTS medication_reminders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    medication_id UUID NOT NULL REFERENCES medications(id),
    scheduled_time TIME NOT NULL,
    is_active BOOL NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_medication_reminders_med ON medication_reminders(medication_id);

-- Visit evidence (photo/document proof for tasks)
CREATE TABLE IF NOT EXISTS visit_evidence (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    visit_id UUID NOT NULL REFERENCES visits(id),
    task_index INT NOT NULL,
    evidence_type TEXT NOT NULL CHECK (evidence_type IN ('photo', 'document', 'signature', 'note')),
    content_url TEXT,
    notes TEXT,
    uploaded_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_visit_evidence_visit ON visit_evidence(visit_id);

-- Daily care summaries (auto-aggregated)
CREATE TABLE IF NOT EXISTS daily_care_summaries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    person_id UUID NOT NULL REFERENCES person_profiles(id),
    summary_date DATE NOT NULL,
    medications_taken INT NOT NULL DEFAULT 0,
    medications_missed INT NOT NULL DEFAULT 0,
    visits_completed INT NOT NULL DEFAULT 0,
    wellness_mood wellness_mood,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(person_id, summary_date)
);
CREATE INDEX IF NOT EXISTS idx_daily_care_summaries_person ON daily_care_summaries(person_id);
