// =============================================================================
// Profile Service — PersonProfile and SeniorProfile CRUD
// =============================================================================

import type { Pool } from 'pg';
import { generateId } from '../helpers';

// -----------------------------------------------------------------------------
// Input types
// -----------------------------------------------------------------------------

export interface CreatePersonProfileData {
  userId: string;
  firstName: string;
  lastName: string;
  dateOfBirth?: Date | null;
  gender?: string | null;
  nationalId?: string | null;
  phone?: string | null;
  address?: string | null;
  city?: string | null;
  district?: string | null;
  emergencyContactName?: string | null;
  emergencyContactPhone?: string | null;
}

export interface UpdatePersonProfileData {
  firstName?: string;
  lastName?: string;
  dateOfBirth?: Date | null;
  gender?: string | null;
  nationalId?: string | null;
  phone?: string | null;
  address?: string | null;
  city?: string | null;
  district?: string | null;
  emergencyContactName?: string | null;
  emergencyContactPhone?: string | null;
}

export interface CreateSeniorProfileData {
  personId: string;
  mobilityLevel?: string | null;
  cognitiveLevelScore?: number | null;
  preferredLanguage?: string | null;
  dietaryRestrictions?: string | null;
  notes?: string | null;
}

export interface UpdateSeniorProfileData {
  mobilityLevel?: string | null;
  cognitiveLevelScore?: number | null;
  preferredLanguage?: string | null;
  dietaryRestrictions?: string | null;
  notes?: string | null;
}

export interface PersonProfileWithUser {
  id: string;
  userId: string;
  firstName: string;
  lastName: string;
  dateOfBirth: Date | null;
  gender: string | null;
  nationalId: string | null;
  phone: string | null;
  address: string | null;
  city: string | null;
  district: string | null;
  emergencyContactName: string | null;
  emergencyContactPhone: string | null;
  createdAt: Date;
  updatedAt: Date;
  user: {
    id: string;
    email: string | null;
    name: string | null;
    phone: string | null;
    role: string;
    kycLevel: string;
    isActive: boolean;
  };
}

export interface SeniorProfileWithPerson {
  id: string;
  personId: string;
  mobilityLevel: string | null;
  cognitiveLevelScore: number | null;
  preferredLanguage: string | null;
  dietaryRestrictions: string | null;
  notes: string | null;
  createdAt: Date;
  updatedAt: Date;
  personProfile: {
    id: string;
    userId: string;
    firstName: string;
    lastName: string;
    dateOfBirth: Date | null;
    gender: string | null;
    nationalId: string | null;
    phone: string | null;
    address: string | null;
    city: string | null;
    district: string | null;
    emergencyContactName: string | null;
    emergencyContactPhone: string | null;
    createdAt: Date;
    updatedAt: Date;
    user: {
      id: string;
      email: string | null;
      name: string | null;
      role: string;
    };
  };
}

// -----------------------------------------------------------------------------
// Service functions
// -----------------------------------------------------------------------------

export async function createPersonProfile(
  pool: Pool,
  data: CreatePersonProfileData,
): Promise<Record<string, unknown>> {
  const id = generateId();
  const result = await pool.query(
    `INSERT INTO person_profiles (
       id, user_id, first_name, last_name, date_of_birth, gender, national_id,
       phone, address, city, district, emergency_contact_name, emergency_contact_phone,
       created_at, updated_at
     ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,NOW(),NOW())
     RETURNING *`,
    [
      id,
      data.userId,
      data.firstName,
      data.lastName,
      data.dateOfBirth ?? null,
      data.gender ?? null,
      data.nationalId ?? null,
      data.phone ?? null,
      data.address ?? null,
      data.city ?? null,
      data.district ?? null,
      data.emergencyContactName ?? null,
      data.emergencyContactPhone ?? null,
    ],
  );
  return result.rows[0];
}

