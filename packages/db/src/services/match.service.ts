// =============================================================================
// Match Service — MatchRequest, scoring, and recommendations
// =============================================================================

import type { Pool } from 'pg';
import { generateId } from '../helpers';
import { matchRequestMachine } from './state-machine';
import type { MatchRequestStatus } from './state-machine';
import type { Gender, ServiceCategory } from '../types';

// -----------------------------------------------------------------------------
// Input / output types
// -----------------------------------------------------------------------------

export type { MatchRequestStatus };

export interface CreateMatchRequestData {
  seniorId: string;
  requestedBy: string;
  serviceCategory: ServiceCategory;
  regionCity: string;
  regionDistrict: string;
  startDate?: Date;
  endDate?: Date;
  scheduleNotes?: string;
  languagePreference?: string;
  genderPreference?: Gender;
  requiresDementiaExperience?: boolean;
  requiresOvernightCare?: boolean;
  additionalNotes?: string;
}

export interface Pagination {
  page: number;
  limit: number;
}

export interface PaginatedResult<T> {
  data: T[];
  total: number;
}

export interface MatchRequestWithRecommendations extends Record<string, unknown> {
  recommendations: Array<Record<string, unknown> & {
    caregiverApplication: Record<string, unknown>;
  }>;
}

// -----------------------------------------------------------------------------
// Scoring types — pure, serialisable data only
// -----------------------------------------------------------------------------

export interface ScoringCriteria {
  regionCity: string;
  regionDistrict: string;
  serviceCategory: ServiceCategory;
  languagePreference?: string | null;
  genderPreference?: Gender | null;
  requiresDementiaExperience: boolean;
  requiresOvernightCare: boolean;
  /** Requested days-of-week (e.g. ["MONDAY", "WEDNESDAY"]) */
  requestedDays?: string[];
}

export interface CandidateData {
  application: Record<string, unknown> & {
    availabilitySlots: Array<{ dayOfWeek: string }>;
    serviceTypes: Array<{ category: string }>;
    languagesSpoken: string;
    preferredGender: string | null;
    hasDementiaExperience: boolean;
    hasOvernightAvailability: boolean;
    smokingStatus: boolean;
    petFriendly: boolean;
  };
  /** Cities/districts served by the caregiver's provider */
  serviceRegions: { city: string; district: string }[];
}

export interface ScoreBreakdown {
  regionMatch: number;
  scheduleOverlap: number;
  serviceTypeMatch: number;
  languageMatch: number;
  genderPreferenceMatch: number;
  dementiaExperience: number;
  mobilitySkills: number;
  smokingPetPreferences: number;
  total: number;
}

// -----------------------------------------------------------------------------
// Pure scoring engine
// -----------------------------------------------------------------------------

