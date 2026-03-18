import { NextRequest } from 'next/server';
import { pool } from '@bominal-senior/db';
import { apiSuccess, apiError } from '@bominal-senior/types/src/api-helpers';

export async function GET(
  _request: NextRequest,
  { params }: { params: Promise<{ id: string }> },
) {
  try {
    // TODO: const session = await auth()
    const { id } = await params;

    const [caseResult, stepsResult] = await Promise.all([
      pool.query(
        'SELECT * FROM eligibility_cases WHERE id = $1',
        [id],
      ),
      pool.query(
        'SELECT * FROM approval_steps WHERE entity_id = $1 ORDER BY step_order',
        [id],
      ),
    ]);

    if (caseResult.rows.length === 0) {
      return apiError('Eligibility case not found', 404);
    }

    const data = {
      ...caseResult.rows[0],
      approval_steps: stepsResult.rows,
    };

    return apiSuccess(data);
  } catch (error) {
    console.error('[GET /api/eligibility/[id]]', error);
    return apiError('Failed to fetch eligibility case', 500);
  }
}
