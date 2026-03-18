import { NextRequest } from 'next/server';
import { pool } from '@bominal-senior/db';
import { auth } from '@bominal-senior/auth';
import { apiSuccess, apiError } from '@bominal-senior/types/src/api-helpers';
import { MedicationEventUpdateSchema } from '@bominal-senior/types';
import {
  getPersonProfileByUserId,
  getTodayEvents,
  updateEventStatus,
} from '@/lib/services';

export async function GET(_request: NextRequest) {
  try {
    const session = await auth();
    if (!session?.user?.id) {
      return apiError('Unauthorized', 401);
    }
    const userId = session.user.id;

    const personProfile = await getPersonProfileByUserId(pool, userId);

    if (!personProfile) {
      return apiError('Person profile not found', 404);
    }

    const events = await getTodayEvents(pool, personProfile.id);

    return apiSuccess(events);
  } catch (error) {
    console.error('[GET /api/medications/events]', error);
    return apiError('Failed to fetch medication events', 500);
  }
}

export async function PATCH(request: NextRequest) {
  try {
    const session = await auth();
    if (!session?.user?.id) {
      return apiError('Unauthorized', 401);
    }
    const userId = session.user.id;

    const body = await request.json();

    if (!body.eventId || typeof body.eventId !== 'string') {
      return apiError('eventId is required', 422);
    }

    const parsed = MedicationEventUpdateSchema.safeParse(body);

    if (!parsed.success) {
      return apiError(parsed.error.errors.map((e) => e.message).join(', '), 422);
    }

    const updated = await updateEventStatus(
      pool,
      body.eventId,
      parsed.data.status,
      userId,
      parsed.data.notes,
    );

    if (!updated) {
      return apiError('Medication event not found', 404);
    }

    return apiSuccess(updated);
  } catch (error) {
    console.error('[PATCH /api/medications/events]', error);
    return apiError('Failed to update medication event', 500);
  }
}
