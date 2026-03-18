import { type ReactNode } from 'react';
import { cn } from '../../lib/utils';

interface TimelineItem {
  date: string;
  title: string;
  description?: string;
  icon?: ReactNode;
  color?: 'default' | 'primary' | 'success' | 'warning' | 'danger';
}

interface TimelineProps {
  items: TimelineItem[];
  className?: string;
}

const colorMap: Record<NonNullable<TimelineItem['color']>, string> = {
  default: 'bg-gray-400',
  primary: 'bg-primary-500',
  success: 'bg-success-500',
  warning: 'bg-warning-500',
  danger: 'bg-danger-500',
};

export function Timeline({ items, className }: TimelineProps) {
  return (
    <ol className={cn('relative flex flex-col gap-0', className)}>
      {items.map((item, idx) => (
        <li key={idx} className="relative flex gap-4 pb-6 last:pb-0">
          <div className="flex flex-col items-center">
            <span
              className={cn(
                'flex h-8 w-8 flex-shrink-0 items-center justify-center rounded-full text-white text-sm',
                colorMap[item.color ?? 'default'],
              )}
            >
              {item.icon ?? <span className="h-2 w-2 rounded-full bg-white" />}
            </span>
            {idx < items.length - 1 && (
              <span className="mt-1 flex-1 w-0.5 bg-gray-200" aria-hidden="true" />
            )}
          </div>
          <div className="pb-2 pt-1 flex-1">
            <p className="text-xs text-gray-500 mb-0.5">{item.date}</p>
            <p className="text-sm font-medium text-gray-900">{item.title}</p>
            {item.description && <p className="mt-0.5 text-sm text-gray-600">{item.description}</p>}
          </div>
        </li>
      ))}
    </ol>
  );
}
