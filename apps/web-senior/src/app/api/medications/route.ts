import { NextRequest } from 'next/server';
import { pool } from '@bominal-senior/db';
import { auth } from '@bominal-senior/auth';
import { apiSuccess, apiError } from '@bominal-senior/types/src/api-helpers';
import { MedicationInputSchema } from '@bominal-senior/types';
import {
  getPersonProfileByUserId,
  getPersonProfile,
  listMedications,
  createMedication,
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

    const medications = await listMedications(pool, personProfile.id);

    return apiSuccess(medications);
  } catch (error) {
    console.error('[GET /api/medications]', error);
    return apiError('Failed to fetch medications', 500);
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
    const parsed = MedicationInputSchema.safeParse(body);

    if (!parsed.success) {
      return apiError(parsed.error.errors.map((e) => e.message).join(', '), 422);
    }

    const { personId, ...rest } = parsed.data;

    const personProfile = await getPersonProfile(pool, personId);

    if (!personProfile) {
      return apiError('Person profile not found', 404);
    }

    const medication = await createMedication(pool, {
      personId,
      name: rest.name,
      dosage: rest.dosage,
      form: rest.form,
      frequency: rest.frequency,
      prescribedBy: rest.prescribedBy,
      startDate: rest.startDate,
      endDate: rest.endDate,
      sideEffects: rest.sideEffects,
      notes: rest.notes,
      createdBy: userId,
    });

    return apiSuccess(medication);
  } catch (error) {
    console.error('[POST /api/medications]', error);
    return apiError('Failed to create medication', 500);
  }
}
