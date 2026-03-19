# Stack Rewrite: Rust + Leptos

## Overview

Full rewrite of the bominal-care platform from TypeScript/Next.js to Rust. Single Leptos SSR application with Axum, WASM client hydration, passkey-first auth, PostgreSQL 18 with RLS, and Tailwind CSS 4.

## Current State

| Component | Tech |
|---|---|
| Language | TypeScript |
| Framework | Next.js 15.5 (5 separate apps) |
| Runtime | Node.js 24 |
| Styling | Tailwind CSS 3.4 |
| DB | PostgreSQL 16, raw `pg` driver |
| Auth | next-auth 5 beta (credentials) |
| Build | Turborepo + pnpm |
| Deploy | Vercel (planned) |

## Target State

| Component | Tech |
|---|---|
| Language | Rust |
| Framework | Leptos 0.7+ (SSR + WASM hydration) |
| HTTP server | Axum |
| Styling | Tailwind CSS 4 |
| DB | PostgreSQL 18, sqlx (compile-time checked) |
| Auth | WebAuthn-rs (passkeys) + custom OAuth |
| Build | Cargo workspace |
| Deploy | Fly.io / Railway / Docker on VPS |

## 1. Project Structure

```
bominal-care/
├── Cargo.toml                  ← workspace root
├── crates/
│   ├── app/                    ← Leptos SSR app (frontend + server)
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── app.rs          ← root App component
│   │   │   ├── routes/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── auth/       ← signin, passkey registration
│   │   │   │   ├── senior/     ← elderly user pages
│   │   │   │   ├── family/     ← family member pages
│   │   │   │   ├── internal/   ← facility staff pages
│   │   │   │   ├── government/ ← government reviewer pages
│   │   │   │   └── caregiver/  ← caregiver pages
│   │   │   ├── components/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── shells/     ← per-role app shells/layouts
│   │   │   │   └── shared/     ← cross-role UI components
│   │   │   └── layouts/
│   │   │       ├── root.rs     ← <html>, <head>, Tailwind, WASM bootstrap
│   │   │       ├── senior.rs
│   │   │       ├── family.rs
│   │   │       ├── internal.rs
│   │   │       ├── government.rs
│   │   │       └── caregiver.rs
│   │   └── Cargo.toml
│   ├── server/                 ← Axum server binary
│   │   ├── src/
│   │   │   ├── main.rs         ← entry point, Axum router
│   │   │   ├── auth/           ← passkey + OAuth handlers
│   │   │   ├── api/            ← REST API routes
│   │   │   ├── middleware/     ← auth guard, RLS context
│   │   │   └── state.rs       ← AppState (db pool, config)
│   │   └── Cargo.toml
│   ├── db/                     ← database layer
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── models.rs       ← sqlx FromRow structs
│   │   │   ├── queries/        ← per-domain query modules
│   │   │   ├── rls.rs          ← RLS context helper
│   │   │   └── seed.rs         ← seed data
│   │   ├── migrations/         ← sqlx migrations (schema + RLS policies)
│   │   └── Cargo.toml
│   └── types/                  ← shared types (roles, enums, DTOs)
│       ├── src/lib.rs
│       └── Cargo.toml
├── style/
│   └── main.css                ← Tailwind 4 entry (@import "tailwindcss")
├── docker/
│   ├── Dockerfile              ← multi-stage Rust build
│   └── docker-compose.yml      ← PostgreSQL 18 + Redis
├── .sqlx/                      ← sqlx offline query cache (for CI builds)
└── tailwind.config.ts          ← not needed in v4, CSS-only config
```

## 2. Leptos SSR + WASM Hydration

### How It Works

- Server renders full HTML on first request (SSR via Axum)
- Client receives HTML + a small WASM binary
- WASM hydrates the page — attaches event handlers, enables interactivity
- Subsequent navigation is client-side (SPA-like) via Leptos router
- Server functions (`#[server]`) handle API calls — no separate REST API needed for UI

### Routing

```rust
#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "Not found">
                // Auth (public)
                <Route path="/auth/signin" view=SignInPage />

                // Role-based route groups
                <ParentRoute path="/" view=SeniorLayout>
                    <Route path="/" view=SeniorDashboard />
                    <Route path="/appointments" view=Appointments />
                    <Route path="/medications" view=Medications />
                    // ...
                </ParentRoute>

                <ParentRoute path="/family" view=FamilyLayout>
                    <Route path="/" view=FamilyDashboard />
                    <Route path="/timeline" view=Timeline />
                    // ...
                </ParentRoute>

                <ParentRoute path="/internal" view=InternalLayout>
                    <Route path="/" view=InternalDashboard />
                    <Route path="/clients" view=Clients />
                    // ...
                </ParentRoute>

                <ParentRoute path="/gov" view=GovernmentLayout>
                    <Route path="/" view=GovernmentDashboard />
                    <Route path="/providers" view=Providers />
                    // ...
                </ParentRoute>

                <ParentRoute path="/caregiver" view=CaregiverLayout>
                    <Route path="/" view=CaregiverDashboard />
                    <Route path="/schedule" view=Schedule />
                    // ...
                </ParentRoute>
            </Routes>
        </Router>
    }
}
```

