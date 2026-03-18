import { NextRequest } from 'next/server';
import { pool } from '@bominal-senior/db';
import { auth } from '@bominal-senior/auth';
import { apiSuccess, apiError } from '@bominal-senior/types/src/api-helpers';
import { getAppointment, updateAppointment } from '@/lib/services';

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

    const appointment = await getAppointment(pool, id);

    if (!appointment) {
      return apiError('Appointment not found', 404);
    }

    return apiSuccess(appointment);
  } catch (error) {
    console.error('[GET /api/appointments/[id]]', error);
    return apiError('Failed to fetch appointment', 500);
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

    const existing = await getAppointment(pool, id);

    if (!existing) {
      return apiError('Appointment not found', 404);
    }

    const updated = await updateAppointment(pool, id, {
      ...(body.institutionName !== undefined && { institutionName: body.institutionName }),
      ...(body.institutionType !== undefined && { institutionType: body.institutionType }),
      ...(body.appointmentDate !== undefined && {
        appointmentDate: new Date(body.appointmentDate),
      }),
      ...(body.status !== undefined && { status: body.status }),
      ...(body.purpose !== undefined && { purpose: body.purpose }),
      ...(body.notes !== undefined && { notes: body.notes }),
      ...(body.address !== undefined && { address: body.address }),
      updatedBy: userId,
    });

    return apiSuccess(updated);
  } catch (error) {
    console.error('[PATCH /api/appointments/[id]]', error);
    return apiError('Failed to update appointment', 500);
  }
}
