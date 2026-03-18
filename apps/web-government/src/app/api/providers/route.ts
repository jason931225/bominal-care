import { NextRequest } from 'next/server';
import { pool } from '@bominal-senior/db';
import { apiSuccess, apiError, parsePagination } from '@bominal-senior/types/src/api-helpers';

export async function GET(request: NextRequest) {
  try {
    // TODO: const session = await auth()
    const { searchParams } = new URL(request.url);
    const { page, limit, skip } = parsePagination(searchParams);
    const type = searchParams.get('type') ?? undefined;
    const city = searchParams.get('city') ?? undefined;
    const district = searchParams.get('district') ?? undefined;
    const isActive = searchParams.get('isActive');

    const conditions: string[] = [];
    const values: unknown[] = [];
    let paramIdx = 1;

    if (type !== undefined) {
      conditions.push(`type = $${paramIdx}`);
      values.push(type);
      paramIdx++;
    }
    if (city !== undefined) {
      conditions.push(`city = $${paramIdx}`);
      values.push(city);
      paramIdx++;
    }
    if (district !== undefined) {
      conditions.push(`district = $${paramIdx}`);
      values.push(district);
      paramIdx++;
    }
    if (isActive !== null) {
      conditions.push(`is_active = $${paramIdx}`);
      values.push(isActive === 'true');
      paramIdx++;
    }

    const where = conditions.length > 0 ? `WHERE ${conditions.join(' AND ')}` : '';

    const [dataResult, countResult] = await Promise.all([
      pool.query(
        `SELECT id, name, type, registration_number, address, city, district, postal_code,
                phone, email, website, license_number, license_expires_at, is_active,
                description, created_at
         FROM provider_organizations ${where}
         ORDER BY name ASC
         LIMIT $${paramIdx} OFFSET $${paramIdx + 1}`,
        [...values, limit, skip],
      ),
      pool.query(`SELECT COUNT(*) FROM provider_organizations ${where}`, values),
    ]);

    const total = parseInt(countResult.rows[0].count, 10);
    return apiSuccess(dataResult.rows, { total, page, limit, totalPages: Math.ceil(total / limit) });
  } catch (error) {
    console.error('[GET /api/providers]', error);
    return apiError('Failed to fetch providers', 500);
  }
}
