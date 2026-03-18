import { type LabelHTMLAttributes } from 'react';
import { cn } from '../../lib/utils';

interface LabelProps extends LabelHTMLAttributes<HTMLLabelElement> {
  required?: boolean;
}

export function Label({ className, required, children, ...props }: LabelProps) {
  return (
    <label className={cn('text-sm font-medium text-gray-700', className)} {...props}>
      {children}
      {required && (
        <span className="ml-1 text-danger-500" aria-hidden="true">
          *
        </span>
      )}
    </label>
  );
}
