'use client';

import Link from 'next/link';
import { usePathname } from 'next/navigation';
import { useSession, signOut } from 'next-auth/react';

// Bottom navigation items — the five primary destinations for senior users
const NAV_ITEMS = [
  { href: '/', label: '홈', icon: HomeIcon, matchExact: true },
  { href: '/appointments', label: '예약', icon: CalendarIcon, matchExact: false },
  { href: '/medications', label: '약물', icon: PillIcon, matchExact: false },
  { href: '/care', label: '케어', icon: HeartIcon, matchExact: false },
  { href: '/more', label: '더보기', icon: GridIcon, matchExact: false },
] as const;

// SVG icon components — inline to avoid external icon library dependency
function HomeIcon({ active }: { active: boolean }) {
  if (active) {
    return (
      <svg className="w-7 h-7 text-primary-600" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
        <path d="M11.47 3.84a.75.75 0 011.06 0l8.69 8.69a.75.75 0 101.06-1.06l-8.689-8.69a2.25 2.25 0 00-3.182 0l-8.69 8.69a.75.75 0 001.061 1.06l8.69-8.69z" />
        <path d="M12 5.432l8.159 8.159c.03.03.06.058.091.086v6.198c0 1.035-.84 1.875-1.875 1.875H15a.75.75 0 01-.75-.75v-4.5a.75.75 0 00-.75-.75h-3a.75.75 0 00-.75.75V21a.75.75 0 01-.75.75H5.625a1.875 1.875 0 01-1.875-1.875v-6.198a2.29 2.29 0 00.091-.086L12 5.432z" />
      </svg>
    );
  }
  return (
    <svg className="w-7 h-7 text-gray-400" fill="none" stroke="currentColor" strokeWidth={1.5} viewBox="0 0 24 24" aria-hidden="true">
      <path strokeLinecap="round" strokeLinejoin="round" d="M2.25 12l8.954-8.955a1.126 1.126 0 011.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25" />
    </svg>
  );
}

function CalendarIcon({ active }: { active: boolean }) {
  if (active) {
    return (
      <svg className="w-7 h-7 text-primary-600" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
        <path d="M12.75 12.75a.75.75 0 11-1.5 0 .75.75 0 011.5 0zM7.5 15.75a.75.75 0 100-1.5.75.75 0 000 1.5zM8.25 17.25a.75.75 0 11-1.5 0 .75.75 0 011.5 0zM9.75 15.75a.75.75 0 100-1.5.75.75 0 000 1.5zM10.5 17.25a.75.75 0 11-1.5 0 .75.75 0 011.5 0zM12 15.75a.75.75 0 100-1.5.75.75 0 000 1.5zM12.75 17.25a.75.75 0 11-1.5 0 .75.75 0 011.5 0zM14.25 15.75a.75.75 0 100-1.5.75.75 0 000 1.5zM15 17.25a.75.75 0 11-1.5 0 .75.75 0 011.5 0zM16.5 15.75a.75.75 0 100-1.5.75.75 0 000 1.5zM15 12.75a.75.75 0 11-1.5 0 .75.75 0 011.5 0zM16.5 13.5a.75.75 0 100-1.5.75.75 0 000 1.5z" />
        <path fillRule="evenodd" d="M6.75 2.25A.75.75 0 017.5 3v1.5h9V3A.75.75 0 0118 3v1.5h.75a3 3 0 013 3v11.25a3 3 0 01-3 3H5.25a3 3 0 01-3-3V7.5a3 3 0 013-3H6V3a.75.75 0 01.75-.75zm13.5 9a1.5 1.5 0 00-1.5-1.5H5.25a1.5 1.5 0 00-1.5 1.5v7.5a1.5 1.5 0 001.5 1.5h13.5a1.5 1.5 0 001.5-1.5v-7.5z" clipRule="evenodd" />
      </svg>
    );
  }
  return (
    <svg className="w-7 h-7 text-gray-400" fill="none" stroke="currentColor" strokeWidth={1.5} viewBox="0 0 24 24" aria-hidden="true">
      <path strokeLinecap="round" strokeLinejoin="round" d="M6.75 3v2.25M17.25 3v2.25M3 18.75V7.5a2.25 2.25 0 012.25-2.25h13.5A2.25 2.25 0 0121 7.5v11.25m-18 0A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75m-18 0v-7.5A2.25 2.25 0 015.25 9h13.5A2.25 2.25 0 0121 11.25v7.5" />
    </svg>
  );
}

