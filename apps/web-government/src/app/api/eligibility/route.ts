import { NextRequest } from 'next/server';
import { pool } from '@bominal-senior/db';
import { apiSuccess, apiError, parsePagination } from '@bominal-senior/types/src/api-helpers';

export async function GET(request: NextRequest) {
  try {
    // TODO: const session = await auth()
    const { searchParams } = new URL(request.url);
    const { page, limit, skip } = parsePagination(searchParams);
    const status = searchParams.get('status') ?? undefined;
    const type = searchParams.get('type') ?? undefined;

    const conditions: string[] = [];
    const values: unknown[] = [];
    let paramIdx = 1;

    if (status !== undefined) {
      conditions.push(`status = $${paramIdx}`);
      values.push(status);
      paramIdx++;
    }
    if (type !== undefined) {
      conditions.push(`type = $${paramIdx}`);
      values.push(type);
      paramIdx++;
    }

    const where = conditions.length > 0 ? `WHERE ${conditions.join(' AND ')}` : '';

    const [dataResult, countResult] = await Promise.all([
      pool.query(
        `SELECT * FROM eligibility_cases ${where}
         ORDER BY created_at DESC
         LIMIT $${paramIdx} OFFSET $${paramIdx + 1}`,
        [...values, limit, skip],
      ),
      pool.query(`SELECT COUNT(*) FROM eligibility_cases ${where}`, values),
    ]);

    const total = parseInt(countResult.rows[0].count, 10);
    return apiSuccess(dataResult.rows, { total, page, limit, totalPages: Math.ceil(total / limit) });
  } catch (error) {
    console.error('[GET /api/eligibility]', error);
    return apiError('Failed to fetch eligibility cases', 500);
  }
}
