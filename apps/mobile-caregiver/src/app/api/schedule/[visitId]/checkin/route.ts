import { NextRequest } from 'next/server';
import { pool } from '@bominal-senior/db';
import { auth } from '@bominal-senior/auth';
import { apiSuccess, apiError } from '@bominal-senior/types/src/api-helpers';
import { VisitCheckInSchema } from '@bominal-senior/types';
import { checkIn } from '@/lib/services';

export async function POST(
  request: NextRequest,
  { params }: { params: Promise<{ visitId: string }> },
) {
  try {
    const session = await auth();
    if (!session?.user?.id) {
      return apiError('Unauthorized', 401);
    }
    const userId = session.user.id;

    const { visitId } = await params;
    const body = await request.json();
    const parsed = VisitCheckInSchema.safeParse(body);

    if (!parsed.success) {
      return apiError(parsed.error.errors.map((e) => e.message).join(', '), 422);
    }

    // Verify caregiver owns this visit
    const appResult = await pool.query(
      `SELECT id FROM caregiver_applications
       WHERE user_id = $1
       LIMIT 1`,
      [userId],
    );

    if (!appResult.rows[0]) {
      return apiError('No caregiver application found', 404);
    }

    const visitResult = await pool.query(
      `SELECT id, caregiver_id FROM visits
       WHERE id = $1`,
      [visitId],
    );

    if (!visitResult.rows[0]) {
      return apiError('Visit not found', 404);
    }

    if (visitResult.rows[0].caregiver_id !== appResult.rows[0].id) {
      return apiError('Not authorized to check in to this visit', 403);
    }

    const updated = await checkIn(pool, visitId, {
      latitude: parsed.data.latitude,
      longitude: parsed.data.longitude,
    });

    return apiSuccess(updated);
  } catch (error) {
    console.error('[POST /api/schedule/[visitId]/checkin]', error);
    if (error instanceof Error) {
      return apiError(error.message, 400);
    }
    return apiError('Failed to check in to visit', 500);
  }
}
