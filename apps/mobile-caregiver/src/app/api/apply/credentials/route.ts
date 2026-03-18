import { NextRequest } from 'next/server';
import { pool } from '@bominal-senior/db';
import { auth } from '@bominal-senior/auth';
import { apiSuccess, apiError } from '@bominal-senior/types/src/api-helpers';
import { CaregiverCredentialInputSchema } from '@bominal-senior/types';
import { addCredential } from '@/lib/services';

export async function GET(_request: NextRequest) {
  try {
    const session = await auth();
    if (!session?.user?.id) {
      return apiError('Unauthorized', 401);
    }
    const userId = session.user.id;

    const appResult = await pool.query(
      `SELECT id FROM caregiver_applications
       WHERE user_id = $1
       ORDER BY created_at DESC
       LIMIT 1`,
      [userId],
    );

    if (!appResult.rows[0]) {
      return apiError('Application not found', 404);
    }

    const credentials = await pool.query(
      `SELECT * FROM caregiver_credentials
       WHERE application_id = $1
       ORDER BY created_at DESC`,
      [appResult.rows[0].id],
    );

    return apiSuccess(credentials.rows);
  } catch (error) {
    console.error('[GET /api/apply/credentials]', error);
    return apiError('Failed to fetch credentials', 500);
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
    const parsed = CaregiverCredentialInputSchema.safeParse(body);

    if (!parsed.success) {
      return apiError(parsed.error.errors.map((e) => e.message).join(', '), 422);
    }

    const appResult = await pool.query(
      `SELECT id FROM caregiver_applications
       WHERE user_id = $1
       ORDER BY created_at DESC
       LIMIT 1`,
      [userId],
    );

    if (!appResult.rows[0]) {
      return apiError('Application not found', 404);
    }

    const credential = await addCredential(pool, appResult.rows[0].id, {
      type: parsed.data.type,
      issuer: parsed.data.issuer,
      issuedAt: parsed.data.issuedAt,
      expiresAt: parsed.data.expiresAt,
      documentUrl: parsed.data.documentUrl,
    });

    return apiSuccess(credential);
  } catch (error) {
    console.error('[POST /api/apply/credentials]', error);
    return apiError('Failed to add credential', 500);
  }
}
