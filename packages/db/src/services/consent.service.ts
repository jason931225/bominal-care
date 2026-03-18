// =============================================================================
// Consent Service — ConsentRecord management
// =============================================================================

import type { Pool } from 'pg';
import { generateId } from '../helpers';
import type { ConsentPurpose } from '../types';

export type { ConsentPurpose };

export interface GrantConsentData {
  subjectPersonId: string;
  purpose: ConsentPurpose;
  grantedBy: string;
  expiresAt?: Date;
}

// -----------------------------------------------------------------------------
// Service functions
// -----------------------------------------------------------------------------

export async function grantConsent(
  pool: Pool,
  data: GrantConsentData,
): Promise<Record<string, unknown>> {
  const id = generateId();
  const result = await pool.query(
    `INSERT INTO consent_records (
       id, subject_person_id, purpose, granted_by, expires_at, is_active, granted_at
     ) VALUES ($1, $2, $3, $4, $5, true, NOW())
     RETURNING *`,
    [id, data.subjectPersonId, data.purpose, data.grantedBy, data.expiresAt ?? null],
  );
  return result.rows[0];
}

/**
 * Marks a consent record as inactive. The `revokedBy` identity should be
 * recorded in an AuditLog entry by the caller — the ConsentRecord schema has
 * no revokedBy column, so we surface the value only via the audit trail.
 */
export async function revokeConsent(
  pool: Pool,
  id: string,
  _revokedBy: string,
): Promise<Record<string, unknown>> {
  const existing = await pool.query('SELECT id FROM consent_records WHERE id = $1', [id]);
  if (!existing.rows[0]) {
    throw new Error(`ConsentRecord not found: ${id}`);
  }

  const result = await pool.query(
    `UPDATE consent_records
     SET is_active = false, revoked_at = NOW()
     WHERE id = $1
     RETURNING *`,
    [id],
  );
  return result.rows[0];
}

export async function getActiveConsent(
  pool: Pool,
  subjectPersonId: string,
  purpose: ConsentPurpose,
): Promise<Record<string, unknown> | null> {
  const result = await pool.query(
    `SELECT * FROM consent_records
     WHERE subject_person_id = $1
       AND purpose = $2
       AND is_active = true
       AND (expires_at IS NULL OR expires_at > NOW())
     ORDER BY granted_at DESC
     LIMIT 1`,
    [subjectPersonId, purpose],
  );
  return result.rows[0] ?? null;
}

export async function hasConsent(
  pool: Pool,
  subjectPersonId: string,
  purpose: ConsentPurpose,
): Promise<boolean> {
  const record = await getActiveConsent(pool, subjectPersonId, purpose);
  return record !== null;
}

export async function getConsentsForPerson(
  pool: Pool,
  subjectPersonId: string,
): Promise<Record<string, unknown>[]> {
  const result = await pool.query(
    `SELECT * FROM consent_records
     WHERE subject_person_id = $1
     ORDER BY granted_at DESC`,
    [subjectPersonId],
  );
  return result.rows;
}

export const ConsentService = {
  grantConsent,
  revokeConsent,
  getActiveConsent,
  hasConsent,
  getConsentsForPerson,
};
