# Operational Features: Medication Instructions, Availability, Matching & Scheduling

**Date:** 2026-03-23
**Status:** Active

---

## Overview

Four interconnected operational features that make the platform usable for real Korean senior care:

1. **Structured medication instructions** — timing relative to meals, free-text for edge cases
2. **Caregiver availability management** — weekly template + date exceptions
3. **Matching algorithm fix** — real time-based overlap, not just day-of-week
4. **Visit scheduling engine** — recurring visit generation with conflict detection and reassignment

---

## 1. Medication Instructions

### Problem

Medications table has `dosage` (text) and `frequency` (enum) but no structured field for Korean dosage instructions like "식후 30분", "공복", or "취침 전". This prevents automated reminder timing and proper display.

### Schema Change

```sql
CREATE TYPE instruction_timing AS ENUM (
  'BEFORE_MEAL',    -- 식전
  'WITH_MEAL',      -- 식사 중
  'AFTER_MEAL',     -- 식후
  'EMPTY_STOMACH',  -- 공복
  'BEDTIME',        -- 취침 전
  'ANYTIME'         -- 시간 무관
);

ALTER TABLE medications ADD COLUMN instruction_timing instruction_timing;
ALTER TABLE medications ADD COLUMN instruction_minutes INT CHECK (instruction_minutes >= 0);
ALTER TABLE medications ADD COLUMN instruction_text TEXT;
```

- `instruction_timing` — structured enum driving display and future reminder logic
- `instruction_minutes` — offset in minutes (e.g., 30 = "식후 30분"). NULL means no specific offset. Only meaningful for BEFORE_MEAL, AFTER_MEAL, BEDTIME. CHECK >= 0.
- `instruction_text` — free-text for anything the enum doesn't cover (e.g., "자몽주스와 함께 복용 금지")

### Korean Display Mapping

| Enum | Korean | With minutes |
|------|--------|-------------|
| BEFORE_MEAL | 식전 | 식전 30분 |
| WITH_MEAL | 식사 중 | (minutes ignored) |
| AFTER_MEAL | 식후 | 식후 30분 |
| EMPTY_STOMACH | 공복 | (minutes ignored) |
| BEDTIME | 취침 전 | 취침 30분 전 |
| ANYTIME | 시간 무관 | (minutes ignored) |

### API Changes

- `POST /api/medications` — accept `instruction_timing`, `instruction_minutes`, `instruction_text`
- `PATCH /api/medications/{id}` — accept same fields for updates
- `GET /api/medications` — return the new fields in response

### Types Changes

- Add `InstructionTiming` enum to `crates/types/src/enums.rs`
- Add fields to `Medication` model in `crates/types/src/lib.rs`
- Add fields to medication input structs in `crates/types/src/inputs.rs`

### Frontend Changes

- Senior medication cards — show timing label below dosage (e.g., "500mg 정제 · 식후 30분")
- Medication detail page — show instruction fields
- Medical portal prescription form — timing dropdown + minutes input + free-text

---

## 2. Caregiver Availability Management

### Problem

`availability_slots` table exists but has no API endpoints. Caregivers can't manage their schedule. No way to block specific dates for emergencies.

### Schema Fix: Migrate `start_time`/`end_time` from TEXT to TIME

The existing `availability_slots` table uses `TEXT` for `start_time` and `end_time`. This must be migrated to `TIME` for correct time comparisons in matching and scheduling.

```sql
-- Migrate availability_slots times from TEXT to TIME
ALTER TABLE availability_slots
  ALTER COLUMN start_time TYPE TIME USING start_time::TIME,
  ALTER COLUMN end_time   TYPE TIME USING end_time::TIME;

-- Replace application_id with user_id for per-user availability
ALTER TABLE availability_slots ADD COLUMN user_id UUID REFERENCES users(id);

-- Backfill user_id from caregiver_applications
UPDATE availability_slots AS s
SET user_id = ca.user_id
FROM caregiver_applications AS ca
WHERE s.application_id = ca.id;
```

