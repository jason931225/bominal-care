# Bominal Care: Production-Ready Platform Capability Design

## Overview

Complete design specification for transforming Bominal Care from a partially-implemented prototype into a production-ready Korean senior care platform. This spec covers all 6+ portals with zero stubs, comprehensive accountability/observability, Korean regulatory compliance, HIS integration, and immutable medical data handling.

**Platform:** Rust/Leptos WASM + Axum + PostgreSQL
**Portals:** Senior, Family, Caregiver, Internal (Provider), Government, Medical Professional, Pharmacy
**Legal context:** 개인정보보호법 (PIPA), 노인장기요양보험법 (LTCI Act), 의료법, 약사법

---

## Table of Contents

1. [Portal Data Flow Architecture](#1-portal-data-flow-architecture)
2. [Medical Handoff Feature](#2-medical-handoff-feature)
3. [Role-Based Access & Audit Boundaries](#3-role-based-access--audit-boundaries)
4. [Policy Engine & Access Control Management](#4-policy-engine--access-control-management)
5. [Immutable Ledger System](#5-immutable-ledger-system)
6. [Codebase Audit — Additional Gaps](#6-codebase-audit--additional-gaps)
7. [Wire Dead Infrastructure](#7-wire-dead-infrastructure)
8. [Senior Safety & Wellness](#8-senior-safety--wellness)
9. [Caregiver Accountability](#9-caregiver-accountability)
10. [Family Features](#10-family-features)
11. [Provider Operations — Internal Portal](#11-provider-operations--internal-portal)
12. [Government Portal](#12-government-portal)
13. [Caregiver Portal](#13-caregiver-portal)
14. [Medical Portal & Handoff Integration](#14-medical-portal--handoff-integration)
15. [Korean Lifestyle Features](#15-korean-lifestyle-features)
16. [Technical Foundation](#16-technical-foundation)
17. [Medication Lifecycle, Reminders & Pharmacist Access](#17-medication-lifecycle-reminders--pharmacist-access)
18. [Regulatory Compliance Audit](#18-regulatory-compliance-audit)
19. [Medical/Pharmacy Professional Portal + HIS-lite + Integration](#19-medicalpharmacy-professional-portal--his-lite--integration)
20. [Comprehensive Accountability & Observability](#20-comprehensive-accountability--observability)

---

## 1. Portal Data Flow Architecture

All portals follow the same established pattern: Leptos page component → `fetch` to Axum API route → DB query → `ApiResponse<T>` envelope.

### Senior Portal — Existing, Wire Remaining Stubs

| Page | API Endpoint | Data |
|------|-------------|------|
| Dashboard | `GET /api/medications/today` + `GET /api/appointments?limit=5` + `GET /api/wellness/today` | Medications, appointments, wellness check-in |
| Appointments | `GET /api/appointments` + `POST /api/appointments` | Full CRUD, paginated |
| Medications | `GET /api/medications` + medication check-off endpoints | List + daily check-off card |
| Profile | `GET /api/profile/me` | Person profile + senior profile |
| Medical History | `GET /api/medical-history` | Condition list |
| Care Plan | `GET /api/care-plans/list?senior=me` | Active care plan |
| Consent | `GET /api/consent` | Granular consent management |
| Services | `GET /api/community-activities?near=me` + static service info | Community activities, services directory |
| Settings | `GET /api/profile/me` + `PATCH /api/profile` | Profile editing, notification prefs |
| Notifications | `GET /api/notifications` | Notification inbox |
| Emergency | `POST /api/emergency/trigger` | SOS with GPS |
| 의료진 입력 (Handoff) | `POST /api/handoff/start` | Medical handoff entry point |

### Internal (Provider) Portal — 7 Core Pages

| Page | API Endpoint | Data |
|------|-------------|------|
| Dashboard | `GET /api/dashboard/provider` | Real aggregated metrics (active seniors, visits today, caregiver utilization) |
| Clients List | `GET /api/profile/seniors` | Paginated senior list with care level, status |
| Client Detail | `GET /api/profile/seniors/{id}` | Full profile + care plan + recent visits |
| Care Plans | `GET /api/care-plans/list` | Org-scoped care plans with status filters |
| Caregivers | `GET /api/caregivers/list` | Staff roster, credentials, assignment status |
| Applications | `GET /api/caregivers/applications` | Pending/approved/rejected applications |
| Schedules | `GET /api/visits/list` + `GET /api/appointments/list` | Calendar view combining visits + appointments |

### Caregiver Portal — 5 Core Pages

| Page | API Endpoint | Data |
|------|-------------|------|
| Dashboard | `GET /api/visits/list?assignee=me&status=scheduled` | Today's visits, upcoming tasks |
| Clients | `GET /api/profile/seniors?caregiver=me` | Assigned seniors with care summaries |
| Schedule | `GET /api/visits/list?assignee=me` | Weekly calendar of visits |
| Tasks | `GET /api/care-plans/list?assignee=me` | Active care plan tasks to complete |
| Profile | `GET /api/profile/me` + `GET /api/caregivers/credentials` | Own profile + credential status |

### Government Portal — 5 Core Pages

| Page | API Endpoint | Data |
|------|-------------|------|
| Dashboard | `GET /api/gov/dashboard` | Regional metrics, provider count, beneficiary stats |
| Providers | `GET /api/gov/providers` | Provider organizations in jurisdiction |
| Beneficiaries | `GET /api/gov/beneficiary-stats` | De-identified LTCI beneficiary aggregates |
| Audit | `GET /api/gov/audit` | Audit log viewer with filters |
| Compliance | `GET /api/gov/provider-compliance` | Provider compliance indicators |

### New/Modified API Routes Required

- `GET /api/providers/list` — provider organization list
- `GET /api/caregivers/credentials` — caregiver credential list
- `GET /api/dashboard/provider` — real aggregated provider dashboard
- `GET /api/gov/dashboard` — de-identified government dashboard
- `GET /api/gov/beneficiary-stats` — de-identified beneficiary statistics
- `GET /api/gov/provider-compliance` — provider compliance matrix
- Existing routes need query parameter support for scoping (`?assignee=me`, `?scope=jurisdiction`, `?caregiver=me`)

---

## 2. Medical Handoff Feature

### User Flow

```
Senior/Caregiver taps "의료진 입력" button
  → Handoff entry screen appears
  → Medical professional selects: 의사 | 간호사 | 약사
  → Enters license number (면허번호)
  → Format validation (lightweight, no external API for demo)
  → Enters their name (이름)
  → Enters institution name (기관명) and institution code (요양기관기호)
  → Scoped mode activates
      → Doctors/Nurses: Appointments tab + Medications tab
      → Pharmacists: Medications tab only (dispensing fields)
      → Top banner: "의료진 입력 모드 — [이름] ([면허종류])"
      → Can only create entries for their own institution
  → Taps "완료" (Done)
  → Confirmation: "입력을 완료하시겠습니까?"
  → Returns to senior's normal view
```

### Data Model

```sql
CREATE TYPE handoff_license_type AS ENUM ('doctor', 'nurse', 'pharmacist');

CREATE TABLE medical_handoff_sessions (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    senior_id           UUID NOT NULL REFERENCES users(id),
    initiated_by        UUID NOT NULL REFERENCES users(id),
    license_type        handoff_license_type NOT NULL,
    license_number      VARCHAR NOT NULL,
    practitioner_name   VARCHAR NOT NULL,
    institution_name    VARCHAR NOT NULL,
    institution_code    VARCHAR NOT NULL,
    linked_professional_id UUID REFERENCES users(id),
    started_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    ended_at            TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

### Audit Integration

Every appointment/medication created during a handoff session gets:
- `created_by_handoff_session_id` FK on the record
- `platform_events` entry with `actor_type: handoff_proxy`, `proxy_license_type`, `proxy_license_number`, `proxy_institution`

### Scoped Mode Constraints

- No access to profile, settings, services, or any other senior data
- Doctors/Nurses: read existing + create new appointments and medications for own institution
- Pharmacists: view prescriptions + add dispensing info + add OTC recommendations for own pharmacy
- Session auto-expires after 30 minutes of inactivity
- Only one active handoff session per senior at a time

### API Routes

| Route | Purpose |
|-------|---------|
| `POST /api/handoff/start` | Validate license format, create session |
| `POST /api/handoff/end` | Close session |
| `GET /api/handoff/active` | Check if handoff is active for current senior |

### Disclaimer

Displayed during handoff entry: "본 입력은 돌봄 조정 목적이며, 공식 진료기록(전자의무기록)이 아닙니다."

---

## 3. Role-Based Access & Audit Boundaries

### Three Trust Levels for Data Entry

| Actor | Trust | Behavior |
|-------|-------|----------|
| Senior (self) | Full | Entries saved directly as `approved` |
| Medical professional (handoff) | High, scoped | Entries saved directly as `approved`, but only for their institution |
| Caregiver | Requires approval | Entries saved as `pending_approval`; senior or family must confirm |

### Permission Matrix — Appointments

| Actor | View | Create | Edit | Delete |
|-------|------|--------|------|--------|
| Senior | Own | Own | Own (future only) | Own (future, unconfirmed only) |
| Family | Linked senior's | Linked senior's → `approved` | Own-created, future only | Own-created, future unconfirmed only |
| Caregiver | Assigned senior's | Assigned senior's → `pending_approval` | No | No |
| Medical (handoff) | Senior's at own institution | Own institution only → `approved` | Own institution entries only | No |
| Internal (provider) | Org-scoped seniors | Org-scoped → `approved` | Org-scoped | No |
| Government | Anonymized aggregates only | No | No | No |
| Platform Admin | All | All | All | Soft-delete only |

### Permission Matrix — Medications

| Actor | View | Create | Edit | Delete |
|-------|------|--------|------|--------|
| Senior | Own | Own | Own | Own (unstarted only) |
| Family | Linked senior's | No | No | No |
| Caregiver | Assigned senior's | Assigned senior's → `pending_approval` | No | No |
| Medical (handoff) | Senior's at own institution | Own institution only → `approved` | Own institution entries only | No |
| Pharmacist (handoff) | Senior's prescriptions | OTC only → `approved` | Dispensing fields for own pharmacy → `approved` | No |
| Internal (provider) | Org-scoped | No | No | No |
| Government | Anonymized aggregates only | No | No | No |
| Platform Admin | All | All | All | Soft-delete only |

### Government Audit Boundary

**Government CAN see (observability layer):**
- Aggregated metrics: beneficiary counts by care level, visit hours by service category, provider utilization rates
- De-identified records: visit logs with provider ID, service type, duration, date — but NO senior_id, name, 주민등록번호, or diagnosis
- Provider compliance: credential expiry rates, incident counts, average response times
- Financial: claim totals by provider, co-payment collection rates

**Government CANNOT see:**
- Any personally identifiable information (PII) of seniors
- Individual medical records, medications, diagnoses
- Family relationship data
- Consent records content (can see that consent exists in aggregate, not the content)

**Technical implementation:**
- Separate `gov_*` materialized views — pre-aggregated, de-identified
- Government API routes (`/api/gov/*`) only query these views, never base tables
- RLS policy for `government_reviewer` role blocks direct table access entirely
- Every government data access is logged in platform_events

---

## 4. Policy Engine & Access Control Management

### Data Model

```sql
CREATE TABLE access_policies (
    id                UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    resource_type     TEXT NOT NULL CHECK (resource_type IN ('appointment','medication','care_plan','visit','incident','profile','senior_profile','consent','audit_log','notification','observation','referral','match_request','provider','caregiver_application','eligibility_case','prescription','clinical_encounter','lab_result')),
    actor_role        user_role NOT NULL,
    action            TEXT NOT NULL CHECK (action IN ('view','create','edit','delete','list','approve','reject','sign','transfer')),
    scope             TEXT NOT NULL CHECK (scope IN ('own','linked','assigned','org','jurisdiction','all')),
    requires_approval BOOLEAN NOT NULL DEFAULT false,
    anonymized        BOOLEAN NOT NULL DEFAULT false,
    enabled           BOOLEAN NOT NULL DEFAULT true,
    system_locked     BOOLEAN NOT NULL DEFAULT false,
    conditions        JSONB,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_by        UUID REFERENCES users(id)
);

CREATE TABLE policy_change_log (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    policy_id   UUID NOT NULL REFERENCES access_policies(id),
    changed_by  UUID NOT NULL REFERENCES users(id),
    old_value   JSONB NOT NULL,
    new_value   JSONB NOT NULL,
    reason      TEXT NOT NULL,
    changed_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

### Runtime Behavior

```
Request comes in
  → Auth extractor resolves user + role
  → Route handler calls policy_engine::check(role, resource, action)
  → Engine queries access_policies (cached in-memory, refreshed on change)
  → Returns: Allowed | Denied | RequiresApproval | AllowedAnonymized
  → Route handler applies result accordingly
```

### Internal Portal — Policy Management Page

Accessible only by `security_admin` and `org_admin` permission levels.

```sql
CREATE TYPE internal_permission_level AS ENUM (
    'staff',
    'manager',
    'security_admin',
    'org_admin'
);
```

UI: Settings (설정) → Access Policies (접근 권한 정책) → Table view of resource × role matrix. Click cell to edit scope, approval requirement, anonymization, and conditions. Every change requires a written justification reason. Full change history visible per policy.

### System-Locked Policies

Some policies cannot be overridden at org level (only platform admin):
- Government can never see PII
- Audit logs are always append-only
- Medical records always require signing

---

## 5. Immutable Ledger System

Medical records are append-only. No updates, no deletes — only new entries that supersede previous ones.

### Ledger Tables

```sql
CREATE TABLE medication_ledger (
    id                      UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    medication_id           UUID NOT NULL,
    version                 INT NOT NULL,
    action                  TEXT NOT NULL,
    data                    JSONB NOT NULL,
    reason                  TEXT,
    actor_id                UUID REFERENCES users(id),
    actor_type              TEXT NOT NULL,
    handoff_session_id      UUID REFERENCES medical_handoff_sessions(id),
    approval_status         TEXT NOT NULL DEFAULT 'approved',
    approved_by             UUID REFERENCES users(id),
    institution_code        VARCHAR,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(medication_id, version)
);

CREATE TABLE appointment_ledger (
    -- Same structure as medication_ledger, with appointment_id instead
    id                      UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    appointment_id          UUID NOT NULL,
    version                 INT NOT NULL,
    action                  TEXT NOT NULL,
    data                    JSONB NOT NULL,
    reason                  TEXT,
    actor_id                UUID REFERENCES users(id),
    actor_type              TEXT NOT NULL,
    handoff_session_id      UUID REFERENCES medical_handoff_sessions(id),
    approval_status         TEXT NOT NULL DEFAULT 'approved',
    approved_by             UUID REFERENCES users(id),
    institution_code        VARCHAR,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(appointment_id, version)
);

CREATE TABLE care_plan_ledger (
    -- Same structure
    id                      UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    care_plan_id            UUID NOT NULL,
    version                 INT NOT NULL,
    action                  TEXT NOT NULL,
    data                    JSONB NOT NULL,
    reason                  TEXT,
    actor_id                UUID REFERENCES users(id),
    actor_type              TEXT NOT NULL,
    handoff_session_id      UUID REFERENCES medical_handoff_sessions(id),
    approval_status         TEXT NOT NULL DEFAULT 'approved',
    approved_by             UUID REFERENCES users(id),
    institution_code        VARCHAR,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(care_plan_id, version)
);
```

### Current State Views

```sql
CREATE VIEW medication_current AS
SELECT DISTINCT ON (medication_id) *
FROM medication_ledger
ORDER BY medication_id, version DESC;
```

### Action Types

| Action | Who Can Do It |
|--------|--------------|
| `created` | Per policy matrix (Section 3) |
| `modified` | Original creator's role or higher trust level, with mandatory reason |
| `cancelled` | Senior (own), Provider (org-scoped), Platform Admin — with mandatory reason |
| `approved` / `rejected` | Senior, Family (for caregiver entries) |

### UI Implications

- Edit buttons show "수정" but create a new version, not in-place update
- Cancel buttons show "취소" with a mandatory reason field
- History tab on any record shows the full version chain as a timeline
- Diff view between versions available to provider and government roles

---

## 6. Codebase Audit — Additional Gaps

### "Defined But Never Wired" — Dead Infrastructure

| System | Exists | Missing |
|--------|--------|---------|
| RBAC permissions | `has_permission()` function with full matrix | Never called — routes only check role lists |
| State machines | 8 workflows fully defined | Never validated before status updates |
| Domain events | 13 event types with builder | Never emitted after operations |
| Input validation | 40+ DTOs with `#[derive(Validate)]` | `.validate()` never called |
| Audit logging | Table + model + API route | Never inserted by any operation |
| i18n translations | Korean + English maps for 30+ keys | Never called — all responses English |
| Data ownership | Scope enum in RBAC | Never enforced — any role-matched user reads any record by ID |

### 18 Tables With Zero Query Coverage

Critical: `incidents`, `family_relationships`, `provider_organizations`, `approval_steps`, `claim_or_subsidy_records`

Important: `service_regions`, `availability_slots`, `service_types`, `users` (admin management)

### 14+ Missing Database Indexes

On frequently-joined FK columns: `appointments.person_id`, `medications.person_id`, `care_plans.senior_id`, `visits.caregiver_id`, `visits.care_plan_id`, `daily_observations.care_plan_id`, `medical_history_entries.person_id`, `incidents.visit_id`, `match_recommendations.match_request_id`, `family_relationships.senior_person_id`, `caregiver_credentials.application_id`, `availability_slots.application_id`, `service_types.application_id`, `approval_steps.case_id`, `claim_or_subsidy_records.case_id`.

### Schema Inconsistencies

- `daily_observations.category` is TEXT but should use `observation_category` enum
- `housing_type` enum defined but not used in any table
- No soft-delete anywhere — all deletes are permanent

---

## 7. Wire Dead Infrastructure

### Handler Pattern — Before vs After

```rust
// BEFORE (current — every handler)
async fn get_appointment(user: AuthUser, Path(id): Path<Uuid>, State(state): State<AppState>) -> impl IntoResponse {
    match appointment::get_appointment(&state.pool, id).await {
        Ok(Some(a)) => ApiResponse::success(a),
        Ok(None) => ApiResponse::not_found("Appointment not found"),
        Err(e) => { tracing::error!("{e}"); ApiResponse::internal_error() }
    }
}

// AFTER (production-ready)
async fn get_appointment(user: AuthUser, Path(id): Path<Uuid>, State(state): State<AppState>) -> impl IntoResponse {
    // 1. RBAC permission check
    require_permission(&user, Resource::Appointment, Action::Read)?;
    // 2. Fetch
    let appointment = appointment::get_appointment(&state.pool, id).await?;
    // 3. Ownership validation
    require_ownership(&user, &appointment, &state.pool).await?;
    // 4. Audit log
    emit_event(&state, EventBuilder::new("appointment.read")
        .actor(&user).entity("appointment", id)
        .sensitivity(Sensitivity::Confidential)
        .category(EventCategory::Clinical)).await;
    // 5. i18n error messages
    ApiResponse::success(appointment)
}
```

### Scope of Change

| Layer | What Gets Added | Scope |
|-------|----------------|-------|
| RBAC enforcement | `require_permission()` at top of every handler | ~50 endpoints |
| Input validation | `input.validate()?` on every POST/PATCH | ~25 handlers |
| Data ownership | `require_ownership()` per resource type | ~50 handlers |
| State machine validation | `state_machine.can_transition(from, to)?` | 8 handlers with status changes |
| Event emission | `emit_event()` after every operation | All ~50 handlers |
| i18n error messages | Replace English strings with `t()` | All error responses |

### Ownership Resolution Rules

```
Appointment  → person_id matches user's person_profile, OR family link, OR assigned caregiver, OR org scope
Medication   → same as appointment
CarePlan     → senior_id matches, OR family link, OR assigned caregiver, OR provider org
Visit        → care_plan ownership chain, OR caregiver_id matches
Notification → user_id matches directly
Consent      → subject_person_id matches, OR granted_by matches
AuditLog     → government/provider/admin only
```

---

## 8. Senior Safety & Wellness

### Daily Wellness Check-in (안부 확인)

```sql
CREATE TABLE wellness_checkins (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    senior_id       UUID NOT NULL REFERENCES users(id),
    checked_in_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    mood            wellness_mood NOT NULL,  -- enum: good, okay, not_great, need_help
    notes           TEXT,
    source          TEXT NOT NULL
);

CREATE TABLE wellness_check_config (
    id                UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    senior_id         UUID NOT NULL UNIQUE REFERENCES users(id),
    enabled           BOOLEAN NOT NULL DEFAULT true,
    check_in_window   TIME NOT NULL DEFAULT '06:00',
    escalation_time   TIME NOT NULL DEFAULT '10:00',
    escalation_chain  JSONB NOT NULL,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

**Senior portal:** Big "오늘의 안부" card at top of dashboard. One tap → mood selection (좋아요/괜찮아요/별로예요/도움필요) → done. Optional notes.

**Family portal:** Wellness status as color dot next to senior's name. Green = checked in. Yellow = window open, not yet. Red = escalation time passed.

**Escalation flow:**
1. `check_in_window` passes → senior gets push notification
2. `escalation_time` passes with no check-in → first person in chain gets alert
3. +30 min → next person
4. +60 min → provider gets alert

### Emergency SOS (Backend-Wired)

```sql
CREATE TABLE emergency_events (
    id               UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    senior_id        UUID NOT NULL REFERENCES users(id),
    triggered_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    latitude         DOUBLE PRECISION,
    longitude        DOUBLE PRECISION,
    accuracy_meters  DOUBLE PRECISION,
    status           emergency_event_status NOT NULL DEFAULT 'triggered',  -- enum: triggered, responders_notified, resolved, false_alarm
    resolved_at      TIMESTAMPTZ,
    resolved_by      UUID REFERENCES users(id),
    resolution_notes TEXT
);
```

When senior taps emergency button:
1. Create `emergency_event` with GPS
2. Send to all contacts in parallel: family, caregiver, provider
3. Include: senior name, location, medical summary (conditions, medications, allergies), emergency contacts
4. Notification type: `EMERGENCY` — bypasses do-not-disturb
5. All events logged in platform_events with `sensitivity: restricted`

### Medication Escalation Chain

```sql
CREATE TABLE medication_escalation_config (
    id                          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    senior_id                   UUID NOT NULL UNIQUE REFERENCES users(id),
    consecutive_miss_threshold  INT NOT NULL DEFAULT 2,
    escalation_chain            JSONB NOT NULL
);
```

After N consecutive missed medication events → trigger escalation chain. Each step is a notification with increasing urgency.

---

## 9. Caregiver Accountability

### GPS Geofencing on Check-in

Store distance on visit record: `check_in_distance_meters DOUBLE PRECISION`.

During check-in, calculate haversine distance between check-in GPS and senior's registered address. If > 200m (configurable per provider), emit WARNING signal. Do not block — caregiver might be at hospital with senior.

### Visit Duration Anomaly Detection

After check-out, compare actual vs scheduled duration:
- < 50% of scheduled → WARNING signal to provider
- < 25% of scheduled → ALERT signal to provider + family
- Provider can review and dismiss or escalate

### Photo Evidence

```sql
CREATE TABLE visit_evidence (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    visit_id    UUID NOT NULL REFERENCES visits(id),
    task_index  INT NOT NULL,
    photo_url   TEXT NOT NULL,
    caption     TEXT,
    captured_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    captured_by UUID NOT NULL REFERENCES users(id)
);
```

Optional per task. Provider configures which task types require photos in care plan.

### Substitute Caregiver Handoff Notes

```sql
CREATE TABLE care_plan_handoff_notes (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    care_plan_id  UUID NOT NULL REFERENCES care_plans(id),
    created_by    UUID NOT NULL REFERENCES users(id),
    content       JSONB NOT NULL,
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

Content structure: `{ access_code, allergies, preferences, behavioral_notes, emergency_protocol }`. Shown prominently to substitute caregivers before their first visit.

---

## 10. Family Features

### Daily Care Summary

```sql
CREATE TABLE daily_care_summaries (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    senior_id       UUID NOT NULL REFERENCES users(id),
    summary_date    DATE NOT NULL,
    medications     JSONB NOT NULL,
    visits          JSONB NOT NULL,
    wellness        JSONB NOT NULL,
    observations    JSONB NOT NULL,
    alerts          JSONB NOT NULL,
    generated_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(senior_id, summary_date)
);
```

Generated at end of day (configurable, default 23:00 KST to capture late-evening events). Family receives push notification with summary. Viewable as timeline in family portal. If events occur after generation (e.g., late check-out), a retroactive update is appended to the next day's summary with a note referencing the original date.

### Real-time Visit Status

Family portal shows live status per visit:
- `SCHEDULED` → "예정됨 — 오후 2:00"
- `CAREGIVER_ACKNOWLEDGED` → "요양보호사 확인됨"
- `IN_PROGRESS` → "방문 중 — 오후 2:05부터" (with elapsed timer)
- `COMPLETED` → "완료 — 2시간 15분"
- `MISSED` → "미방문 ⚠️"

Polling every 60s initially. SSE upgrade in later phase.

### Care Quality Indicators

Weekly score computed from:
- Medication adherence rate (taken / total scheduled)
- Visit completion rate (completed / total scheduled)
- Wellness check-in rate (days checked in / 7)
- Incident count (inverse)

Displayed as traffic light: Green (≥80%), Yellow (50-79%), Red (<50%).

### Approval Workflow for Caregiver Entries

Family portal approvals page expands to show pending:
- Medications submitted by caregiver
- Appointments submitted by caregiver
- One-tap approve/reject with optional reason

---

## 11. Provider Operations — Internal Portal

Every page production-ready with real API calls:

| Page | Functionality |
|------|--------------|
| **Dashboard** | Real aggregations: active seniors count, today's visits (completed/total), caregiver utilization %, open incidents, pending applications. Each card links to detail. |
| **Clients List** | Paginated, searchable by name/care level/status. Filters: care level, active/inactive, assigned caregiver. Bulk assign caregiver. |
| **Client Detail** | Full profile + care plan + visit history + medication list + observations timeline + incident history. Edit care plan inline. |
| **Caregivers List** | Staff roster with credential status (valid/expiring/expired), active client count, current week hours. Filter by credential status. |
| **Caregiver Detail** | Profile + credentials + assigned clients + schedule + performance metrics (punctuality, visit duration compliance, task completion rate). |
| **Applications** | Inbox of pending caregiver applications. Review workflow: identity → credentials → background → approve/reject. |
| **Schedules** | Weekly calendar view across all caregivers. Assignment interface. Conflict detection (double-booked caregiver or senior). |
| **Schedule Conflicts** | Flagged conflicts: overlapping visits, caregiver overtime, unassigned visits. One-click reassignment. |
| **Quality** | Real KPIs: medication adherence rate across all seniors, visit completion rate, average punctuality, incident rate per 100 visits. Trend charts (weekly). |
| **Incidents** | Full CRUD. Create with severity, link to visit, assign investigator. Resolution workflow: investigate → action → close. |
| **Referrals** | Incoming/outgoing referral management. Create referral to another provider. Track status through state machine. |
| **Compliance** | Credential expiration dashboard: caregivers with expiring certs in 30/60/90 days. Mandatory training tracking. |
| **Reports** | Monthly report generation: visit hours by caregiver, service utilization by senior, incident summary, financial summary. Export as PDF. |
| **Settings** | Organization profile, notification preferences, branding. |
| **Policy Management** | Access control UI (Section 4) — `security_admin` and `org_admin` only. Invisible to `staff` and `manager`. |

### New API Routes

| Route | Purpose |
|-------|---------|
| `GET /api/providers/{id}` | Provider org detail |
| `PATCH /api/providers/{id}` | Update provider org |
| `GET /api/providers/{id}/stats` | Aggregated stats for org |
| `GET /api/caregivers/roster` | All caregivers in org with stats |
| `GET /api/caregivers/{id}/performance` | Performance metrics |
| `POST /api/incidents` | Create incident |
| `GET /api/incidents` | List incidents |
| `PATCH /api/incidents/{id}` | Update incident |
| `GET /api/reports/monthly` | Monthly report data |
| `GET /api/schedules/conflicts` | Scheduling conflicts |
| `POST /api/schedules/assign` | Assign visit to caregiver |
| `GET /api/credentials/expiring` | Expiring credentials |
| `GET /api/dashboard/provider` | Aggregated dashboard |

---

## 12. Government Portal

Every page production-ready with real data from de-identified materialized views.

| Page | Functionality |
|------|--------------|
| **Dashboard** | Beneficiary count by care level, provider count by type/status, visit hours this month, incident rate, pending eligibility cases. |
| **Providers** | Full list with compliance scores. Detail: registration info, service regions, caregiver count, quality grade. |
| **Beneficiaries** | De-identified aggregates: count by care level, utilization rates, average visits/month, adherence rates by region. No PII. |
| **Eligibility** | Case review workflow: list pending → review documentation → approve/deny → appeal handling. Full approval_steps management. |
| **Audit** | Query audit logs by date range, action type, entity type, provider. Export for NHIS inspectors. |
| **Observability** | Signal viewer: filter by severity, type, date range. Acknowledge signals. Trends. |
| **Programs** | Program management: LTCI, 노인돌봄, 치매안심센터, 응급안전. Beneficiary counts, utilization, budget vs actual. |
| **Compliance** | Provider compliance matrix: which providers meet which requirements. Flag non-compliant providers. |

### Government Materialized Views

```sql
CREATE MATERIALIZED VIEW gov_beneficiary_stats AS
SELECT care_level, COUNT(*) as count, city, district
FROM senior_profiles sp
JOIN person_profiles pp ON sp.person_id = pp.id
GROUP BY care_level, city, district
HAVING COUNT(*) >= 5;  -- k-anonymity: suppress groups with fewer than 5 individuals to prevent re-identification

CREATE MATERIALIZED VIEW gov_visit_stats AS
SELECT date_trunc('month', scheduled_start) as month,
       status, COUNT(*),
       AVG(EXTRACT(EPOCH FROM (actual_end - actual_start))/3600) as avg_hours
FROM visits GROUP BY 1, 2;

CREATE MATERIALIZED VIEW gov_provider_compliance AS
SELECT po.id, po.name, po.type,
       COUNT(DISTINCT ca.id) as caregiver_count,
       COUNT(DISTINCT cc.id) FILTER (WHERE cc.status = 'EXPIRED') as expired_credentials
FROM provider_organizations po
LEFT JOIN caregiver_applications ca ON ca.provider_id = po.id
LEFT JOIN caregiver_credentials cc ON cc.application_id = ca.id
GROUP BY po.id, po.name, po.type;
```

### New API Routes

`GET /api/gov/beneficiary-stats`, `GET /api/gov/visit-stats`, `GET /api/gov/provider-compliance`, `GET /api/gov/eligibility-cases`, `PATCH /api/gov/eligibility-cases/{id}`.

---

## 13. Caregiver Portal

Every page production-ready:

| Page | Functionality |
|------|--------------|
| **Dashboard** | Today's visits with countdown to next, quick check-in button, unread notifications, weekly hours summary. |
| **Schedule** | Week view with daily visit cards. Tap for detail. Check-in/check-out flow with GPS capture. |
| **Check-in Flow** | Confirm senior address → GPS capture → start visit → task checklist → photo evidence (if required) → notes → check-out → GPS capture → submit. |
| **Clients** | Assigned seniors with care level, last visit, upcoming visit. Detail: care plan, medications, handoff notes, emergency info. |
| **Client Medications** | View assigned senior's medication list. Submit entries as `pending_approval`. See approval status. |
| **Tasks** | Active care plan tasks across all clients. Mark complete with optional photo. |
| **Incident Report** | Create incident linked to visit. Severity selection, description, immediate actions taken. |
| **Profile** | Own profile, credentials (status + expiry dates), availability slots management. |
| **Application Wizard** | Full 7-step flow: identity (PASS 본인인증, NOT 주민등록번호) → credentials (upload) → service regions → availability → services offered → references → review → submit. |
| **Notifications** | Notification inbox with filters. Mark read. |
| **Settings** | Notification preferences, language, availability management. |

### New API Routes

`GET /api/visits/today?assignee=me`, `POST /api/visits/{id}/tasks/{index}/evidence`, `GET /api/caregivers/me/credentials`, `PATCH /api/caregivers/me/availability`.

---

## 14. Medical Portal & Handoff Integration

### Medical Portal Pages

See Section 19.2 for the authoritative medical professional portal page list (includes Clinical Notes/SOAP in addition to the pages below). Section 19.3 covers the pharmacy portal pages.

### Handoff → Account Linking

When a medical professional registers, system checks for matching license numbers in `medical_handoff_sessions`. If found, offers to link previous entries to the new account, populating their patient list retroactively.

---

## 15. Korean Lifestyle Features

### Benefit Utilization Tracker

```sql
CREATE TABLE benefit_utilization (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    senior_id       UUID NOT NULL REFERENCES users(id),
    month           DATE NOT NULL,
    monthly_cap     DECIMAL(12,2) NOT NULL,
    used_amount     DECIMAL(12,2) NOT NULL DEFAULT 0,
    copayment_tier  TEXT NOT NULL DEFAULT 'standard',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(senior_id, month),
    CHECK (EXTRACT(DAY FROM month) = 1)  -- always store as 1st of month
);
```

Senior + family dashboards show progress bar: "이번 달 ₩850,000 / ₩1,417,200 사용"

### Co-payment Estimator

Before booking a service, show estimated 본인부담금 with tier-aware calculation:
- Exempt (기초생활수급자): 0%
- 60% reduction: 재가 6%, 시설 8%
- 40% reduction: 재가 9%, 시설 12%
- Standard: 재가 15%, 시설 20%

### Public Holiday Awareness

```sql
CREATE TABLE korean_holidays (
    id      UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    date    DATE NOT NULL UNIQUE,
    name    TEXT NOT NULL,
    type    TEXT NOT NULL
);
```

Visit scheduling warns if scheduled on holiday. Pharmacy/office hours adjusted.

### Seasonal Health Alerts

```sql
CREATE TABLE seasonal_alerts (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    alert_type  TEXT NOT NULL,
    title       TEXT NOT NULL,
    message     TEXT NOT NULL,
    active_from DATE NOT NULL,
    active_to   DATE NOT NULL,
    severity    TEXT NOT NULL,
    target_roles user_role[] NOT NULL
);
```

Summer: 폭염 경보. Winter: 한파/독감 alerts. Push to all seniors + caregivers.

### Community Activities (경로당)

```sql
CREATE TABLE community_activities (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name        TEXT NOT NULL,
    location    TEXT NOT NULL,
    city        TEXT NOT NULL,
    district    TEXT NOT NULL,
    schedule    TEXT NOT NULL,
    type        TEXT NOT NULL,
    phone       TEXT,
    latitude    DOUBLE PRECISION,
    longitude   DOUBLE PRECISION,
    is_active   BOOLEAN NOT NULL DEFAULT true
);
```

Senior services page shows nearby activities with schedule.

---

## 16. Technical Foundation

### CSRF Protection

Generate CSRF token per session, include in meta tag, validate on all POST/PATCH/DELETE.

### Content-Security-Policy

```
default-src 'self';
script-src 'self' 'wasm-unsafe-eval';
style-src 'self' 'unsafe-inline';
font-src 'self' cdn.jsdelivr.net;
img-src 'self' data:;
connect-src 'self';
frame-ancestors 'none'
```

### Missing Indexes (Migration 0004)

Add all 14+ missing FK indexes identified in Section 6.

### Schema Fixes (Migration 0004)

- Change `daily_observations.category` from TEXT to `observation_category` enum
- Add `deleted_at TIMESTAMPTZ` nullable to core tables for soft-delete
- Add `check_in_distance_meters DOUBLE PRECISION` to visits
- Add `approval_status`, `submitted_by`, `approved_by`, `approved_at` to appointments and medications

### Rate Limiting

Replace the current global `tower::RateLimitLayer` with `governor` + `tower-governor` for per-IP rate limiting:

| Endpoint Group | Limit | Window |
|---------------|-------|--------|
| Auth (login/register) | 5 requests | 60 seconds |
| API writes (POST/PATCH/DELETE) | 30 requests per user | 60 seconds |
| API reads (GET) | 100 requests per user | 60 seconds |
| Global fallback | 200 requests per IP | 60 seconds |
| Handoff start | 3 requests per senior | 60 seconds |

Use `governor::Quota` with `tower-governor` middleware. Rate limit headers (`X-RateLimit-Remaining`, `X-RateLimit-Reset`) included in responses. 429 responses include `Retry-After` header.

### Bounded API Cache

Replace unbounded thread-local HashMap with LRU cache: max 100 entries, 5-min TTL. Clear on auth state change.

### PWA Manifest

`manifest.json` with app name, icons, `display: standalone`, `theme_color`. Service worker for offline shell caching.

### All Uncovered Tables

Write query modules for all 18 tables identified in Section 6 that currently lack queries.

---

## 17. Medication Lifecycle, Reminders & Pharmacist Access

### Prescription Duration Tracking

The medication ledger `data` JSONB includes:

```json
{
    "name": "아스피린",
    "dosage": "100mg",
    "form": "정제",
    "frequency": "ONCE_DAILY",
    "prescribed_by": "김의사",
    "institution_name": "서울내과의원",
    "institution_code": "12345678",
    "prescribed_date": "2026-03-19",
    "prescribed_days": 30,
    "total_quantity": 30,
    "remaining_quantity": 22,
    "expiry_date": "2026-04-18",
    "dispensed_date": "2026-03-19",
    "dispensed_by": "박약사",
    "pharmacy_name": "건강약국",
    "pharmacy_code": "87654321",
    "generic_substitution": false,
    "drug_interactions": ["메트포르민과 병용 시 저혈당 주의"],
    "storage_instructions": "실온보관, 직사광선 차단",
    "side_effect_warnings": ["위장장애", "출혈경향"]
}
```

**No refills in Korean medical system.** Every new supply requires a new 처방전 from a doctor.

**Expiry notifications:**
- 5일 remaining → "아스피린 5일분 남았습니다. 진료 예약이 필요합니다."
- 0일 remaining → "처방이 만료되었습니다. 의사 진료를 예약하세요."
- Links to appointment creation page pre-filled with purpose "처방 갱신"

### Medication Reminder & Check-off System

```sql
CREATE TABLE medication_reminders (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    medication_id   UUID NOT NULL REFERENCES medications(id),
    senior_id       UUID NOT NULL REFERENCES users(id),
    reminder_times  JSONB NOT NULL,
    reminder_method TEXT NOT NULL DEFAULT 'push',
    snooze_minutes  INT NOT NULL DEFAULT 15,
    max_snoozes     INT NOT NULL DEFAULT 3,
    enabled         BOOLEAN NOT NULL DEFAULT true,
    created_by      UUID NOT NULL REFERENCES users(id),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

**Check-off flow (1-2 taps):**

```
Dashboard → "오늘의 복약" card

┌─────────────────────────────────────┐
│ 오전 8:00 아침 식후                  │
│                                     │
│ ☐ 아스피린 100mg                    │
│ ☐ 메트포르민 500mg                  │
│ ☐ 란소프라졸 15mg                   │
│                                     │
│ [ 모두 복용 완료 ✓ ]                │
└─────────────────────────────────────┘
```

- Tap individual → check off one → `medication_event` with `TAKEN`
- Tap "모두 복용 완료" → batch check-off for time slot
- Missed window (+2 hours default) → auto `MISSED` event
- Each check-off is an immutable ledger entry

**Reminder chain:**
1. Push at scheduled time
2. No check-off in 15 min → snooze reminder
3. After max snoozes → `REMINDER_SENT`
4. Window expires → `MISSED`
5. Consecutive misses → escalation chain (Section 8)

### Pharmacist Handoff — Scoped View

Pharmacists see a different view than doctors/nurses:
- View existing prescriptions (verify what was prescribed)
- Add dispensing information: dispensed_date, pharmacy, generic_substitution, drug_interactions, storage_instructions, side_effect_warnings
- Update remaining_quantity on new dispensing
- Add OTC medication entries with `source: pharmacist_recommendation`
- Cannot create appointments or view appointment details

---

## 18. Regulatory Compliance Audit

### 18.1 CRITICAL — 주민등록번호 수집 금지

**Current state:** Caregiver application wizard has 주민등록번호 input. **This is illegal** since August 2014 (개인정보보호법).

**Fix:** Remove entirely. Use PASS 본인인증 (SKT/KT/LGU+) or 공동인증서.

```sql
CREATE TABLE identity_verifications (
    id                UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id           UUID NOT NULL REFERENCES users(id),
    method            TEXT NOT NULL,
    verification_id   TEXT NOT NULL,
    verified_name     TEXT NOT NULL,
    verified_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    expires_at        TIMESTAMPTZ NOT NULL
);
```

We verify identity; we never store 주민등록번호.

### 18.2 CRITICAL — PIPA Consent Model

**Current:** 4 coarse purposes. **Required:** Separate, granular consents per PIPA.

```sql
CREATE TYPE consent_purpose_v2 AS ENUM (
    'personal_info_collection',
    'sensitive_info_processing',
    'third_party_medical',
    'third_party_government',
    'third_party_provider',
    'third_party_family',
    'third_party_caregiver',
    'marketing'
);
```

Each consent record must store: policy_version, consent_text_hash, collection_items, retention_period_days.

```sql
ALTER TABLE consent_records ADD COLUMN policy_version INT;
ALTER TABLE consent_records ADD COLUMN consent_text_hash TEXT;
ALTER TABLE consent_records ADD COLUMN collection_items TEXT[];
ALTER TABLE consent_records ADD COLUMN retention_period_days INT;
```

**Consent migration strategy:**
1. Create `consent_purpose_v2` enum alongside existing `consent_purpose`
2. Add `purpose_v2 consent_purpose_v2` column to `consent_records`
3. Migrate existing data:
   - `NoShare` → no mapping (these are opt-outs, keep as-is with `purpose_v2 = NULL`)
   - `MedicalShare` → create two new consent rows: `personal_info_collection` + `third_party_medical`
   - `GovernmentShare` → create two new consent rows: `personal_info_collection` + `third_party_government`
   - `BothShare` → create three new consent rows: `personal_info_collection` + `third_party_medical` + `third_party_government`
4. All migrated consents get `policy_version = 1`, current timestamp, and `consent_text_hash` of the v1 policy text
5. Drop old `purpose` column after migration verified
6. Rename `purpose_v2` to `purpose`
7. **PIPA re-consent requirement:** Existing seniors must be prompted to re-consent under the new granular model on next login. `sensitive_info_processing` consent (건강정보) was never separately collected and MUST be obtained before continued health data processing. Display a consent collection screen that blocks access to health features until `sensitive_info_processing` is granted.

### 18.3 CRITICAL — 본인부담금 Reduction Tiers

```sql
CREATE TYPE copayment_tier AS ENUM ('exempt', 'reduction_60', 'reduction_40', 'standard');
ALTER TABLE senior_profiles ADD COLUMN copayment_tier copayment_tier NOT NULL DEFAULT 'standard';
```

| Tier | 재가급여 | 시설급여 |
|------|---------|---------|
| exempt (기초생활수급자) | 0% | 0% |
| reduction_60 (저소득) | 6% | 8% |
| reduction_40 (중저소득) | 9% | 12% |
| standard | 15% | 20% |

2026 addition: 중증 수급자 방문간호 first 3 visits — 본인부담금 면제.

### 18.4 CRITICAL — 요양보호사 = 자격증, NOT 면허

```sql
CREATE TYPE credential_classification AS ENUM ('license', 'qualification');
```

- 면허 (license): 의사, 간호사, 약사 — issued by 보건복지부
- 자격증 (qualification): 요양보호사, 사회복지사 — issued by 시·도지사 after 320-hour training + exam

### 18.5 HIGH — 대체조제 Notification Requirement (약사법 제27조)

```sql
CREATE TABLE generic_substitution_records (
    id                              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    medication_ledger_id            UUID NOT NULL,
    original_drug                   TEXT NOT NULL,
    substituted_drug                TEXT NOT NULL,
    substitution_basis              TEXT NOT NULL,
    doctor_notified                 BOOLEAN NOT NULL DEFAULT false,
    doctor_notification_deadline    TIMESTAMPTZ NOT NULL,
    doctor_notified_at              TIMESTAMPTZ,
    patient_notified_at             TIMESTAMPTZ NOT NULL,
    pharmacist_license              TEXT NOT NULL,
    pharmacy_code                   TEXT NOT NULL,
    created_at                      TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

System alert if `doctor_notified = false` and deadline passed.

### 18.6 HIGH — 인지지원등급 Missing + care_level Migration

Korean LTCI has 6 grades. The existing schema uses `INTEGER` for `care_level`. This requires a data migration.

```sql
-- Step 1: Create the new enum
CREATE TYPE care_level_enum AS ENUM (
    'level_1', 'level_2', 'level_3', 'level_4', 'level_5', 'cognitive'
);

-- Step 2: Add temporary column
ALTER TABLE senior_profiles ADD COLUMN care_level_new care_level_enum;

-- Step 3: Migrate data
UPDATE senior_profiles SET care_level_new = CASE
    WHEN care_level = 1 THEN 'level_1'::care_level_enum
    WHEN care_level = 2 THEN 'level_2'::care_level_enum
    WHEN care_level = 3 THEN 'level_3'::care_level_enum
    WHEN care_level = 4 THEN 'level_4'::care_level_enum
    WHEN care_level = 5 THEN 'level_5'::care_level_enum
    ELSE NULL
END;

-- Step 4: Swap columns
ALTER TABLE senior_profiles DROP COLUMN care_level;
ALTER TABLE senior_profiles RENAME COLUMN care_level_new TO care_level;
```

**Rust type change:** `SeniorProfile.care_level` changes from `Option<i32>` to `Option<CareLevelEnum>`. `SeniorProfileInput` validation changes from `range(min=1, max=5)` to enum variant validation. All queries touching `care_level` must be updated. Existing `care_level = NULL` rows remain NULL (no care level assessed yet).

### 18.7 HIGH — Platform Legal Position

We are a care coordination platform (돌봄 조정 플랫폼), not an electronic medical record system, EXCEPT for institutions using HIS-lite (Section 19), which operates under 의료법 제23조.

Stated in: terms of service, privacy policy, handoff UI disclaimer.

### 18.8 HIGH — Data Retention Periods

```sql
CREATE TABLE data_retention_policies (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    data_category   TEXT NOT NULL UNIQUE,
    retention_days  INT NOT NULL,
    legal_basis     TEXT NOT NULL,
    auto_archive    BOOLEAN NOT NULL DEFAULT true
);

-- Seed:
-- care_coordination_records: 1825 days (5 years)
-- consent_records: 1825 days after revocation
-- audit_logs/platform_events: 1825 days
-- identity_verification: 1095 days (3 years)
-- emergency_events: 3650 days (10 years)
-- clinical_encounters: 3650 days (10 years, per 의료법)
-- prescriptions (dispensed document): 730 days (2 years, per 의료법 시행규칙 제15조)
-- prescriptions (prescriber record in system): 3650 days (10 years, as part of clinical encounter per 의료법 제22조)
-- lab_results: 1825 days (5 years, per 의료법)
```

### 18.9 HIGH — PASS Re-verification for Sensitive Operations

| Operation | Verification Required |
|-----------|---------------------|
| First login | PASS or 공동인증서 |
| Granting consent | Re-verify via PASS |
| Accessing other person's data | PASS + verified relationship |
| Exporting personal data | Re-verify via PASS |
| Changing copayment tier | Government reviewer only |

### 18.10 HIGH — NHIS Institution Codes

```sql
ALTER TABLE provider_organizations ADD COLUMN nhis_institution_code TEXT UNIQUE;
ALTER TABLE provider_organizations ADD COLUMN medical_institution_code TEXT UNIQUE;
```

---

## 19. Medical/Pharmacy Professional Portal + HIS-lite + Integration

### 19.1 Professional Registration & Account Linking

```sql
CREATE TABLE medical_professional_profiles (
    id                      UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id                 UUID NOT NULL UNIQUE REFERENCES users(id),
    license_type            handoff_license_type NOT NULL,
    license_number          TEXT NOT NULL UNIQUE,
    specialty               TEXT,
    institution_id          UUID REFERENCES provider_organizations(id),
    institution_role        TEXT,
    verified_at             TIMESTAMPTZ,
    verification_method     TEXT NOT NULL,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

On registration, system links previous handoff sessions by matching license_number.

### 19.2 Medical Professional Portal Pages

| Page | Functionality |
|------|--------------|
| Dashboard | Today's appointments with Bominal seniors, pending prescriptions, alerts |
| 내 환자 (My Patients) | Linked seniors via handoff history + institution roster |
| 환자 상세 (Patient Detail) | Full view (with active consent): demographics, medications, appointments, care plan, visits, observations |
| 진료 기록 (Clinical Notes) | SOAP notes, immutable after signing, addendum-only corrections |
| 처방 관리 (Prescriptions) | Create prescriptions with drug interaction checking |
| 검사 결과 (Lab Results) | Enter/view with reference ranges, abnormal flagging |
| 진료 의뢰 (Referrals) | Referrals to other institutions with record attachment |
| 문서 전송 (Document Transfer) | Inter-institution transfer with patient consent gate |
| 일정 (Schedule) | Appointment calendar for institution's Bominal seniors |

### 19.3 Pharmacy Portal Pages

| Page | Functionality |
|------|--------------|
| Dashboard | Dispensing queue, pending prescriptions, interaction alerts, prescription-expiring seniors |
| 조제 대기 (Dispensing Queue) | Prescriptions awaiting dispensing from linked prescribers |
| 조제 확인 (Dispensing Confirmation) | Confirm dispensed, record substitution, add 복약지도 notes |
| 환자 복약 현황 (Medication Status) | Adherence data, remaining days, expiry alerts |
| 약물 상호작용 (Drug Interactions) | Cross-check all active medications, flag interactions |
| 대체조제 기록 (Substitution Records) | Substitution log with prescriber notification tracking |
| 문서 전송 (Document Transfer) | Medication record exchange |

### 19.4 HIS-lite Module (Small Clinics Without Existing HIS)

#### 처방전달시스템 (OCS-lite)

```sql
CREATE TABLE prescriptions (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    patient_person_id   UUID NOT NULL REFERENCES person_profiles(id),
    prescriber_id       UUID NOT NULL REFERENCES users(id),
    institution_id      UUID NOT NULL REFERENCES provider_organizations(id),
    status              TEXT NOT NULL DEFAULT 'drafted',
    prescribed_at       TIMESTAMPTZ,
    signed_at           TIMESTAMPTZ,
    items               JSONB NOT NULL,
    diagnosis_codes     TEXT[],
    notes               TEXT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

Prescription auto-creates medication_ledger entries when `status = 'signed'`. Pharmacy receives in dispensing queue.

#### 진료기록 관리 (EMR-lite)

```sql
CREATE TABLE clinical_encounters (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    patient_person_id   UUID NOT NULL REFERENCES person_profiles(id),
    clinician_id        UUID NOT NULL REFERENCES users(id),
    institution_id      UUID NOT NULL REFERENCES provider_organizations(id),
    encounter_type      TEXT NOT NULL,
    encounter_date      TIMESTAMPTZ NOT NULL,
    subjective          TEXT,
    objective           TEXT,
    assessment          TEXT,
    plan                TEXT,
    diagnosis_codes     TEXT[],
    vitals              JSONB,
    signed_by           UUID REFERENCES users(id),
    signed_at           TIMESTAMPTZ,
    is_addendum         BOOLEAN NOT NULL DEFAULT false,
    addendum_to         UUID REFERENCES clinical_encounters(id),
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

Immutable after signing. Corrections create addendum entries only.

#### 수납/원무 (Billing-lite)

```sql
CREATE TABLE clinical_billing (
    id                      UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    encounter_id            UUID NOT NULL REFERENCES clinical_encounters(id),
    patient_person_id       UUID NOT NULL REFERENCES person_profiles(id),
    institution_id          UUID NOT NULL REFERENCES provider_organizations(id),
    insurance_claim_amount  DECIMAL(12,2),
    patient_copay_amount    DECIMAL(12,2),
    ltci_service_code       TEXT,
    ltci_claim_amount       DECIMAL(12,2),
    copayment_tier          TEXT NOT NULL DEFAULT 'standard',
    payment_status          TEXT NOT NULL DEFAULT 'pending',
    paid_at                 TIMESTAMPTZ,
    receipt_number          TEXT,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

#### 검사결과 (Lab Results)

```sql
CREATE TABLE lab_results (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    patient_person_id   UUID NOT NULL REFERENCES person_profiles(id),
    ordered_by          UUID NOT NULL REFERENCES users(id),
    institution_id      UUID NOT NULL REFERENCES provider_organizations(id),
    test_category       TEXT NOT NULL,
    test_name           TEXT NOT NULL,
    test_date           TIMESTAMPTZ NOT NULL,
    results             JSONB NOT NULL,
    interpretation      TEXT,
    reviewed_by         UUID REFERENCES users(id),
    reviewed_at         TIMESTAMPTZ,
    encounter_id        UUID REFERENCES clinical_encounters(id),
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

#### Patient Allergies

```sql
CREATE TABLE patient_allergies (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    patient_person_id   UUID NOT NULL REFERENCES person_profiles(id),
    allergen            TEXT NOT NULL,
    reaction_type       TEXT NOT NULL,
    severity            TEXT NOT NULL,
    noted_by            UUID NOT NULL REFERENCES users(id),
    noted_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    is_active           BOOLEAN NOT NULL DEFAULT true
);
```

### 19.5 Medical Document Transfer System

```sql
CREATE TABLE document_transfer_requests (
    id                          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    patient_person_id           UUID NOT NULL REFERENCES person_profiles(id),
    source_institution_id       UUID NOT NULL REFERENCES provider_organizations(id),
    source_type                 TEXT NOT NULL,
    dest_institution_id         UUID NOT NULL REFERENCES provider_organizations(id),
    dest_type                   TEXT NOT NULL,
    document_type               TEXT NOT NULL,
    document_references         UUID[],
    consent_record_id           UUID REFERENCES consent_records(id),
    patient_approved            BOOLEAN NOT NULL DEFAULT false,
    patient_approved_at         TIMESTAMPTZ,
    patient_approval_method     TEXT,
    status                      TEXT NOT NULL DEFAULT 'requested',
    requested_by                UUID NOT NULL REFERENCES users(id),
    requested_at                TIMESTAMPTZ NOT NULL DEFAULT now(),
    completed_at                TIMESTAMPTZ,
    expires_at                  TIMESTAMPTZ NOT NULL,
    transfer_hash               TEXT,
    created_at                  TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

**Transfer flow:**
1. Doctor requests transfer → selects patient, destination, document types
2. System checks active third_party_medical consent
3. If no consent: status `patient_pending` → senior receives notification → approves in-app or via PASS
4. If consent exists: goes to `approved`
5. Documents packaged and transferred
6. HIS-lite ↔ HIS-lite: direct within platform
7. HIS-lite ↔ External: via FHIR bundle
8. Integrity hash recorded, both parties + patient receive confirmation

**Patient-initiated transfer:** Senior or family can request from Family portal → "문서 관리" → requires PASS 본인인증.

### 19.6 Integration Layer for Existing HIS

```sql
CREATE TABLE his_integrations (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    institution_id      UUID NOT NULL REFERENCES provider_organizations(id),
    integration_type    TEXT NOT NULL,
    endpoint_url        TEXT NOT NULL,
    auth_method         TEXT NOT NULL,
    auth_credentials_encrypted  TEXT NOT NULL,  -- AES-256-GCM encrypted; key from env SECRET_ENCRYPTION_KEY
    status              TEXT NOT NULL DEFAULT 'testing',
    last_sync_at        TIMESTAMPTZ,
    sync_direction      TEXT NOT NULL DEFAULT 'bidirectional',
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE his_sync_log (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    integration_id  UUID NOT NULL REFERENCES his_integrations(id),
    direction       TEXT NOT NULL,
    resource_type   TEXT NOT NULL,
    resource_count  INT NOT NULL,
    status          TEXT NOT NULL,
    error_message   TEXT,
    synced_at       TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

**FHIR R4 resources exchanged:**

| FHIR Resource | Maps To | Direction |
|---------------|---------|-----------|
| Patient | person_profiles + senior_profiles | Bidirectional |
| MedicationRequest | prescriptions / medication_ledger | Bidirectional |
| MedicationDispense | medication_ledger (dispensing) | Inbound |
| Encounter | clinical_encounters | Bidirectional |
| Observation | lab_results / daily_observations | Bidirectional |
| DiagnosticReport | lab_results (grouped) | Inbound |
| DocumentReference | document_transfer_requests | Bidirectional |
| AllergyIntolerance | patient_allergies | Bidirectional |
| Condition | medical_history_entries | Bidirectional |
| CarePlan | care_plans | Outbound |
| ServiceRequest | institution_referrals | Bidirectional |

**마이헬스웨이 (My Healthway) integration:**

```sql
CREATE TABLE myhealthway_connections (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    patient_person_id   UUID NOT NULL REFERENCES person_profiles(id),
    myhealthway_token   TEXT NOT NULL,
    connected_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_sync_at        TIMESTAMPTZ,
    status              TEXT NOT NULL DEFAULT 'connected'
);
```

### 19.7 의료법 제23조 Compliance (HIS-lite)

| Requirement | Implementation |
|-------------|---------------|
| 전자서명 | Every encounter/prescription requires signed_at + signed_by |
| 진본성 (Authenticity) | Immutable after signing; addendum-only corrections |
| 무결성 (Integrity) | SHA256 hash chain via platform_events |
| 보존기간 | 진료기록 10 years, 처방전 2 years, 검사 5 years |
| 보안 시설/장비 | AES-256 at rest, TLS 1.3 in transit, access logged |
| 접근 통제 | RBAC + data ownership + policy engine |
| 감사 추적 | Every access logged in platform_events |

### 19.8 Updated Permission Matrix

| Actor | Prescriptions | Clinical Notes | Lab Results | Document Transfer | Dispensing |
|-------|--------------|----------------|-------------|-------------------|------------|
| Doctor (own institution) | Create, Sign, View | Create, Sign, View, Addend | Order, View, Interpret | Request, Approve (source) | No |
| Nurse (own institution) | View | View, Create (unsigned) | View, Enter vitals | View status | No |
| Pharmacist (own pharmacy) | View (for dispensing) | No | No | Request medication records | Confirm, Substitute, Record |
| Senior | View own (simplified) | View own (simplified) | View own | Approve/reject transfers | View status |
| Family | View linked (with consent) | No | View linked (with consent) | Initiate on behalf | No |
| Caregiver | View assigned (meds only) | No | No | No | No |
| Government | Aggregated de-identified | No | No | No | No |

---

## 20. Comprehensive Accountability & Observability

### 20.1 Unified Event Spine

Every action across the platform flows through a single event spine:

```sql
CREATE TABLE platform_events (
    id                    UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_id              TEXT NOT NULL UNIQUE,

    actor_id              UUID REFERENCES users(id),
    actor_role            user_role,
    actor_type            TEXT NOT NULL,
    actor_ip              INET,
    actor_user_agent      TEXT,
    actor_session_id      TEXT,

    proxy_license_type    TEXT,
    proxy_license_number  TEXT,
    proxy_institution     TEXT,

    action                TEXT NOT NULL,
    entity_type           TEXT NOT NULL,
    entity_id             UUID,

    parent_event_id       UUID REFERENCES platform_events(id),
    correlation_id        UUID,

    before_state          JSONB,
    after_state           JSONB,
    diff                  JSONB,
    metadata              JSONB,

    sensitivity           TEXT NOT NULL,
    category              TEXT NOT NULL,

    event_hash            TEXT NOT NULL,  -- SHA256 of: action + entity_type + entity_id + actor_id + after_state + created_at
    previous_hash         TEXT,          -- hash of the previous event for the SAME entity_type + entity_id (per-entity chain, not global)

    created_at            TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_events_actor ON platform_events(actor_id, created_at DESC);
CREATE INDEX idx_events_entity ON platform_events(entity_type, entity_id, created_at DESC);
CREATE INDEX idx_events_action ON platform_events(action, created_at DESC);
CREATE INDEX idx_events_correlation ON platform_events(correlation_id) WHERE correlation_id IS NOT NULL;
CREATE INDEX idx_events_category ON platform_events(category, created_at DESC);
CREATE INDEX idx_events_sensitivity ON platform_events(sensitivity, created_at DESC);
```

**This table is NEVER updated. NEVER deleted. Append-only.**

### 20.2 Exhaustive Event Tracking

**Categories tracked:**

- **Clinical:** Medication CRUD, prescription signing/cancellation, clinical encounters, lab results, drug interactions, generic substitution, document transfers, allergy recording
- **Care Operations:** Visit scheduling/check-in/check-out, duration anomaly flagging, task completion, care plan changes, observations, incident CRUD
- **Senior Safety:** Wellness check-in/miss/escalation, emergency SOS trigger/resolve, medication escalation, prescription expiry alerts
- **Access & Identity:** Login/logout (all methods), PASS verification, handoff session start/end, data access for another person, data export
- **Consent & Policy:** Consent grant/revoke/expire, access policy changes, document transfer consent
- **Financial:** Co-payment calculation, payment processing, insurance claims, billing records
- **Administrative:** Account CRUD, role changes, provider org updates, credential verification, HIS integration sync
- **System:** Health checks, integrity verification, metric recording

### 20.3 Transparency Dashboards

**Senior — "내 기록 열람 이력":** Every person who viewed, modified, or received the senior's data. Flagging button for suspicious access.

**Caregiver — "내 활동 기록":** All visit actions, medication entries, GPS data, task completions, performance metrics (평균 방문 시간, 정시 도착률, 업무 완료율).

**Provider — "기관 활동 대시보드":** Real-time feed of all organization events. Filter by category, actor, severity, date. Export capability.

**Government — "감사 조회":** De-identified event log. Filter by period, institution, type, severity. Drill-down with legal authority. NHIS inspector export format.

**Medical Professional — "환자 접근 이력":** Every patient record access logged with timestamp, consent reference, what was viewed.

### 20.4 Alerting & Anomaly Detection

```sql
CREATE TABLE alert_rules (
    id                UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    institution_id    UUID REFERENCES provider_organizations(id),
    rule_name         TEXT NOT NULL,
    condition         JSONB NOT NULL,
    severity          TEXT NOT NULL,
    recipients        JSONB NOT NULL,
    cooldown_minutes  INT NOT NULL DEFAULT 60,
    enabled           BOOLEAN NOT NULL DEFAULT true,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE alert_firings (
    id                      UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    rule_id                 UUID NOT NULL REFERENCES alert_rules(id),
    triggered_by_event_id   UUID NOT NULL REFERENCES platform_events(id),
    severity                TEXT NOT NULL,
    message                 TEXT NOT NULL,
    acknowledged            BOOLEAN NOT NULL DEFAULT false,
    acknowledged_by         UUID REFERENCES users(id),
    acknowledged_at         TIMESTAMPTZ,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

**Built-in alert rules (non-deletable seed data):**

| Rule | Condition | Severity |
|------|-----------|----------|
| Visit duration < 50% scheduled | visit.check_out AND duration_ratio < 0.5 | warning |
| Visit duration < 25% scheduled | visit.check_out AND duration_ratio < 0.25 | alert |
| Check-in > 200m from address | visit.check_in AND distance > 200 | warning |
| 3+ consecutive medication misses | medication.missed AND consecutive >= 3 | alert |
| Wellness check-in missed past escalation | wellness.missed_escalation | alert |
| Emergency SOS triggered | emergency.triggered | critical |
| Failed login attempts > 5 in 10min | auth.failed AND count > 5 | critical |
| Data access without treatment relationship | access.read AND no_relationship | alert |
| Policy change made | policy.changed | warning |
| Credential expiring within 30 days | credential.expiring AND days <= 30 | warning |
| Prescription expiring within 5 days | prescription.expiring AND days <= 5 | info |
| Generic substitution notification overdue | substitution.notification_overdue | alert |
| PIPA consent expiring within 30 days | consent.expiring AND days <= 30 | warning |
| Document transfer pending > 48 hours | transfer.pending AND hours > 48 | warning |
| Lab result critical value | lab.result AND flag = CRITICAL | critical |
| HIS sync failure | his.sync_failed | alert |
| Unusual data access pattern (> 2σ from normal) | Statistical anomaly detection | warning |

### 20.5 Integrity Verification

**Hash chain is per-entity, not global.** Each `(entity_type, entity_id)` pair has its own chain. This avoids concurrency issues — multiple Axum workers can write events for different entities simultaneously without contention. For events on the SAME entity, use a PostgreSQL advisory lock keyed on `entity_id` to serialize chain maintenance:

```rust
// Pseudocode for event insertion with chain maintenance
let lock_key = entity_id.as_u128() as i64;
sqlx::query!("SELECT pg_advisory_xact_lock($1)", lock_key).execute(&mut *tx).await?;
let previous = sqlx::query_scalar!("SELECT event_hash FROM platform_events WHERE entity_type = $1 AND entity_id = $2 ORDER BY created_at DESC LIMIT 1", entity_type, entity_id).fetch_optional(&mut *tx).await?;
let hash_input = format!("{}{}{}{}{}{}", action, entity_type, entity_id, actor_id, serde_json::to_string(&after_state)?, created_at.to_rfc3339());
let event_hash = sha256::digest(hash_input);
// Insert with event_hash and previous_hash
```

```sql
CREATE TABLE integrity_verifications (
    id                UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    verification_date DATE NOT NULL,
    entity_type       TEXT NOT NULL,
    total_chains      INT NOT NULL,
    valid_chains      INT NOT NULL,
    broken_chains     INT NOT NULL,
    broken_chain_ids  UUID[],
    verified_at       TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

Nightly verification job. Broken chain → CRITICAL alert to platform admin.

### 20.6 System Metrics

```sql
CREATE TABLE system_metrics (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    metric_name   TEXT NOT NULL,
    metric_value  DOUBLE PRECISION NOT NULL,
    dimensions    JSONB,
    recorded_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE MATERIALIZED VIEW system_health_hourly AS
SELECT
    date_trunc('hour', recorded_at) as hour,
    metric_name,
    AVG(metric_value) as avg_val,
    percentile_cont(0.99) WITHIN GROUP (ORDER BY metric_value) as p99,
    COUNT(*) as sample_count
FROM system_metrics
GROUP BY 1, 2;
```

Platform admin dashboard: API latency per endpoint, error rate, active sessions, DB performance, WASM load time, event processing lag, storage per table, HIS sync health.

### 20.7 Compliance Reporting (Automated Monthly)

| Report | Recipients | Contents |
|--------|-----------|----------|
| PIPA 개인정보 처리 현황 | 개인정보보호책임자 | Consent counts, access counts, revocations, retention compliance |
| LTCI 급여 제공 현황 | Provider admin, NHIS | Visit hours by service, caregiver utilization, beneficiary counts, claims |
| 의료진 활동 보고 | Institution admin | Encounters, prescriptions signed, lab results, transfers |
| 보안 감사 보고 | Security admin | Login attempts, policy changes, anomalies, integrity verification |
| 요양보호사 성과 보고 | Provider admin | Punctuality, duration compliance, task completion, incidents |
| 환자 안전 보고 | Provider admin | SOS events, medication escalations, wellness compliance, incidents |

```sql
CREATE TABLE compliance_reports (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    report_type     TEXT NOT NULL,
    report_period   TEXT NOT NULL,
    institution_id  UUID REFERENCES provider_organizations(id),
    generated_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    data            JSONB NOT NULL,
    pdf_url         TEXT,
    reviewed_by     UUID REFERENCES users(id),
    reviewed_at     TIMESTAMPTZ
);
```

### 20.8 Data Provenance — Full Chain of Custody

For any piece of data, the complete chain is reconstructable from platform_events:

```
medication_id: abc-123 chain:
  1. Created by 김의사 (의사면허 #12345) via handoff at 서울내과의원
  2. Dispensed by 박약사 (약사면허 #67890) via handoff at 건강약국
  3. Handoff sessions linked to registered accounts
  4. Check-off: TAKEN by senior at 08:05 (scheduled 08:00)
  5. Reminder sent (push), Snooze sent, Window expired: MISSED
  6. Consecutive miss → escalation → caregiver notified
  7. Caregiver confirmed taken late via phone
  8. Remaining quantity alert at 5 days
  9. Prescription expiry alert → auto-suggest appointment
```

Every link in the chain is an immutable platform_event with actor, timestamp, context, and hash.

---

## New Tables Summary

| Table | Section | Purpose |
|-------|---------|---------|
| medical_handoff_sessions | 2 | Device handoff session tracking |
| access_policies | 4 | Configurable access control |
| policy_change_log | 4 | Policy change audit trail |
| medication_ledger | 5 | Immutable medication records |
| appointment_ledger | 5 | Immutable appointment records |
| care_plan_ledger | 5 | Immutable care plan records |
| wellness_checkins | 8 | Daily wellness check-ins |
| wellness_check_config | 8 | Check-in escalation configuration |
| emergency_events | 8 | Emergency SOS events |
| medication_escalation_config | 8 | Consecutive miss escalation config |
| visit_evidence | 9 | Photo evidence for visit tasks |
| care_plan_handoff_notes | 9 | Substitute caregiver notes |
| daily_care_summaries | 10 | End-of-day family summaries |
| benefit_utilization | 15 | Monthly LTCI benefit tracking |
| korean_holidays | 15 | Public holiday calendar |
| seasonal_alerts | 15 | Seasonal health alerts |
| community_activities | 15 | 경로당 / community activities |
| medication_reminders | 17 | Per-medication reminder config |
| identity_verifications | 18 | PASS 본인인증 records |
| generic_substitution_records | 18 | 대체조제 tracking |
| data_retention_policies | 18 | Per-category retention config |
| medical_professional_profiles | 19 | Doctor/nurse/pharmacist profiles |
| prescriptions | 19 | OCS-lite prescription orders |
| clinical_encounters | 19 | EMR-lite SOAP notes |
| clinical_billing | 19 | Billing/copay tracking |
| lab_results | 19 | Lab test results |
| patient_allergies | 19 | Allergy/intolerance records |
| document_transfer_requests | 19 | Inter-institution transfer |
| his_integrations | 19 | External HIS connections |
| his_sync_log | 19 | HIS sync audit trail |
| myhealthway_connections | 19 | 마이헬스웨이 patient links |
| platform_events | 20 | Unified event spine |
| alert_rules | 20 | Configurable alert rules |
| alert_firings | 20 | Alert firing history |
| integrity_verifications | 20 | Hash chain verification log |
| system_metrics | 20 | Platform health metrics |
| compliance_reports | 20 | Automated compliance reports |

**Total new tables: 37**
**Total new API routes: ~60**
**Total modified handlers: ~50**
**Total new portal pages: ~50+**
**New migration: 0004 (schema additions + indexes + enum changes)**

---

## New Enum Types Summary

| Enum | Values |
|------|--------|
| handoff_license_type | doctor, nurse, pharmacist |
| consent_purpose_v2 | personal_info_collection, sensitive_info_processing, third_party_medical, third_party_government, third_party_provider, third_party_family, third_party_caregiver, marketing |
| copayment_tier | exempt, reduction_60, reduction_40, standard |
| credential_classification | license, qualification |
| care_level_enum | level_1, level_2, level_3, level_4, level_5, cognitive |
| internal_permission_level | staff, manager, security_admin, org_admin |
| wellness_mood | good, okay, not_great, need_help |
| emergency_event_status | triggered, responders_notified, resolved, false_alarm |

### Modified Existing Enums

| Enum | Change |
|------|--------|
| user_role | Add `PharmacyStaff` variant — separate from `MedicalStaff` to distinguish portal access and permissions. Pharmacists registering get `PharmacyStaff` role; doctors/nurses get `MedicalStaff`. |
| notification_type | Add `Emergency` variant — bypasses do-not-disturb, used for SOS events |

### Deprecated/Removed Enums

| Enum | Action |
|------|--------|
| consent_purpose (old) | Replaced by consent_purpose_v2 after migration (see Section 18.2) |

### Role Clarification: PartnerOperator

The existing `PartnerOperator` role is retained for third-party service partners (식사배달, 이동서비스 etc.) who integrate with the platform. They have read-only access to assigned senior service requests and can update delivery/service status. No portal changes in this spec — existing functionality is sufficient. Future spec may expand this role.

---

## Regulatory References

- [개인정보 보호법 (PIPA)](https://www.law.go.kr/lsEfInfoP.do?lsiSeq=195062)
- [개인정보보호법 시행령](https://www.law.go.kr/LSW/lsInfoP.do?lsId=011468)
- [약사법 제27조 (대체조제)](https://www.health.kr/drug_info/basedrug/law27.html)
- [의료법 제23조 (전자의무기록)](https://lbox.kr/v2/statute/의료법/본문%20>%20제2장%20>%20제2절%20>%20제23조)
- [노인장기요양보험법](https://www.longtermcare.or.kr/npbs/)
- [주민등록번호 수집 금지 가이드라인](https://www.privacy.go.kr/cmm/fms/FileDown.do?atchFileId=FILE_000000000815216&fileSn=0)
- [2026년도 장기요양보험료율](http://www.mohw.go.kr/board.es?mid=a10503000000&bid=0027&list_no=1487817&act=view)
- [본인부담금 감경 기준](https://www.easylaw.go.kr/CSP/CnpClsMain.laf?csmSeq=2038&ccfNo=3&cciNo=3&cnpClsNo=1)
