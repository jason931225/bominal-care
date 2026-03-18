import { NextRequest } from 'next/server';
import { pool } from '@bominal-senior/db';
import { apiSuccess, apiError, parsePagination } from '@bominal-senior/types/src/api-helpers';
import { ReferralInputSchema } from '@bominal-senior/types';

// TODO: const session = await auth()
const DEV_TENANT_ID = 'dev-provider-org-001';

export async function GET(request: NextRequest) {
  try {
    // TODO: const session = await auth()
    const { searchParams } = new URL(request.url);
    const { page, limit, skip } = parsePagination(searchParams);
    const direction = searchParams.get('direction') ?? 'both';

    const conditions: string[] = [];
    const values: unknown[] = [];
    let paramIdx = 1;

    if (direction === 'outbound') {
      conditions.push(`from_provider_id = $${paramIdx}`);
      values.push(DEV_TENANT_ID);
      paramIdx++;
    } else if (direction === 'inbound') {
      conditions.push(`to_provider_id = $${paramIdx}`);
      values.push(DEV_TENANT_ID);
      paramIdx++;
    } else {
      conditions.push(`(from_provider_id = $${paramIdx} OR to_provider_id = $${paramIdx + 1})`);
      values.push(DEV_TENANT_ID, DEV_TENANT_ID);
      paramIdx += 2;
    }

    const where = `WHERE ${conditions.join(' AND ')}`;

    const [dataResult, countResult] = await Promise.all([
      pool.query(
        `SELECT * FROM institution_referrals ${where}
         ORDER BY referred_at DESC
         LIMIT $${paramIdx} OFFSET $${paramIdx + 1}`,
        [...values, limit, skip],
      ),
      pool.query(`SELECT COUNT(*) FROM institution_referrals ${where}`, values),
    ]);

    const total = parseInt(countResult.rows[0].count, 10);

    return apiSuccess(dataResult.rows, {
      total,
      page,
      limit,
      totalPages: Math.ceil(total / limit),
    });
  } catch (error) {
    console.error('[GET /api/referrals]', error);
    return apiError('Failed to fetch referrals', 500);
  }
}

export async function POST(request: NextRequest) {
  try {
    // TODO: const session = await auth()
    const body = await request.json();
    const parsed = ReferralInputSchema.safeParse({
      ...body,
      fromProviderId: body.fromProviderId ?? DEV_TENANT_ID,
    });

    if (!parsed.success) {
      return apiError(parsed.error.errors.map((e) => e.message).join(', '), 422);
    }

    // Enforce that the from provider is the current provider
    if (parsed.data.fromProviderId !== DEV_TENANT_ID) {
      return apiError('Can only create referrals from your own organization', 403);
    }

    // Verify toProvider exists
    const toProviderResult = await pool.query(
      'SELECT id FROM provider_organizations WHERE id = $1',
      [parsed.data.toProviderId],
    );

    if (!toProviderResult.rows[0]) {
      return apiError('Target provider organization not found', 404);
    }

    const { createReferral } = await import('@bominal-senior/db/src/services/referral.service');
    const referral = await createReferral(pool, {
      fromProviderId: parsed.data.fromProviderId,
      toProviderId: parsed.data.toProviderId,
      seniorPersonId: parsed.data.seniorPersonId,
      reason: parsed.data.reason,
      notes: parsed.data.notes,
    });

    return apiSuccess(referral);
  } catch (error) {
    console.error('[POST /api/referrals]', error);
    return apiError('Failed to create referral', 500);
  }
}
