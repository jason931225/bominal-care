import { cn } from '../../lib/utils';

type StatusColor = 'default' | 'primary' | 'success' | 'warning' | 'danger' | 'info';

export type StatusMap = Record<string, { label?: string; color: StatusColor }>;

const colorClasses: Record<StatusColor, string> = {
  default: 'bg-gray-100 text-gray-700',
  primary: 'bg-primary-100 text-primary-700',
  success: 'bg-success-50 text-success-700',
  warning: 'bg-warning-50 text-warning-700',
  danger: 'bg-danger-50 text-danger-700',
  info: 'bg-info-50 text-info-700',
};

const defaultStatusMap: StatusMap = {
  active: { label: '활성', color: 'success' },
  inactive: { label: '비활성', color: 'default' },
  pending: { label: '대기중', color: 'warning' },
  error: { label: '오류', color: 'danger' },
  completed: { label: '완료', color: 'primary' },
};

interface StatusBadgeProps {
  status: string;
  statusMap?: StatusMap;
  className?: string;
}

export function StatusBadge({ status, statusMap, className }: StatusBadgeProps) {
  const map = statusMap ?? defaultStatusMap;
  const config = map[status] ?? { label: status, color: 'default' as StatusColor };
  return (
    <span
      className={cn(
        'inline-flex items-center rounded-full px-2.5 py-1 text-xs font-medium',
        colorClasses[config.color],
        className,
      )}
    >
      {config.label ?? status}
    </span>
  );
}
