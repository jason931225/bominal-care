import { NextRequest } from 'next/server';
import { pool } from '@bominal-senior/db';
import { auth } from '@bominal-senior/auth';
import { apiSuccess, apiError } from '@bominal-senior/types/src/api-helpers';
import { IncidentInputSchema } from '@bominal-senior/types';
import { createSignal } from '@/lib/services';

export async function POST(request: NextRequest) {
  try {
    const session = await auth();
    if (!session?.user?.id) {
      return apiError('Unauthorized', 401);
    }
    const userId = session.user.id;

    const body = await request.json();
    const parsed = IncidentInputSchema.safeParse(body);

    if (!parsed.success) {
      return apiError(parsed.error.errors.map((e) => e.message).join(', '), 422);
    }

    // If visitId provided, verify caregiver is assigned to that visit
    if (parsed.data.visitId) {
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
        `SELECT caregiver_id FROM visits
         WHERE id = $1`,
        [parsed.data.visitId],
      );

      if (!visitResult.rows[0]) {
        return apiError('Visit not found', 404);
      }

      if (visitResult.rows[0].caregiver_id !== appResult.rows[0].id) {
        return apiError('Not authorized to report incident for this visit', 403);
      }
    }

    const signal = await createSignal(pool, {
      eventType: 'INCIDENT_CREATED',
      severity: parsed.data.severity === 'LOW' ? 'INFO'
        : parsed.data.severity === 'MEDIUM' ? 'WARNING'
        : parsed.data.severity === 'HIGH' ? 'ALERT'
        : 'CRITICAL',
      actorUserId: userId,
      entityType: parsed.data.visitId ? 'visit' : undefined,
      entityId: parsed.data.visitId ?? undefined,
      message: parsed.data.title,
      metadata: {
        description: parsed.data.description,
        occurredAt: parsed.data.occurredAt,
        visitId: parsed.data.visitId,
      },
    });

    return apiSuccess(signal);
  } catch (error) {
    console.error('[POST /api/incidents]', error);
    return apiError('Failed to report incident', 500);
  }
}
