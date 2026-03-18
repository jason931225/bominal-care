// =============================================================================
// Notification Service
// =============================================================================

import type { Pool } from 'pg';
import { generateId } from '../helpers';
import type { NotificationType } from '../types';

export interface CreateNotificationData {
  userId: string;
  type?: NotificationType;
  title: string;
  message: string;
  link?: string;
}

export interface Pagination {
  page: number;
  limit: number;
}

export interface PaginatedResult<T> {
  data: T[];
  total: number;
}

// -----------------------------------------------------------------------------
// Service functions
// -----------------------------------------------------------------------------

export async function createNotification(
  pool: Pool,
  data: CreateNotificationData,
): Promise<Record<string, unknown>> {
  const id = generateId();
  const result = await pool.query(
    `INSERT INTO notifications (
       id, user_id, type, title, message, link, is_read, created_at, updated_at
     ) VALUES ($1,$2,$3,$4,$5,$6,false,NOW(),NOW())
     RETURNING *`,
    [
      id,
      data.userId,
      data.type ?? 'INFO',
      data.title,
      data.message,
      data.link ?? null,
    ],
  );
  return result.rows[0];
}

export async function markAsRead(
  pool: Pool,
  id: string,
): Promise<Record<string, unknown>> {
  const existing = await pool.query('SELECT id FROM notifications WHERE id = $1', [id]);
  if (!existing.rows[0]) {
    throw new Error(`Notification not found: ${id}`);
  }

  const result = await pool.query(
    `UPDATE notifications
     SET is_read = true, read_at = NOW(), updated_at = NOW()
     WHERE id = $1
     RETURNING *`,
    [id],
  );
  return result.rows[0];
}

export async function markAllAsRead(
  pool: Pool,
  userId: string,
): Promise<{ count: number }> {
  const result = await pool.query(
    `UPDATE notifications
     SET is_read = true, read_at = NOW(), updated_at = NOW()
     WHERE user_id = $1 AND is_read = false`,
    [userId],
  );
  return { count: result.rowCount ?? 0 };
}

export async function getUnreadCount(
  pool: Pool,
  userId: string,
): Promise<number> {
  const result = await pool.query(
    'SELECT COUNT(*) FROM notifications WHERE user_id = $1 AND is_read = false',
    [userId],
  );
  return parseInt(result.rows[0].count, 10);
}

export async function listNotifications(
  pool: Pool,
  userId: string,
  pagination: Pagination,
): Promise<PaginatedResult<Record<string, unknown>>> {
  const offset = (pagination.page - 1) * pagination.limit;

  const [dataResult, countResult] = await Promise.all([
    pool.query(
      `SELECT * FROM notifications WHERE user_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3`,
      [userId, pagination.limit, offset],
    ),
    pool.query('SELECT COUNT(*) FROM notifications WHERE user_id = $1', [userId]),
  ]);

  return {
    data: dataResult.rows,
    total: parseInt(countResult.rows[0].count, 10),
  };
}

export const NotificationService = {
  createNotification,
  markAsRead,
  markAllAsRead,
  getUnreadCount,
  listNotifications,
};
