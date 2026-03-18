'use client';

import Link from 'next/link';
import { useSession, signOut } from 'next-auth/react';

interface NavItem {
  label: string;
  href: string;
  icon: React.ReactNode;
  activeIcon: React.ReactNode;
}

function ScheduleIcon({ filled }: { filled?: boolean }) {
  return filled ? (
    <svg className="w-6 h-6" viewBox="0 0 24 24" fill="currentColor">
      <path d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
    </svg>
  ) : (
    <svg className="w-6 h-6" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.8}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
    </svg>
  );
}

function ClientsIcon({ filled }: { filled?: boolean }) {
  return filled ? (
    <svg className="w-6 h-6" viewBox="0 0 24 24" fill="currentColor">
      <path d="M17 21v-2a4 4 0 00-4-4H5a4 4 0 00-4 4v2M23 21v-2a4 4 0 00-3-3.87M16 3.13a4 4 0 010 7.75M9 7a4 4 0 110 8 4 4 0 010-8z" />
    </svg>
  ) : (
    <svg className="w-6 h-6" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.8}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M17 21v-2a4 4 0 00-4-4H5a4 4 0 00-4 4v2M23 21v-2a4 4 0 00-3-3.87M16 3.13a4 4 0 010 7.75M9 7a4 4 0 110 8 4 4 0 010-8z" />
    </svg>
  );
}

function TasksIcon({ filled }: { filled?: boolean }) {
  return filled ? (
    <svg className="w-6 h-6" viewBox="0 0 24 24" fill="currentColor">
      <path fillRule="evenodd" d="M9 2a1 1 0 000 2h2a1 1 0 100-2H9zM4 5a2 2 0 012-2 3 3 0 003 3h2a3 3 0 003-3 2 2 0 012 2v11a2 2 0 01-2 2H6a2 2 0 01-2-2V5zm3 4a1 1 0 000 2h.01a1 1 0 100-2H7zm3 0a1 1 0 000 2h3a1 1 0 100-2h-3zm-3 4a1 1 0 100 2h.01a1 1 0 100-2H7zm3 0a1 1 0 100 2h3a1 1 0 100-2h-3z" clipRule="evenodd" />
    </svg>
  ) : (
    <svg className="w-6 h-6" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.8}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4" />
    </svg>
  );
}

function BellIcon({ filled }: { filled?: boolean }) {
  return filled ? (
    <svg className="w-6 h-6" viewBox="0 0 24 24" fill="currentColor">
      <path d="M10 2a6 6 0 00-6 6v3.586l-.707.707A1 1 0 004 14h16a1 1 0 00.707-1.707L20 11.586V8a6 6 0 00-6-6h-4zM10 18a3 3 0 006 0H10z" />
    </svg>
  ) : (
    <svg className="w-6 h-6" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.8}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" />
    </svg>
  );
}

function ProfileIcon({ filled }: { filled?: boolean }) {
  return filled ? (
    <svg className="w-6 h-6" viewBox="0 0 24 24" fill="currentColor">
      <path fillRule="evenodd" d="M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z" clipRule="evenodd" />
    </svg>
  ) : (
    <svg className="w-6 h-6" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.8}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
    </svg>
  );
}

interface CaregiverAppShellProps {
  children: React.ReactNode;
  activeTab: 'schedule' | 'clients' | 'tasks' | 'notifications' | 'profile';
  title?: string;
  showBackButton?: boolean;
  backHref?: string;
  headerRight?: React.ReactNode;
}

export default function CaregiverAppShell({
  children,
  activeTab,
  title,
  showBackButton = false,
  backHref,
  headerRight,
}: CaregiverAppShellProps) {
  const navItems: NavItem[] = [
    {
      label: '일정',
      href: '/schedule',
      icon: <ScheduleIcon />,
      activeIcon: <ScheduleIcon filled />,
    },
    {
      label: '이용자',
      href: '/clients',
      icon: <ClientsIcon />,
      activeIcon: <ClientsIcon filled />,
    },
    {
      label: '업무',
      href: '/tasks',
      icon: <TasksIcon />,
      activeIcon: <TasksIcon filled />,
    },
    {
      label: '알림',
      href: '/notifications',
      icon: <BellIcon />,
      activeIcon: <BellIcon filled />,
    },
    {
      label: '내정보',
      href: '/profile',
      icon: <ProfileIcon />,
      activeIcon: <ProfileIcon filled />,
    },
  ];

  const tabMap: Record<CaregiverAppShellProps['activeTab'], string> = {
    schedule: '/schedule',
    clients: '/clients',
    tasks: '/tasks',
    notifications: '/notifications',
    profile: '/profile',
  };

  const { data: session } = useSession();

  return (
    <div className="min-h-screen bg-slate-50 flex flex-col">
      {/* Header */}
      {title && (
        <header className="sticky top-0 z-20 bg-white border-b border-slate-100">
          <div className="flex items-center px-4 h-14 top-safe">
            {showBackButton && backHref && (
              <Link
                href={backHref}
                className="mr-3 p-2 -ml-2 rounded-full active:bg-slate-100 transition-colors"
                aria-label="뒤로"
              >
                <svg
                  className="w-5 h-5 text-slate-700"
                  fill="none"
                  stroke="currentColor"
                  strokeWidth={2.5}
                  viewBox="0 0 24 24"
                >
                  <path strokeLinecap="round" strokeLinejoin="round" d="M15 19l-7-7 7-7" />
                </svg>
              </Link>
            )}
            <h1 className="flex-1 text-base font-semibold text-slate-900">{title}</h1>
            <div className="flex items-center gap-1">
              {session?.user?.name && (
                <span className="text-xs text-slate-500 mr-1">{session.user.name}</span>
              )}
              {session && (
                <button
                  onClick={() => signOut({ callbackUrl: '/auth/signin' })}
                  className="p-2 rounded-full text-slate-400 hover:text-slate-600 active:bg-slate-100 transition-colors"
                  aria-label="로그아웃"
                >
                  <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                    <path strokeLinecap="round" strokeLinejoin="round" d="M15.75 9V5.25A2.25 2.25 0 0013.5 3h-6a2.25 2.25 0 00-2.25 2.25v13.5A2.25 2.25 0 007.5 21h6a2.25 2.25 0 002.25-2.25V15m3 0l3-3m0 0l-3-3m3 3H9" />
                  </svg>
                </button>
              )}
              {headerRight && <div>{headerRight}</div>}
            </div>
          </div>
        </header>
      )}

      {/* Content */}
      <main className="flex-1 overflow-y-auto bottom-safe">{children}</main>

      {/* Bottom Navigation */}
      <nav className="fixed bottom-0 left-0 right-0 z-30 bg-white border-t border-slate-200">
        <div
          className="flex items-stretch"
          style={{ paddingBottom: 'env(safe-area-inset-bottom, 0px)' }}
        >
          {navItems.map((item) => {
            const isActive = tabMap[activeTab] === item.href;
            return (
              <Link
                key={item.href}
                href={item.href}
                className={`flex-1 flex flex-col items-center justify-center py-2 gap-0.5 active:bg-slate-50 transition-colors ${
                  isActive ? 'text-blue-600' : 'text-slate-400'
                }`}
              >
                {isActive ? item.activeIcon : item.icon}
                <span className={`text-xs font-medium ${isActive ? 'text-blue-600' : 'text-slate-400'}`}>
                  {item.label}
                </span>
              </Link>
            );
          })}
        </div>
      </nav>
    </div>
  );
}
