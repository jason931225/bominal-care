import { cn } from '../../lib/utils';

interface LoadingSpinnerProps {
  message?: string;
  size?: 'sm' | 'md' | 'lg';
  className?: string;
  centered?: boolean;
}

const sizeClasses = {
  sm: 'h-5 w-5 border-2',
  md: 'h-8 w-8 border-2',
  lg: 'h-12 w-12 border-4',
};

export function LoadingSpinner({ message, size = 'md', className, centered = true }: LoadingSpinnerProps) {
  return (
    <div
      className={cn(
        'flex flex-col items-center justify-center gap-3',
        centered && 'py-12',
        className,
      )}
      role="status"
      aria-label={message ?? '로딩 중...'}
    >
      <div
        className={cn(
          'animate-spin rounded-full border-gray-200 border-t-primary-600',
          sizeClasses[size],
        )}
        aria-hidden="true"
      />
      {message && <p className="text-sm text-gray-500">{message}</p>}
    </div>
  );
}
