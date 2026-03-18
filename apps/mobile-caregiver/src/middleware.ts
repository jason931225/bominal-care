import { NextResponse } from 'next/server';
import type { NextRequest } from 'next/server';

export default function middleware(request: NextRequest) {
  // Skip auth for public routes and API auth endpoints
  if (
    request.nextUrl.pathname.startsWith('/auth') ||
    request.nextUrl.pathname.startsWith('/api/auth')
  ) {
    return NextResponse.next();
  }

  // Check for session cookie (set by NextAuth after login)
  const sessionToken = request.cookies.get('authjs.session-token')?.value
    ?? request.cookies.get('__Secure-authjs.session-token')?.value;

  if (!sessionToken) {
    return NextResponse.redirect(new URL('/auth/signin', request.url));
  }

  // Role enforcement is handled by individual pages/API routes
  // to avoid importing @bominal-senior/auth (which depends on pg) in Edge Runtime
  return NextResponse.next();
}

export const config = {
  matcher: ['/((?!_next/static|_next/image|favicon.ico|public).*)'],
};