**Rust model update:** Change `start_time: String` and `end_time: String` in `AvailabilitySlot` to `start_time: chrono::NaiveTime` and `end_time: chrono::NaiveTime`.

### Schema: Exceptions (new)

```sql
CREATE TABLE availability_exceptions (
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
```

### API Endpoints

```
GET    /api/availability              — list my weekly slots (by user_id)
PUT    /api/availability              — replace weekly slots (bulk set)
GET    /api/availability/exceptions   — list my exceptions
POST   /api/availability/exceptions   — add exception (block or extra day)
DELETE /api/availability/exceptions/{id} — remove exception (ownership check: user_id must match)
```

All endpoints require `require_permission()`. DELETE verifies `exception.user_id == authenticated_user.id` before deletion.

### Conflict Cascade on Blocking

When a caregiver creates a blocking exception on a date with existing visits:

1. Query affected visits: `SELECT ... FROM visits WHERE caregiver_id = $1 AND scheduled_start::DATE = $2 AND status IN ('SCHEDULED', 'CAREGIVER_ACKNOWLEDGED') FOR UPDATE` (row-level lock prevents concurrent modification)
2. Transition affected visits to `NEEDS_REASSIGNMENT`
3. System queries other approved caregivers for the senior with overlapping availability
4. Top match receives an offer notification
5. If accepted → visit reassigned (back to `SCHEDULED` with new caregiver_id) → family receives approval notification
6. Family approves or rejects substitute
7. If no one accepts within 12h → family notified, visit status remains `NEEDS_REASSIGNMENT`

**Escalation (≤24h before visit):**
- Same flow as above
- Additionally: emit `platform_events` entry with `entity_type = "visit"`, `action = "reassignment_urgent"`
- This event is a hook point for future handlers — **not implemented now**, just the event emission

**Transaction boundary:** Steps 1-2 (find affected visits + transition status) must happen in a single serializable transaction with `SELECT ... FOR UPDATE` to prevent race conditions.

### Visit Status Addition

Add `NEEDS_REASSIGNMENT` to visit status enum and state machine:

```
SCHEDULED → NEEDS_REASSIGNMENT
CAREGIVER_ACKNOWLEDGED → NEEDS_REASSIGNMENT
NEEDS_REASSIGNMENT → SCHEDULED (when reassigned to new caregiver)
NEEDS_REASSIGNMENT → CANCELLED (if no substitute found)
```

State machine HashMap entry:
```rust
NeedsReassignment => vec![Scheduled, Cancelled],
```

The `add_escapes` helper will automatically add `Cancelled` (which is correct and idempotent since we already list it).

**Update `get_upcoming_visits()`:** Change hardcoded status filter from `IN ('SCHEDULED', 'CAREGIVER_ACKNOWLEDGED')` to `IN ('SCHEDULED', 'CAREGIVER_ACKNOWLEDGED', 'NEEDS_REASSIGNMENT')` so these visits remain visible to seniors and families.

### Frontend: Caregiver Portal

- **Availability settings page** (`/caregiver/profile/availability`)
  - Weekly grid with day toggles + start/end time selectors
  - Save button calls `PUT /api/availability`
  - "일정 변경" section showing exceptions list
  - "날짜 차단" button → date picker + reason input → `POST /api/availability/exceptions`
  - Each exception has a delete button

---

## 3. Matching Algorithm Fix

### Problem

Current scoring checks `availability_slots.day_of_week` but ignores `start_time`/`end_time`. A caregiver available Mon 9-5 matches a request for Mon 10pm.

### Schema: Add Structured Schedule to Match Requests

The existing `match_requests` table and `MatchRequestInput` only have `schedule_notes: Option<String>` (free text). The matching fix needs structured schedule data.

**New join table:**
```sql
CREATE TABLE match_request_schedule (
  id               UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  match_request_id UUID NOT NULL REFERENCES match_requests(id) ON DELETE CASCADE,
  day_of_week      day_of_week NOT NULL,
  start_time       TIME NOT NULL,
  end_time         TIME NOT NULL
);
```

