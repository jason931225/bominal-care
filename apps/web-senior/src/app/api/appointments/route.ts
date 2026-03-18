import { NextRequest } from 'next/server';
import { pool } from '@bominal-senior/db';
import { auth } from '@bominal-senior/auth';
import { apiSuccess, apiError, parsePagination } from '@bominal-senior/types/src/api-helpers';
import { AppointmentInputSchema } from '@bominal-senior/types';
import {
  getPersonProfileByUserId,
  getPersonProfile,
  listAppointments,
  createAppointment,
} from '@/lib/services';

export async function GET(request: NextRequest) {
  try {
    const session = await auth();
    if (!session?.user?.id) {
      return apiError('Unauthorized', 401);
    }
    const userId = session.user.id;

    const { searchParams } = new URL(request.url);
    const { page, limit } = parsePagination(searchParams);

    const personProfile = await getPersonProfileByUserId(pool, userId);

    if (!personProfile) {
      return apiError('Person profile not found', 404);
    }

    const { data: appointments, total } = await listAppointments(pool, personProfile.id, {
      page,
      limit,
    });

    return apiSuccess(appointments, {
      total,
      page,
      limit,
      totalPages: Math.ceil(total / limit),
    });
  } catch (error) {
    console.error('[GET /api/appointments]', error);
    return apiError('Failed to fetch appointments', 500);
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
    const parsed = AppointmentInputSchema.safeParse(body);

    if (!parsed.success) {
      return apiError(parsed.error.errors.map((e) => e.message).join(', '), 422);
    }

    const personProfile = await getPersonProfile(pool, parsed.data.personId);

    if (!personProfile) {
      return apiError('Person profile not found', 404);
    }

    const appointment = await createAppointment(pool, {
      personId: parsed.data.personId,
      institutionName: parsed.data.institutionName,
      institutionType: parsed.data.institutionType,
      appointmentDate: parsed.data.appointmentDate,
      purpose: parsed.data.purpose,
      notes: parsed.data.notes,
      address: parsed.data.address,
      createdBy: userId,
    });

    return apiSuccess(appointment);
  } catch (error) {
    console.error('[POST /api/appointments]', error);
    return apiError('Failed to create appointment', 500);
  }
}
