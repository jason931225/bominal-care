'use client';
import { type ReactNode } from 'react';
import { cn } from '../../lib/utils';

interface ConsentToggleProps {
  id: string;
  label: string;
  explanation?: string | ReactNode;
  checked: boolean;
  onChange: (checked: boolean) => void;
  required?: boolean;
  disabled?: boolean;
  className?: string;
}

export function ConsentToggle({ id, label, explanation, checked, onChange, required, disabled, className }: ConsentToggleProps) {
  return (
    <div className={cn('rounded-xl border border-gray-200 bg-white p-4', className)}>
      <div className="flex items-start justify-between gap-4">
        <div className="flex-1 min-w-0">
          <label htmlFor={id} className="block font-semibold text-gray-900 text-sm cursor-pointer">
            {label}
            {required && <span className="ml-1 text-danger-500" aria-hidden="true">*</span>}
          </label>
          {explanation && (
            <p className="mt-1 text-xs text-gray-500 leading-relaxed">{explanation}</p>
          )}
        </div>
        <button
          type="button"
          role="switch"
          id={id}
          aria-checked={checked}
          disabled={disabled}
          onClick={() => onChange(!checked)}
          className={cn(
            'flex-shrink-0 relative inline-flex h-7 w-12 items-center rounded-full transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary-500',
            checked ? 'bg-primary-600' : 'bg-gray-200',
            disabled && 'opacity-50 cursor-not-allowed',
          )}
        >
          <span
            className={cn(
              'inline-block h-5 w-5 transform rounded-full bg-white shadow transition-transform',
              checked ? 'translate-x-6' : 'translate-x-1',
            )}
          />
        </button>
      </div>
    </div>
  );
}
