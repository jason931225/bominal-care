export { handlers, auth, signIn, signOut } from './config';
export { hasPermission, getAllPermissions, ROLE_PERMISSIONS } from './rbac';
export type { Resource, Action, Scope, Permission } from './rbac';
export { withAuth, requireRole, checkConsent } from './middleware';
import './types';

// Client-side session helpers
export { AuthSessionProvider } from './session-provider';
export { useSession, signIn as clientSignIn, signOut as clientSignOut } from './session-provider';

// Convenience re-exports for API route handlers
import { handlers } from './config';
export const { GET, POST } = handlers;
