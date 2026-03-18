// =============================================================================
// Care Plan Service
// =============================================================================

import type { Pool } from 'pg';
import { generateId } from '../helpers';
import { carePlanMachine } from './state-machine';
import type { CarePlanStatus } from './state-machine';

// -----------------------------------------------------------------------------
// Input / output types
// -----------------------------------------------------------------------------

export type { CarePlanStatus };

export interface CreateCarePlanData {
  seniorId: string;
  providerId?: string;
  title: string;
  description?: string;
  startDate?: Date;
  endDate?: Date;
  goals?: unknown;
  createdBy?: string;
}

export interface UpdateCarePlanData {
  title?: string;
  description?: string;
  startDate?: Date;
  endDate?: Date;
  goals?: unknown;
  providerId?: string;
  updatedBy?: string;
}

export interface Pagination {
  page: number;
  limit: number;
}

export interface PaginatedResult<T> {
  data: T[];
  total: number;
}

export interface CarePlanWithDetails extends Record<string, unknown> {
  visits: Record<string, unknown>[];
  dailyObservations: Record<string, unknown>[];
}

// -----------------------------------------------------------------------------
// Service functions
// -----------------------------------------------------------------------------

export async function createCarePlan(
  pool: Pool,
  data: CreateCarePlanData,
): Promise<Record<string, unknown>> {
  const id = generateId();
  const result = await pool.query(
    `INSERT INTO care_plans (
       id, senior_id, provider_id, status, title, description,
       start_date, end_date, goals, created_by, created_at, updated_at
     ) VALUES ($1,$2,$3,'DRAFT',$4,$5,$6,$7,$8,$9,NOW(),NOW())
     RETURNING *`,
    [
      id,
      data.seniorId,
      data.providerId ?? null,
      data.title,
      data.description ?? null,
      data.startDate ?? null,
      data.endDate ?? null,
      data.goals !== undefined ? JSON.stringify(data.goals) : null,
      data.createdBy ?? null,
    ],
  );
  return result.rows[0];
}

export async function activateCarePlan(
  pool: Pool,
  id: string,
): Promise<Record<string, unknown>> {
  const existing = await pool.query(
    'SELECT id, status FROM care_plans WHERE id = $1',
    [id],
  );
  if (!existing.rows[0]) {
    throw new Error(`CarePlan not found: ${id}`);
  }

  const currentStatus = existing.rows[0].status as CarePlanStatus;

  if (!carePlanMachine.canTransition(currentStatus, 'ACTIVE')) {
    throw new Error(`Cannot activate care plan in status: ${currentStatus}`);
  }

  const result = await pool.query(
    `UPDATE care_plans SET status = 'ACTIVE', updated_at = NOW() WHERE id = $1 RETURNING *`,
    [id],
  );
  return result.rows[0];
}

export async function updateCarePlan(
  pool: Pool,
  id: string,
  data: UpdateCarePlanData,
): Promise<Record<string, unknown>> {
  const existing = await pool.query('SELECT id FROM care_plans WHERE id = $1', [id]);
  if (!existing.rows[0]) {
    throw new Error(`CarePlan not found: ${id}`);
  }

  const setClauses: string[] = ['updated_at = NOW()'];
  const values: unknown[] = [];
  let paramIdx = 1;

  if (data.title !== undefined) {
    setClauses.push(`title = $${paramIdx}`);
    values.push(data.title);
    paramIdx++;
  }
  if (data.description !== undefined) {
    setClauses.push(`description = $${paramIdx}`);
    values.push(data.description);
    paramIdx++;
  }
  if (data.startDate !== undefined) {
    setClauses.push(`start_date = $${paramIdx}`);
    values.push(data.startDate);
    paramIdx++;
  }
  if (data.endDate !== undefined) {
    setClauses.push(`end_date = $${paramIdx}`);
    values.push(data.endDate);
    paramIdx++;
  }
  if (data.goals !== undefined) {
    setClauses.push(`goals = $${paramIdx}`);
    values.push(JSON.stringify(data.goals));
    paramIdx++;
  }
  if (data.providerId !== undefined) {
    setClauses.push(`provider_id = $${paramIdx}`);
    values.push(data.providerId);
    paramIdx++;
  }
  if (data.updatedBy !== undefined) {
    setClauses.push(`updated_by = $${paramIdx}`);
    values.push(data.updatedBy);
    paramIdx++;
  }

  values.push(id);
  const result = await pool.query(
    `UPDATE care_plans SET ${setClauses.join(', ')} WHERE id = $${paramIdx} RETURNING *`,
    values,
  );
  return result.rows[0];
}

export async function getCarePlan(
  pool: Pool,
  id: string,
): Promise<CarePlanWithDetails | null> {
  const [planResult, visitResult, obsResult] = await Promise.all([
    pool.query('SELECT * FROM care_plans WHERE id = $1', [id]),
    pool.query(
      'SELECT * FROM visits WHERE care_plan_id = $1 ORDER BY scheduled_start ASC',
      [id],
    ),
    pool.query(
      'SELECT * FROM daily_observations WHERE care_plan_id = $1 ORDER BY date DESC',
      [id],
    ),
  ]);

  if (!planResult.rows[0]) return null;

  return {
    ...planResult.rows[0],
    visits: visitResult.rows,
    dailyObservations: obsResult.rows,
  };
}

export async function listCarePlans(
  pool: Pool,
  seniorId: string,
  pagination: Pagination,
): Promise<PaginatedResult<Record<string, unknown>>> {
  const offset = (pagination.page - 1) * pagination.limit;

  const [dataResult, countResult] = await Promise.all([
    pool.query(
      `SELECT * FROM care_plans WHERE senior_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3`,
      [seniorId, pagination.limit, offset],
    ),
    pool.query('SELECT COUNT(*) FROM care_plans WHERE senior_id = $1', [seniorId]),
  ]);

  return {
    data: dataResult.rows,
    total: parseInt(countResult.rows[0].count, 10),
  };
}

export const CarePlanService = {
  createCarePlan,
  activateCarePlan,
  updateCarePlan,
  getCarePlan,
  listCarePlans,
};
