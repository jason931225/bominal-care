// =============================================================================
// Caregiver Application Service
// =============================================================================

import type { Pool } from 'pg';
import { generateId } from '../helpers';
import { caregiverApplicationMachine } from './state-machine';
import type { CaregiverApplicationStatus } from './state-machine';
import type { Gender, CredentialType } from '../types';

// -----------------------------------------------------------------------------
// Input / output types
// -----------------------------------------------------------------------------

export type { CaregiverApplicationStatus };

export interface CreateApplicationData {
  providerId?: string;
  experienceYears?: number;
  bio?: string;
  specializations?: string;
  hasDementiaExperience?: boolean;
  hasOvernightAvailability?: boolean;
  smokingStatus?: boolean;
  petFriendly?: boolean;
  preferredGender?: Gender | null;
  languagesSpoken?: string;
}

export interface UpdateApplicationData {
  providerId?: string;
  experienceYears?: number;
  bio?: string;
  specializations?: string;
  hasDementiaExperience?: boolean;
  hasOvernightAvailability?: boolean;
  smokingStatus?: boolean;
  petFriendly?: boolean;
  preferredGender?: Gender | null;
  languagesSpoken?: string;
}

export interface ApplicationFilters {
  status?: CaregiverApplicationStatus;
  providerId?: string;
}

export interface Pagination {
  page: number;
  limit: number;
}

export interface PaginatedResult<T> {
  data: T[];
  total: number;
}

export interface AddCredentialData {
  type: CredentialType;
  issuer?: string;
  issuedAt?: Date;
  expiresAt?: Date;
  documentUrl?: string;
}

export interface ApplicationWithRelations extends Record<string, unknown> {
  credentials: Record<string, unknown>[];
  availabilitySlots: Record<string, unknown>[];
  serviceTypes: Record<string, unknown>[];
}

// -----------------------------------------------------------------------------
// Service functions
// -----------------------------------------------------------------------------

export async function createApplication(
  pool: Pool,
  userId: string,
  data: CreateApplicationData,
): Promise<Record<string, unknown>> {
  const id = generateId();
  const result = await pool.query(
    `INSERT INTO caregiver_applications (
       id, user_id, status, provider_id, experience_years, bio, specializations,
       has_dementia_experience, has_overnight_availability, smoking_status,
       pet_friendly, preferred_gender, languages_spoken, created_at, updated_at
     ) VALUES ($1,$2,'DRAFT',$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,NOW(),NOW())
     RETURNING *`,
    [
      id,
      userId,
      data.providerId ?? null,
      data.experienceYears ?? null,
      data.bio ?? null,
      data.specializations ?? null,
      data.hasDementiaExperience ?? false,
      data.hasOvernightAvailability ?? false,
      data.smokingStatus ?? false,
      data.petFriendly ?? true,
      data.preferredGender ?? null,
      data.languagesSpoken ?? 'ko',
    ],
  );
  return result.rows[0];
}

export async function updateApplication(
  pool: Pool,
  id: string,
  data: UpdateApplicationData,
): Promise<Record<string, unknown>> {
  const existing = await pool.query(
    'SELECT id FROM caregiver_applications WHERE id = $1',
    [id],
  );
  if (!existing.rows[0]) {
    throw new Error(`CaregiverApplication not found: ${id}`);
  }

  const setClauses: string[] = ['updated_at = NOW()'];
  const values: unknown[] = [];
  let paramIdx = 1;

  const fieldMap: Partial<Record<keyof UpdateApplicationData, string>> = {
    providerId: 'provider_id',
    experienceYears: 'experience_years',
    bio: 'bio',
    specializations: 'specializations',
    hasDementiaExperience: 'has_dementia_experience',
    hasOvernightAvailability: 'has_overnight_availability',
    smokingStatus: 'smoking_status',
    petFriendly: 'pet_friendly',
    preferredGender: 'preferred_gender',
    languagesSpoken: 'languages_spoken',
  };

  for (const [key, col] of Object.entries(fieldMap) as [keyof UpdateApplicationData, string][]) {
    if (data[key] !== undefined) {
      setClauses.push(`${col} = $${paramIdx}`);
      values.push(data[key]);
      paramIdx++;
    }
  }

  values.push(id);
  const result = await pool.query(
    `UPDATE caregiver_applications SET ${setClauses.join(', ')} WHERE id = $${paramIdx} RETURNING *`,
    values,
  );
  return result.rows[0];
}

export async function submitApplication(
  pool: Pool,
  id: string,
): Promise<Record<string, unknown>> {
  const existing = await pool.query(
    'SELECT id, status FROM caregiver_applications WHERE id = $1',
    [id],
  );
  if (!existing.rows[0]) {
    throw new Error(`CaregiverApplication not found: ${id}`);
  }

  const currentStatus = existing.rows[0].status as CaregiverApplicationStatus;

  if (!caregiverApplicationMachine.canTransition(currentStatus, 'SUBMITTED')) {
    throw new Error(
      `Cannot transition application from ${currentStatus} to SUBMITTED`,
    );
  }

  const result = await pool.query(
    `UPDATE caregiver_applications
     SET status = 'SUBMITTED', submitted_at = NOW(), updated_at = NOW()
     WHERE id = $1
     RETURNING *`,
    [id],
  );
  return result.rows[0];
}

