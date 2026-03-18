import { type ReactNode } from 'react';
import { cn } from '../../lib/utils';

interface ScheduleBlockProps {
  startTime: string;
  endTime?: string;
  title: string;
  description?: string;
  color?: 'primary' | 'secondary' | 'success' | 'warning' | 'danger' | 'info';
  icon?: ReactNode;
  className?: string;
}

const colorMap: Record<NonNullable<ScheduleBlockProps['color']>, string> = {
  primary: 'border-l-primary-500 bg-primary-50',
  secondary: 'border-l-secondary-500 bg-secondary-50',
  success: 'border-l-success-500 bg-success-50',
  warning: 'border-l-warning-500 bg-warning-50',
  danger: 'border-l-danger-500 bg-danger-50',
  info: 'border-l-info-500 bg-info-50',
};

export function ScheduleBlock({ startTime, endTime, title, description, color = 'primary', icon, className }: ScheduleBlockProps) {
  return (
    <div
      className={cn(
        'flex items-start gap-3 rounded-r-lg border-l-4 px-3 py-3',
        colorMap[color],
        className,
      )}
    >
      <div className="flex flex-col items-center text-xs text-gray-500 min-w-[48px] flex-shrink-0">
        <span className="font-semibold">{startTime}</span>
        {endTime && <span>~ {endTime}</span>}
      </div>
      {icon && <span className="flex-shrink-0 mt-0.5 text-gray-600">{icon}</span>}
      <div className="flex-1 min-w-0">
        <p className="font-semibold text-gray-900 text-sm">{title}</p>
        {description && <p className="mt-0.5 text-xs text-gray-500">{description}</p>}
      </div>
    </div>
  );
}