export function scoreCandidateAtomic(
  criteria: ScoringCriteria,
  candidate: CandidateData,
): ScoreBreakdown {
  const { application, serviceRegions } = candidate;

  // 1. Region match (required — 0 if no match): 25 points
  const regionMatch = serviceRegions.some(
    (r) =>
      r.city === criteria.regionCity && r.district === criteria.regionDistrict,
  )
    ? 25
    : 0;

  // Short-circuit: no region match means candidate is ineligible
  if (regionMatch === 0) {
    return {
      regionMatch: 0,
      scheduleOverlap: 0,
      serviceTypeMatch: 0,
      languageMatch: 0,
      genderPreferenceMatch: 0,
      dementiaExperience: 0,
      mobilitySkills: 0,
      smokingPetPreferences: 0,
      total: 0,
    };
  }

  // 2. Schedule overlap: 20 points
  let scheduleOverlap = 0;
  if (criteria.requestedDays && criteria.requestedDays.length > 0) {
    const availableDays = new Set(application.availabilitySlots.map((s) => s.dayOfWeek));
    const matchedDays = criteria.requestedDays.filter((d) => availableDays.has(d));
    scheduleOverlap = Math.round((matchedDays.length / criteria.requestedDays.length) * 20);
  } else {
    // No preference specified — full score
    scheduleOverlap = 20;
  }

  // 3. Service type match: 15 points
  const serviceTypeMatch = application.serviceTypes.some(
    (st) => st.category === criteria.serviceCategory,
  )
    ? 15
    : 0;

  // 4. Language match: 10 points
  let languageMatch = 0;
  if (criteria.languagePreference) {
    const spoken = application.languagesSpoken.split(',').map((l) => l.trim().toLowerCase());
    languageMatch = spoken.includes(criteria.languagePreference.toLowerCase()) ? 10 : 0;
  } else {
    languageMatch = 10;
  }

  // 5. Gender preference match: 10 points
  let genderPreferenceMatch = 0;
  if (criteria.genderPreference) {
    genderPreferenceMatch =
      application.preferredGender === null ||
      application.preferredGender === criteria.genderPreference
        ? 10
        : 0;
  } else {
    genderPreferenceMatch = 10;
  }

  // 6. Dementia experience: 10 points
  const dementiaExperience =
    !criteria.requiresDementiaExperience || application.hasDementiaExperience ? 10 : 0;

  // 7. Mobility skills (overnight availability as proxy): 5 points
  const mobilitySkills =
    !criteria.requiresOvernightCare || application.hasOvernightAvailability ? 5 : 0;

  // 8. Smoking / pet preferences: 5 points
  const smokingPetPreferences =
    (!application.smokingStatus ? 2.5 : 0) + (application.petFriendly ? 2.5 : 0);

  const total =
    regionMatch +
    scheduleOverlap +
    serviceTypeMatch +
    languageMatch +
    genderPreferenceMatch +
    dementiaExperience +
    mobilitySkills +
    smokingPetPreferences;

  return {
    regionMatch,
    scheduleOverlap,
    serviceTypeMatch,
    languageMatch,
    genderPreferenceMatch,
    dementiaExperience,
    mobilitySkills,
    smokingPetPreferences,
    total,
  };
}

// -----------------------------------------------------------------------------
// Service functions
// -----------------------------------------------------------------------------

export async function createMatchRequest(
  pool: Pool,
  data: CreateMatchRequestData,
): Promise<Record<string, unknown>> {
  const id = generateId();
  const result = await pool.query(
    `INSERT INTO match_requests (
       id, senior_id, requested_by, status, service_category,
       region_city, region_district, start_date, end_date,
       schedule_notes, language_preference, gender_preference,
       requires_dementia_experience, requires_overnight_care,
       additional_notes, created_at, updated_at
     ) VALUES ($1,$2,$3,'CREATED',$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,NOW(),NOW())
     RETURNING *`,
    [
      id,
      data.seniorId,
      data.requestedBy,
      data.serviceCategory,
      data.regionCity,
      data.regionDistrict,
      data.startDate ?? null,
      data.endDate ?? null,
      data.scheduleNotes ?? null,
      data.languagePreference ?? null,
      data.genderPreference ?? null,
      data.requiresDementiaExperience ?? false,
      data.requiresOvernightCare ?? false,
      data.additionalNotes ?? null,
    ],
  );
  return result.rows[0];
}

