// =============================================================================
// Visit Service
// =============================================================================

import type { Pool } from 'pg';
import { generateId } from '../helpers';
import { visitMachine } from './state-machine';
import type { VisitStatus } from '../types';

// -----------------------------------------------------------------------------
// Events — simple inline type (not imported from @bominal-senior/events)
// -----------------------------------------------------------------------------

export type VisitEvent =
  | { type: 'visit.missed'; visitId: string; carePlanId: string; caregiverId: string }
  | { type: 'visit.completed'; visitId: string; carePlanId: string; caregiverId: string };

/** Override this in your app to handle domain events. */
let eventEmitter: ((event: VisitEvent) => void) | null = null;

export function setVisitEventEmitter(fn: (event: VisitEvent) => void): void {
  eventEmitter = fn;
}

function emit(event: VisitEvent): void {
  eventEmitter?.(event);
}

// -----------------------------------------------------------------------------
// Input / output types
// -----------------------------------------------------------------------------

export interface ScheduleVisitData {
  carePlanId: string;
  caregiverId: string;
  scheduledStart: Date;
  scheduledEnd: Date;
  tasks?: unknown;
  notes?: string;
}

export interface CheckInData {
  latitude?: number;
  longitude?: number;
}

export interface CheckOutData {
  latitude?: number;
  longitude?: number;
  notes?: string;
}

export interface VisitFilters {
  caregiverId?: string;
  carePlanId?: string;
  status?: VisitStatus;
  dateRange?: { from: Date; to: Date };
}

export interface Pagination {
  page: number;
  limit: number;
}

export interface PaginatedResult<T> {
  data: T[];
  total: number;
}

export interface VisitWithRelations extends Record<string, unknown> {
  carePlan: Record<string, unknown>;
  caregiver: Record<string, unknown>;
}

// -----------------------------------------------------------------------------
// Service functions
// -----------------------------------------------------------------------------

export async function scheduleVisit(
  pool: Pool,
  data: ScheduleVisitData,
): Promise<Record<string, unknown>> {
  const id = generateId();
  const result = await pool.query(
    `INSERT INTO visits (
       id, care_plan_id, caregiver_id, status, scheduled_start, scheduled_end,
       tasks, notes, created_at, updated_at
     ) VALUES ($1,$2,$3,'SCHEDULED',$4,$5,$6,$7,NOW(),NOW())
     RETURNING *`,
    [
      id,
      data.carePlanId,
      data.caregiverId,
      data.scheduledStart,
      data.scheduledEnd,
      data.tasks !== undefined ? JSON.stringify(data.tasks) : null,
      data.notes ?? null,
    ],
  );
  return result.rows[0];
}

export async function acknowledgeVisit(
  pool: Pool,
  id: string,
): Promise<Record<string, unknown>> {
  const existing = await pool.query('SELECT id, status FROM visits WHERE id = $1', [id]);
  if (!existing.rows[0]) {
    throw new Error(`Visit not found: ${id}`);
  }

  const currentStatus = existing.rows[0].status as VisitStatus;

  if (!visitMachine.canTransition(currentStatus, 'CAREGIVER_ACKNOWLEDGED')) {
    throw new Error(`Cannot acknowledge visit in status: ${currentStatus}`);
  }

  const result = await pool.query(
    `UPDATE visits SET status = 'CAREGIVER_ACKNOWLEDGED', updated_at = NOW() WHERE id = $1 RETURNING *`,
    [id],
  );
  return result.rows[0];
}

export async function checkIn(
  pool: Pool,
  id: string,
  data: CheckInData,
): Promise<Record<string, unknown>> {
  const existing = await pool.query('SELECT id, status FROM visits WHERE id = $1', [id]);
  if (!existing.rows[0]) {
    throw new Error(`Visit not found: ${id}`);
  }

  const currentStatus = existing.rows[0].status as VisitStatus;

  if (!visitMachine.canTransition(currentStatus, 'IN_PROGRESS')) {
    throw new Error(`Cannot check in on visit in status: ${currentStatus}`);
  }

  const result = await pool.query(
    `UPDATE visits
     SET status = 'IN_PROGRESS', actual_start = NOW(),
         check_in_latitude = $1, check_in_longitude = $2, updated_at = NOW()
     WHERE id = $3
     RETURNING *`,
    [data.latitude ?? null, data.longitude ?? null, id],
  );
  return result.rows[0];
}

