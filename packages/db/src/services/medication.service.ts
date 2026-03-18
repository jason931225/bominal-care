// =============================================================================
// Medication Service
// =============================================================================

import type { Pool } from 'pg';
import { generateId } from '../helpers';
import { medicationEventMachine } from './state-machine';
import type { MedicationEventStatus, MedicationFrequency, DayOfWeek } from '../types';

// -----------------------------------------------------------------------------
// Events
// -----------------------------------------------------------------------------

export type MedicationEvent2 =
  | { type: 'medication.taken'; eventId: string; medicationId: string; personId: string }
  | { type: 'medication.missed'; eventId: string; medicationId: string; personId: string };

let eventEmitter: ((event: MedicationEvent2) => void) | null = null;

export function setMedicationEventEmitter(fn: (event: MedicationEvent2) => void): void {
  eventEmitter = fn;
}

function emit(event: MedicationEvent2): void {
  eventEmitter?.(event);
}

// -----------------------------------------------------------------------------
// Input / output types
// -----------------------------------------------------------------------------

export type { MedicationEventStatus };

export interface CreateMedicationData {
  personId: string;
  name: string;
  dosage: string;
  form: string;
  frequency: MedicationFrequency;
  prescribedBy?: string;
  prescribedAt?: Date;
  startDate?: Date;
  endDate?: Date;
  sideEffects?: string;
  notes?: string;
  createdBy?: string;
}

export interface UpdateMedicationData {
  name?: string;
  dosage?: string;
  form?: string;
  frequency?: MedicationFrequency;
  prescribedBy?: string;
  prescribedAt?: Date;
  startDate?: Date;
  endDate?: Date;
  isActive?: boolean;
  sideEffects?: string;
  notes?: string;
  updatedBy?: string;
}

export interface AddScheduleData {
  timeOfDay: string;
  dayOfWeek?: DayOfWeek;
  isActive?: boolean;
}

export interface DateRange {
  from: Date;
  to: Date;
}

export interface MedicationWithSchedules extends Record<string, unknown> {
  schedules: Record<string, unknown>[];
}

// -----------------------------------------------------------------------------
// Service functions
// -----------------------------------------------------------------------------

export async function createMedication(
  pool: Pool,
  data: CreateMedicationData,
): Promise<Record<string, unknown>> {
  const id = generateId();
  const result = await pool.query(
    `INSERT INTO medications (
       id, person_id, name, dosage, form, frequency,
       prescribed_by, prescribed_at, start_date, end_date,
       is_active, side_effects, notes, created_by, created_at, updated_at
     ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,true,$11,$12,$13,NOW(),NOW())
     RETURNING *`,
    [
      id,
      data.personId,
      data.name,
      data.dosage,
      data.form,
      data.frequency,
      data.prescribedBy ?? null,
      data.prescribedAt ?? null,
      data.startDate ?? null,
      data.endDate ?? null,
      data.sideEffects ?? null,
      data.notes ?? null,
      data.createdBy ?? null,
    ],
  );
  return result.rows[0];
}

export async function updateMedication(
  pool: Pool,
  id: string,
  data: UpdateMedicationData,
): Promise<Record<string, unknown>> {
  const existing = await pool.query('SELECT id FROM medications WHERE id = $1', [id]);
  if (!existing.rows[0]) {
    throw new Error(`Medication not found: ${id}`);
  }

  const setClauses: string[] = ['updated_at = NOW()'];
  const values: unknown[] = [];
  let paramIdx = 1;

  const fieldMap: Partial<Record<keyof UpdateMedicationData, string>> = {
    name: 'name',
    dosage: 'dosage',
    form: 'form',
    frequency: 'frequency',
    prescribedBy: 'prescribed_by',
    prescribedAt: 'prescribed_at',
    startDate: 'start_date',
    endDate: 'end_date',
    isActive: 'is_active',
    sideEffects: 'side_effects',
    notes: 'notes',
    updatedBy: 'updated_by',
  };

  for (const [key, col] of Object.entries(fieldMap) as [keyof UpdateMedicationData, string][]) {
    if (data[key] !== undefined) {
      setClauses.push(`${col} = $${paramIdx}`);
      values.push(data[key]);
      paramIdx++;
    }
  }

  values.push(id);
  const result = await pool.query(
    `UPDATE medications SET ${setClauses.join(', ')} WHERE id = $${paramIdx} RETURNING *`,
    values,
  );
  return result.rows[0];
}

