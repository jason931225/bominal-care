import { NextRequest } from 'next/server';
import { pool } from '@bominal-senior/db';
import { apiSuccess, apiError, parsePagination } from '@bominal-senior/types/src/api-helpers';
import { scheduleVisit } from '@/lib/services';

// TODO: const session = await auth()
const DEV_TENANT_ID = 'dev-provider-org-001';

export async function GET(request: NextRequest) {
  try {
    // TODO: const session = await auth()
    const { searchParams } = new URL(request.url);
    const { page, limit, skip } = parsePagination(searchParams);
    const status = searchParams.get('status') ?? undefined;
    const fromDate = searchParams.get('from') ? new Date(searchParams.get('from')!) : undefined;
    const toDate = searchParams.get('to') ? new Date(searchParams.get('to')!) : undefined;

    // The listVisits service filters on visits table directly, but we need
    // to filter by provider through the care_plan join. Use raw query for
    // the provider-scoped listing with nested relations.
    const conditions: string[] = ['cp.provider_id = $1'];
    const values: unknown[] = [DEV_TENANT_ID];
    let paramIdx = 2;

    if (status !== undefined) {
      conditions.push(`v.status = $${paramIdx}`);
      values.push(status);
      paramIdx++;
    }
    if (fromDate !== undefined) {
      conditions.push(`v.scheduled_start >= $${paramIdx}`);
      values.push(fromDate);
      paramIdx++;
    }
    if (toDate !== undefined) {
      conditions.push(`v.scheduled_start <= $${paramIdx}`);
      values.push(toDate);
      paramIdx++;
    }

    const where = `WHERE ${conditions.join(' AND ')}`;

    const [dataResult, countResult] = await Promise.all([
      pool.query(
        `SELECT
           v.*,
           cp.id            AS cp_id,
           cp.senior_id     AS cp_senior_id,
           cp.title         AS cp_title,
           sp.id            AS sp_id,
           sp.person_id     AS sp_person_id,
           pp.korean_name   AS pp_korean_name,
           pp.english_name  AS pp_english_name,
           ca.id            AS cg_id,
           ca.bio           AS cg_bio,
           ca.languages_spoken AS cg_languages_spoken
         FROM visits v
         JOIN care_plans cp ON cp.id = v.care_plan_id
         JOIN senior_profiles sp ON sp.id = cp.senior_id
         JOIN person_profiles pp ON pp.id = sp.person_id
         LEFT JOIN caregiver_applications ca ON ca.id = v.caregiver_id
         ${where}
         ORDER BY v.scheduled_start ASC
         LIMIT $${paramIdx} OFFSET $${paramIdx + 1}`,
        [...values, limit, skip],
      ),
      pool.query(
        `SELECT COUNT(*)
         FROM visits v
         JOIN care_plans cp ON cp.id = v.care_plan_id
         ${where}`,
        values,
      ),
    ]);

    const total = parseInt(countResult.rows[0].count, 10);

    const visits = dataResult.rows.map((row: Record<string, unknown>) => ({
      ...extractVisitFields(row),
      carePlan: {
        id: row.cp_id,
        seniorId: row.cp_senior_id,
        title: row.cp_title,
        seniorProfile: {
          id: row.sp_id,
          personId: row.sp_person_id,
          personProfile: {
            koreanName: row.pp_korean_name,
            englishName: row.pp_english_name,
          },
        },
      },
      caregiver: {
        id: row.cg_id,
        bio: row.cg_bio,
        languagesSpoken: row.cg_languages_spoken,
      },
    }));

    return apiSuccess(visits, {
      total,
      page,
      limit,
      totalPages: Math.ceil(total / limit),
    });
  } catch (error) {
    console.error('[GET /api/schedules]', error);
    return apiError('Failed to fetch schedules', 500);
  }
}

export async function POST(request: NextRequest) {
  try {
    // TODO: const session = await auth()
    const body = await request.json();

    if (!body.carePlanId || !body.caregiverId || !body.scheduledStart || !body.scheduledEnd) {
      return apiError('carePlanId, caregiverId, scheduledStart, and scheduledEnd are required', 422);
    }

    const visit = await scheduleVisit(pool, {
      carePlanId: body.carePlanId,
      caregiverId: body.caregiverId,
      scheduledStart: new Date(body.scheduledStart),
      scheduledEnd: new Date(body.scheduledEnd),
      tasks: body.tasks,
      notes: body.notes,
    });

    return apiSuccess(visit);
  } catch (error) {
    console.error('[POST /api/schedules]', error);
    return apiError('Failed to schedule visit', 500);
  }
}

/** Extract visit columns from a joined row, excluding aliased join columns. */
function extractVisitFields(row: Record<string, unknown>): Record<string, unknown> {
  const joinPrefixes = ['cp_', 'sp_', 'pp_', 'cg_'];
  const result: Record<string, unknown> = {};
  for (const [key, value] of Object.entries(row)) {
    if (!joinPrefixes.some((prefix) => key.startsWith(prefix))) {
      result[key] = value;
    }
  }
  return result;
}
