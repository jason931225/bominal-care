import { NextRequest } from 'next/server';
import { pool } from '@bominal-senior/db';
import { AuditService } from '@bominal-senior/db/src/services';
import { apiSuccess, apiError, parsePagination } from '@bominal-senior/types/src/api-helpers';
import type { AuditAction } from '@bominal-senior/db/src/types';

export async function GET(request: NextRequest) {
  try {
    // TODO: const session = await auth()
    const { searchParams } = new URL(request.url);
    const { page, limit } = parsePagination(searchParams);
    const action = searchParams.get('action') ?? undefined;
    const entityType = searchParams.get('entityType') ?? undefined;

    const result = await AuditService.listLogs(
      pool,
      {
        action: action as AuditAction | undefined,
        entityType,
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
    console.error('[GET /api/audit]', error);
    return apiError('Failed to fetch audit logs', 500);
  }
}
