import { type ReactNode } from 'react';
import { cva, type VariantProps } from 'class-variance-authority';
import { cn } from '../../lib/utils';

const cardGridVariants = cva('grid gap-4', {
  variants: {
    columns: {
      1: 'grid-cols-1',
      2: 'grid-cols-1 sm:grid-cols-2',
      3: 'grid-cols-1 sm:grid-cols-2 lg:grid-cols-3',
      4: 'grid-cols-1 sm:grid-cols-2 lg:grid-cols-4',
    },
  },
  defaultVariants: { columns: 3 },
});

interface CardGridProps extends VariantProps<typeof cardGridVariants> {
  children: ReactNode;
  className?: string;
}

export function CardGrid({ columns, children, className }: CardGridProps) {
  return <div className={cn(cardGridVariants({ columns, className }))}>{children}</div>;
}
