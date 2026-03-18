// =============================================================================
// Referral Service — InstitutionReferral management
// =============================================================================

import type { Pool } from 'pg';
import { generateId } from '../helpers';
import { referralMachine } from './state-machine';
import type { InstitutionReferralStatus } from '../types';

// -----------------------------------------------------------------------------
// Events
// -----------------------------------------------------------------------------

export type ReferralEvent = {
  type: 'referral.updated';
  referralId: string;
  fromProviderId: string;
  toProviderId: string;
  status: InstitutionReferralStatus;
};

let eventEmitter: ((event: ReferralEvent) => void) | null = null;

export function setReferralEventEmitter(fn: (event: ReferralEvent) => void): void {
  eventEmitter = fn;
}

function emit(event: ReferralEvent): void {
  eventEmitter?.(event);
}

// -----------------------------------------------------------------------------
// Input / output types
// -----------------------------------------------------------------------------

export interface CreateReferralData {
  fromProviderId: string;
  toProviderId: string;
  seniorPersonId: string;
  reason?: string;
  notes?: string;
}

export interface ReferralFilters {
  fromProviderId?: string;
  toProviderId?: string;
  seniorPersonId?: string;
}

export interface Pagination {
  page: number;
  limit: number;
}

export interface PaginatedResult<T> {
  data: T[];
  total: number;
}

export interface ReferralWithProviders extends Record<string, unknown> {
  fromProvider: Record<string, unknown>;
  toProvider: Record<string, unknown>;
}

// -----------------------------------------------------------------------------
// Service functions
// -----------------------------------------------------------------------------

export async function createReferral(
  pool: Pool,
  data: CreateReferralData,
): Promise<Record<string, unknown>> {
  const id = generateId();
  const result = await pool.query(
    `INSERT INTO institution_referrals (
       id, from_provider_id, to_provider_id, senior_person_id,
       status, reason, notes, referred_at, created_at, updated_at
     ) VALUES ($1,$2,$3,$4,'CREATED',$5,$6,NOW(),NOW(),NOW())
     RETURNING *`,
    [
      id,
      data.fromProviderId,
      data.toProviderId,
      data.seniorPersonId,
      data.reason ?? null,
      data.notes ?? null,
    ],
  );
  return result.rows[0];
}

export async function updateStatus(
  pool: Pool,
  id: string,
  status: InstitutionReferralStatus,
  notes?: string,
): Promise<Record<string, unknown>> {
  const existing = await pool.query(
    'SELECT * FROM institution_referrals WHERE id = $1',
    [id],
  );
  if (!existing.rows[0]) {
    throw new Error(`InstitutionReferral not found: ${id}`);
  }

  const currentStatus = existing.rows[0].status as InstitutionReferralStatus;

  if (!referralMachine.canTransition(currentStatus, status)) {
    throw new Error(
      `Invalid referral status transition: ${currentStatus} → ${status}`,
    );
  }

  const setClauses: string[] = ['status = $1', 'updated_at = NOW()'];
  const values: unknown[] = [status];
  let paramIdx = 2;

  if (notes !== undefined) {
    setClauses.push(`notes = $${paramIdx}`);
    values.push(notes);
    paramIdx++;
  }
  if (status === 'ACCEPTED') {
    setClauses.push(`accepted_at = NOW()`);
  }
  if (status === 'DISCHARGED') {
    setClauses.push(`discharged_at = NOW()`);
  }

  values.push(id);
  const result = await pool.query(
    `UPDATE institution_referrals SET ${setClauses.join(', ')} WHERE id = $${paramIdx} RETURNING *`,
    values,
  );

  const updated = result.rows[0] as Record<string, unknown>;

  emit({
    type: 'referral.updated',
    referralId: updated.id as string,
    fromProviderId: updated.from_provider_id as string,
    toProviderId: updated.to_provider_id as string,
    status: updated.status as InstitutionReferralStatus,
  });

  return updated;
}

export async function getReferral(
  pool: Pool,
  id: string,
): Promise<ReferralWithProviders | null> {
  const refResult = await pool.query(
    'SELECT * FROM institution_referrals WHERE id = $1',
    [id],
  );
  if (!refResult.rows[0]) return null;

  const referral = refResult.rows[0] as Record<string, unknown>;

  const [fromResult, toResult] = await Promise.all([
    pool.query('SELECT * FROM provider_organizations WHERE id = $1', [referral.from_provider_id]),
    pool.query('SELECT * FROM provider_organizations WHERE id = $1', [referral.to_provider_id]),
  ]);

  return {
    ...referral,
    fromProvider: fromResult.rows[0] ?? {},
    toProvider: toResult.rows[0] ?? {},
  };
}

export async function listReferrals(
  pool: Pool,
  filters: ReferralFilters,
  pagination: Pagination,
): Promise<PaginatedResult<Record<string, unknown>>> {
  const conditions: string[] = [];
  const values: unknown[] = [];
  let paramIdx = 1;

  if (filters.fromProviderId !== undefined) {
    conditions.push(`from_provider_id = $${paramIdx}`);
    values.push(filters.fromProviderId);
    paramIdx++;
  }
  if (filters.toProviderId !== undefined) {
    conditions.push(`to_provider_id = $${paramIdx}`);
    values.push(filters.toProviderId);
    paramIdx++;
  }
  if (filters.seniorPersonId !== undefined) {
    conditions.push(`senior_person_id = $${paramIdx}`);
    values.push(filters.seniorPersonId);
    paramIdx++;
  }

  const where = conditions.length > 0 ? `WHERE ${conditions.join(' AND ')}` : '';
  const offset = (pagination.page - 1) * pagination.limit;

  const [dataResult, countResult] = await Promise.all([
    pool.query(
      `SELECT * FROM institution_referrals ${where} ORDER BY referred_at DESC LIMIT $${paramIdx} OFFSET $${paramIdx + 1}`,
      [...values, pagination.limit, offset],
    ),
    pool.query(`SELECT COUNT(*) FROM institution_referrals ${where}`, values),
  ]);

  return {
    data: dataResult.rows,
    total: parseInt(countResult.rows[0].count, 10),
  };
}

export const ReferralService = {
  createReferral,
  updateStatus,
  getReferral,
  listReferrals,
  setReferralEventEmitter,
};
