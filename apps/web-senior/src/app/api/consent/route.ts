import { NextRequest } from 'next/server';
import { pool } from '@bominal-senior/db';
import { auth } from '@bominal-senior/auth';
import { apiSuccess, apiError } from '@bominal-senior/types/src/api-helpers';
import { ConsentInputSchema } from '@bominal-senior/types';
import {
  getPersonProfileByUserId,
  getPersonProfile,
  grantConsent,
  revokeConsent,
  getConsentsForPerson,
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

    const consents = await getConsentsForPerson(pool, personProfile.id);

    return apiSuccess(consents);
  } catch (error) {
    console.error('[GET /api/consent]', error);
    return apiError('Failed to fetch consent records', 500);
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
    const parsed = ConsentInputSchema.safeParse(body);

    if (!parsed.success) {
      return apiError(parsed.error.errors.map((e) => e.message).join(', '), 422);
    }

    const personProfile = await getPersonProfile(pool, parsed.data.subjectPersonId);

    if (!personProfile) {
      return apiError('Person profile not found', 404);
    }

    const consent = await grantConsent(pool, {
      subjectPersonId: parsed.data.subjectPersonId,
      purpose: parsed.data.purpose,
      grantedBy: userId,
      expiresAt: parsed.data.expiresAt,
    });

    return apiSuccess(consent);
  } catch (error) {
    console.error('[POST /api/consent]', error);
    return apiError('Failed to grant consent', 500);
  }
}

export async function DELETE(request: NextRequest) {
  try {
    const session = await auth();
    if (!session?.user?.id) {
      return apiError('Unauthorized', 401);
    }
    const userId = session.user.id;

    const { searchParams } = new URL(request.url);
    const consentId = searchParams.get('id');

    if (!consentId) {
      return apiError('Consent id is required', 422);
    }

    const revoked = await revokeConsent(pool, consentId, userId);

    return apiSuccess(revoked);
  } catch (error) {
    console.error('[DELETE /api/consent]', error);
    return apiError('Failed to revoke consent', 500);
  }
}
