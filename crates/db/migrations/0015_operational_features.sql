-- =============================================================================
-- Migration 0015: Operational Features
-- Medication instructions, availability management, matching schedules
-- =============================================================================

-- 1. Medication instruction timing enum
DO $$ BEGIN
    CREATE TYPE instruction_timing AS ENUM (
        'BEFORE_MEAL', 'WITH_MEAL', 'AFTER_MEAL',
        'EMPTY_STOMACH', 'BEDTIME', 'ANYTIME'
    );
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

-- 2. Medication instruction columns
ALTER TABLE medications ADD COLUMN IF NOT EXISTS instruction_timing instruction_timing;
ALTER TABLE medications ADD COLUMN IF NOT EXISTS instruction_minutes INT CHECK (instruction_minutes >= 0);
ALTER TABLE medications ADD COLUMN IF NOT EXISTS instruction_text TEXT;
ALTER TABLE medications ADD COLUMN IF NOT EXISTS total_quantity INT CHECK (total_quantity > 0);
ALTER TABLE medications ADD COLUMN IF NOT EXISTS doses_per_intake INT NOT NULL DEFAULT 1;

-- 3. Medication schedule reminder columns
ALTER TABLE medication_schedules ADD COLUMN IF NOT EXISTS reminder_enabled BOOLEAN NOT NULL DEFAULT TRUE;
ALTER TABLE medication_schedules ADD COLUMN IF NOT EXISTS reminder_minutes_before INT NOT NULL DEFAULT 10;

-- 4. Availability slots: TEXT → TIME migration + add user_id
ALTER TABLE availability_slots
    ALTER COLUMN start_time TYPE TIME USING start_time::TIME,
    ALTER COLUMN end_time TYPE TIME USING end_time::TIME;

ALTER TABLE availability_slots ADD COLUMN IF NOT EXISTS user_id UUID REFERENCES users(id);

-- Backfill user_id from caregiver_applications
UPDATE availability_slots AS s
SET user_id = ca.user_id
FROM caregiver_applications AS ca
WHERE s.application_id = ca.id
  AND s.user_id IS NULL;

-- 5. Availability exceptions table
CREATE TABLE IF NOT EXISTS availability_exceptions (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID NOT NULL REFERENCES users(id),
    exception_date  DATE NOT NULL,
    is_available    BOOLEAN NOT NULL DEFAULT FALSE,
    start_time      TIME,
    end_time        TIME,
    reason          TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, exception_date)
);

-- 6. Visit status: add NEEDS_REASSIGNMENT
ALTER TYPE visit_status ADD VALUE IF NOT EXISTS 'NEEDS_REASSIGNMENT';

-- 7. Match request schedule table
CREATE TABLE IF NOT EXISTS match_request_schedule (
    id               UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    match_request_id UUID NOT NULL REFERENCES match_requests(id) ON DELETE CASCADE,
    day_of_week      day_of_week NOT NULL,
    start_time       TIME NOT NULL,
    end_time         TIME NOT NULL
);

-- 8. Match request status: add NO_CANDIDATES
ALTER TYPE match_request_status ADD VALUE IF NOT EXISTS 'NO_CANDIDATES';
