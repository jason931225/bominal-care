import { type ReactNode } from 'react';
import { cn } from '../../lib/utils';

type ChangeType = 'increase' | 'decrease' | 'neutral';

interface StatWidgetProps {
  label: string;
  value: string | number;
  change?: string | number;
  changeType?: ChangeType;
  icon?: ReactNode;
  className?: string;
}

const changeColors: Record<ChangeType, string> = {
  increase: 'text-success-700',
  decrease: 'text-danger-700',
  neutral: 'text-gray-500',
};

const changeIcons: Record<ChangeType, string> = {
  increase: '↑',
  decrease: '↓',
  neutral: '→',
};

export function StatWidget({ label, value, change, changeType = 'neutral', icon, className }: StatWidgetProps) {
  return (
    <div className={cn('bg-white rounded-lg border border-gray-200 p-4 flex items-start gap-4', className)}>
      {icon && (
        <span className="flex h-10 w-10 flex-shrink-0 items-center justify-center rounded-lg bg-primary-50 text-primary-600">
          {icon}
        </span>
      )}
      <div className="flex-1 min-w-0">
        <p className="text-sm text-gray-500 truncate">{label}</p>
        <p className="mt-1 text-2xl font-bold text-gray-900">{value}</p>
        {change !== undefined && (
          <p className={cn('mt-1 text-sm font-medium', changeColors[changeType])}>
            {changeIcons[changeType]} {change}
          </p>
        )}
      </div>
    </div>
  );
}
