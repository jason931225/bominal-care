'use client';

import Link from 'next/link';
import { usePathname } from 'next/navigation';
import { useState } from 'react';
import { useSession, signOut } from 'next-auth/react';

// ---------------------------------------------------------------------------
// SVG Icon Components
// ---------------------------------------------------------------------------

function HomeIcon() {
  return (
    <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
    </svg>
  );
}

function ClipboardIcon() {
  return (
    <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-3 7h3m-3 4h3m-6-4h.01M9 16h.01" />
    </svg>
  );
}

function BellIcon() {
  return (
    <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" />
    </svg>
  );
}

function ChartIcon() {
  return (
    <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
    </svg>
  );
}

function HeartIcon() {
  return (
    <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
    </svg>
  );
}

function PillIcon() {
  return (
    <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M4.5 12.75l3-3m0 0l3-3m-3 3L4.5 6.75m3 3l3 3M14.25 3.75L19.5 9m0 0l-5.25 5.25M19.5 9l-5.25-5.25M19.5 9l5.25 5.25" />
      <rect x="3" y="11" width="18" height="6" rx="3" transform="rotate(-45 12 12)" />
    </svg>
  );
}

function CheckIcon() {
  return (
    <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
    </svg>
  );
}

function HandshakeIcon() {
  return (
    <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M18 13V6a2 2 0 00-2-2H8a2 2 0 00-2 2v7m12 0l-4 4m4-4h-4m-8 0l4 4m-4-4h4m-4 4v3a2 2 0 002 2h8a2 2 0 002-2v-3" />
    </svg>
  );
}

function SearchIcon() {
  return (
    <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
    </svg>
  );
}

function DocumentIcon() {
  return (
    <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
    </svg>
  );
}

function CreditCardIcon() {
  return (
    <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z" />
    </svg>
  );
}

function GovernmentIcon() {
  return (
    <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M8 14v3m4-3v3m4-3v3M3 21h18M3 10h18M12 3l9 7H3l9-7z" />
    </svg>
  );
}

function UserIcon() {
  return (
    <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
    </svg>
  );
}

function GearIcon() {
  return (
    <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
      <path strokeLinecap="round" strokeLinejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
    </svg>
  );
}

// ---------------------------------------------------------------------------
// Data
// ---------------------------------------------------------------------------

const SENIORS = [
  { id: 'senior-1', name: '김복순 (모)', age: 78 },
  { id: 'senior-2', name: '이정남 (부)', age: 81 },
];

interface NavItem {
  href: string;
  label: string;
  icon: () => React.ReactElement;
  badge?: number;
}

interface NavSection {
  section: string;
  items: readonly NavItem[];
}

const NAV_ITEMS: readonly NavSection[] = [
  {
    section: '홈',
    items: [
      { href: '/', label: '대시보드', icon: HomeIcon },
      { href: '/timeline', label: '케어 타임라인', icon: ClipboardIcon },
      { href: '/notifications', label: '알림', icon: BellIcon, badge: 3 },
      { href: '/observability', label: '모니터링', icon: ChartIcon },
    ],
  },
  {
    section: '케어 관리',
    items: [
      { href: '/care', label: '케어 플랜', icon: HeartIcon },
      { href: '/medications', label: '복약 현황', icon: PillIcon },
      { href: '/approvals', label: '승인 대기', icon: CheckIcon, badge: 2 },
      { href: '/help-senior', label: '대리 서비스', icon: HandshakeIcon },
    ],
  },
  {
    section: '매칭 & 계약',
    items: [
      { href: '/matching', label: '매칭 요청', icon: SearchIcon },
      { href: '/documents', label: '문서 관리', icon: DocumentIcon },
      { href: '/payments', label: '결제 내역', icon: CreditCardIcon },
    ],
  },
  {
    section: '지원',
    items: [
      { href: '/eligibility', label: '수급 자격', icon: GovernmentIcon },
      { href: '/profile', label: '내 프로필', icon: UserIcon },
      { href: '/settings', label: '설정', icon: GearIcon },
    ],
  },
];

interface BottomNavItem {
  href: string;
  label: string;
  icon: () => React.ReactElement;
  badge?: number;
}

const BOTTOM_NAV: readonly BottomNavItem[] = [
  { href: '/', label: '홈', icon: HomeIcon },
  { href: '/timeline', label: '타임라인', icon: ClipboardIcon },
  { href: '/notifications', label: '알림', icon: BellIcon, badge: 3 },
  { href: '/matching', label: '매칭', icon: SearchIcon },
  { href: '/profile', label: '프로필', icon: UserIcon },
];

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

