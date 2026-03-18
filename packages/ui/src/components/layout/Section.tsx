import { type ReactNode, type HTMLAttributes } from 'react';
import { cn } from '../../lib/utils';

interface SectionProps extends HTMLAttributes<HTMLElement> {
  title?: string;
  description?: string;
  children: ReactNode;
  noPadding?: boolean;
}

export function Section({ title, description, children, noPadding = false, className, ...props }: SectionProps) {
  return (
    <section className={cn('bg-white rounded-lg border border-gray-200', className)} {...props}>
      {(title || description) && (
        <div className="px-4 py-4 border-b border-gray-200">
          {title && <h2 className="text-base font-semibold text-gray-900">{title}</h2>}
          {description && <p className="mt-1 text-sm text-gray-600">{description}</p>}
        </div>
      )}
      <div className={cn(!noPadding && 'p-4')}>{children}</div>
    </section>
  );
}
