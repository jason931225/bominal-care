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
    const { page, limit, skip } = parsePagination(searchParams);
    const unreadOnly = searchParams.get('unread') === 'true';

    if (unreadOnly) {
      // listNotifications does not support an unread filter, use raw query
      const [dataResult, countResult] = await Promise.all([
        pool.query(
          `SELECT * FROM notifications
           WHERE user_id = $1 AND is_read = false
           ORDER BY created_at DESC
           LIMIT $2 OFFSET $3`,
          [userId, limit, skip],
        ),
        pool.query(
          `SELECT COUNT(*) FROM notifications
           WHERE user_id = $1 AND is_read = false`,
          [userId],
        ),
      ]);

      const total = parseInt(countResult.rows[0].count, 10);

      return apiSuccess(dataResult.rows, {
        total,
        page,
        limit,
        totalPages: Math.ceil(total / limit),
      });
    }

    const { data: notifications, total } = await listNotifications(pool, userId, {
      page,
      limit,
    });

    return apiSuccess(notifications, {
      total,
      page,
      limit,
      totalPages: Math.ceil(total / limit),
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

    await markAllAsRead(pool, userId);

    return apiSuccess({ message: 'All notifications marked as read' });
  } catch (error) {
    console.error('[PATCH /api/notifications]', error);
    return apiError('Failed to mark notifications as read', 500);
  }
}