export async function addSchedule(
  pool: Pool,
  medicationId: string,
  data: AddScheduleData,
): Promise<Record<string, unknown>> {
  const id = generateId();
  const result = await pool.query(
    `INSERT INTO medication_schedules (id, medication_id, time_of_day, day_of_week, is_active, created_at, updated_at)
     VALUES ($1,$2,$3,$4,$5,NOW(),NOW())
     RETURNING *`,
    [id, medicationId, data.timeOfDay, data.dayOfWeek ?? null, data.isActive ?? true],
  );
  return result.rows[0];
}

/**
 * Generates MedicationEvent records for every active schedule within the given
 * date range, skipping events that already exist.
 */
export async function generateEvents(
  pool: Pool,
  medicationId: string,
  dateRange: DateRange,
): Promise<Record<string, unknown>[]> {
  const medResult = await pool.query(
    `SELECT m.*, ms.id AS sched_id, ms.time_of_day, ms.day_of_week
     FROM medications m
     JOIN medication_schedules ms ON ms.medication_id = m.id AND ms.is_active = true
     WHERE m.id = $1`,
    [medicationId],
  );

  if (medResult.rows.length === 0) {
    throw new Error(`Medication not found: ${medicationId}`);
  }

  const schedules = medResult.rows.map((row: Record<string, unknown>) => ({
    id: row.sched_id as string,
    timeOfDay: row.time_of_day as string,
    dayOfWeek: row.day_of_week as DayOfWeek | null,
  }));

  const eventsToInsert: Array<{ medicationId: string; scheduledFor: Date }> = [];

  const current = new Date(dateRange.from);
  current.setHours(0, 0, 0, 0);
  const end = new Date(dateRange.to);
  end.setHours(23, 59, 59, 999);

  const DAY_NAMES: DayOfWeek[] = [
    'SUNDAY', 'MONDAY', 'TUESDAY', 'WEDNESDAY', 'THURSDAY', 'FRIDAY', 'SATURDAY',
  ];

  while (current <= end) {
    const dayName = DAY_NAMES[current.getDay()] as DayOfWeek;

    for (const schedule of schedules) {
      if (schedule.dayOfWeek !== null && schedule.dayOfWeek !== dayName) {
        continue;
      }

      const [hours, minutes] = schedule.timeOfDay.split(':').map(Number);
      const scheduledFor = new Date(current);
      scheduledFor.setHours(hours ?? 0, minutes ?? 0, 0, 0);

      eventsToInsert.push({ medicationId, scheduledFor });
    }

    current.setDate(current.getDate() + 1);
  }

  if (eventsToInsert.length === 0) {
    return [];
  }

  // Insert events, skipping duplicates via ON CONFLICT DO NOTHING
  const client = await pool.connect();
  try {
    await client.query('BEGIN');

    for (const evt of eventsToInsert) {
      const evtId = generateId();
      await client.query(
        `INSERT INTO medication_events (id, medication_id, scheduled_for, status, created_at, updated_at)
         VALUES ($1,$2,$3,'SCHEDULED',NOW(),NOW())
         ON CONFLICT (medication_id, scheduled_for) DO NOTHING`,
        [evtId, evt.medicationId, evt.scheduledFor],
      );
    }

    await client.query('COMMIT');
  } catch (err) {
    await client.query('ROLLBACK');
    throw err;
  } finally {
    client.release();
  }

  // Return newly created events in range
  const eventResult = await pool.query(
    `SELECT * FROM medication_events
     WHERE medication_id = $1
       AND scheduled_for >= $2
       AND scheduled_for <= $3
     ORDER BY scheduled_for ASC`,
    [medicationId, dateRange.from, dateRange.to],
  );

  return eventResult.rows;
}

