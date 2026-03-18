import { NextRequest } from 'next/server';
import { pool } from '@bominal-senior/db';
import { apiSuccess, apiError } from '@bominal-senior/types/src/api-helpers';
import { selectRecommendation } from '@/lib/services';

export async function POST(
  request: NextRequest,
  { params }: { params: Promise<{ requestId: string }> },
) {
  try {
    // TODO: const session = await auth()
    const { requestId } = await params;
    const body = await request.json();

    if (!body.recommendationId || typeof body.recommendationId !== 'string') {
      return apiError('recommendationId is required', 422);
    }

    // Verify the recommendation belongs to this match request
    const recResult = await pool.query(
      `SELECT id, match_request_id FROM match_recommendations WHERE id = $1`,
      [body.recommendationId],
    );

    if (recResult.rows.length === 0) {
      return apiError('Recommendation not found', 404);
    }

    const recommendation = recResult.rows[0] as { id: string; match_request_id: string };

    if (recommendation.match_request_id !== requestId) {
      return apiError('Recommendation does not belong to this match request', 400);
    }

    const selected = await selectRecommendation(pool, body.recommendationId);

    return apiSuccess(selected);
  } catch (error) {
    console.error('[POST /api/matching/[requestId]/select]', error);
    return apiError('Failed to select recommendation', 500);
  }
}
