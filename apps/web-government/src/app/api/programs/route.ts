import { NextRequest } from 'next/server';
import { pool } from '@bominal-senior/db';
import { apiSuccess, apiError, parsePagination } from '@bominal-senior/types/src/api-helpers';

export async function GET(request: NextRequest) {
  try {
    // TODO: const session = await auth()
    const { searchParams } = new URL(request.url);
    const { page, limit, skip } = parsePagination(searchParams);
    const status = searchParams.get('status') ?? undefined;
    const seniorId = searchParams.get('seniorId') ?? undefined;
    const programName = searchParams.get('programName') ?? undefined;

    const conditions: string[] = [];
    const values: unknown[] = [];
    let paramIdx = 1;

    if (status !== undefined) {
      conditions.push(`ec.status = $${paramIdx}`);
      values.push(status);
      paramIdx++;
    }
    if (seniorId !== undefined) {
      conditions.push(`ec.senior_id = $${paramIdx}`);
      values.push(seniorId);
      paramIdx++;
    }
    if (programName !== undefined) {
      conditions.push(`ec.program_name ILIKE $${paramIdx}`);
      values.push(`%${programName}%`);
      paramIdx++;
    }

    const where = conditions.length > 0 ? `WHERE ${conditions.join(' AND ')}` : '';

    const [casesResult, countResult] = await Promise.all([
      pool.query(
        `SELECT
           ec.id,
           ec.senior_id,
           ec.status,
           ec.program_name,
           ec.application_date,
           ec.determination_date,
           ec.notes,
           ec.denial_reason,
           ec.created_at,
           ec.updated_at,
           jsonb_build_object(
             'id', sp.id,
             'personProfile', jsonb_build_object(
               'id', pp.id,
               'koreanName', pp.korean_name,
               'englishName', pp.english_name,
               'city', pp.city,
               'district', pp.district
             )
           ) AS "seniorProfile",
           COALESCE(
             (SELECT jsonb_agg(
               jsonb_build_object(
                 'id', ast.id,
                 'caseId', ast.case_id,
                 'stepName', ast.step_name,
                 'stepOrder', ast.step_order,
                 'status', ast.status,
                 'assignedTo', ast.assigned_to,
                 'completedAt', ast.completed_at,
                 'notes', ast.notes,
                 'createdAt', ast.created_at,
                 'updatedAt', ast.updated_at
               ) ORDER BY ast.step_order ASC
             )
             FROM approval_steps ast
             WHERE ast.case_id = ec.id),
             '[]'::jsonb
           ) AS "approvalSteps"
         FROM eligibility_cases ec
         JOIN senior_profiles sp ON sp.id = ec.senior_id
         JOIN person_profiles pp ON pp.id = sp.person_id
         ${where}
         ORDER BY ec.created_at DESC
         LIMIT $${paramIdx} OFFSET $${paramIdx + 1}`,
        [...values, limit, skip],
      ),
      pool.query(`SELECT COUNT(*) FROM eligibility_cases ec ${where}`, values),
    ]);

    const total = parseInt(countResult.rows[0].count, 10);
    return apiSuccess(casesResult.rows, { total, page, limit, totalPages: Math.ceil(total / limit) });
  } catch (error) {
    console.error('[GET /api/programs]', error);
    return apiError('Failed to fetch district programs', 500);
  }
}
