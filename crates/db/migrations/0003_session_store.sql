-- =============================================================================
-- tower-sessions session store table
-- Required by tower-sessions-sqlx-store
-- =============================================================================

CREATE TABLE IF NOT EXISTS tower_sessions (
  id TEXT PRIMARY KEY NOT NULL,
  data BYTEA NOT NULL,
  expiry_date TIMESTAMPTZ NOT NULL
);

-- =============================================================================
-- WebAuthn credentials table
-- =============================================================================

CREATE TABLE webauthn_credentials (
  id UUID NOT NULL DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL,
  credential JSONB NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

  CONSTRAINT webauthn_credentials_pkey PRIMARY KEY (id),
  CONSTRAINT webauthn_credentials_user_id_fkey FOREIGN KEY (user_id)
    REFERENCES users (id) ON DELETE CASCADE
);

CREATE INDEX webauthn_credentials_user_id_idx ON webauthn_credentials (user_id);
