import { NextRequest } from 'next/server';
import { pool } from '@bominal-senior/db';
import { apiSuccess, apiError, parsePagination } from '@bominal-senior/types/src/api-helpers';
import { listApplications } from '@/lib/services';

// TODO: const session = await auth()
const DEV_TENANT_ID = 'dev-provider-org-001';

export async function GET(request: NextRequest) {
  try {
    // TODO: const session = await auth()
    const { searchParams } = new URL(request.url);
    const { page, limit } = parsePagination(searchParams);

    // List pending applications under this provider
    const result = await listApplications(
      pool,
      {
        providerId: DEV_TENANT_ID,
        status: 'SUBMITTED' as never,
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
    console.error('[GET /api/caregivers/applications]', error);
    return apiError('Failed to fetch applications', 500);
  }
}
