import { NextRequest } from 'next/server';
import { pool } from '@bominal-senior/db';
import { auth } from '@bominal-senior/auth';
import { apiSuccess, apiError, parsePagination } from '@bominal-senior/types/src/api-helpers';
import { getPersonProfileByUserId, listSignals } from '@/lib/services';

export async function GET(request: NextRequest) {
  try {
    const session = await auth();
    if (!session?.user?.id) {
      return apiError('Unauthorized', 401);
    }
    const userId = session.user.id;

    const { searchParams } = new URL(request.url);
    const { page, limit } = parsePagination(searchParams);
    const seniorPersonId = searchParams.get('seniorPersonId');
    const severity = searchParams.get('severity') ?? undefined;

    if (!seniorPersonId) {
      return apiError('seniorPersonId query parameter is required', 422);
    }

    // Verify family relationship
    const familyProfile = await getPersonProfileByUserId(pool, userId);

    if (!familyProfile) {
      return apiError('Family profile not found', 404);
    }

    const relationshipResult = await pool.query(
      `SELECT id FROM family_relationships
       WHERE senior_person_id = $1 AND family_person_id = $2
       LIMIT 1`,
      [seniorPersonId, familyProfile.id],
    );

    if (relationshipResult.rows.length === 0) {
      return apiError('No linked relationship to this senior', 403);
    }

    const result = await listSignals(
      pool,
      {
        subjectPersonId: seniorPersonId,
        ...(severity !== undefined && { severity: severity as never }),
      },
      { page, limit },
    );

    return apiSuccess(result.data, {
      total: result.total,
      page,
      limit,
      totalPages: Math.ceil(result.total / limit),
    });
  } catch (error) {
    console.error('[GET /api/observability]', error);
    return apiError('Failed to fetch observability signals', 500);
  }
}
