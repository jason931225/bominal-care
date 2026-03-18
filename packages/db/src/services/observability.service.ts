// =============================================================================
// Observability Service — ObservabilitySignal management and dashboard stats
// =============================================================================

import type { Pool } from 'pg';
import { generateId } from '../helpers';
import type { ObservabilityEventType, SignalSeverity } from '../types';

export interface CreateSignalData {
  eventType: ObservabilityEventType;
  severity?: SignalSeverity;
  subjectPersonId?: string;
  actorUserId?: string;
  entityType?: string;
  entityId?: string;
  message: string;
  metadata?: unknown;
}

export interface SignalFilters {
  eventType?: ObservabilityEventType;
  severity?: SignalSeverity;
  subjectPersonId?: string;
  acknowledged?: boolean;
}

export interface Pagination {
  page: number;
  limit: number;
}

export interface PaginatedResult<T> {
  data: T[];
  total: number;
}

export interface DashboardStats {
  byEventType: Array<{ eventType: ObservabilityEventType; count: number }>;
  bySeverity: Array<{ severity: SignalSeverity; count: number }>;
  total: number;
  unacknowledged: number;
}

// -----------------------------------------------------------------------------
// Service functions
// -----------------------------------------------------------------------------

export async function createSignal(
  pool: Pool,
  data: CreateSignalData,
): Promise<Record<string, unknown>> {
  const id = generateId();
  const result = await pool.query(
    `INSERT INTO observability_signals (
       id, event_type, severity, subject_person_id, actor_user_id,
       entity_type, entity_id, message, metadata, created_at
     ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,NOW())
     RETURNING *`,
    [
      id,
      data.eventType,
      data.severity ?? 'INFO',
      data.subjectPersonId ?? null,
      data.actorUserId ?? null,
      data.entityType ?? null,
      data.entityId ?? null,
      data.message,
      data.metadata !== undefined ? JSON.stringify(data.metadata) : null,
    ],
  );
  return result.rows[0];
}

export async function acknowledgeSignal(
  pool: Pool,
  id: string,
  userId: string,
): Promise<Record<string, unknown>> {
  const existing = await pool.query(
    'SELECT id FROM observability_signals WHERE id = $1',
    [id],
  );
  if (!existing.rows[0]) {
    throw new Error(`ObservabilitySignal not found: ${id}`);
  }

  const result = await pool.query(
    `UPDATE observability_signals
     SET acknowledged_at = NOW(), acknowledged_by = $1
     WHERE id = $2
     RETURNING *`,
    [userId, id],
  );
  return result.rows[0];
}

export async function listSignals(
  pool: Pool,
  filters: SignalFilters,
  pagination: Pagination,
): Promise<PaginatedResult<Record<string, unknown>>> {
  const conditions: string[] = [];
  const values: unknown[] = [];
  let paramIdx = 1;

  if (filters.eventType !== undefined) {
    conditions.push(`event_type = $${paramIdx}`);
    values.push(filters.eventType);
    paramIdx++;
  }
  if (filters.severity !== undefined) {
    conditions.push(`severity = $${paramIdx}`);
    values.push(filters.severity);
    paramIdx++;
  }
  if (filters.subjectPersonId !== undefined) {
    conditions.push(`subject_person_id = $${paramIdx}`);
    values.push(filters.subjectPersonId);
    paramIdx++;
  }
  if (filters.acknowledged !== undefined) {
    if (filters.acknowledged) {
      conditions.push(`acknowledged_at IS NOT NULL`);
    } else {
      conditions.push(`acknowledged_at IS NULL`);
    }
  }

  const where = conditions.length > 0 ? `WHERE ${conditions.join(' AND ')}` : '';
  const offset = (pagination.page - 1) * pagination.limit;

  const [dataResult, countResult] = await Promise.all([
    pool.query(
      `SELECT * FROM observability_signals ${where} ORDER BY created_at DESC LIMIT $${paramIdx} OFFSET $${paramIdx + 1}`,
      [...values, pagination.limit, offset],
    ),
    pool.query(`SELECT COUNT(*) FROM observability_signals ${where}`, values),
  ]);

  return {
    data: dataResult.rows,
    total: parseInt(countResult.rows[0].count, 10),
  };
}

export async function getDashboardStats(
  pool: Pool,
  filters?: Pick<SignalFilters, 'subjectPersonId'>,
): Promise<DashboardStats> {
  const conditions: string[] = [];
  const baseValues: unknown[] = [];
  let paramIdx = 1;

  if (filters?.subjectPersonId !== undefined) {
    conditions.push(`subject_person_id = $${paramIdx}`);
    baseValues.push(filters.subjectPersonId);
    paramIdx++;
  }

  const where = conditions.length > 0 ? `WHERE ${conditions.join(' AND ')}` : '';
  const unacknowledgedWhere =
    conditions.length > 0
      ? `WHERE ${conditions.join(' AND ')} AND acknowledged_at IS NULL`
      : `WHERE acknowledged_at IS NULL`;

  const [byEventTypeResult, bySeverityResult, totalResult, unacknowledgedResult] = await Promise.all([
    pool.query(
      `SELECT event_type, COUNT(*) AS count FROM observability_signals ${where} GROUP BY event_type ORDER BY count DESC`,
      baseValues,
    ),
    pool.query(
      `SELECT severity, COUNT(*) AS count FROM observability_signals ${where} GROUP BY severity ORDER BY count DESC`,
      baseValues,
    ),
    pool.query(`SELECT COUNT(*) FROM observability_signals ${where}`, baseValues),
    pool.query(`SELECT COUNT(*) FROM observability_signals ${unacknowledgedWhere}`, baseValues),
  ]);

  return {
    byEventType: byEventTypeResult.rows.map((r: Record<string, unknown>) => ({
      eventType: r.event_type as ObservabilityEventType,
      count: parseInt(r.count as string, 10),
    })),
    bySeverity: bySeverityResult.rows.map((r: Record<string, unknown>) => ({
      severity: r.severity as SignalSeverity,
      count: parseInt(r.count as string, 10),
    })),
    total: parseInt(totalResult.rows[0].count, 10),
    unacknowledged: parseInt(unacknowledgedResult.rows[0].count, 10),
  };
}

export const ObservabilityService = {
  createSignal,
  acknowledgeSignal,
  listSignals,
  getDashboardStats,
};
