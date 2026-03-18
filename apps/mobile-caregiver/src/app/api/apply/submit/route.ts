import { NextRequest } from 'next/server';
import { pool } from '@bominal-senior/db';
import { auth } from '@bominal-senior/auth';
import { apiSuccess, apiError } from '@bominal-senior/types/src/api-helpers';
import { submitApplication } from '@/lib/services';

export async function POST(_request: NextRequest) {
  try {
    const session = await auth();
    if (!session?.user?.id) {
      return apiError('Unauthorized', 401);
    }
    const userId = session.user.id;

    const result = await pool.query(
      `SELECT id, status FROM caregiver_applications
       WHERE user_id = $1
       ORDER BY created_at DESC
       LIMIT 1`,
      [userId],
    );

    if (!result.rows[0]) {
      return apiError('Application not found', 404);
    }

    if (result.rows[0].status !== 'DRAFT') {
      return apiError('Only draft applications can be submitted', 400);
    }

    const submitted = await submitApplication(pool, result.rows[0].id);

    return apiSuccess(submitted);
  } catch (error) {
    console.error('[POST /api/apply/submit]', error);
    if (error instanceof Error) {
      return apiError(error.message, 400);
    }
    return apiError('Failed to submit application', 500);
  }
}
