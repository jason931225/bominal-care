# Bominal Care — Two-Pillar Product Spec & Implementation Plan

**Date:** 2026-03-22
**Status:** Active

## Product Vision

Bominal Care is expanding from a single-product senior care portal into **two distinct product pillars** sharing a common platform:

1. **시니어 라이프스타일 플랫폼 (Senior Lifestyle Platform)** — consumer-facing portal for seniors, families, caregivers, and providers
2. **의료정보시스템 (Medical Information System / HIS-lite)** — medical-grade system for clinics, pharmacies, and hospitals that lack existing HIS

Both products share identity, consent, and event infrastructure but have different regulatory requirements, user bases, and go-to-market timelines.

---

## Architecture

```
+------------------------------------------------------------------+
|                    Shared Platform Layer                          |
|  Identity . Consent . Event Spine . RBAC . Audit . Notifications |
|  PostgreSQL . Axum API . Auth (WebAuthn/OAuth/PASS)              |
+----------+-----------------------------------+-------------------+
           |                                   |
    +------v----------------------+  +---------v---------------------+
    |  Pillar 1: Lifestyle        |  |  Pillar 2: Medical Info Sys   |
    |                             |  |                               |
    |  Senior Portal              |  |  Medical Portal (Doctor/Nurse)|
    |  Family Portal              |  |  Pharmacy Portal (Pharmacist) |
    |  Caregiver Portal           |  |  HIS-lite (OCS/EMR)          |
    |  Provider Portal            |  |  FHIR Integration            |
    |  Government Portal          |  |  Lab Results                 |
    |                             |  |  Document Transfer           |
    |  Regulatory: PIPA           |  |  Regulatory: PIPA +          |
    |  + LTCI Act                 |  |  Medical Act + Pharmacy Act  |
    +-----------------------------+  +-------------------------------+
```

### Why Two Pillars

| Dimension | Pillar 1: Lifestyle | Pillar 2: HIS-lite |
|-----------|--------------------|--------------------|
| Target user | Seniors, families, caregivers, providers, government | Doctors, nurses, pharmacists at small clinics |
| Regulatory | PIPA, LTCI Act | PIPA + Medical Act Art. 23 + Pharmacy Act |
| Data immutability | Append-preferred, soft-delete OK | Strictly immutable after signing, addendum-only |
| Audit standard | Platform events, compliance reports | Hash-chain integrity, 10-year medical retention |
| MVP scope | Consumer adoption first | Institutional adoption, requires certification |
| Revenue model | SaaS per-provider, freemium for consumers | Per-institution license |

---

## Tech Stack

| Component | Technology |
|-----------|-----------|
| Language | Rust (2024 edition) |
| Frontend | Leptos 0.8 CSR (WASM) |
| API Server | Axum 0.8 |
| Database | PostgreSQL + sqlx (runtime queries) |
| Auth | WebAuthn-rs + OAuth2 + Demo login |
| Sessions | tower-sessions + PostgreSQL store |
| Types | 27 enums, 32+ model structs, RBAC matrix, state machines |
| Deployment | Firebase Hosting (CDN) + Cloud Run (asia-northeast3 Seoul) |
| CI/CD | GitHub Actions + Docker multi-stage build |

---

## Implementation Phases

### Phase 0: Fix Build Infrastructure [DONE]
- Removed mold linker from .cargo/config.toml
- Fixed Cloud Run PORT env var in server
- Fixed Dockerfile (removed nonexistent .sqlx/)
- Fixed CI pipeline
- Added .env.example

### Phase 1: Wire Pillar 1 Pages to API
All backend routes exist. Wire 18 frontend pages from static demo data to real API calls.

**Senior Portal (1 file):** services.rs
**Caregiver Portal (4 files):** schedule.rs, clients.rs, tasks.rs, apply.rs
**Family Portal (7 files):** approvals.rs, care.rs, documents.rs, eligibility.rs, help.rs, matching.rs, payments.rs

### Phase 2: Wire Safety Infrastructure
- RBAC ownership enforcement in ~38 handlers
- Event emission in ~50 handlers
- State machine validation in 8 handlers
- i18n error messages

### Phase 3: Wire Pillar 2 (Medical)
**Medical Portal (5 files):** mod.rs, patients.rs, prescriptions.rs, appointments.rs, history.rs
**New Pharmacy Portal:** ~5 pages under /pharmacy/*

### Phase 4: Regulatory Compliance
- Remove 주민번호 input (PIPA violation)
- Wire PIPA granular consent
- Wire copayment tiers
- Wire credential classification

### Phase 5: Deployment
- Docker build verification
- Cloud SQL provisioning
- Firebase Hosting + Cloud Run deploy
- Seed data creation

---

## Key Decisions

1. **CSR over SSR**: Leptos runs client-side (WASM) with Trunk, not server-side rendered. API calls go from browser to Axum backend.
2. **No sqlx compile-time macros**: All queries use runtime `sqlx::query()`/`sqlx::query_as()`. Only `sqlx::migrate!` is used (embeds migrations at compile time).
3. **Firebase Hosting + Cloud Run**: Static WASM assets served via Firebase CDN, API requests proxied to Cloud Run backend.
4. **Seoul region**: All infrastructure in asia-northeast3 for Korean data residency.
