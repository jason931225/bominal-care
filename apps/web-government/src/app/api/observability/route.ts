import { NextRequest } from 'next/server';
import { pool } from '@bominal-senior/db';
import { auth } from '@bominal-senior/auth';
import { apiSuccess, apiError, parsePagination } from '@bominal-senior/types/src/api-helpers';
import { ObservabilityService } from '@bominal-senior/db/src/services';

export async function GET(request: NextRequest) {
  try {
    const session = await auth();
    if (!session?.user?.id) {
      return apiError('Unauthorized', 401);
    }

    const { searchParams } = new URL(request.url);
    const { page, limit } = parsePagination(searchParams);
    const severity = searchParams.get('severity') ?? undefined;
    const eventType = searchParams.get('eventType') ?? undefined;

    const [signals, stats] = await Promise.all([
      ObservabilityService.listSignals(
        pool,
        {
          severity: severity as Parameters<typeof ObservabilityService.listSignals>[1]['severity'],
          eventType: eventType as Parameters<typeof ObservabilityService.listSignals>[1]['eventType'],
        },
        { page, limit },
      ),
      ObservabilityService.getDashboardStats(pool),
    ]);

    return apiSuccess({
      signals: signals.data,
      stats,
      meta: { total: signals.total, page, limit },
    });
  } catch (error) {
    console.error('[GET /api/observability]', error);
    return apiError('Failed to fetch observability data', 500);
  }
}