### Server Functions

Replace REST API routes with Leptos server functions:

```rust
#[server(GetAppointments)]
async fn get_appointments() -> Result<Vec<Appointment>, ServerFnError> {
    let user = extract_user().await?;
    let db = extract_db().await?;
    db::queries::appointments::list_for_user(&db, &user).await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}
```

Server functions are called from components like normal async functions but execute on the server. No manual fetch/axios needed.

## 3. Database: sqlx + PostgreSQL 18

### Why sqlx (not Diesel)

- Compile-time query verification against the real database
- No DSL to learn — write actual SQL
- Async-native (works with Tokio/Axum)
- Supports migrations
- Lighter than Diesel (no schema.rs codegen)

### Models

```rust
#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub role: UserRole,
    pub kyc_level: KycLevel,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### Queries

```rust
// Compile-time checked against the database
pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_optional(pool)
        .await
}
```

### Migrations

sqlx migrations in `crates/db/migrations/`:
- `0001_initial_schema.sql` — all tables
- `0002_rls_policies.sql` — RLS policies
- `0003_seed_data.sql` — demo data

Run via `sqlx migrate run` or programmatically at startup.

## 4. Row-Level Security (RLS)

### Session Context

Axum middleware sets PostgreSQL session variables on each request:

```rust
pub async fn with_rls<T, F>(
    pool: &PgPool,
    user_id: Uuid,
    role: UserRole,
    tenant_id: Option<Uuid>,
    f: F,
) -> Result<T>
where
    F: FnOnce(&mut PgConnection) -> BoxFuture<'_, Result<T>>,
{
    let mut tx = pool.begin().await?;
    sqlx::query("SET LOCAL app.current_user_id = $1")
        .bind(user_id.to_string())
        .execute(&mut *tx).await?;
    sqlx::query("SET LOCAL app.current_role = $1")
        .bind(role.as_str())
        .execute(&mut *tx).await?;
    if let Some(tid) = tenant_id {
        sqlx::query("SET LOCAL app.current_tenant_id = $1")
            .bind(tid.to_string())
            .execute(&mut *tx).await?;
    }
    let result = f(&mut tx).await?;
    tx.commit().await?;
    Ok(result)
}
```

### Policy Design

| Role | Access Rule |
|---|---|
| SENIOR | Own records only (`user_id = current_setting('app.current_user_id')`) |
| FAMILY | Own records + linked seniors (via `family_links`) |
| CAREGIVER_APPLICANT | Own application records only |
| CAREGIVER_APPROVED | Own records + assigned client records |
| PROVIDER_ADMIN | All records within institution (`tenant_id` match) |
| PROVIDER_STAFF | Institution records, limited by assignment |
| GOVERNMENT_REVIEWER | All records in jurisdiction (read-only) |
| PLATFORM_ADMIN | Bypass RLS |

## 5. Auth: Passkeys + OAuth

### Dependencies

- `webauthn-rs` — WebAuthn/passkey server implementation
- `oauth2` crate — OAuth2 client for Kakao/Naver/Google
- `tower-sessions` or `axum-extra` — session management
- `argon2` — password hashing (if needed for fallback)

### Auth Flow

1. **Passkey (primary)** — WebAuthn registration + authentication via `webauthn-rs`
2. **OAuth (secondary)** — Kakao, Naver, Google via `oauth2` crate
3. **Demo credentials** — available only in debug builds (`#[cfg(debug_assertions)]`)

### Axum Auth Routes

```
POST /api/auth/webauthn/register/start     ← generate registration options
POST /api/auth/webauthn/register/finish     ← verify and store credential
POST /api/auth/webauthn/login/start         ← generate authentication options
POST /api/auth/webauthn/login/finish        ← verify assertion, create session
GET  /api/auth/oauth/{provider}/start       ← redirect to OAuth provider
GET  /api/auth/oauth/{provider}/callback    ← handle OAuth callback
POST /api/auth/logout                       ← destroy session
```

### Session Management

- Server-side sessions stored in PostgreSQL (or Redis)
- Session cookie: `HttpOnly`, `Secure`, `SameSite=Lax`
- Axum extractor: `AuthUser` — extracts user from session on protected routes

```rust
async fn dashboard(user: AuthUser) -> impl IntoResponse {
    // user is guaranteed authenticated here
}
```

## 6. Tailwind CSS 4