interface FamilyAppShellProps {
  children: React.ReactNode;
}

export default function FamilyAppShell({ children }: FamilyAppShellProps) {
  const pathname = usePathname();
  const { data: session } = useSession();
  const userName = session?.user?.name ?? '가족';
  const userInitial = userName.charAt(0);
  const [selectedSenior, setSelectedSenior] = useState(SENIORS[0]);
  const [sidebarOpen, setSidebarOpen] = useState(false);
  const [seniorDropdownOpen, setSeniorDropdownOpen] = useState(false);

  const totalBadge = NAV_ITEMS.flatMap((s) => s.items).reduce(
    (sum, item) => sum + (item.badge ?? 0),
    0
  );

  return (
    <div className="min-h-screen bg-gray-50 flex flex-col">
      {/* Top Bar */}
      <header className="bg-white border-b border-gray-200 sticky top-0 z-40">
        <div className="flex items-center justify-between px-4 h-14">
          {/* Left: Hamburger (mobile) + Logo */}
          <div className="flex items-center gap-3">
            <button
              className="lg:hidden p-1.5 rounded-md text-gray-500 hover:bg-gray-100"
              onClick={() => setSidebarOpen(!sidebarOpen)}
              aria-label="메뉴 열기"
            >
              <span className="block w-5 h-0.5 bg-current mb-1" />
              <span className="block w-5 h-0.5 bg-current mb-1" />
              <span className="block w-5 h-0.5 bg-current" />
            </button>
            <Link href="/" className="flex items-center gap-2">
              <span className="text-blue-600 text-xl">
                <svg className="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                  <path strokeLinecap="round" strokeLinejoin="round" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4" />
                </svg>
              </span>
              <span className="font-bold text-gray-900 hidden sm:block">가족 케어 포털</span>
            </Link>
          </div>

          {/* Center: Senior Selector */}
          <div className="relative">
            <button
              onClick={() => setSeniorDropdownOpen(!seniorDropdownOpen)}
              className="flex items-center gap-2 px-3 py-1.5 bg-blue-50 border border-blue-200 rounded-full text-sm font-medium text-blue-700 hover:bg-blue-100 transition-colors"
            >
              <span className="w-6 h-6 rounded-full bg-blue-200 flex items-center justify-center text-xs">
                👴
              </span>
              <span className="max-w-32 truncate">{selectedSenior.name}</span>
              <span className="text-blue-400">▾</span>
            </button>
            {seniorDropdownOpen && (
              <div className="absolute top-full left-1/2 -translate-x-1/2 mt-1 w-56 bg-white rounded-xl shadow-lg border border-gray-200 py-1 z-50">
                <p className="px-3 py-1.5 text-xs text-gray-400 font-medium uppercase tracking-wide">
                  돌봄 대상자 선택
                </p>
                {SENIORS.map((s) => (
                  <button
                    key={s.id}
                    onClick={() => {
                      setSelectedSenior(s);
                      setSeniorDropdownOpen(false);
                    }}
                    className={`w-full flex items-center gap-2 px-3 py-2 text-sm hover:bg-gray-50 transition-colors ${
                      selectedSenior.id === s.id ? 'text-blue-600 bg-blue-50' : 'text-gray-700'
                    }`}
                  >
                    <span className="text-base">👴</span>
                    <div className="text-left">
                      <p className="font-medium">{s.name}</p>
                      <p className="text-xs text-gray-400">{s.age}세</p>
                    </div>
                    {selectedSenior.id === s.id && <span className="ml-auto text-blue-500">✓</span>}
                  </button>
                ))}
                <hr className="my-1 border-gray-100" />
                <button className="w-full flex items-center gap-2 px-3 py-2 text-sm text-gray-500 hover:bg-gray-50">
                  <span>+</span>
                  <span>돌봄 대상자 추가</span>
                </button>
              </div>
            )}
          </div>

          {/* Right: Notifications + Avatar */}
          <div className="flex items-center gap-2">
            <Link
              href="/notifications"
              className="relative p-2 rounded-full text-gray-500 hover:bg-gray-100"
            >
              <BellIcon />
              {totalBadge > 0 && (
                <span className="absolute top-1 right-1 w-4 h-4 bg-red-500 text-white text-xs rounded-full flex items-center justify-center font-bold">
                  {totalBadge}
                </span>
              )}
            </Link>
            <Link href="/profile" className="w-8 h-8 rounded-full bg-blue-600 text-white text-sm font-bold flex items-center justify-center hover:bg-blue-700">
              {userInitial}
            </Link>
            {session && (
              <button
                onClick={() => signOut({ callbackUrl: '/auth/signin' })}
                className="p-2 rounded-full text-gray-400 hover:text-gray-600 hover:bg-gray-100 transition-colors"
                aria-label="로그아웃"
              >
                <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                  <path strokeLinecap="round" strokeLinejoin="round" d="M15.75 9V5.25A2.25 2.25 0 0013.5 3h-6a2.25 2.25 0 00-2.25 2.25v13.5A2.25 2.25 0 007.5 21h6a2.25 2.25 0 002.25-2.25V15m3 0l3-3m0 0l-3-3m3 3H9" />
                </svg>
              </button>
            )}
          </div>
        </div>
      </header>

      <div className="flex flex-1 overflow-hidden">
        {/* Sidebar Overlay (mobile) */}
        {sidebarOpen && (
          <div
            className="fixed inset-0 bg-black/40 z-30 lg:hidden"
            onClick={() => setSidebarOpen(false)}
          />
        )}

        {/* Sidebar */}
        <aside
          className={`
            fixed lg:static inset-y-0 left-0 z-40
            w-64 bg-white border-r border-gray-200
            transform transition-transform duration-200
            flex flex-col
            ${sidebarOpen ? 'translate-x-0' : '-translate-x-full'}
            lg:translate-x-0 lg:flex
            top-14
          `}
        >
          <nav className="flex-1 overflow-y-auto py-4 px-3">
            {NAV_ITEMS.map((section) => (
              <div key={section.section} className="mb-5">
                <p className="text-xs font-semibold text-gray-400 uppercase tracking-wider px-3 mb-1.5">
                  {section.section}
                </p>
                {section.items.map((item) => {
                  const isActive = pathname === item.href;
                  const Icon = item.icon;
                  return (
                    <Link
                      key={item.href}
                      href={item.href}
                      onClick={() => setSidebarOpen(false)}
                      className={`
                        flex items-center gap-2.5 px-3 py-2 rounded-lg mb-0.5 text-sm font-medium
                        transition-colors
                        ${isActive
                          ? 'bg-blue-50 text-blue-700'
                          : 'text-gray-600 hover:bg-gray-100 hover:text-gray-900'
                        }
                      `}
                    >
                      <span className="w-5 text-center flex items-center justify-center"><Icon /></span>
                      <span className="flex-1">{item.label}</span>
                      {item.badge && item.badge > 0 && (
                        <span className="w-5 h-5 bg-red-500 text-white text-xs rounded-full flex items-center justify-center font-bold">
                          {item.badge}
                        </span>
                      )}
                    </Link>
                  );
                })}
              </div>
            ))}
          </nav>

          {/* Sidebar Footer */}
          <div className="border-t border-gray-100 p-4">
            <div className="flex items-center gap-3">
              <div className="w-8 h-8 rounded-full bg-blue-600 text-white text-sm font-bold flex items-center justify-center">
                {userInitial}
              </div>
              <div className="flex-1 min-w-0">
                <p className="text-sm font-medium text-gray-900 truncate">{userName}</p>
                <p className="text-xs text-gray-500 truncate">{session?.user?.email ?? ''}</p>
              </div>
            </div>
          </div>
        </aside>

        {/* Main Content */}
        <main className="flex-1 overflow-y-auto pb-16 lg:pb-0">
          {children}
        </main>
      </div>

      {/* Bottom Tab Bar (Mobile) */}
      <nav className="lg:hidden fixed bottom-0 inset-x-0 bg-white border-t border-gray-200 z-40">
        <div className="flex">
          {BOTTOM_NAV.map((item) => {
            const isActive = pathname === item.href;
            const Icon = item.icon;
            return (
              <Link
                key={item.href}
                href={item.href}
                className={`
                  flex-1 flex flex-col items-center py-2 text-xs font-medium
                  ${isActive ? 'text-blue-600' : 'text-gray-500'}
                  relative
                `}
              >
                <span className="mb-0.5"><Icon /></span>
                <span>{item.label}</span>
                {item.badge && item.badge > 0 && (
                  <span className="absolute top-1.5 right-1/4 w-4 h-4 bg-red-500 text-white text-xs rounded-full flex items-center justify-center font-bold">
                    {item.badge}
                  </span>
                )}
              </Link>
            );
          })}
        </div>
      </nav>
    </div>
  );
}
