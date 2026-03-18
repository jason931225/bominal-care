import { NextRequest } from 'next/server';
import { pool } from '@bominal-senior/db';
import { auth } from '@bominal-senior/auth';
import { apiSuccess, apiError } from '@bominal-senior/types/src/api-helpers';
import { DailyObservationInputSchema } from '@bominal-senior/types';
import { createSignal } from '@/lib/services';

export async function POST(
  request: NextRequest,
  { params }: { params: Promise<{ clientId: string }> },
) {
  try {
    const session = await auth();
    if (!session?.user?.id) {
      return apiError('Unauthorized', 401);
    }
    const userId = session.user.id;

    const { clientId } = await params;
    const body = await request.json();
    const parsed = DailyObservationInputSchema.safeParse(body);

    if (!parsed.success) {
      return apiError(parsed.error.errors.map((e) => e.message).join(', '), 422);
    }

    // Verify caregiver has an active visit for this client's care plan
    const appResult = await pool.query(
      `SELECT id FROM caregiver_applications
       WHERE user_id = $1
       LIMIT 1`,
      [userId],
    );

    if (!appResult.rows[0]) {
      return apiError('No caregiver application found', 404);
    }

    const carePlanResult = await pool.query(
      `SELECT id, senior_id FROM care_plans
       WHERE id = $1`,
      [parsed.data.carePlanId],
    );

    if (!carePlanResult.rows[0]) {
      return apiError('Care plan not found', 404);
    }

    // Verify caregiver has a visit for this care plan
    const visitResult = await pool.query(
      `SELECT id FROM visits
       WHERE care_plan_id = $1
         AND caregiver_id = $2
         AND status IN ('IN_PROGRESS', 'COMPLETED')
       LIMIT 1`,
      [parsed.data.carePlanId, appResult.rows[0].id],
    );

    if (!visitResult.rows[0]) {
      return apiError('No active visit found for this care plan', 403);
    }

    const signal = await createSignal(pool, {
      eventType: 'SYMPTOM_REPORTED',
      severity: 'INFO',
      subjectPersonId: clientId,
      actorUserId: userId,
      entityType: 'care_plan',
      entityId: parsed.data.carePlanId,
      message: `Daily observation: ${parsed.data.category}`,
      metadata: {
        category: parsed.data.category,
        date: parsed.data.date,
        value: parsed.data.value,
        notes: parsed.data.notes,
        recordedBy: appResult.rows[0].id,
      },
    });

    return apiSuccess(signal);
  } catch (error) {
    console.error('[POST /api/clients/[clientId]/observations]', error);
    return apiError('Failed to create daily observation', 500);
  }
}
