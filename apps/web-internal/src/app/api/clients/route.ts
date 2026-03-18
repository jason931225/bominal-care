import { NextRequest } from 'next/server';
import { pool } from '@bominal-senior/db';
import { apiSuccess, apiError, parsePagination } from '@bominal-senior/types/src/api-helpers';

// TODO: const session = await auth()
const DEV_TENANT_ID = 'dev-provider-org-001';

export async function GET(request: NextRequest) {
  try {
    // TODO: const session = await auth()
    const { searchParams } = new URL(request.url);
    const { page, limit, skip } = parsePagination(searchParams);

    // Verify provider org exists
    const orgResult = await pool.query(
      'SELECT id FROM provider_organizations WHERE id = $1',
      [DEV_TENANT_ID],
    );

    if (!orgResult.rows[0]) {
      return apiError('Provider organization not found', 404);
    }

    // Clients are seniors with active care plans under this provider
    const [dataResult, countResult] = await Promise.all([
      pool.query(
        `SELECT
           cp.*,
           sp.id          AS sp_id,
           sp.person_id   AS sp_person_id,
           pp.id          AS pp_id,
           pp.korean_name AS pp_korean_name,
           pp.english_name AS pp_english_name,
           pp.phone       AS pp_phone,
           pp.city        AS pp_city,
           pp.district    AS pp_district
         FROM care_plans cp
         JOIN senior_profiles sp ON sp.id = cp.senior_id
         JOIN person_profiles pp ON pp.id = sp.person_id
         WHERE cp.provider_id = $1
         ORDER BY cp.created_at DESC
         LIMIT $2 OFFSET $3`,
        [DEV_TENANT_ID, limit, skip],
      ),
      pool.query(
        'SELECT COUNT(*) FROM care_plans WHERE provider_id = $1',
        [DEV_TENANT_ID],
      ),
    ]);

    const total = parseInt(countResult.rows[0].count, 10);

    const carePlans = dataResult.rows.map((row: Record<string, unknown>) => ({
      ...extractCarePlanFields(row),
      seniorProfile: {
        id: row.sp_id,
        personId: row.sp_person_id,
        personProfile: {
          id: row.pp_id,
          koreanName: row.pp_korean_name,
          englishName: row.pp_english_name,
          phone: row.pp_phone,
          city: row.pp_city,
          district: row.pp_district,
        },
      },
    }));

    return apiSuccess(carePlans, {
      total,
      page,
      limit,
      totalPages: Math.ceil(total / limit),
    });
  } catch (error) {
    console.error('[GET /api/clients]', error);
    return apiError('Failed to fetch clients', 500);
  }
}

/** Extract care_plan columns from a joined row, excluding aliased join columns. */
function extractCarePlanFields(row: Record<string, unknown>): Record<string, unknown> {
  const joinPrefixes = ['sp_', 'pp_'];
  const result: Record<string, unknown> = {};
  for (const [key, value] of Object.entries(row)) {
    if (!joinPrefixes.some((prefix) => key.startsWith(prefix))) {
      result[key] = value;
    }
  }
  return result;
}