- Single `style/main.css` with `@import "tailwindcss"`
- Custom theme via `@theme` block in CSS
- Tailwind CLI scans `.rs` files for class names in `view!` macros
- Build: `npx @tailwindcss/cli -i style/main.css -o style/output.css --watch`
- No PostCSS, no Autoprefixer, no JS config file

### Tailwind + Leptos Integration

Tailwind scans Leptos `view!` macro content for class names:

```rust
view! {
    <div class="min-h-screen bg-gray-50 flex items-center justify-center">
        <h1 class="text-2xl font-bold text-indigo-600">"Hello"</h1>
    </div>
}
```

Configure content scanning in `style/main.css`:

```css
@import "tailwindcss";
@source "../crates/app/src/**/*.rs";
```

## 7. Build & Development

### Cargo Workspace

```toml
# Root Cargo.toml
[workspace]
members = ["crates/*"]

[workspace.dependencies]
leptos = { version = "0.7", features = ["ssr", "hydrate"] }
axum = "0.8"
sqlx = { version = "0.8", features = ["runtime-tokio", "postgres", "uuid", "chrono"] }
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
webauthn-rs = "0.5"
```

### Dev Workflow

```bash
# Start Postgres 18 + Redis
docker compose up -d

# Run migrations
cargo sqlx migrate run

# Dev server (SSR + WASM hot reload)
cargo leptos watch

# Tailwind (separate terminal)
npx @tailwindcss/cli -i style/main.css -o style/output.css --watch
```

`cargo leptos watch` compiles both server (native) and client (WASM) targets, serves on `localhost:3000` with hot reload.

### Production Build

```bash
cargo leptos build --release
```

Produces:
- Single binary (`target/release/bominal-server`) — serves SSR + static assets
- WASM bundle (`target/site/pkg/`) — client hydration
- CSS (`target/site/style/`) — Tailwind output

## 8. Docker

```dockerfile
# Multi-stage build
FROM rust:1.85 AS builder
RUN cargo install cargo-leptos
WORKDIR /app
COPY . .
RUN cargo leptos build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/bominal-server /usr/local/bin/
COPY --from=builder /app/target/site /site
ENV LEPTOS_SITE_ROOT=/site
EXPOSE 3000
CMD ["bominal-server"]
```

### docker-compose.yml

```yaml
services:
  postgres:
    image: postgres:18
    environment:
      POSTGRES_USER: bominalcare
      POSTGRES_PASSWORD: bominalcare
      POSTGRES_DB: bominalcare
    ports:
      - "5433:5432"
    volumes:
      - postgres-data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U bominalcare"]

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"

  app:
    build: .
    ports:
      - "3000:3000"
    depends_on:
      postgres:
        condition: service_healthy
    environment:
      DATABASE_URL: postgresql://bominalcare:bominalcare@postgres:5432/bominalcare
      REDIS_URL: redis://redis:6379
```

## 9. Deployment

- **Not Vercel** — Leptos doesn't run on Vercel (no Node.js runtime)
- **Options:** Fly.io, Railway, bare Docker on any VPS, or Shuttle.rs (Rust-native PaaS)
- **Recommended: Fly.io** — deploy Docker container, edge regions, simple CLI
- **Domain:** senior.bominal.com via Cloudflare DNS → Fly.io
- **Database:** Neon PostgreSQL (already set up)
- **Delete all 5 Vercel projects**

## 10. What Gets Deleted

- All `apps/` directory (5 Next.js apps)
- All `packages/` directory (TypeScript packages)
- `package.json`, `pnpm-lock.yaml`, `pnpm-workspace.yaml`, `turbo.json`, `.npmrc`, `.prettierrc`
- `node_modules/`
- `vercel.json`
- `.github/workflows/ci.yml` (rewrite for Rust CI)

The existing codebase serves as reference for:
- Page structure and routes
- Korean UI text and labels
- Business logic (query patterns, auth flow, role permissions)
- Seed data

## Scope Exclusions

- No new business features
- No i18n changes — Korean UI text ported as-is
- No mobile-native app (caregiver stays as responsive web)

## Risk Areas

1. **Leptos maturity** — younger ecosystem than React/Next.js, fewer community resources
2. **WASM bundle size** — can be large; needs optimization (wasm-opt, compression)
3. **Tailwind + Rust** — works but tooling is less polished than JS ecosystem
4. **WebAuthn-rs** — solid crate but less battle-tested than SimpleWebAuthn
5. **Build times** — Rust compilation is slower than TypeScript; incremental builds help
6. **Team knowledge** — anyone else working on this needs to know Rust

## Testing Plan

- `cargo test` — unit tests for DB queries, auth logic, business rules
- `cargo leptos build` — verify full build succeeds
- Browser test: passkey registration + login flow
- Verify RLS: each role can only access authorized rows
- Visual check: each portal renders correctly with Tailwind
- Load test: verify SSR performance under concurrency
