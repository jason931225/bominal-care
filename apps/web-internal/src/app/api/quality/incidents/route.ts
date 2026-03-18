import { NextRequest } from 'next/server';
import { pool } from '@bominal-senior/db';
import { apiSuccess, apiError, parsePagination } from '@bominal-senior/types/src/api-helpers';
import { createSignal, listSignals } from '@/lib/services';

// TODO: const session = await auth()
// TODO: Use tenantId from session to scope signals by provider once listSignals supports provider filtering

export async function GET(request: NextRequest) {
  try {
    // TODO: const session = await auth()
    const { searchParams } = new URL(request.url);
    const { page, limit } = parsePagination(searchParams);
    const severity = searchParams.get('severity') ?? undefined;
    const acknowledged = searchParams.get('acknowledged');

    const result = await listSignals(
      pool,
      {
        ...(severity !== undefined && { severity: severity as never }),
        ...(acknowledged === 'true' && { acknowledged: true }),
        ...(acknowledged === 'false' && { acknowledged: false }),
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
    console.error('[GET /api/quality/incidents]', error);
    return apiError('Failed to fetch incidents', 500);
  }
}

export async function POST(request: NextRequest) {
  try {
    // TODO: const session = await auth()
    const body = await request.json();

    if (!body.eventType || !body.message) {
      return apiError('eventType and message are required', 422);
    }

    const signal = await createSignal(pool, {
      eventType: body.eventType,
      severity: body.severity,
      subjectPersonId: body.subjectPersonId,
      actorUserId: body.actorUserId,
      entityType: body.entityType,
      entityId: body.entityId,
      message: body.message,
      metadata: body.metadata,
    });

    return apiSuccess(signal);
  } catch (error) {
    console.error('[POST /api/quality/incidents]', error);
    return apiError('Failed to create incident signal', 500);
  }
}
