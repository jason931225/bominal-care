'use client';
import { forwardRef, type ButtonHTMLAttributes, type ReactNode } from 'react';
import { cva, type VariantProps } from 'class-variance-authority';
import { cn } from '../../lib/utils';

const largeButtonVariants = cva(
  'inline-flex flex-col items-center justify-center gap-2 rounded-xl font-semibold transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary-500 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 min-h-[56px] px-6 py-3 text-senior-lg',
  {
    variants: {
      variant: {
        primary: 'bg-primary-600 text-white hover:bg-primary-700 active:bg-primary-800',
        secondary: 'bg-secondary-600 text-white hover:bg-secondary-700 active:bg-secondary-800',
        outline: 'border-2 border-primary-600 text-primary-600 hover:bg-primary-50',
        ghost: 'bg-gray-100 text-gray-800 hover:bg-gray-200',
        danger: 'bg-danger-500 text-white hover:bg-danger-700',
      },
      fullWidth: {
        true: 'w-full',
        false: '',
      },
    },
    defaultVariants: { variant: 'primary', fullWidth: false },
  },
);

interface LargeButtonProps
  extends ButtonHTMLAttributes<HTMLButtonElement>,
    VariantProps<typeof largeButtonVariants> {
  icon?: ReactNode;
  children?: ReactNode;
}

export const LargeButton = forwardRef<HTMLButtonElement, LargeButtonProps>(
  ({ className, variant, fullWidth, icon, children, ...props }, ref) => (
    <button
      ref={ref}
      className={cn(largeButtonVariants({ variant, fullWidth, className }))}
      {...props}
    >
      {icon && <span className="h-7 w-7">{icon}</span>}
      {children && <span>{children}</span>}
    </button>
  ),
);

LargeButton.displayName = 'LargeButton';