function PillIcon({ active }: { active: boolean }) {
  if (active) {
    return (
      <svg className="w-7 h-7 text-primary-600" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
        <path fillRule="evenodd" d="M10.5 3.798a3.187 3.187 0 00-4.508 0L3.798 5.99a3.187 3.187 0 000 4.508l5.702 5.703a3.187 3.187 0 004.509 0l2.191-2.192a3.187 3.187 0 000-4.508L10.5 3.798zm3.372 7.15l-4.053-4.053L12.012 4.7a1.687 1.687 0 012.386 0l1.846 1.846a1.687 1.687 0 010 2.386l-2.372 2.016z" clipRule="evenodd" />
        <path d="M18 13.5a.75.75 0 00-.75.75v4.5h-4.5a.75.75 0 000 1.5h4.5v4.5a.75.75 0 001.5 0v-4.5h4.5a.75.75 0 000-1.5h-4.5v-4.5a.75.75 0 00-.75-.75z" />
      </svg>
    );
  }
  return (
    <svg className="w-7 h-7 text-gray-400" fill="none" stroke="currentColor" strokeWidth={1.5} viewBox="0 0 24 24" aria-hidden="true">
      <path strokeLinecap="round" strokeLinejoin="round" d="M9.75 3.104v5.714a2.25 2.25 0 01-.659 1.591L5 14.5M9.75 3.104c-.251.023-.501.05-.75.082m.75-.082a24.301 24.301 0 014.5 0m0 0v5.714c0 .597.237 1.17.659 1.591L19.8 15.3M14.25 3.104c.251.023.501.05.75.082M19.8 15.3l-1.57.393A9.065 9.065 0 0112 15a9.065 9.065 0 00-6.23.693L5 14.5m14.8.8l1.402 1.402c1.232 1.232.65 3.318-1.067 3.611A48.309 48.309 0 0112 21c-2.773 0-5.491-.235-8.135-.687-1.718-.293-2.3-2.379-1.067-3.61L5 14.5" />
    </svg>
  );
}

function HeartIcon({ active }: { active: boolean }) {
  if (active) {
    return (
      <svg className="w-7 h-7 text-primary-600" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
        <path d="M11.645 20.91l-.007-.003-.022-.012a15.247 15.247 0 01-.383-.218 25.18 25.18 0 01-4.244-3.17C4.688 15.36 2.25 12.174 2.25 8.25 2.25 5.322 4.714 3 7.688 3A5.5 5.5 0 0112 5.052 5.5 5.5 0 0116.313 3c2.973 0 5.437 2.322 5.437 5.25 0 3.925-2.438 7.111-4.739 9.256a25.175 25.175 0 01-4.244 3.17 15.247 15.247 0 01-.383.219l-.022.012-.007.004-.003.001a.752.752 0 01-.704 0l-.003-.001z" />
      </svg>
    );
  }
  return (
    <svg className="w-7 h-7 text-gray-400" fill="none" stroke="currentColor" strokeWidth={1.5} viewBox="0 0 24 24" aria-hidden="true">
      <path strokeLinecap="round" strokeLinejoin="round" d="M21 8.25c0-2.485-2.099-4.5-4.688-4.5-1.935 0-3.597 1.126-4.312 2.733-.715-1.607-2.377-2.733-4.313-2.733C5.1 3.75 3 5.765 3 8.25c0 7.22 9 12 9 12s9-4.78 9-12z" />
    </svg>
  );
}

function GridIcon({ active }: { active: boolean }) {
  if (active) {
    return (
      <svg className="w-7 h-7 text-primary-600" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
        <path fillRule="evenodd" d="M3 6a3 3 0 013-3h2.25a3 3 0 013 3v2.25a3 3 0 01-3 3H6a3 3 0 01-3-3V6zm9.75 0a3 3 0 013-3H18a3 3 0 013 3v2.25a3 3 0 01-3 3h-2.25a3 3 0 01-3-3V6zM3 15.75a3 3 0 013-3h2.25a3 3 0 013 3V18a3 3 0 01-3 3H6a3 3 0 01-3-3v-2.25zm9.75 0a3 3 0 013-3H18a3 3 0 013 3V18a3 3 0 01-3 3h-2.25a3 3 0 01-3-3v-2.25z" clipRule="evenodd" />
      </svg>
    );
  }
  return (
    <svg className="w-7 h-7 text-gray-400" fill="none" stroke="currentColor" strokeWidth={1.5} viewBox="0 0 24 24" aria-hidden="true">
      <path strokeLinecap="round" strokeLinejoin="round" d="M3.75 6A2.25 2.25 0 016 3.75h2.25A2.25 2.25 0 0110.5 6v2.25a2.25 2.25 0 01-2.25 2.25H6a2.25 2.25 0 01-2.25-2.25V6zM3.75 15.75A2.25 2.25 0 016 13.5h2.25a2.25 2.25 0 012.25 2.25V18a2.25 2.25 0 01-2.25 2.25H6A2.25 2.25 0 013.75 18v-2.25zM13.5 6a2.25 2.25 0 012.25-2.25H18A2.25 2.25 0 0120.25 6v2.25A2.25 2.25 0 0118 10.5h-2.25a2.25 2.25 0 01-2.25-2.25V6zM13.5 15.75a2.25 2.25 0 012.25-2.25H18a2.25 2.25 0 012.25 2.25V18A2.25 2.25 0 0118 20.25h-2.25A2.25 2.25 0 0113.5 18v-2.25z" />
    </svg>
  );
}

