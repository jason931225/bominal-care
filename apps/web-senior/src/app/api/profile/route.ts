import { NextRequest } from 'next/server';
import { pool } from '@bominal-senior/db';
import { auth } from '@bominal-senior/auth';
import { apiSuccess, apiError } from '@bominal-senior/types/src/api-helpers';
import { PersonProfileInputSchema } from '@bominal-senior/types';
import {
  getPersonProfileByUserId,
  updatePersonProfile,
} from '@/lib/services';

export async function GET(_request: NextRequest) {
  try {
    const session = await auth();
    if (!session?.user?.id) {
      return apiError('Unauthorized', 401);
    }
    const userId = session.user.id;

    const profile = await getPersonProfileByUserId(pool, userId);

    if (!profile) {
      return apiError('Profile not found', 404);
    }

    return apiSuccess(profile);
  } catch (error) {
    console.error('[GET /api/profile]', error);
    return apiError('Failed to fetch profile', 500);
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

    const profile = await getPersonProfileByUserId(pool, userId);

    if (!profile) {
      return apiError('Profile not found', 404);
    }

    // Validate only the fields being patched
    const allowedFields = PersonProfileInputSchema.partial();
    const parsed = allowedFields.safeParse(body);

    if (!parsed.success) {
      return apiError(parsed.error.errors.map((e) => e.message).join(', '), 422);
    }

    const updated = await updatePersonProfile(pool, profile.id, parsed.data);

    return apiSuccess(updated);
  } catch (error) {
    console.error('[PATCH /api/profile]', error);
    return apiError('Failed to update profile', 500);
  }
}