export async function transitionStatus(
  pool: Pool,
  id: string,
  newStatus: CaregiverApplicationStatus,
  reviewedBy?: string,
  rejectionReason?: string,
): Promise<Record<string, unknown>> {
  const existing = await pool.query(
    'SELECT id, status FROM caregiver_applications WHERE id = $1',
    [id],
  );
  if (!existing.rows[0]) {
    throw new Error(`CaregiverApplication not found: ${id}`);
  }

  const currentStatus = existing.rows[0].status as CaregiverApplicationStatus;

  if (!caregiverApplicationMachine.canTransition(currentStatus, newStatus)) {
    throw new Error(
      `Invalid transition for caregiver application: ${currentStatus} → ${newStatus}`,
    );
  }

  const setClauses: string[] = ['status = $1', 'updated_at = NOW()'];
  const values: unknown[] = [newStatus];
  let paramIdx = 2;

  if (reviewedBy !== undefined) {
    setClauses.push(`reviewed_by = $${paramIdx}`, `reviewed_at = NOW()`);
    values.push(reviewedBy);
    paramIdx++;
  }
  if (rejectionReason !== undefined) {
    setClauses.push(`rejection_reason = $${paramIdx}`);
    values.push(rejectionReason);
    paramIdx++;
  }

  values.push(id);
  const result = await pool.query(
    `UPDATE caregiver_applications SET ${setClauses.join(', ')} WHERE id = $${paramIdx} RETURNING *`,
    values,
  );
  return result.rows[0];
}

export async function getApplication(
  pool: Pool,
  id: string,
): Promise<ApplicationWithRelations | null> {
  const [appResult, credResult, slotResult, serviceResult] = await Promise.all([
    pool.query('SELECT * FROM caregiver_applications WHERE id = $1', [id]),
    pool.query('SELECT * FROM caregiver_credentials WHERE application_id = $1', [id]),
    pool.query('SELECT * FROM availability_slots WHERE application_id = $1', [id]),
    pool.query('SELECT * FROM service_types WHERE application_id = $1', [id]),
  ]);

  if (!appResult.rows[0]) return null;

  return {
    ...appResult.rows[0],
    credentials: credResult.rows,
    availabilitySlots: slotResult.rows,
    serviceTypes: serviceResult.rows,
  };
}

export async function listApplications(
  pool: Pool,
  filters: ApplicationFilters,
  pagination: Pagination,
): Promise<PaginatedResult<Record<string, unknown>>> {
  const conditions: string[] = [];
  const values: unknown[] = [];
  let paramIdx = 1;

  if (filters.status !== undefined) {
    conditions.push(`status = $${paramIdx}`);
    values.push(filters.status);
    paramIdx++;
  }
  if (filters.providerId !== undefined) {
    conditions.push(`provider_id = $${paramIdx}`);
    values.push(filters.providerId);
    paramIdx++;
  }

  const where = conditions.length > 0 ? `WHERE ${conditions.join(' AND ')}` : '';
  const offset = (pagination.page - 1) * pagination.limit;

  const [dataResult, countResult] = await Promise.all([
    pool.query(
      `SELECT * FROM caregiver_applications ${where} ORDER BY created_at DESC LIMIT $${paramIdx} OFFSET $${paramIdx + 1}`,
      [...values, pagination.limit, offset],
    ),
    pool.query(`SELECT COUNT(*) FROM caregiver_applications ${where}`, values),
  ]);

  return {
    data: dataResult.rows,
    total: parseInt(countResult.rows[0].count, 10),
  };
}

export async function addCredential(
  pool: Pool,
  applicationId: string,
  data: AddCredentialData,
): Promise<Record<string, unknown>> {
  const id = generateId();
  const result = await pool.query(
    `INSERT INTO caregiver_credentials (
       id, application_id, type, status, issuer, issued_at, expires_at, document_url, created_at, updated_at
     ) VALUES ($1,$2,$3,'PENDING',$4,$5,$6,$7,NOW(),NOW())
     RETURNING *`,
    [
      id,
      applicationId,
      data.type,
      data.issuer ?? null,
      data.issuedAt ?? null,
      data.expiresAt ?? null,
      data.documentUrl ?? null,
    ],
  );
  return result.rows[0];
}

export async function verifyCredential(
  pool: Pool,
  credentialId: string,
  verifiedBy: string,
): Promise<Record<string, unknown>> {
  const existing = await pool.query(
    'SELECT id FROM caregiver_credentials WHERE id = $1',
    [credentialId],
  );
  if (!existing.rows[0]) {
    throw new Error(`CaregiverCredential not found: ${credentialId}`);
  }

  const result = await pool.query(
    `UPDATE caregiver_credentials
     SET status = 'VERIFIED', verified_at = NOW(), verified_by = $1, updated_at = NOW()
     WHERE id = $2
     RETURNING *`,
    [verifiedBy, credentialId],
  );
  return result.rows[0];
}

export const CaregiverApplicationService = {
  createApplication,
  updateApplication,
  submitApplication,
  transitionStatus,
  getApplication,
  listApplications,
  addCredential,
  verifyCredential,
};