// Emergency button — fixed to bottom-right, always accessible
function EmergencyButton() {
  return (
    <Link
      href="/emergency"
      className="fixed bottom-24 right-4 z-50
                 w-16 h-16 rounded-full
                 bg-danger-500 hover:bg-danger-700 active:bg-danger-700
                 shadow-lg shadow-danger-500/40
                 flex flex-col items-center justify-center gap-0.5
                 text-white font-bold text-xs
                 border-2 border-white
                 transition-transform duration-150 active:scale-95
                 select-none"
      aria-label="긴급 연락"
    >
      {/* Emergency siren icon */}
      <svg className="w-6 h-6" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
        <path d="M12 2a1 1 0 011 1v1a1 1 0 11-2 0V3a1 1 0 011-1zm4.22 2.22a1 1 0 011.42 1.42l-.71.7a1 1 0 11-1.41-1.41l.7-.71zM3.34 5.64a1 1 0 011.42-1.42l.7.71A1 1 0 114.05 6.35l-.71-.71zM21 11h-1a1 1 0 110-2h1a1 1 0 110 2zM4 11H3a1 1 0 110-2h1a1 1 0 110 2zm14 6H6a2 2 0 010-4h12a2 2 0 010 4zm-1-6a5 5 0 00-10 0h10zm-5 8a2 2 0 100 4 2 2 0 000-4z" />
      </svg>
      <span>긴급</span>
    </Link>
  );
}

// Top navigation bar — shows the app name, user info, and notification bell
function TopBar() {
  const { data: session, status } = useSession();
  const isLoading = status === 'loading';
  const userName = session?.user?.name;

  return (
    <header className="fixed top-0 left-0 right-0 z-40 bg-white border-b border-gray-100 shadow-sm">
      <div className="max-w-lg mx-auto flex items-center justify-between px-4 h-14">
        {/* App logo / name */}
        <Link href="/" className="flex items-center gap-2 min-h-touch" aria-label="홈으로 이동">
          <div className="w-8 h-8 rounded-lg bg-primary-600 flex items-center justify-center">
            <svg className="w-5 h-5 text-white" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
              <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 14H9V8h2v8zm4 0h-2V8h2v8z" />
            </svg>
          </div>
          <span className="text-senior-lg font-bold text-primary-700">{isLoading ? '시니어 포털' : userName ? `${userName}님` : '시니어 포털'}</span>
        </Link>

        <div className="flex items-center gap-1">
          {/* Notification bell */}
          <Link
            href="/notifications"
            className="relative flex items-center justify-center w-10 h-10 rounded-full hover:bg-gray-100 transition-colors"
            aria-label="알림"
          >
            <svg className="w-6 h-6 text-gray-600" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24" aria-hidden="true">
              <path strokeLinecap="round" strokeLinejoin="round" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" />
            </svg>
            <span className="absolute top-1.5 right-1.5 w-2.5 h-2.5 bg-danger-500 rounded-full border border-white" aria-label="읽지 않은 알림" />
          </Link>

          {/* Sign out button */}
          {session && (
            <button
              onClick={() => signOut({ callbackUrl: '/auth/signin' })}
              className="flex items-center justify-center w-10 h-10 rounded-full hover:bg-gray-100 transition-colors"
              aria-label="로그아웃"
            >
              <svg className="w-5 h-5 text-gray-500" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" d="M15.75 9V5.25A2.25 2.25 0 0013.5 3h-6a2.25 2.25 0 00-2.25 2.25v13.5A2.25 2.25 0 007.5 21h6a2.25 2.25 0 002.25-2.25V15m3 0l3-3m0 0l-3-3m3 3H9" />
              </svg>
            </button>
          )}
        </div>
      </div>
    </header>
  );
}

// Bottom navigation bar — primary navigation pattern for mobile senior users
function BottomNavBar() {
  const pathname = usePathname();

  return (
    <nav
      className="fixed bottom-0 left-0 right-0 z-40 bg-white border-t border-gray-200 shadow-[0_-2px_8px_rgba(0,0,0,0.06)]"
      aria-label="주요 메뉴"
    >
      <div className="max-w-lg mx-auto flex items-stretch">
        {NAV_ITEMS.map(({ href, label, icon: Icon }) => {
          const isRoot = (href as string) === '/';
          const finalActive = isRoot ? pathname === '/' : pathname.startsWith(href);

          return (
            <Link
              key={href}
              href={href}
              className={`bottom-nav-item py-2 ${finalActive ? 'active' : ''}`}
              aria-current={finalActive ? 'page' : undefined}
            >
              <Icon active={finalActive} />
              <span
                className={`text-xs font-medium ${finalActive ? 'text-primary-600' : 'text-gray-500'}`}
              >
                {label}
              </span>
            </Link>
          );
        })}
      </div>
    </nav>
  );
}

// Main app shell — wraps all senior portal pages
export default function SeniorAppShell({ children }: { children: React.ReactNode }) {
  return (
    <div className="text-scale-senior min-h-screen bg-gray-50">
      <TopBar />
      <main className="pt-14" id="main-content">
        {children}
      </main>
      <BottomNavBar />
      <EmergencyButton />
    </div>
  );
}
