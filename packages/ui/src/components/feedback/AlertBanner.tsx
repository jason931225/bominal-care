'use client';
import { type ReactNode } from 'react';
import { cn } from '../../lib/utils';

type AlertVariant = 'info' | 'success' | 'warning' | 'danger';

interface AlertBannerProps {
  variant?: AlertVariant;
  title: string;
  description?: string;
  onDismiss?: () => void;
  className?: string;
  children?: ReactNode;
}

const variantStyles: Record<AlertVariant, string> = {
  info: 'bg-info-50 border-info-200 text-info-900',
  success: 'bg-success-50 border-success-200 text-success-900',
  warning: 'bg-warning-50 border-warning-200 text-warning-900',
  danger: 'bg-danger-50 border-danger-200 text-danger-900',
};

const iconMap: Record<AlertVariant, string> = {
  info: 'ℹ',
  success: '✓',
  warning: '⚠',
  danger: '✕',
};

const iconBg: Record<AlertVariant, string> = {
  info: 'bg-info-500 text-white',
  success: 'bg-success-500 text-white',
  warning: 'bg-warning-500 text-white',
  danger: 'bg-danger-500 text-white',
};

export function AlertBanner({ variant = 'info', title, description, onDismiss, className, children }: AlertBannerProps) {
  return (
    <div
      role="alert"
      className={cn(
        'flex items-start gap-3 w-full rounded-lg border px-4 py-3',
        variantStyles[variant],
        className,
      )}
    >
      <span className={cn('flex h-6 w-6 flex-shrink-0 items-center justify-center rounded-full text-xs font-bold', iconBg[variant])}>
        {iconMap[variant]}
      </span>
      <div className="flex-1 min-w-0">
        <p className="font-semibold text-sm">{title}</p>
        {description && <p className="mt-0.5 text-sm opacity-80">{description}</p>}
        {children}
      </div>
      {onDismiss && (
        <button
          type="button"
          onClick={onDismiss}
          className="flex-shrink-0 opacity-60 hover:opacity-100 transition-opacity ml-2"
          aria-label="닫기"
        >
          ✕
        </button>
      )}
    </div>
  );
}
