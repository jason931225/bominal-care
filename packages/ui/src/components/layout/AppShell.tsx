import { type ReactNode } from 'react';
import { cn } from '../../lib/utils';

interface AppShellProps {
  children: ReactNode;
  sidebar?: ReactNode;
  topBar?: ReactNode;
  bottomNav?: ReactNode;
  className?: string;
}

export function AppShell({ children, sidebar, topBar, bottomNav, className }: AppShellProps) {
  return (
    <div className="min-h-screen flex flex-col bg-gray-50">
      {topBar && <header className="sticky top-0 z-40">{topBar}</header>}
      <div className="flex flex-1 overflow-hidden">
        {sidebar && (
          <aside className="hidden md:flex flex-shrink-0">{sidebar}</aside>
        )}
        <main className={cn('flex-1 overflow-y-auto', bottomNav && 'pb-16 md:pb-0', className)}>
          {children}
        </main>
      </div>
      {bottomNav && (
        <nav className="fixed bottom-0 left-0 right-0 z-40 md:hidden">{bottomNav}</nav>
      )}
    </div>
  );
}
