import { NextRequest } from 'next/server';
import { pool } from '@bominal-senior/db';
import { auth } from '@bominal-senior/auth';
import { apiSuccess, apiError, parsePagination } from '@bominal-senior/types/src/api-helpers';
import { listNotifications, markAllAsRead } from '@/lib/services';

export async function GET(request: NextRequest) {
  try {
    const session = await auth();
    if (!session?.user?.id) {
      return apiError('Unauthorized', 401);
    }
    const userId = session.user.id;

    const { searchParams } = new URL(request.url);
    const { page, limit } = parsePagination(searchParams);

    const result = await listNotifications(pool, userId, { page, limit });

    return apiSuccess(result.data, {
      total: result.total,
      page,
      limit,
      totalPages: Math.ceil(result.total / limit),
    });
  } catch (error) {
    console.error('[GET /api/notifications]', error);
    return apiError('Failed to fetch notifications', 500);
  }
}

export async function PATCH(_request: NextRequest) {
  try {
    const session = await auth();
    if (!session?.user?.id) {
      return apiError('Unauthorized', 401);
    }
    const userId = session.user.id;

    const result = await markAllAsRead(pool, userId);

    return apiSuccess({ message: 'All notifications marked as read', count: result.count });
  } catch (error) {
    console.error('[PATCH /api/notifications]', error);
    return apiError('Failed to mark notifications as read', 500);
  }
}
