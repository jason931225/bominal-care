// =============================================================================
// Audit Service — AuditLog management
// =============================================================================

import type { Pool } from 'pg';
import { generateId } from '../helpers';
import type { AuditAction } from '../types';

export interface LogAuditData {
  userId?: string;
  action: AuditAction;
  entityType?: string;
  entityId?: string;
  oldValue?: unknown;
  newValue?: unknown;
  ipAddress?: string;
  userAgent?: string;
}

export interface AuditLogFilters {
  userId?: string;
  action?: AuditAction;
  entityType?: string;
  entityId?: string;
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

// -----------------------------------------------------------------------------
// Service functions
// -----------------------------------------------------------------------------

export async function log(
  pool: Pool,
  data: LogAuditData,
): Promise<Record<string, unknown>> {
  const id = generateId();
  const result = await pool.query(
    `INSERT INTO audit_logs (
       id, user_id, action, entity_type, entity_id,
       old_value, new_value, ip_address, user_agent, created_at
     ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,NOW())
     RETURNING *`,
    [
      id,
      data.userId ?? null,
      data.action,
      data.entityType ?? null,
      data.entityId ?? null,
      data.oldValue !== undefined ? JSON.stringify(data.oldValue) : null,
      data.newValue !== undefined ? JSON.stringify(data.newValue) : null,
      data.ipAddress ?? null,
      data.userAgent ?? null,
    ],
  );
  return result.rows[0];
}

export async function listLogs(
  pool: Pool,
  filters: AuditLogFilters,
  pagination: Pagination,
): Promise<PaginatedResult<Record<string, unknown>>> {
  const conditions: string[] = [];
  const values: unknown[] = [];
  let paramIdx = 1;

  if (filters.userId !== undefined) {
    conditions.push(`user_id = $${paramIdx}`);
    values.push(filters.userId);
    paramIdx++;
  }
  if (filters.action !== undefined) {
    conditions.push(`action = $${paramIdx}`);
    values.push(filters.action);
    paramIdx++;
  }
  if (filters.entityType !== undefined) {
    conditions.push(`entity_type = $${paramIdx}`);
    values.push(filters.entityType);
    paramIdx++;
  }
  if (filters.entityId !== undefined) {
    conditions.push(`entity_id = $${paramIdx}`);
    values.push(filters.entityId);
    paramIdx++;
  }
  if (filters.dateRange !== undefined) {
    conditions.push(`created_at >= $${paramIdx} AND created_at <= $${paramIdx + 1}`);
    values.push(filters.dateRange.from, filters.dateRange.to);
    paramIdx += 2;
  }

  const where = conditions.length > 0 ? `WHERE ${conditions.join(' AND ')}` : '';
  const offset = (pagination.page - 1) * pagination.limit;

  const [dataResult, countResult] = await Promise.all([
    pool.query(
      `SELECT * FROM audit_logs ${where} ORDER BY created_at DESC LIMIT $${paramIdx} OFFSET $${paramIdx + 1}`,
      [...values, pagination.limit, offset],
    ),
    pool.query(`SELECT COUNT(*) FROM audit_logs ${where}`, values),
  ]);

  return {
    data: dataResult.rows,
    total: parseInt(countResult.rows[0].count, 10),
  };
}

export const AuditService = {
  log,
  listLogs,
};
