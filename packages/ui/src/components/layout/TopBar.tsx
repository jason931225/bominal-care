import { type ReactNode } from 'react';
import { cn } from '../../lib/utils';

interface TopBarUser {
  name: string;
  avatarUrl?: string;
}

interface TopBarProps {
  title: string;
  actions?: ReactNode;
  user?: TopBarUser;
  className?: string;
}

export function TopBar({ title, actions, user, className }: TopBarProps) {
  return (
    <div
      className={cn(
        'flex items-center justify-between h-14 px-4 bg-white border-b border-gray-200 shadow-sm',
        className,
      )}
    >
      <span className="text-lg font-semibold text-gray-900 truncate">{title}</span>
      <div className="flex items-center gap-3">
        {actions}
        {user && (
          <div className="flex items-center gap-2">
            {user.avatarUrl ? (
              <img src={user.avatarUrl} alt={user.name} className="h-8 w-8 rounded-full object-cover" />
            ) : (
              <span className="inline-flex h-8 w-8 items-center justify-center rounded-full bg-primary-100 text-primary-700 text-xs font-medium">
                {user.name.slice(0, 2)}
              </span>
            )}
            <span className="hidden sm:block text-sm text-gray-700">{user.name}</span>
          </div>
        )}
      </div>
    </div>
  );
}
