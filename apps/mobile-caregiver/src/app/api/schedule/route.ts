import { NextRequest } from 'next/server';
import { pool } from '@bominal-senior/db';
import { auth } from '@bominal-senior/auth';
import { apiSuccess, apiError, parsePagination } from '@bominal-senior/types/src/api-helpers';
import { listVisits } from '@/lib/services';

export async function GET(request: NextRequest) {
  try {
    const session = await auth();
    if (!session?.user?.id) {
      return apiError('Unauthorized', 401);
    }
    const userId = session.user.id;

    const { searchParams } = new URL(request.url);
    const { page, limit } = parsePagination(searchParams);

    // Find the user's approved caregiver application
    const appResult = await pool.query(
      `SELECT id FROM caregiver_applications
       WHERE user_id = $1
         AND status IN ('APPROVED_PRIVATE_PAY', 'APPROVED_UNDER_PROVIDER')
       ORDER BY created_at DESC
       LIMIT 1`,
      [userId],
    );

    if (!appResult.rows[0]) {
      return apiError('No approved caregiver application found', 404);
    }

    const { data: visits, total } = await listVisits(
      pool,
      {
        caregiverId: appResult.rows[0].id,
        dateRange: { from: new Date(), to: new Date('2099-12-31') },
      },
      { page, limit },
    );

    return apiSuccess(visits, {
      total,
      page,
      limit,
      totalPages: Math.ceil(total / limit),
    });
  } catch (error) {
    console.error('[GET /api/schedule]', error);
    return apiError('Failed to fetch schedule', 500);
  }
}