**Input struct addition:**
```rust
pub struct ScheduleSlot {
    pub day_of_week: DayOfWeek,
    pub start_time: String,  // "14:00" — validated and parsed to NaiveTime before DB insert
    pub end_time: String,    // "16:00"
}

// Add to MatchRequestInput:
pub requested_schedule: Option<Vec<ScheduleSlot>>,
```

### Fix: Time-Based Overlap Scoring

```
For each slot in match_request_schedule:
  Check caregiver availability_slots WHERE:
    slot.day_of_week = requested.day_of_week
    AND slot.start_time <= requested.start_time
    AND slot.end_time >= requested.end_time
    AND slot.user_id = caregiver.user_id
  Also check availability_exceptions: exclude if blocked on any specific requested date

Scoring:
  - ALL requested slots covered → 20 pts
  - 80%+ covered → 10 pts
  - <80% covered → 0 pts (disqualify)

This replaces the existing proportional scoring for schedule overlap.
```

### Matching Trigger

`POST /api/match-requests` now:
1. Insert match request with status `CREATED`
2. Insert `match_request_schedule` rows from `requested_schedule`
3. Call `search_candidates()` **within a single transaction** that:
   - Transitions status to `SEARCHING`
   - Runs scoring with time-based overlap
   - Inserts recommendations
   - Transitions to `RECOMMENDATIONS_READY`
   - If scoring fails or finds zero candidates → transitions to `NO_CANDIDATES` (new status)
4. Return the match request with final status

### Match Request Status Addition

Add `NO_CANDIDATES` to match request status enum:
```
SEARCHING → NO_CANDIDATES (when zero eligible caregivers found)
```
Frontend displays: "조건에 맞는 요양보호사를 찾을 수 없습니다. 검색 조건을 조정해주세요."

---

## 4. Visit Scheduling Engine

### Problem

Visits are created manually one at a time. No recurring weekly pattern. No conflict checking.

### New Function: `generate_recurring_visits()`

```rust
pub struct RecurringPattern {
    pub days: Vec<DayOfWeek>,
    pub start_time: NaiveTime,         // parsed from input, not String
    pub end_time: NaiveTime,
    pub service_type: String,
    pub weeks: u32,                    // must be 1..=52
    pub start_date: NaiveDate,
}

pub struct ScheduleResult {
    pub created: Vec<Visit>,
    pub skipped: Vec<SkippedDate>,
}

pub struct SkippedDate {
    pub date: NaiveDate,
    pub reason: String,
}
```

**Validation (400 errors):**
- `end_time <= start_time` → "종료 시간은 시작 시간 이후여야 합니다"
- `weeks == 0 || weeks > 52` → "주 수는 1~52 사이여야 합니다"
- `days` is empty → "근무 요일을 선택하세요"

**Logic:**
1. For each week (1..=weeks), for each day in pattern.days:
   - Compute the calendar date
   - Check caregiver `availability_slots`: slot covers this day + time range?
   - Check `availability_exceptions`: date blocked?
   - Check existing `visits`: overlapping visit for this caregiver?
   - If all clear → insert visit with status `SCHEDULED`
   - If conflict → add to `skipped` with Korean reason string
2. Run entire operation in a transaction
3. Return `ScheduleResult`

### API Endpoint

```
POST /api/visits/schedule
{
  "care_plan_id": "...",
  "caregiver_id": "...",
  "days": ["MONDAY", "WEDNESDAY", "FRIDAY"],
  "start_time": "14:00",
  "end_time": "16:00",
  "service_type": "방문요양",
  "weeks": 4,
  "start_date": "2026-03-24"
}

Response (200):
{
  "success": true,
  "data": {
    "created": [ ...visit objects... ],
    "skipped": [
      { "date": "2026-04-02", "reason": "요양보호사 일정 차단" }
    ]
  }
}

Response (400):
{
  "success": false,
  "error": "종료 시간은 시작 시간 이후여야 합니다"
}
```

---

## Migration Summary

**New migration file: `0015_operational_features.sql`**