export async function updatePersonProfile(
  pool: Pool,
  id: string,
  data: UpdatePersonProfileData,
): Promise<Record<string, unknown>> {
  const existing = await pool.query('SELECT id FROM person_profiles WHERE id = $1', [id]);
  if (!existing.rows[0]) {
    throw new Error(`PersonProfile not found: ${id}`);
  }

  const setClauses: string[] = ['updated_at = NOW()'];
  const values: unknown[] = [];
  let paramIdx = 1;

  const fieldMap: Record<keyof UpdatePersonProfileData, string> = {
    firstName: 'first_name',
    lastName: 'last_name',
    dateOfBirth: 'date_of_birth',
    gender: 'gender',
    nationalId: 'national_id',
    phone: 'phone',
    address: 'address',
    city: 'city',
    district: 'district',
    emergencyContactName: 'emergency_contact_name',
    emergencyContactPhone: 'emergency_contact_phone',
  };

  for (const [key, col] of Object.entries(fieldMap) as [keyof UpdatePersonProfileData, string][]) {
    if (data[key] !== undefined) {
      setClauses.push(`${col} = $${paramIdx}`);
      values.push(data[key]);
      paramIdx++;
    }
  }

  values.push(id);
  const result = await pool.query(
    `UPDATE person_profiles SET ${setClauses.join(', ')} WHERE id = $${paramIdx} RETURNING *`,
    values,
  );
  return result.rows[0];
}

export async function getPersonProfile(
  pool: Pool,
  id: string,
): Promise<PersonProfileWithUser | null> {
  const result = await pool.query(
    `SELECT
       pp.*,
       u.id        AS u_id,
       u.email     AS u_email,
       u.name      AS u_name,
       u.phone     AS u_phone,
       u.role      AS u_role,
       u.kyc_level AS u_kyc_level,
       u.is_active AS u_is_active
     FROM person_profiles pp
     JOIN users u ON u.id = pp.user_id
     WHERE pp.id = $1`,
    [id],
  );
  if (!result.rows[0]) return null;
  return mapPersonProfileWithUser(result.rows[0]);
}

export async function getPersonProfileByUserId(
  pool: Pool,
  userId: string,
): Promise<PersonProfileWithUser | null> {
  const result = await pool.query(
    `SELECT
       pp.*,
       u.id        AS u_id,
       u.email     AS u_email,
       u.name      AS u_name,
       u.phone     AS u_phone,
       u.role      AS u_role,
       u.kyc_level AS u_kyc_level,
       u.is_active AS u_is_active
     FROM person_profiles pp
     JOIN users u ON u.id = pp.user_id
     WHERE pp.user_id = $1`,
    [userId],
  );
  if (!result.rows[0]) return null;
  return mapPersonProfileWithUser(result.rows[0]);
}

export async function createSeniorProfile(
  pool: Pool,
  data: CreateSeniorProfileData,
): Promise<Record<string, unknown>> {
  const id = generateId();
  const result = await pool.query(
    `INSERT INTO senior_profiles (
       id, person_id, mobility_level, cognitive_level_score,
       preferred_language, dietary_restrictions, notes, created_at, updated_at
     ) VALUES ($1,$2,$3,$4,$5,$6,$7,NOW(),NOW())
     RETURNING *`,
    [
      id,
      data.personId,
      data.mobilityLevel ?? null,
      data.cognitiveLevelScore ?? null,
      data.preferredLanguage ?? null,
      data.dietaryRestrictions ?? null,
      data.notes ?? null,
    ],
  );
  return result.rows[0];
}

export async function updateSeniorProfile(
  pool: Pool,
  id: string,
  data: UpdateSeniorProfileData,
): Promise<Record<string, unknown>> {
  const existing = await pool.query('SELECT id FROM senior_profiles WHERE id = $1', [id]);
  if (!existing.rows[0]) {
    throw new Error(`SeniorProfile not found: ${id}`);
  }

  const setClauses: string[] = ['updated_at = NOW()'];
  const values: unknown[] = [];
  let paramIdx = 1;

  const fieldMap: Record<keyof UpdateSeniorProfileData, string> = {
    mobilityLevel: 'mobility_level',
    cognitiveLevelScore: 'cognitive_level_score',
    preferredLanguage: 'preferred_language',
    dietaryRestrictions: 'dietary_restrictions',
    notes: 'notes',
  };

  for (const [key, col] of Object.entries(fieldMap) as [keyof UpdateSeniorProfileData, string][]) {
    if (data[key] !== undefined) {
      setClauses.push(`${col} = $${paramIdx}`);
      values.push(data[key]);
      paramIdx++;
    }
  }

  values.push(id);
  const result = await pool.query(
    `UPDATE senior_profiles SET ${setClauses.join(', ')} WHERE id = $${paramIdx} RETURNING *`,
    values,
  );
  return result.rows[0];
}

