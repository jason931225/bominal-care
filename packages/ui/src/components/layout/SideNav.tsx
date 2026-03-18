'use client';
import { type ReactNode } from 'react';
import { cn } from '../../lib/utils';

interface NavItem {
  href: string;
  icon: ReactNode;
  label: string;
  badge?: string | number;
  active?: boolean;
}

interface SideNavProps {
  items: NavItem[];
  collapsed?: boolean;
  className?: string;
  onNavigate?: (href: string) => void;
}

export function SideNav({ items, collapsed = false, className, onNavigate }: SideNavProps) {
  return (
    <nav
      className={cn(
        'flex flex-col gap-1 bg-white border-r border-gray-200 py-4 transition-all duration-200',
        collapsed ? 'w-16' : 'w-56',
        className,
      )}
    >
      {items.map((item) => (
        <a
          key={item.href}
          href={item.href}
          onClick={(e) => {
            if (onNavigate) {
              e.preventDefault();
              onNavigate(item.href);
            }
          }}
          className={cn(
            'flex items-center gap-3 mx-2 px-3 py-2 rounded-md text-sm font-medium transition-colors',
            item.active
              ? 'bg-primary-50 text-primary-700'
              : 'text-gray-600 hover:bg-gray-100 hover:text-gray-900',
          )}
          aria-current={item.active ? 'page' : undefined}
          title={collapsed ? item.label : undefined}
        >
          <span className="flex-shrink-0 h-5 w-5">{item.icon}</span>
          {!collapsed && <span className="truncate">{item.label}</span>}
          {!collapsed && item.badge !== undefined && (
            <span className="ml-auto inline-flex h-5 min-w-5 items-center justify-center rounded-full bg-primary-600 px-1 text-xs text-white">
              {item.badge}
            </span>
          )}
        </a>
      ))}
    </nav>
  );
}