export async function searchCandidates(
  pool: Pool,
  matchRequestId: string,
): Promise<Record<string, unknown>[]> {
  const mrResult = await pool.query(
    'SELECT * FROM match_requests WHERE id = $1',
    [matchRequestId],
  );
  if (!mrResult.rows[0]) {
    throw new Error(`MatchRequest not found: ${matchRequestId}`);
  }

  const matchRequest = mrResult.rows[0] as Record<string, unknown>;
  const currentStatus = matchRequest.status as MatchRequestStatus;

  if (!matchRequestMachine.canTransition(currentStatus, 'SEARCHING')) {
    throw new Error(
      `Cannot begin search for match request in status: ${currentStatus}`,
    );
  }

  // Transition to SEARCHING
  await pool.query(
    `UPDATE match_requests SET status = 'SEARCHING', updated_at = NOW() WHERE id = $1`,
    [matchRequestId],
  );

  // Fetch approved caregivers with their slots and service types
  const [appResult, slotResult, serviceResult, regionResult] = await Promise.all([
    pool.query(
      `SELECT ca.*, po.id AS provider_org_id
       FROM caregiver_applications ca
       LEFT JOIN provider_organizations po ON po.id = ca.provider_id
       WHERE ca.status IN ('APPROVED_PRIVATE_PAY', 'APPROVED_UNDER_PROVIDER')`,
    ),
    pool.query(
      `SELECT * FROM availability_slots
       WHERE application_id IN (
         SELECT id FROM caregiver_applications
         WHERE status IN ('APPROVED_PRIVATE_PAY', 'APPROVED_UNDER_PROVIDER')
       ) AND is_active = true`,
    ),
    pool.query(
      `SELECT * FROM service_types
       WHERE application_id IN (
         SELECT id FROM caregiver_applications
         WHERE status IN ('APPROVED_PRIVATE_PAY', 'APPROVED_UNDER_PROVIDER')
       ) AND is_active = true`,
    ),
    pool.query(
      `SELECT sr.* FROM service_regions sr
       JOIN provider_organizations po ON po.id = sr.provider_id
       JOIN caregiver_applications ca ON ca.provider_id = po.id
       WHERE ca.status IN ('APPROVED_PRIVATE_PAY', 'APPROVED_UNDER_PROVIDER')
         AND sr.is_active = true`,
    ),
  ]);

  // Group slots, service types, and regions by application id / provider id
  const slotsByApp = new Map<string, Array<{ dayOfWeek: string }>>();
  for (const slot of slotResult.rows) {
    const list = slotsByApp.get(slot.application_id) ?? [];
    list.push({ dayOfWeek: slot.day_of_week });
    slotsByApp.set(slot.application_id, list);
  }

  const servicesByApp = new Map<string, Array<{ category: string }>>();
  for (const svc of serviceResult.rows) {
    const list = servicesByApp.get(svc.application_id) ?? [];
    list.push({ category: svc.category });
    servicesByApp.set(svc.application_id, list);
  }

  const regionsByProvider = new Map<string, Array<{ city: string; district: string }>>();
  for (const reg of regionResult.rows) {
    const list = regionsByProvider.get(reg.provider_id) ?? [];
    list.push({ city: reg.city, district: reg.district });
    regionsByProvider.set(reg.provider_id, list);
  }

  const criteria: ScoringCriteria = {
    regionCity: matchRequest.region_city as string,
    regionDistrict: matchRequest.region_district as string,
    serviceCategory: matchRequest.service_category as ServiceCategory,
    languagePreference: matchRequest.language_preference as string | null,
    genderPreference: matchRequest.gender_preference as Gender | null,
    requiresDementiaExperience: matchRequest.requires_dementia_experience as boolean,
    requiresOvernightCare: matchRequest.requires_overnight_care as boolean,
  };

  // Score each candidate
  const scored = appResult.rows
    .map((app: Record<string, unknown>) => {
      const serviceRegions = regionsByProvider.get(app.provider_id as string) ?? [];

      const candidateData: CandidateData = {
        application: {
          ...app,
          availabilitySlots: slotsByApp.get(app.id as string) ?? [],
          serviceTypes: servicesByApp.get(app.id as string) ?? [],
          languagesSpoken: (app.languages_spoken as string) ?? '',
          preferredGender: (app.preferred_gender as string | null),
          hasDementiaExperience: app.has_dementia_experience as boolean,
          hasOvernightAvailability: app.has_overnight_availability as boolean,
          smokingStatus: app.smoking_status as boolean,
          petFriendly: app.pet_friendly as boolean,
        },
        serviceRegions,
      };

      const breakdown = scoreCandidateAtomic(criteria, candidateData);
      return { app, breakdown };
    })
    .filter((c) => c.breakdown.total > 0)
    .sort((a, b) => b.breakdown.total - a.breakdown.total);

  // Delete previous recommendations
  await pool.query('DELETE FROM match_recommendations WHERE match_request_id = $1', [matchRequestId]);

  // Persist recommendations in a transaction
  const client = await pool.connect();
  let recommendations: Record<string, unknown>[] = [];
  try {
    await client.query('BEGIN');

    for (let index = 0; index < scored.length; index++) {
      const { app, breakdown } = scored[index]!;
      const recId = generateId();
      const recResult = await client.query(
        `INSERT INTO match_recommendations (
           id, match_request_id, caregiver_application_id, score,
           score_breakdown, rank, is_selected, created_at, updated_at
         ) VALUES ($1,$2,$3,$4,$5,$6,false,NOW(),NOW())
         RETURNING *`,
        [
          recId,
          matchRequestId,
          app.id,
          breakdown.total,
          JSON.stringify(breakdown),
          index + 1,
        ],
      );
      recommendations.push(recResult.rows[0]);
    }

    // Transition to RECOMMENDATIONS_READY
    await client.query(
      `UPDATE match_requests SET status = 'RECOMMENDATIONS_READY', updated_at = NOW() WHERE id = $1`,
      [matchRequestId],
    );

    await client.query('COMMIT');
  } catch (err) {
    await client.query('ROLLBACK');
    throw err;
  } finally {
    client.release();
  }

  return recommendations;
}

