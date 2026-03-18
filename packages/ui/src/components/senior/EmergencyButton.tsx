'use client';
import { type ButtonHTMLAttributes } from 'react';
import { cn } from '../../lib/utils';

interface EmergencyButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  label?: string;
  fixed?: boolean;
}

export function EmergencyButton({ label = '긴급 호출', fixed = false, className, onClick, ...props }: EmergencyButtonProps) {
  return (
    <button
      type="button"
      onClick={onClick}
      aria-label={label}
      className={cn(
        'flex flex-col items-center justify-center gap-2 rounded-full bg-danger-500 text-white font-bold shadow-lg',
        'hover:bg-danger-700 active:scale-95 transition-all',
        'focus-visible:outline-none focus-visible:ring-4 focus-visible:ring-danger-300',
        'min-h-[80px] min-w-[80px] text-sm',
        fixed && 'fixed bottom-6 right-6 z-50 h-20 w-20',
        !fixed && 'w-full h-20',
        className,
      )}
      {...props}
    >
      <span aria-hidden="true" className="text-2xl">🆘</span>
      <span>{label}</span>
    </button>
  );
}
