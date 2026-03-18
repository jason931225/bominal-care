# Korea Senior Portal — Agentic LLM Build Prompt

You are a senior full-stack engineer, solution architect, and pragmatic startup CTO. Build a working **Korea Senior Portal**: a comprehensive portal for all things senior, with care coordination and benefits as the operational core.

## Product thesis
Build one platform that supports:
- seniors
- family members / guardians
- prospective caregivers
- approved caregivers
- provider organizations
- nursing hospitals / clinics / pharmacies / elder-focused medical institutions
- transport / meal / partner vendors
- district offices / municipalities / government reviewers
- internal platform admins

This is **not** a subsidy-only product and **not** just a caregiver marketplace.

### Critical business rule
**Users can use the service without eligibility.**
Eligibility is required only when the user wants:
- subsidized programs
- public-program routing
- government review
- claim / voucher / benefit workflows

The portal must therefore support:
1. open consumer mode
2. subsidized care mode
3. medical coordination mode
4. government oversight mode

## Korea-specific guardrails
- Keep institution accountability for reimbursed long-term-care flows. Do not model subsidized care as an unrestricted freelancer marketplace.
- Add a consent engine so a user can choose to share selected information with:
  - medical institutions
  - government institutions
  - both
  - neither
- Model nursing hospitals (요양병원) and other elder-related medical institutions as medical entities, not generic vendors.
- Include **medical history + medicine tracking** as core modules.
- Support local-government / district-office information and benefit routing.
- Add **daily observability** for government, caregivers, care recipients, and families.
- Treat telemedicine as a bounded coordination feature, not a generic diagnosis marketplace.
- Use role-based and purpose-based data access.

## Required user-facing surfaces
### 1. Senior portal
- Today dashboard
- Appointments
- Care schedule
- Medication reminders and logs
- Medical history summary
- Rides / meals / partner services
- Housing / silver-town search
- Jobs / volunteering / community opportunities
- Emergency help
- Consent center

### 2. Family portal
- Shared timeline
- Notifications / incident feed
- Matching / provider comparison
- Payment visibility
- Documents
- Approval workflows
- Delegated help for senior
- Observability dashboard
- Medication oversight

### 3. Caregiver applicant portal
Must include application flow for review:
- identity verification
- certification verification
- service region
- service schedule
- types of service
- references / interview / review status
- approval / rejection / suspend state

### 4. Approved caregiver app
- client schedule
- relevant client information only
- care plan access
- check-in / check-out
- tasks
- notes
- incident capture
- medication / observation entry
- observability event submission

### 5. Provider organization portal
- client roster
- caregiver roster
- care plans
- schedules
- partner referrals
- quality review
- claims-ready service records
- compliance tracking
- housing / medical / partner coordination

### 6. Medical institution portal
- nursing hospital referrals
- clinic referrals
- medication coordination
- medical history review (with consent)
- discharge planning
- transfer tracking
- appointment coordination
- telehealth coordination hooks

### 7. Government / district-office portal
- provider registry
- subsidy eligibility workflow
- manual / automatic approval control
- incident and complaint dashboard
- waitlist visibility
- district information cards
- observability dashboards
- audit exports
- program configuration

### 8. Partner operations portals
- transport
- meals
- home services
- housing / silver-town operators
- jobs / community opportunities providers

## Database requirements
Use PostgreSQL + Prisma unless you have a strong reason otherwise.

Must include at minimum:
- Tenant
- User
- PersonProfile
- SeniorProfile
- FamilyRelationship
- ConsentRecord
- EligibilityCase
- ApprovalStep
- ProviderOrganization
- CaregiverApplication
- CaregiverCredential
- ServiceRegion
- AvailabilitySlot
- ServiceType
- MatchRequest
- MatchRecommendation
- CarePlan
- Visit
- DailyObservation
- MedicalHistoryEntry
- Medication
- MedicationSchedule
- MedicationEvent
- Appointment
- InstitutionReferral
- Incident
- ObservabilitySignal
- Notification
- ClaimOrSubsidyRecord
- AuditLog

## Matching logic requirements
Support search and auto-match across:
- visiting caregivers
- provider organizations
- nursing hospitals
- clinics / rehab / dementia programs
- pharmacies
- transport / meal partners
- silver towns / housing
- jobs / opportunities

Matching inputs should include:
- district / service region
- schedule
- service type
- subsidy eligibility status
- language
- gender preference
- dementia experience
- mobility support skills
- smoking / pet preferences
- overnight availability
- family constraints
- need for medical coordination

## Consent requirements
Implement purpose-bound consent:
- no sharing
- medical-share only
- government-share only
- both
- revocable
- time-limited
- auditable

## Observability requirements
Implement a rules/event layer.
Capture and expose:
- visit completed / missed
- medication taken / missed
- meal delivered / missed
- transport completed / failed
- symptom reported
- abnormal reading
- incident filed
- housing / transfer step updated
- subsidy case status changed

Views:
- senior
- family
- caregiver
- supervisor
- government reviewer

## Build output
Return:
1. product brief
2. system architecture
3. RBAC model
4. DB schema
5. API routes
6. monorepo structure
7. frontend apps
8. seed data
9. setup instructions
10. deployment instructions
11. roadmap

## Non-negotiables
- Do not block basic use behind eligibility.
- Do not skip medical history or medicine tracking.
- Do not skip government portal or district-office information.
- Do not skip caregiver application review flow.
- Do not skip nursing hospital / medical institution connection.
- Do not skip daily observability.
- Do not treat all data as equally visible to all roles.
- Do not build only wireframes.

Ship a real MVP foundation.