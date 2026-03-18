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
    const status = searchParams.get('status') ?? undefined;

    const conditions: string[] = ['ca.provider_id = $1'];
    const values: unknown[] = [DEV_TENANT_ID];
    let paramIdx = 2;

    if (status !== undefined) {
      conditions.push(`ca.status = $${paramIdx}`);
      values.push(status);
      paramIdx++;
    } else {
      conditions.push(`ca.status IN ('APPROVED_PRIVATE_PAY', 'APPROVED_UNDER_PROVIDER')`);
    }

    const where = `WHERE ${conditions.join(' AND ')}`;

    const [dataResult, countResult] = await Promise.all([
      pool.query(
        `SELECT ca.*,
           COALESCE(
             json_agg(DISTINCT cc.*) FILTER (WHERE cc.id IS NOT NULL),
             '[]'
           ) AS credentials,
           COALESCE(
             json_agg(DISTINCT st.*) FILTER (WHERE st.id IS NOT NULL),
             '[]'
           ) AS service_types
         FROM caregiver_applications ca
         LEFT JOIN caregiver_credentials cc ON cc.application_id = ca.id
         LEFT JOIN service_types st ON st.application_id = ca.id
         ${where}
         GROUP BY ca.id
         ORDER BY ca.created_at DESC
         LIMIT $${paramIdx} OFFSET $${paramIdx + 1}`,
        [...values, limit, skip],
      ),
      pool.query(
        `SELECT COUNT(*) FROM caregiver_applications ca ${where}`,
        values,
      ),
    ]);

    const total = parseInt(countResult.rows[0].count, 10);

    return apiSuccess(dataResult.rows, {
      total,
      page,
      limit,
      totalPages: Math.ceil(total / limit),
    });
  } catch (error) {
    console.error('[GET /api/caregivers]', error);
    return apiError('Failed to fetch caregivers', 500);
  }
}