export async function selectRecommendation(
  pool: Pool,
  recommendationId: string,
): Promise<Record<string, unknown>> {
  const recResult = await pool.query(
    'SELECT * FROM match_recommendations WHERE id = $1',
    [recommendationId],
  );
  if (!recResult.rows[0]) {
    throw new Error(`MatchRecommendation not found: ${recommendationId}`);
  }

  const recommendation = recResult.rows[0] as Record<string, unknown>;

  const mrResult = await pool.query(
    'SELECT * FROM match_requests WHERE id = $1',
    [recommendation.match_request_id],
  );
  if (!mrResult.rows[0]) {
    throw new Error(`MatchRequest not found: ${recommendation.match_request_id as string}`);
  }

  const matchRequest = mrResult.rows[0] as Record<string, unknown>;
  const currentStatus = matchRequest.status as MatchRequestStatus;

  if (!matchRequestMachine.canTransition(currentStatus, 'SELECTED')) {
    throw new Error(
      `Cannot select recommendation for match request in status: ${currentStatus}`,
    );
  }

  const client = await pool.connect();
  let updated: Record<string, unknown> = {};
  try {
    await client.query('BEGIN');

    const updatedRec = await client.query(
      `UPDATE match_recommendations
       SET is_selected = true, selected_at = NOW(), updated_at = NOW()
       WHERE id = $1
       RETURNING *`,
      [recommendationId],
    );
    updated = updatedRec.rows[0];

    await client.query(
      `UPDATE match_requests SET status = 'SELECTED', updated_at = NOW() WHERE id = $1`,
      [recommendation.match_request_id],
    );

    await client.query('COMMIT');
  } catch (err) {
    await client.query('ROLLBACK');
    throw err;
  } finally {
    client.release();
  }

  return updated;
}

export async function getMatchRequest(
  pool: Pool,
  id: string,
): Promise<MatchRequestWithRecommendations | null> {
  const [mrResult, recResult] = await Promise.all([
    pool.query('SELECT * FROM match_requests WHERE id = $1', [id]),
    pool.query(
      `SELECT mr.*, ca.*,
              mr.id AS rec_id
       FROM match_recommendations mr
       JOIN caregiver_applications ca ON ca.id = mr.caregiver_application_id
       WHERE mr.match_request_id = $1
       ORDER BY mr.rank ASC`,
      [id],
    ),
  ]);

  if (!mrResult.rows[0]) return null;

  // Build recommendations with nested caregiverApplication
  const recommendations = recResult.rows.map((row: Record<string, unknown>) => {
    return {
      ...row,
      caregiverApplication: { ...row },
    };
  });

  return {
    ...mrResult.rows[0],
    recommendations,
  };
}

export async function listMatchRequests(
  pool: Pool,
  seniorId: string | undefined,
  pagination: Pagination,
): Promise<PaginatedResult<Record<string, unknown>>> {
  const conditions: string[] = [];
  const values: unknown[] = [];
  let paramIdx = 1;

  if (seniorId !== undefined) {
    conditions.push(`senior_id = $${paramIdx}`);
    values.push(seniorId);
    paramIdx++;
  }

  const where = conditions.length > 0 ? `WHERE ${conditions.join(' AND ')}` : '';
  const offset = (pagination.page - 1) * pagination.limit;

  const [dataResult, countResult] = await Promise.all([
    pool.query(
      `SELECT * FROM match_requests ${where} ORDER BY created_at DESC LIMIT $${paramIdx} OFFSET $${paramIdx + 1}`,
      [...values, pagination.limit, offset],
    ),
    pool.query(`SELECT COUNT(*) FROM match_requests ${where}`, values),
  ]);

  return {
    data: dataResult.rows,
    total: parseInt(countResult.rows[0].count, 10),
  };
}

export const MatchService = {
  createMatchRequest,
  searchCandidates,
  scoreCandidateAtomic,
  selectRecommendation,
  getMatchRequest,
  listMatchRequests,
};
