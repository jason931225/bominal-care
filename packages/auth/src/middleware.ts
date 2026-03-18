import { NextResponse } from 'next/server';
import type { NextRequest } from 'next/server';
import type { Pool } from 'pg';
type UserRole =
  | 'SENIOR'
  | 'FAMILY'
  | 'CAREGIVER_APPLICANT'
  | 'CAREGIVER_APPROVED'
  | 'PROVIDER_ADMIN'
  | 'PROVIDER_STAFF'
  | 'MEDICAL_STAFF'
  | 'GOVERNMENT_REVIEWER'
  | 'PARTNER_OPERATOR'
  | 'PLATFORM_ADMIN';
import { auth } from './config';
import { hasPermission } from './rbac';
import type { Resource, Action, Scope, Permission } from './rbac';

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

type RouteHandler = (req: NextRequest, context: unknown) => Promise<Response> | Response;

// ---------------------------------------------------------------------------
// withAuth
// ---------------------------------------------------------------------------

/**
 * Wraps a Next.js route handler to enforce authentication and optional
 * permission checks. Returns 401 when unauthenticated, 403 when the session
 * user lacks the required permissions.
 */
export function withAuth(
  handler: RouteHandler,
  requiredPermissions: Permission[] = [],
): RouteHandler {
  return async (req: NextRequest, context: unknown): Promise<Response> => {
    const session = await auth();

    if (!session?.user?.id) {
      return NextResponse.json(
        { success: false, data: null, error: 'Unauthenticated' },
        { status: 401 },
      );
    }

    const { role } = session.user;

    for (const perm of requiredPermissions) {
      if (!hasPermission(role, perm.resource, perm.action, perm.scope)) {
        return NextResponse.json(
          {
            success: false,
            data: null,
            error: `Forbidden: missing ${perm.action} on ${perm.resource}`,
          },
          { status: 403 },
        );
      }
    }

    return handler(req, context);
  };
}

// ---------------------------------------------------------------------------
// requireRole
// ---------------------------------------------------------------------------

/**
 * Middleware factory that checks whether the current session user holds one
 * of the allowed roles. Returns 401 when unauthenticated, 403 when the role
 * does not match.
 */
export async function requireRole(
  ...roles: UserRole[]
): Promise<NextResponse | null> {
  const session = await auth();

  if (!session?.user?.id) {
    return NextResponse.json(
      { success: false, data: null, error: 'Unauthenticated' },
      { status: 401 },
    );
  }

  if (!roles.includes(session.user.role as UserRole)) {
    return NextResponse.json(
      {
        success: false,
        data: null,
        error: `Forbidden: role ${session.user.role} is not allowed`,
      },
      { status: 403 },
    );
  }

  return null;
}

// ---------------------------------------------------------------------------
// checkConsent
// ---------------------------------------------------------------------------

/**
 * Checks whether an active, non-expired consent record exists for the given
 * subject person and purpose. Returns true when consent is present and
 * currently valid.
 *
 * @param pool - A `pg` Pool instance used to query the database.
 */
export async function checkConsent(
  pool: Pool,
  subjectPersonId: string,
  purpose: string,
): Promise<boolean> {
  const now = new Date();

  const result = await pool.query<{ id: string }>(
    `SELECT id FROM consent_records
     WHERE subject_person_id = $1
       AND purpose = $2
       AND is_active = TRUE
       AND (expires_at IS NULL OR expires_at > $3)
     LIMIT 1`,
    [subjectPersonId, purpose, now],
  );

  return result.rowCount !== null && result.rowCount > 0;
}

// Re-export commonly needed types so callers can import from a single path.
export type { Resource, Action, Scope, Permission };
