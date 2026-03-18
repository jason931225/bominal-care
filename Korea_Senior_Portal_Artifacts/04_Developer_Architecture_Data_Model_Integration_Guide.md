# Korea Senior Portal — Developer Architecture, Data Model, and Integration Guide

## 1. Architecture thesis
Build one platform with a shared identity, consent, and event layer, then hang multiple role-specific experiences from it.

Suggested monorepo:
- apps/web-senior
- apps/web-family
- apps/web-provider
- apps/web-government
- apps/mobile-caregiver
- packages/db
- packages/ui
- packages/auth
- packages/events
- packages/integrations
- packages/types

Recommended stack:
- Next.js + TypeScript
- PostgreSQL
- Prisma
- Auth.js / Clerk / equivalent
- Zod validation
- Background jobs for event processing
- Object storage for documents
- Managed message queue / event bus

## 2. Core bounded contexts
### Identity & Delegation
Users, profiles, guardianship, delegated access, KYC level, consent.

### Discovery & Matching
Directories, search, auto-match, rankings, service regions, schedules.

### Care Operations
Care plans, visits, tasks, daily observations, incidents, alerts.

### Medical Coordination
Medical history, medication list, appointments, referrals, nursing hospital / clinic coordination.

### Benefits Overlay
Eligibility cases, approval steps, subsidy program rules, claim/subsidy records.

### Daily Observability
Event ingestion, rule evaluation, audience-specific dashboards, notifications.

### Opportunities / Housing
Jobs, community programs, housing / silver-town listings, tours, waitlists.

## 3. Key state machines
### CaregiverApplication
draft -> submitted -> identity_verified -> credential_review -> approved_private_pay OR approved_under_provider -> suspended OR rejected

### EligibilityCase
not_started -> screening -> docs_missing -> under_review -> approved OR denied -> appealed -> final

### MatchRequest
created -> searching -> recommendations_ready -> selected -> booked -> fulfilled OR cancelled

### Visit
scheduled -> caregiver_acknowledged -> in_progress -> completed OR missed OR cancelled

### MedicationEvent
scheduled -> reminder_sent -> taken OR missed OR held -> escalated_if_needed

### InstitutionReferral
created -> accepted -> booked -> attended -> discharged OR closed

## 4. Minimal table blueprint
- users
- person_profiles
- senior_profiles
- family_relationships
- consent_records
- provider_organizations
- caregiver_applications
- caregiver_credentials
- service_regions
- availability_slots
- service_types
- match_requests
- match_recommendations
- care_plans
- visits
- daily_observations
- medical_history_entries
- medications
- medication_schedules
- medication_events
- appointments
- institution_referrals
- incidents
- observability_signals
- notifications
- eligibility_cases
- approval_steps
- claim_or_subsidy_records
- audit_logs

Every sensitive row should carry:
- tenant_id
- subject_person_id
- created_at / updated_at
- created_by / updated_by
- visibility / purpose metadata where relevant

## 5. Access-control model
Dimensions:
- role
- organization
- purpose
- relationship to senior
- assignment scope
- consent scope

Examples:
- caregiver sees only assigned client data required for service delivery
- family sees data delegated by senior/guardian setting
- government reviewer sees program-relevant case data, not full consumer marketplace data
- provider supervisors see staff and client data within their org scope
- medical institutions see medical-share data only when consent exists

## 6. API surface
### Identity
/auth/*
/profiles/*
/consents/*
/delegations/*

### Discovery
/providers/*
/caregivers/*
/institutions/*
/housing/*
/opportunities/*
/match-requests/*
/match-recommendations/*

### Operations
/care-plans/*
/visits/*
/daily-observations/*
/incidents/*
/notifications/*

### Medical
/medical-history/*
/medications/*
/medication-events/*
/appointments/*
/referrals/*

### Benefits
/eligibility-cases/*
/approval-steps/*
/subsidy-records/*

### Oversight
/analytics/*
/observability/*
/government/reports/*
/audit-logs/*

## 7. Observability design
Use an append-only event table or event bus.
Events:
- visit.completed
- visit.missed
- medication.taken
- medication.missed
- meal.delivered
- meal.failed
- transport.completed
- transport.failed
- symptom.reported
- incident.created
- eligibility.status_changed
- referral.updated

Rule engine examples:
- if 2+ missed medication events in 48h -> family + caregiver supervisor alert
- if no visit check-in within grace window -> supervisor alert
- if symptom severity high and medical-share consent exists -> medical review queue
- if subsidy case missing docs for 7 days -> family reminder + case-worker task

## 8. Integration surface implementation notes
### Immediate
- identity verification
- payment processor
- Kakao/SMS/IVR messaging
- map/routing provider
- document/e-signature provider
- My Healthway adapter boundary (even if mocked first)
- NHIS/SSIS adapter boundary (even if manual import/export in MVP)

### Near-term
- hospital EMR integration
- pharmacy integration
- jobs/community listings feeds
- housing partner feeds
- devices/RPM

Build integrations as connectors with:
- auth strategy
- inbound webhook handler
- outbound sync job
- field mapping config
- audit trail
- retry / dead letter handling

## 9. UI principles
- low-cognitive-load default for senior surface
- timeline-first for family surface
- task-first for caregiver mobile
- roster + exceptions for provider surface
- queue + filters + exports for government surface

## 10. Seed demo scenario
Create:
- one senior
- one family member
- one caregiver applicant
- one approved caregiver
- one home-care provider org
- one nursing hospital
- one clinic
- one district office program
- one silver-town listing
- one job opportunity
- one eligibility case
- one care plan
- one medicine schedule
- one missed medication event
- one visit
- one observability alert

## 11. MVP trap to avoid
Do not force the user into a subsidy workflow too early. The portal must stay useful before eligibility, outside eligibility, and after eligibility.