1. `instruction_timing` enum type
2. Three new columns on `medications` (with CHECK constraint on minutes)
3. Migrate `availability_slots.start_time`/`end_time` from TEXT to TIME
4. Add `user_id` column to `availability_slots` + backfill from `caregiver_applications`
5. `availability_exceptions` table
6. `NEEDS_REASSIGNMENT` value added to `visit_status` enum
7. `match_request_schedule` table
8. `NO_CANDIDATES` value added to `match_request_status` enum

---

## Files to Modify

### Backend — Types
| File | Changes |
|------|---------|
| `crates/types/src/enums.rs` | Add `InstructionTiming`, `NeedsReassignment` to VisitStatus, `NoCandidates` to MatchRequestStatus |
| `crates/types/src/lib.rs` | Add instruction fields to `Medication` model, update `AvailabilitySlot` times to `NaiveTime` |
| `crates/types/src/state_machines.rs` | Add `NeedsReassignment` transitions to visit machine, `NoCandidates` to match request machine |
| `crates/types/src/inputs.rs` | Add instruction fields to medication inputs, add `ScheduleSlot` + `requested_schedule` to `MatchRequestInput` |

### Backend — DB Queries
| File | Changes |
|------|---------|
| `crates/db/migrations/0015_operational_features.sql` | New migration |
| `crates/db/src/queries/medication.rs` | Update create/update to include instruction fields |
| `crates/db/src/queries/availability_slot.rs` | Add user_id-based CRUD, exceptions CRUD |
| `crates/db/src/queries/visit.rs` | Add `generate_recurring_visits()`, update `get_upcoming_visits()` status filter |
| `crates/db/src/queries/match_request.rs` | Fix `search_candidates()` time overlap, wrap in transaction, handle NO_CANDIDATES, insert schedule rows |

### Backend — Routes
| File | Changes |
|------|---------|
| `crates/server/src/routes/mod.rs` | Register availability routes |
| `crates/server/src/routes/availability.rs` | New file: GET/PUT slots, GET/POST/DELETE exceptions |
| `crates/server/src/routes/visits.rs` | Add `POST /visits/schedule` |
| `crates/server/src/routes/match_requests.rs` | Wire auto-matching on POST, insert schedule rows |
| `crates/server/src/routes/medications.rs` | Accept instruction fields in create/update |

### Frontend
| File | Changes |
|------|---------|
| `crates/app/src/pages/senior/medications.rs` | Display instruction timing on cards and detail page |
| `crates/app/src/pages/caregiver/profile.rs` | Wire availability settings page with exceptions |
| `crates/app/src/pages/medical/prescriptions.rs` | Add timing dropdown to prescription form |
| `crates/app/src/i18n.rs` | Add instruction timing Korean labels |

---

## Verification

1. **Medications:** Create medication with `instruction_timing: "AFTER_MEAL"`, `instruction_minutes: 30`. Verify senior portal shows "식후 30분". Verify minutes CHECK rejects negative values.
2. **Availability:** Caregiver sets weekly schedule via `PUT /api/availability`. Block a date via `POST /api/availability/exceptions`. Verify `GET` returns both. Verify `DELETE` rejects other user's exception (403).
3. **Matching:** Create match request with `requested_schedule: [{day: MONDAY, start: "14:00", end: "16:00"}]`. Verify only caregivers with Mon 14-16 availability appear. Verify blocked caregiver excluded. Verify zero-candidate case returns `NO_CANDIDATES` status.
4. **Scheduling:** `POST /api/visits/schedule` with 4-week Mon/Wed/Fri pattern. Verify visits created. Add caregiver exception on one date. Verify that date shows in `skipped`. Verify invalid inputs return 400.
5. **Reassignment:** Caregiver blocks a date with existing visit >24h away. Verify visit → `NEEDS_REASSIGNMENT`. Block a date <24h away. Verify `platform_events` has `reassignment_urgent` entry. Verify `get_upcoming_visits` still returns visits in `NEEDS_REASSIGNMENT` status.
6. **State machines:** Verify `NeedsReassignment → Scheduled` (reassignment success) and `NeedsReassignment → Cancelled` (no substitute) transitions work. Verify invalid transitions are rejected.
