// =============================================================================
// Medical History Service
// =============================================================================

import type { Pool } from 'pg';
import { generateId } from '../helpers';

// -----------------------------------------------------------------------------
// Input types
// -----------------------------------------------------------------------------

export interface CreateMedicalHistoryEntryData {
  personId: string;
  condition: string;
  diagnosedAt?: Date;
  treatedBy?: string;
  status?: string;
  notes?: string;
  createdBy?: string;
}

export interface UpdateMedicalHistoryEntryData {
  condition?: string;
  diagnosedAt?: Date;
  treatedBy?: string;
  status?: string;
  notes?: string;
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

// -----------------------------------------------------------------------------
// Service functions
// -----------------------------------------------------------------------------

export async function createEntry(
  pool: Pool,
  data: CreateMedicalHistoryEntryData,
): Promise<Record<string, unknown>> {
  const id = generateId();
  const result = await pool.query(
    `INSERT INTO medical_history_entries (
       id, person_id, condition, diagnosed_at, treated_by,
       status, notes, created_by, created_at, updated_at
     ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,NOW(),NOW())
     RETURNING *`,
    [
      id,
      data.personId,
      data.condition,
      data.diagnosedAt ?? null,
      data.treatedBy ?? null,
      data.status ?? 'active',
      data.notes ?? null,
      data.createdBy ?? null,
    ],
  );
  return result.rows[0];
}

export async function updateEntry(
  pool: Pool,
  id: string,
  data: UpdateMedicalHistoryEntryData,
): Promise<Record<string, unknown>> {
  const existing = await pool.query(
    'SELECT id FROM medical_history_entries WHERE id = $1',
    [id],
  );
  if (!existing.rows[0]) {
    throw new Error(`MedicalHistoryEntry not found: ${id}`);
  }

  const setClauses: string[] = ['updated_at = NOW()'];
  const values: unknown[] = [];
  let paramIdx = 1;

  const fieldMap: Partial<Record<keyof UpdateMedicalHistoryEntryData, string>> = {
    condition: 'condition',
    diagnosedAt: 'diagnosed_at',
    treatedBy: 'treated_by',
    status: 'status',
    notes: 'notes',
    updatedBy: 'updated_by',
  };

  for (const [key, col] of Object.entries(fieldMap) as [keyof UpdateMedicalHistoryEntryData, string][]) {
    if (data[key] !== undefined) {
      setClauses.push(`${col} = $${paramIdx}`);
      values.push(data[key]);
      paramIdx++;
    }
  }

  values.push(id);
  const result = await pool.query(
    `UPDATE medical_history_entries SET ${setClauses.join(', ')} WHERE id = $${paramIdx} RETURNING *`,
    values,
  );
  return result.rows[0];
}

export async function getEntry(
  pool: Pool,
  id: string,
): Promise<Record<string, unknown> | null> {
  const result = await pool.query(
    'SELECT * FROM medical_history_entries WHERE id = $1',
    [id],
  );
  return result.rows[0] ?? null;
}

export async function listEntries(
  pool: Pool,
  personId: string,
  pagination: Pagination,
): Promise<PaginatedResult<Record<string, unknown>>> {
  const offset = (pagination.page - 1) * pagination.limit;

  const [dataResult, countResult] = await Promise.all([
    pool.query(
      `SELECT * FROM medical_history_entries WHERE person_id = $1 ORDER BY diagnosed_at DESC LIMIT $2 OFFSET $3`,
      [personId, pagination.limit, offset],
    ),
    pool.query('SELECT COUNT(*) FROM medical_history_entries WHERE person_id = $1', [personId]),
  ]);

  return {
    data: dataResult.rows,
    total: parseInt(countResult.rows[0].count, 10),
  };
}

export async function getActiveConditions(
  pool: Pool,
  personId: string,
): Promise<Record<string, unknown>[]> {
  const result = await pool.query(
    `SELECT * FROM medical_history_entries
     WHERE person_id = $1
       AND status IN ('active', 'chronic')
     ORDER BY diagnosed_at DESC`,
    [personId],
  );
  return result.rows;
}

export const MedicalHistoryService = {
  createEntry,
  updateEntry,
  getEntry,
  listEntries,
  getActiveConditions,
};
