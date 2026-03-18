// =============================================================================
// Appointment Service
// =============================================================================

import type { Pool } from 'pg';
import { generateId } from '../helpers';
import { appointmentMachine } from './state-machine';
import type { AppointmentStatus, ProviderType } from '../types';

// -----------------------------------------------------------------------------
// Input types
// -----------------------------------------------------------------------------

export type { AppointmentStatus };

export interface CreateAppointmentData {
  personId: string;
  institutionName: string;
  institutionType?: ProviderType;
  appointmentDate: Date;
  purpose?: string;
  notes?: string;
  address?: string;
  createdBy?: string;
}

export interface UpdateAppointmentData {
  institutionName?: string;
  institutionType?: ProviderType;
  appointmentDate?: Date;
  purpose?: string;
  notes?: string;
  address?: string;
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

export async function createAppointment(
  pool: Pool,
  data: CreateAppointmentData,
): Promise<Record<string, unknown>> {
  const id = generateId();
  const result = await pool.query(
    `INSERT INTO appointments (
       id, person_id, institution_name, institution_type, appointment_date,
       status, purpose, notes, address, created_by, created_at, updated_at
     ) VALUES ($1,$2,$3,$4,$5,'SCHEDULED',$6,$7,$8,$9,NOW(),NOW())
     RETURNING *`,
    [
      id,
      data.personId,
      data.institutionName,
      data.institutionType ?? null,
      data.appointmentDate,
      data.purpose ?? null,
      data.notes ?? null,
      data.address ?? null,
      data.createdBy ?? null,
    ],
  );
  return result.rows[0];
}

export async function updateAppointment(
  pool: Pool,
  id: string,
  data: UpdateAppointmentData,
): Promise<Record<string, unknown>> {
  const existing = await pool.query('SELECT id FROM appointments WHERE id = $1', [id]);
  if (!existing.rows[0]) {
    throw new Error(`Appointment not found: ${id}`);
  }

  const setClauses: string[] = ['updated_at = NOW()'];
  const values: unknown[] = [];
  let paramIdx = 1;

  const fieldMap: Partial<Record<keyof UpdateAppointmentData, string>> = {
    institutionName: 'institution_name',
    institutionType: 'institution_type',
    appointmentDate: 'appointment_date',
    purpose: 'purpose',
    notes: 'notes',
    address: 'address',
    updatedBy: 'updated_by',
  };

  for (const [key, col] of Object.entries(fieldMap) as [keyof UpdateAppointmentData, string][]) {
    if (data[key] !== undefined) {
      setClauses.push(`${col} = $${paramIdx}`);
      values.push(data[key]);
      paramIdx++;
    }
  }

  values.push(id);
  const result = await pool.query(
    `UPDATE appointments SET ${setClauses.join(', ')} WHERE id = $${paramIdx} RETURNING *`,
    values,
  );
  return result.rows[0];
}

export async function updateStatus(
  pool: Pool,
  id: string,
  status: AppointmentStatus,
): Promise<Record<string, unknown>> {
  const existing = await pool.query('SELECT id, status FROM appointments WHERE id = $1', [id]);
  if (!existing.rows[0]) {
    throw new Error(`Appointment not found: ${id}`);
  }

  const currentStatus = existing.rows[0].status as AppointmentStatus;

  if (!appointmentMachine.canTransition(currentStatus, status)) {
    throw new Error(
      `Invalid appointment status transition: ${currentStatus} → ${status}`,
    );
  }

  const result = await pool.query(
    `UPDATE appointments SET status = $1, updated_at = NOW() WHERE id = $2 RETURNING *`,
    [status, id],
  );
  return result.rows[0];
}

export async function getAppointment(
  pool: Pool,
  id: string,
): Promise<Record<string, unknown> | null> {
  const result = await pool.query('SELECT * FROM appointments WHERE id = $1', [id]);
  return result.rows[0] ?? null;
}

export async function listAppointments(
  pool: Pool,
  personId: string,
  pagination: Pagination,
): Promise<PaginatedResult<Record<string, unknown>>> {
  const offset = (pagination.page - 1) * pagination.limit;

  const [dataResult, countResult] = await Promise.all([
    pool.query(
      `SELECT * FROM appointments WHERE person_id = $1 ORDER BY appointment_date DESC LIMIT $2 OFFSET $3`,
      [personId, pagination.limit, offset],
    ),
    pool.query('SELECT COUNT(*) FROM appointments WHERE person_id = $1', [personId]),
  ]);

  return {
    data: dataResult.rows,
    total: parseInt(countResult.rows[0].count, 10),
  };
}

export async function getUpcomingAppointments(
  pool: Pool,
  personId: string,
  limit: number,
): Promise<Record<string, unknown>[]> {
  const result = await pool.query(
    `SELECT * FROM appointments
     WHERE person_id = $1
       AND appointment_date >= NOW()
       AND status IN ('SCHEDULED', 'CONFIRMED')
     ORDER BY appointment_date ASC
     LIMIT $2`,
    [personId, limit],
  );
  return result.rows;
}

export const AppointmentService = {
  createAppointment,
  updateAppointment,
  updateStatus,
  getAppointment,
  listAppointments,
  getUpcomingAppointments,
};
