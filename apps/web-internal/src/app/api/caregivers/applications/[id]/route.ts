import { NextRequest } from 'next/server';
import { pool } from '@bominal-senior/db';
import { auth } from '@bominal-senior/auth';
import { apiSuccess, apiError } from '@bominal-senior/types/src/api-helpers';
import { CaregiverApplicationUpdateSchema } from '@bominal-senior/types';
import { getApplication, transitionApplicationStatus } from '@/lib/services';

export async function GET(
  _request: NextRequest,
  { params }: { params: Promise<{ id: string }> },
) {
  try {
    const session = await auth();
    if (!session?.user?.id) {
      return apiError('Unauthorized', 401);
    }

    const { id } = await params;

    const application = await getApplication(pool, id);

    if (!application) {
      return apiError('Application not found', 404);
    }

    return apiSuccess(application);
  } catch (error) {
    console.error('[GET /api/caregivers/applications/[id]]', error);
    return apiError('Failed to fetch application', 500);
  }
}

export async function PATCH(
  request: NextRequest,
  { params }: { params: Promise<{ id: string }> },
) {
  try {
    const session = await auth();
    if (!session?.user?.id) {
      return apiError('Unauthorized', 401);
    }
    const userId = session.user.id;

    const { id } = await params;
    const body = await request.json();
    const parsed = CaregiverApplicationUpdateSchema.safeParse(body);

    if (!parsed.success) {
      return apiError(parsed.error.errors.map((e) => e.message).join(', '), 422);
    }

    // Verify application belongs to this provider
    const appResult = await pool.query(
      'SELECT id, provider_id, status FROM caregiver_applications WHERE id = $1',
      [id],
    );

    if (!appResult.rows[0]) {
      return apiError('Application not found', 404);
    }

    // TODO: Compare provider_id against session's tenant/org instead of DEV_TENANT_ID
    if (appResult.rows[0].provider_id !== session.user.id) {
      return apiError('Not authorized to review this application', 403);
    }

    const updated = await transitionApplicationStatus(
      pool,
      id,
      parsed.data.status,
      parsed.data.reviewedBy ?? userId,
      parsed.data.rejectionReason,
    );

    return apiSuccess(updated);
  } catch (error) {
    console.error('[PATCH /api/caregivers/applications/[id]]', error);
    if (error instanceof Error) {
      return apiError(error.message, 400);
    }
    return apiError('Failed to review application', 500);
  }
}