export async function updateEventStatus(
  pool: Pool,
  eventId: string,
  status: MedicationEventStatus,
  recordedBy?: string,
  notes?: string,
): Promise<Record<string, unknown>> {
  const existing = await pool.query(
    `SELECT me.*, m.person_id
     FROM medication_events me
     JOIN medications m ON m.id = me.medication_id
     WHERE me.id = $1`,
    [eventId],
  );
  if (!existing.rows[0]) {
    throw new Error(`MedicationEvent not found: ${eventId}`);
  }

  const row = existing.rows[0] as Record<string, unknown>;
  const currentStatus = row.status as MedicationEventStatus;

  if (!medicationEventMachine.canTransition(currentStatus, status)) {
    throw new Error(
      `Invalid medication event transition: ${currentStatus} → ${status}`,
    );
  }

  const setClauses: string[] = ['status = $1', 'updated_at = NOW()'];
  const values: unknown[] = [status];
  let paramIdx = 2;

  if (status === 'TAKEN') {
    setClauses.push(`taken_at = NOW()`);
  }
  if (recordedBy !== undefined) {
    setClauses.push(`recorded_by = $${paramIdx}`);
    values.push(recordedBy);
    paramIdx++;
  }
  if (notes !== undefined) {
    setClauses.push(`notes = $${paramIdx}`);
    values.push(notes);
    paramIdx++;
  }

  values.push(eventId);
  const result = await pool.query(
    `UPDATE medication_events SET ${setClauses.join(', ')} WHERE id = $${paramIdx} RETURNING *`,
    values,
  );

  const updated = result.rows[0] as Record<string, unknown>;

  if (status === 'TAKEN') {
    emit({
      type: 'medication.taken',
      eventId: updated.id as string,
      medicationId: updated.medication_id as string,
      personId: row.person_id as string,
    });
  } else if (status === 'MISSED') {
    emit({
      type: 'medication.missed',
      eventId: updated.id as string,
      medicationId: updated.medication_id as string,
      personId: row.person_id as string,
    });
  }

  return updated;
}

export async function getTodayEvents(
  pool: Pool,
  personId: string,
): Promise<Record<string, unknown>[]> {
  const todayStart = new Date();
  todayStart.setHours(0, 0, 0, 0);
  const todayEnd = new Date();
  todayEnd.setHours(23, 59, 59, 999);

  const result = await pool.query(
    `SELECT me.*
     FROM medication_events me
     JOIN medications m ON m.id = me.medication_id
     WHERE m.person_id = $1
       AND m.is_active = true
       AND me.scheduled_for >= $2
       AND me.scheduled_for <= $3
     ORDER BY me.scheduled_for ASC`,
    [personId, todayStart, todayEnd],
  );
  return result.rows;
}

export async function getOverdueEvents(
  pool: Pool,
  personId: string,
): Promise<Record<string, unknown>[]> {
  const result = await pool.query(
    `SELECT me.*
     FROM medication_events me
     JOIN medications m ON m.id = me.medication_id
     WHERE m.person_id = $1
       AND m.is_active = true
       AND me.scheduled_for < NOW()
       AND me.status IN ('MISSED', 'SCHEDULED', 'REMINDER_SENT')
     ORDER BY me.scheduled_for ASC`,
    [personId],
  );
  return result.rows;
}

export async function listMedications(
  pool: Pool,
  personId: string,
): Promise<MedicationWithSchedules[]> {
  const [medResult, schedResult] = await Promise.all([
    pool.query(
      `SELECT * FROM medications WHERE person_id = $1 AND is_active = true ORDER BY name ASC`,
      [personId],
    ),
    pool.query(
      `SELECT ms.* FROM medication_schedules ms
       JOIN medications m ON m.id = ms.medication_id
       WHERE m.person_id = $1 AND m.is_active = true AND ms.is_active = true`,
      [personId],
    ),
  ]);

  const schedsByMed = new Map<string, Record<string, unknown>[]>();
  for (const sched of schedResult.rows) {
    const list = schedsByMed.get(sched.medication_id) ?? [];
    list.push(sched);
    schedsByMed.set(sched.medication_id, list);
  }

  return medResult.rows.map((med: Record<string, unknown>) => ({
    ...med,
    schedules: schedsByMed.get(med.id as string) ?? [],
  }));
}

export const MedicationService = {
  createMedication,
  updateMedication,
  addSchedule,
  generateEvents,
  updateEventStatus,
  getTodayEvents,
  getOverdueEvents,
  listMedications,
  setMedicationEventEmitter,
};
