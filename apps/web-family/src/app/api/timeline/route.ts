import { NextRequest } from 'next/server';
import { pool } from '@bominal-senior/db';
import { auth } from '@bominal-senior/auth';
import { apiSuccess, apiError, parsePagination } from '@bominal-senior/types/src/api-helpers';
import { getPersonProfileByUserId } from '@/lib/services';

export async function GET(request: NextRequest) {
  try {
    const session = await auth();
    if (!session?.user?.id) {
      return apiError('Unauthorized', 401);
    }
    const userId = session.user.id;

    const { searchParams } = new URL(request.url);
    const { page, limit, skip } = parsePagination(searchParams);
    const seniorPersonId = searchParams.get('seniorPersonId');

    if (!seniorPersonId) {
      return apiError('seniorPersonId query parameter is required', 422);
    }

    // Verify family relationship exists before returning data
    const familyProfile = await getPersonProfileByUserId(pool, userId);

    if (!familyProfile) {
      return apiError('Family profile not found', 404);
    }

    const relationshipResult = await pool.query(
      `SELECT id FROM family_relationships
       WHERE senior_person_id = $1 AND family_person_id = $2
       LIMIT 1`,
      [seniorPersonId, familyProfile.id],
    );

    if (relationshipResult.rows.length === 0) {
      return apiError('No linked relationship to this senior', 403);
    }

    const thirtyDaysAgo = new Date(Date.now() - 30 * 24 * 60 * 60 * 1000);
    const sevenDaysAgo = new Date(Date.now() - 7 * 24 * 60 * 60 * 1000);

    // Gather care timeline events: visits, medication events, appointments
    const [visitsResult, medicationEventsResult, appointmentsResult] = await Promise.all([
      pool.query(
        `SELECT v.*, cp.id AS care_plan_id, cp.senior_id, cp.status AS care_plan_status
         FROM visits v
         JOIN care_plans cp ON cp.id = v.care_plan_id
         WHERE cp.senior_id IS NOT NULL
           AND v.scheduled_start >= $1
         ORDER BY v.scheduled_start DESC
         LIMIT $2 OFFSET $3`,
        [thirtyDaysAgo, limit, skip],
      ),
      pool.query(
        `SELECT me.*, m.id AS medication_id, m.name AS medication_name,
                m.person_id AS medication_person_id
         FROM medication_events me
         JOIN medications m ON m.id = me.medication_id
         WHERE m.person_id = $1
           AND me.scheduled_for >= $2
         ORDER BY me.scheduled_for DESC
         LIMIT $3`,
        [seniorPersonId, sevenDaysAgo, limit],
      ),
      pool.query(
        `SELECT * FROM appointments
         WHERE person_id = $1
           AND appointment_date >= $2
         ORDER BY appointment_date DESC
         LIMIT $3`,
        [seniorPersonId, thirtyDaysAgo, limit],
      ),
    ]);

    const visits = visitsResult.rows;
    const medicationEvents = medicationEventsResult.rows;
    const appointments = appointmentsResult.rows;

    const timeline = {
      visits,
      medicationEvents,
      appointments,
      seniorPersonId,
    };

    const total = visits.length + medicationEvents.length + appointments.length;

    return apiSuccess(timeline, {
      total,
      page,
      limit,
      totalPages: Math.ceil(total / limit),
    });
  } catch (error) {
    console.error('[GET /api/timeline]', error);
    return apiError('Failed to fetch care timeline', 500);
  }
}
