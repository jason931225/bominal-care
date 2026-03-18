import { NextRequest } from 'next/server';
import { pool } from '@bominal-senior/db';
import { auth } from '@bominal-senior/auth';
import { apiSuccess, apiError } from '@bominal-senior/types/src/api-helpers';
import { updateMedication } from '@/lib/services';

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

    // listMedications returns medications with schedules for a person;
    // for single lookup we use the pool directly via a lightweight query
    const result = await pool.query(
      `SELECT m.*,
              COALESCE(json_agg(ms.*) FILTER (WHERE ms.id IS NOT NULL), '[]') AS schedules,
              COALESCE(
                (SELECT json_agg(me.* ORDER BY me.scheduled_for DESC)
                 FROM medication_events me
                 WHERE me.medication_id = m.id
                 LIMIT 10),
                '[]'
              ) AS events
       FROM medications m
       LEFT JOIN medication_schedules ms ON ms.medication_id = m.id
       WHERE m.id = $1
       GROUP BY m.id`,
      [id],
    );

    if (result.rows.length === 0) {
      return apiError('Medication not found', 404);
    }

    return apiSuccess(result.rows[0]);
  } catch (error) {
    console.error('[GET /api/medications/[id]]', error);
    return apiError('Failed to fetch medication', 500);
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

    const updated = await updateMedication(pool, id, {
      ...(body.name !== undefined && { name: body.name }),
      ...(body.dosage !== undefined && { dosage: body.dosage }),
      ...(body.form !== undefined && { form: body.form }),
      ...(body.frequency !== undefined && { frequency: body.frequency }),
      ...(body.prescribedBy !== undefined && { prescribedBy: body.prescribedBy }),
      ...(body.startDate !== undefined && { startDate: new Date(body.startDate) }),
      ...(body.endDate !== undefined && { endDate: new Date(body.endDate) }),
      ...(body.isActive !== undefined && { isActive: body.isActive }),
      ...(body.sideEffects !== undefined && { sideEffects: body.sideEffects }),
      ...(body.notes !== undefined && { notes: body.notes }),
      updatedBy: userId,
    });

    if (!updated) {
      return apiError('Medication not found', 404);
    }

    return apiSuccess(updated);
  } catch (error) {
    console.error('[PATCH /api/medications/[id]]', error);
    return apiError('Failed to update medication', 500);
  }
}