export async function getSeniorProfile(
  pool: Pool,
  personId: string,
): Promise<SeniorProfileWithPerson | null> {
  const result = await pool.query(
    `SELECT
       sp.*,
       pp.id                       AS pp_id,
       pp.user_id                  AS pp_user_id,
       pp.first_name               AS pp_first_name,
       pp.last_name                AS pp_last_name,
       pp.date_of_birth            AS pp_date_of_birth,
       pp.gender                   AS pp_gender,
       pp.national_id              AS pp_national_id,
       pp.phone                    AS pp_phone,
       pp.address                  AS pp_address,
       pp.city                     AS pp_city,
       pp.district                 AS pp_district,
       pp.emergency_contact_name   AS pp_emergency_contact_name,
       pp.emergency_contact_phone  AS pp_emergency_contact_phone,
       pp.created_at               AS pp_created_at,
       pp.updated_at               AS pp_updated_at,
       u.id                        AS u_id,
       u.email                     AS u_email,
       u.name                      AS u_name,
       u.role                      AS u_role
     FROM senior_profiles sp
     JOIN person_profiles pp ON pp.id = sp.person_id
     JOIN users u ON u.id = pp.user_id
     WHERE sp.person_id = $1`,
    [personId],
  );
  if (!result.rows[0]) return null;
  return mapSeniorProfileWithPerson(result.rows[0]);
}

// -----------------------------------------------------------------------------
// Row mappers
// -----------------------------------------------------------------------------

function mapPersonProfileWithUser(row: Record<string, unknown>): PersonProfileWithUser {
  return {
    id: row.id as string,
    userId: row.user_id as string,
    firstName: (row.korean_name as string) ?? '',
    lastName: '',
    dateOfBirth: row.date_of_birth as Date | null,
    gender: row.gender as string | null,
    nationalId: row.national_id as string | null,
    phone: row.phone as string | null,
    address: row.address as string | null,
    city: row.city as string | null,
    district: row.district as string | null,
    emergencyContactName: row.emergency_contact_name as string | null,
    emergencyContactPhone: row.emergency_contact_phone as string | null,
    createdAt: row.created_at as Date,
    updatedAt: row.updated_at as Date,
    user: {
      id: row.u_id as string,
      email: row.u_email as string | null,
      name: row.u_name as string | null,
      phone: row.u_phone as string | null,
      role: row.u_role as string,
      kycLevel: row.u_kyc_level as string,
      isActive: row.u_is_active as boolean,
    },
  };
}

function mapSeniorProfileWithPerson(row: Record<string, unknown>): SeniorProfileWithPerson {
  return {
    id: row.id as string,
    personId: row.person_id as string,
    mobilityLevel: row.mobility_level as string | null,
    cognitiveLevelScore: row.cognitive_level_score as number | null,
    preferredLanguage: row.preferred_language as string | null,
    dietaryRestrictions: row.dietary_restrictions as string | null,
    notes: row.notes as string | null,
    createdAt: row.created_at as Date,
    updatedAt: row.updated_at as Date,
    personProfile: {
      id: row.pp_id as string,
      userId: row.pp_user_id as string,
      firstName: row.pp_first_name as string,
      lastName: row.pp_last_name as string,
      dateOfBirth: row.pp_date_of_birth as Date | null,
      gender: row.pp_gender as string | null,
      nationalId: row.pp_national_id as string | null,
      phone: row.pp_phone as string | null,
      address: row.pp_address as string | null,
      city: row.pp_city as string | null,
      district: row.pp_district as string | null,
      emergencyContactName: row.pp_emergency_contact_name as string | null,
      emergencyContactPhone: row.pp_emergency_contact_phone as string | null,
      createdAt: row.pp_created_at as Date,
      updatedAt: row.pp_updated_at as Date,
      user: {
        id: row.u_id as string,
        email: row.u_email as string | null,
        name: row.u_name as string | null,
        role: row.u_role as string,
      },
    },
  };
}

export const ProfileService = {
  createPersonProfile,
  updatePersonProfile,
  getPersonProfile,
  getPersonProfileByUserId,
  createSeniorProfile,
  updateSeniorProfile,
  getSeniorProfile,
};
