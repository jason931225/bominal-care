import { type HTMLAttributes } from 'react';
import { cn } from '../../lib/utils';

interface SkeletonProps extends HTMLAttributes<HTMLDivElement> {
  width?: string | number;
  height?: string | number;
  rounded?: boolean;
  circle?: boolean;
  lines?: number;
}

export function Skeleton({ className, width, height, rounded = false, circle = false, lines, style, ...props }: SkeletonProps) {
  if (lines && lines > 1) {
    return (
      <div className="flex flex-col gap-2">
        {Array.from({ length: lines }).map((_, i) => (
          <Skeleton
            key={i}
            width={i === lines - 1 && lines > 1 ? '75%' : width}
            height={height ?? 16}
            rounded={rounded}
            className={className}
          />
        ))}
      </div>
    );
  }

  return (
    <div
      className={cn(
        'animate-pulse bg-gray-200',
        circle ? 'rounded-full' : rounded ? 'rounded-md' : 'rounded',
        className,
      )}
      style={{
        width: width ?? '100%',
        height: height ?? 16,
        ...style,
      }}
      aria-hidden="true"
      {...props}
    />
  );
}
