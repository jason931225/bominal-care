import { type ImgHTMLAttributes } from 'react';
import { cva, type VariantProps } from 'class-variance-authority';
import { cn } from '../../lib/utils';

const avatarVariants = cva('inline-flex items-center justify-center rounded-full bg-primary-100 text-primary-700 font-medium overflow-hidden flex-shrink-0', {
  variants: {
    size: {
      sm: 'h-8 w-8 text-xs',
      md: 'h-10 w-10 text-sm',
      lg: 'h-14 w-14 text-lg',
    },
  },
  defaultVariants: { size: 'md' },
});

interface AvatarProps extends Omit<ImgHTMLAttributes<HTMLImageElement>, 'src'>, VariantProps<typeof avatarVariants> {
  src?: string;
  name?: string;
}

function getInitials(name: string): string {
  const parts = name.trim().split(/\s+/);
  if (parts.length >= 2) return (parts[0][0] + parts[parts.length - 1][0]).toUpperCase();
  return name.slice(0, 2).toUpperCase();
}

export function Avatar({ className, size, src, name, alt, ...props }: AvatarProps) {
  return (
    <span className={cn(avatarVariants({ size, className }))}>
      {src ? (
        <img
          src={src}
          alt={alt ?? name ?? 'avatar'}
          className="h-full w-full object-cover"
          {...props}
        />
      ) : name ? (
        <span aria-label={name}>{getInitials(name)}</span>
      ) : (
        <svg className="h-1/2 w-1/2 text-primary-400" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
          <path d="M12 12c2.7 0 4.8-2.1 4.8-4.8S14.7 2.4 12 2.4 7.2 4.5 7.2 7.2 9.3 12 12 12zm0 2.4c-3.2 0-9.6 1.6-9.6 4.8v2.4h19.2v-2.4c0-3.2-6.4-4.8-9.6-4.8z" />
        </svg>
      )}
    </span>
  );
}
