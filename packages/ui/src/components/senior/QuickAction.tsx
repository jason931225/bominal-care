import { type ReactNode } from 'react';
import { cn } from '../../lib/utils';

interface QuickActionProps {
  icon: ReactNode;
  label: string;
  href?: string;
  onClick?: () => void;
  badge?: string | number;
  className?: string;
}

export function QuickAction({ icon, label, href, onClick, badge, className }: QuickActionProps) {
  const shared = cn(
    'flex flex-col items-center justify-center gap-2 rounded-xl border border-gray-200 bg-white p-4',
    'hover:bg-primary-50 hover:border-primary-200 transition-colors min-h-[90px]',
    'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary-500',
    className,
  );

  const content = (
    <>
      <span className="relative flex h-10 w-10 items-center justify-center rounded-full bg-primary-100 text-primary-700">
        {icon}
        {badge !== undefined && (
          <span className="absolute -top-1 -right-1 inline-flex h-5 min-w-5 items-center justify-center rounded-full bg-danger-500 px-1 text-[10px] font-bold text-white">
            {badge}
          </span>
        )}
      </span>
      <span className="text-xs font-medium text-gray-700 text-center leading-tight">{label}</span>
    </>
  );

  if (href) {
    return <a href={href} className={shared}>{content}</a>;
  }

  return (
    <button type="button" onClick={onClick} className={shared}>
      {content}
    </button>
  );
}
