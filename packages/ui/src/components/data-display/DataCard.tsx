import { type ReactNode, type HTMLAttributes } from 'react';
import { cn } from '../../lib/utils';

interface DataCardProps extends HTMLAttributes<HTMLDivElement> {
  title?: string;
  actions?: ReactNode;
  image?: string;
  imageAlt?: string;
  children: ReactNode;
}

export function DataCard({ title, actions, image, imageAlt, children, className, ...props }: DataCardProps) {
  return (
    <div
      className={cn('bg-white rounded-lg border border-gray-200 overflow-hidden', className)}
      {...props}
    >
      {image && (
        <img src={image} alt={imageAlt ?? ''} className="w-full h-40 object-cover" />
      )}
      {(title || actions) && (
        <div className="flex items-start justify-between gap-4 px-4 pt-4 pb-2">
          {title && <h3 className="text-base font-semibold text-gray-900">{title}</h3>}
          {actions && <div className="flex items-center gap-2 flex-shrink-0">{actions}</div>}
        </div>
      )}
      <div className="px-4 pb-4">{children}</div>
    </div>
  );
}
