'use client';
import { type ReactNode } from 'react';
import { cn } from '../../lib/utils';

interface BottomNavItem {
  href: string;
  icon: ReactNode;
  label: string;
  active?: boolean;
}

interface BottomNavBarProps {
  items: BottomNavItem[];
  className?: string;
  onNavigate?: (href: string) => void;
}

export function BottomNavBar({ items, className, onNavigate }: BottomNavBarProps) {
  const limitedItems = items.slice(0, 5);
  return (
    <div
      className={cn(
        'flex items-stretch bg-white border-t border-gray-200 safe-bottom',
        className,
      )}
    >
      {limitedItems.map((item) => (
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
            'flex flex-1 flex-col items-center justify-center gap-1 py-2 min-h-touch text-xs font-medium transition-colors',
            item.active ? 'text-primary-600' : 'text-gray-500 hover:text-gray-900',
          )}
          aria-current={item.active ? 'page' : undefined}
        >
          <span className="h-6 w-6">{item.icon}</span>
          <span>{item.label}</span>
        </a>
      ))}
    </div>
  );
}
