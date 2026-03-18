import { NextRequest } from 'next/server';
import { pool } from '@bominal-senior/db';
import { apiSuccess, apiError } from '@bominal-senior/types/src/api-helpers';
import { getMatchRequest } from '@/lib/services';

export async function GET(
  _request: NextRequest,
  { params }: { params: Promise<{ requestId: string }> },
) {
  try {
    // TODO: const session = await auth()
    const { requestId } = await params;

    const matchRequest = await getMatchRequest(pool, requestId);

    if (!matchRequest) {
      return apiError('Match request not found', 404);
    }

    return apiSuccess(matchRequest);
  } catch (error) {
    console.error('[GET /api/matching/[requestId]]', error);
    return apiError('Failed to fetch match request', 500);
  }
}
