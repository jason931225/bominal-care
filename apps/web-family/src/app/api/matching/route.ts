import { NextRequest } from 'next/server';
import { pool } from '@bominal-senior/db';
import { auth } from '@bominal-senior/auth';
import { apiSuccess, apiError, parsePagination } from '@bominal-senior/types/src/api-helpers';
import { MatchRequestInputSchema } from '@bominal-senior/types';
import { createMatchRequest, listMatchRequests } from '@/lib/services';

export async function GET(request: NextRequest) {
  try {
    const session = await auth();
    if (!session?.user?.id) {
      return apiError('Unauthorized', 401);
    }

    const { searchParams } = new URL(request.url);
    const { page, limit } = parsePagination(searchParams);
    const seniorId = searchParams.get('seniorId') ?? undefined;

    const result = await listMatchRequests(pool, seniorId, { page, limit });

    return apiSuccess(result.data, {
      total: result.total,
      page,
      limit,
      totalPages: Math.ceil(result.total / limit),
    });
  } catch (error) {
    console.error('[GET /api/matching]', error);
    return apiError('Failed to fetch match requests', 500);
  }
}

export async function POST(request: NextRequest) {
  try {
    const session = await auth();
    if (!session?.user?.id) {
      return apiError('Unauthorized', 401);
    }
    const userId = session.user.id;

    const body = await request.json();
    const parsed = MatchRequestInputSchema.safeParse(body);

    if (!parsed.success) {
      return apiError(parsed.error.errors.map((e) => e.message).join(', '), 422);
    }

    // Ensure senior exists
    const seniorResult = await pool.query(
      `SELECT id FROM senior_profiles WHERE person_id = $1`,
      [parsed.data.seniorId],
    );

    if (seniorResult.rows.length === 0) {
      return apiError('Senior profile not found', 404);
    }

    const seniorProfile = seniorResult.rows[0] as { id: string };

    const matchRequest = await createMatchRequest(pool, {
      seniorId: seniorProfile.id,
      requestedBy: userId,
      serviceCategory: parsed.data.serviceCategory,
      regionCity: parsed.data.regionCity,
      regionDistrict: parsed.data.regionDistrict,
      startDate: parsed.data.startDate,
      endDate: parsed.data.endDate,
      scheduleNotes: parsed.data.scheduleNotes,
      languagePreference: parsed.data.languagePreference,
      genderPreference: parsed.data.genderPreference as never,
      requiresDementiaExperience: parsed.data.requiresDementiaExperience,
      requiresOvernightCare: parsed.data.requiresOvernightCare,
      additionalNotes: parsed.data.additionalNotes,
    });

    return apiSuccess(matchRequest);
  } catch (error) {
    console.error('[POST /api/matching]', error);
    return apiError('Failed to create match request', 500);
  }
}
