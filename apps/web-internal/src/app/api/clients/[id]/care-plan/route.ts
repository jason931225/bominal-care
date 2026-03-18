import { NextRequest } from 'next/server';
import { pool } from '@bominal-senior/db';
import { auth } from '@bominal-senior/auth';
import { apiSuccess, apiError } from '@bominal-senior/types/src/api-helpers';
import { CarePlanInputSchema } from '@bominal-senior/types';
import { getCarePlan, createCarePlan, updateCarePlan } from '@/lib/services';

export async function GET(
  _request: NextRequest,
  { params }: { params: Promise<{ id: string }> },
) {
  try {
    const session = await auth();
    if (!session?.user?.id) {
      return apiError('Unauthorized', 401);
    }
    const userId = session.user.id;

    const { id } = await params;

    // id here is the client (senior profile person id) — find their active care plan
    const carePlanResult = await pool.query(
      `SELECT id FROM care_plans
       WHERE senior_id IN (SELECT id FROM senior_profiles WHERE person_id = $1)
         AND provider_id = $2
       ORDER BY created_at DESC
       LIMIT 1`,
      [id, userId],
    );

    if (!carePlanResult.rows[0]) {
      return apiError('Care plan not found', 404);
    }

    const carePlan = await getCarePlan(pool, carePlanResult.rows[0].id);

    if (!carePlan) {
      return apiError('Care plan not found', 404);
    }

    return apiSuccess(carePlan);
  } catch (error) {
    console.error('[GET /api/clients/[id]/care-plan]', error);
    return apiError('Failed to fetch care plan', 500);
  }
}

export async function POST(
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
    const parsed = CarePlanInputSchema.safeParse({ ...body, seniorId: body.seniorId ?? id });

    if (!parsed.success) {
      return apiError(parsed.error.errors.map((e) => e.message).join(', '), 422);
    }

    // Verify senior profile exists
    const seniorResult = await pool.query(
      'SELECT id FROM senior_profiles WHERE person_id = $1',
      [id],
    );

    if (!seniorResult.rows[0]) {
      return apiError('Senior profile not found', 404);
    }

    const carePlan = await createCarePlan(pool, {
      seniorId: seniorResult.rows[0].id,
      providerId: userId,
      title: parsed.data.title,
      description: parsed.data.description,
      startDate: parsed.data.startDate,
      endDate: parsed.data.endDate,
      goals: parsed.data.goals as never,
      createdBy: userId,
    });

    return apiSuccess(carePlan);
  } catch (error) {
    console.error('[POST /api/clients/[id]/care-plan]', error);
    return apiError('Failed to create care plan', 500);
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

    const carePlanResult = await pool.query(
      `SELECT id FROM care_plans
       WHERE senior_id IN (SELECT id FROM senior_profiles WHERE person_id = $1)
         AND provider_id = $2
       ORDER BY created_at DESC
       LIMIT 1`,
      [id, userId],
    );

    if (!carePlanResult.rows[0]) {
      return apiError('Care plan not found', 404);
    }

    const updated = await updateCarePlan(pool, carePlanResult.rows[0].id, {
      title: body.title,
      description: body.description,
      startDate: body.startDate ? new Date(body.startDate) : undefined,
      endDate: body.endDate ? new Date(body.endDate) : undefined,
      goals: body.goals,
      updatedBy: userId,
    });

    return apiSuccess(updated);
  } catch (error) {
    console.error('[PATCH /api/clients/[id]/care-plan]', error);
    return apiError('Failed to update care plan', 500);
  }
}
