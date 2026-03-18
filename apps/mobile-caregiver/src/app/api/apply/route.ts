import { NextRequest } from 'next/server';
import { pool } from '@bominal-senior/db';
import { auth } from '@bominal-senior/auth';
import { apiSuccess, apiError } from '@bominal-senior/types/src/api-helpers';
import { CaregiverApplicationInputSchema } from '@bominal-senior/types';
import { createApplication, updateApplication, getApplication } from '@/lib/services';

export async function GET(_request: NextRequest) {
  try {
    const session = await auth();
    if (!session?.user?.id) {
      return apiError('Unauthorized', 401);
    }
    const userId = session.user.id;

    const result = await pool.query(
      `SELECT id FROM caregiver_applications
       WHERE user_id = $1
       ORDER BY created_at DESC
       LIMIT 1`,
      [userId],
    );

    if (!result.rows[0]) {
      return apiError('No application found for this user', 404);
    }

    const application = await getApplication(pool, result.rows[0].id);

    if (!application) {
      return apiError('No application found for this user', 404);
    }

    return apiSuccess(application);
  } catch (error) {
    console.error('[GET /api/apply]', error);
    return apiError('Failed to fetch application', 500);
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
    const parsed = CaregiverApplicationInputSchema.safeParse(body);

    if (!parsed.success) {
      return apiError(parsed.error.errors.map((e) => e.message).join(', '), 422);
    }

    // Check for existing application to prevent duplicates
    const existing = await pool.query(
      `SELECT id FROM caregiver_applications
       WHERE user_id = $1 AND status = 'DRAFT'
       LIMIT 1`,
      [userId],
    );

    if (existing.rows[0]) {
      return apiError('An active draft application already exists', 409);
    }

    const application = await createApplication(pool, userId, {
      providerId: parsed.data.providerId,
      experienceYears: parsed.data.experienceYears,
      bio: parsed.data.bio,
      specializations: parsed.data.specializations,
      hasDementiaExperience: parsed.data.hasDementiaExperience,
      hasOvernightAvailability: parsed.data.hasOvernightAvailability,
      smokingStatus: parsed.data.smokingStatus,
      petFriendly: parsed.data.petFriendly,
      preferredGender: parsed.data.preferredGender,
      languagesSpoken: parsed.data.languagesSpoken,
    });

    return apiSuccess(application);
  } catch (error) {
    console.error('[POST /api/apply]', error);
    return apiError('Failed to create application', 500);
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

    const result = await pool.query(
      `SELECT id, status FROM caregiver_applications
       WHERE user_id = $1
       ORDER BY created_at DESC
       LIMIT 1`,
      [userId],
    );

    if (!result.rows[0]) {
      return apiError('Application not found', 404);
    }

    if (result.rows[0].status !== 'DRAFT') {
      return apiError('Only draft applications can be updated', 400);
    }

    const updated = await updateApplication(pool, result.rows[0].id, {
      providerId: body.providerId,
      experienceYears: body.experienceYears,
      bio: body.bio,
      specializations: body.specializations,
      hasDementiaExperience: body.hasDementiaExperience,
      hasOvernightAvailability: body.hasOvernightAvailability,
      smokingStatus: body.smokingStatus,
      petFriendly: body.petFriendly,
      preferredGender: body.preferredGender,
      languagesSpoken: body.languagesSpoken,
    });

    return apiSuccess(updated);
  } catch (error) {
    console.error('[PATCH /api/apply]', error);
    return apiError('Failed to update application', 500);
  }
}