export async function checkOut(
  pool: Pool,
  id: string,
  data: CheckOutData,
): Promise<Record<string, unknown>> {
  const existing = await pool.query('SELECT id, status FROM visits WHERE id = $1', [id]);
  if (!existing.rows[0]) {
    throw new Error(`Visit not found: ${id}`);
  }

  const currentStatus = existing.rows[0].status as VisitStatus;

  if (!visitMachine.canTransition(currentStatus, 'COMPLETED')) {
    throw new Error(`Cannot check out on visit in status: ${currentStatus}`);
  }

  const setClauses = [
    'status = \'COMPLETED\'',
    'actual_end = NOW()',
    'check_out_latitude = $1',
    'check_out_longitude = $2',
    'updated_at = NOW()',
  ];
  const values: unknown[] = [data.latitude ?? null, data.longitude ?? null];
  let paramIdx = 3;

  if (data.notes !== undefined) {
    setClauses.push(`notes = $${paramIdx}`);
    values.push(data.notes);
    paramIdx++;
  }

  values.push(id);
  const result = await pool.query(
    `UPDATE visits SET ${setClauses.join(', ')} WHERE id = $${paramIdx} RETURNING *`,
    values,
  );

  const updated = result.rows[0] as Record<string, unknown>;

  emit({
    type: 'visit.completed',
    visitId: updated.id as string,
    carePlanId: updated.care_plan_id as string,
    caregiverId: updated.caregiver_id as string,
  });

  return updated;
}

export async function markMissed(
  pool: Pool,
  id: string,
): Promise<Record<string, unknown>> {
  const existing = await pool.query('SELECT id, status FROM visits WHERE id = $1', [id]);
  if (!existing.rows[0]) {
    throw new Error(`Visit not found: ${id}`);
  }

  const currentStatus = existing.rows[0].status as VisitStatus;

  if (!visitMachine.canTransition(currentStatus, 'MISSED')) {
    throw new Error(`Cannot mark visit as missed in status: ${currentStatus}`);
  }

  const result = await pool.query(
    `UPDATE visits SET status = 'MISSED', updated_at = NOW() WHERE id = $1 RETURNING *`,
    [id],
  );

  const updated = result.rows[0] as Record<string, unknown>;

  emit({
    type: 'visit.missed',
    visitId: updated.id as string,
    carePlanId: updated.care_plan_id as string,
    caregiverId: updated.caregiver_id as string,
  });

  return updated;
}

export async function getVisit(
  pool: Pool,
  id: string,
): Promise<VisitWithRelations | null> {
  const visitResult = await pool.query('SELECT * FROM visits WHERE id = $1', [id]);
  if (!visitResult.rows[0]) return null;

  const visit = visitResult.rows[0] as Record<string, unknown>;

  const [planResult, caregiverResult] = await Promise.all([
    pool.query('SELECT * FROM care_plans WHERE id = $1', [visit.care_plan_id]),
    pool.query('SELECT * FROM caregiver_applications WHERE id = $1', [visit.caregiver_id]),
  ]);

  return {
    ...visit,
    carePlan: planResult.rows[0] ?? {},
    caregiver: caregiverResult.rows[0] ?? {},
  };
}

export async function listVisits(
  pool: Pool,
  filters: VisitFilters,
  pagination: Pagination,
): Promise<PaginatedResult<Record<string, unknown>>> {
  const conditions: string[] = [];
  const values: unknown[] = [];
  let paramIdx = 1;

  if (filters.caregiverId !== undefined) {
    conditions.push(`caregiver_id = $${paramIdx}`);
    values.push(filters.caregiverId);
    paramIdx++;
  }
  if (filters.carePlanId !== undefined) {
    conditions.push(`care_plan_id = $${paramIdx}`);
    values.push(filters.carePlanId);
    paramIdx++;
  }
  if (filters.status !== undefined) {
    conditions.push(`status = $${paramIdx}`);
    values.push(filters.status);
    paramIdx++;
  }
  if (filters.dateRange !== undefined) {
    conditions.push(`scheduled_start >= $${paramIdx} AND scheduled_start <= $${paramIdx + 1}`);
    values.push(filters.dateRange.from, filters.dateRange.to);
    paramIdx += 2;
  }

  const where = conditions.length > 0 ? `WHERE ${conditions.join(' AND ')}` : '';
  const offset = (pagination.page - 1) * pagination.limit;

  const [dataResult, countResult] = await Promise.all([
    pool.query(
      `SELECT * FROM visits ${where} ORDER BY scheduled_start ASC LIMIT $${paramIdx} OFFSET $${paramIdx + 1}`,
      [...values, pagination.limit, offset],
    ),
    pool.query(`SELECT COUNT(*) FROM visits ${where}`, values),
  ]);

  return {
    data: dataResult.rows,
    total: parseInt(countResult.rows[0].count, 10),
  };
}

export async function getUpcomingVisits(
  pool: Pool,
  caregiverId: string,
  limit: number,
): Promise<Record<string, unknown>[]> {
  const result = await pool.query(
    `SELECT * FROM visits
     WHERE caregiver_id = $1
       AND status IN ('SCHEDULED', 'CAREGIVER_ACKNOWLEDGED')
       AND scheduled_start >= NOW()
     ORDER BY scheduled_start ASC
     LIMIT $2`,
    [caregiverId, limit],
  );
  return result.rows;
}

export const VisitService = {
  scheduleVisit,
  acknowledgeVisit,
  checkIn,
  checkOut,
  markMissed,
  getVisit,
  listVisits,
  getUpcomingVisits,
  setVisitEventEmitter,
};
