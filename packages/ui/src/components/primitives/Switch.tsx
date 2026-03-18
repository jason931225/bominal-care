'use client';
import { type ChangeEvent } from 'react';
import { cn } from '../../lib/utils';

interface SwitchProps {
  id?: string;
  label?: string;
  description?: string;
  checked?: boolean;
  disabled?: boolean;
  onChange?: (checked: boolean) => void;
  className?: string;
}

export function Switch({ id, label, description, checked = false, disabled = false, onChange, className }: SwitchProps) {
  const switchId = id ?? label?.toLowerCase().replace(/\s+/g, '-');

  function handleChange(e: ChangeEvent<HTMLInputElement>) {
    onChange?.(e.target.checked);
  }

  return (
    <div className={cn('flex items-start gap-3', className)}>
      <label htmlFor={switchId} className={cn('relative inline-flex cursor-pointer', disabled && 'cursor-not-allowed')}>
        <input
          type="checkbox"
          role="switch"
          id={switchId}
          checked={checked}
          disabled={disabled}
          onChange={handleChange}
          className="sr-only"
          aria-checked={checked}
        />
        <span
          className={cn(
            'block h-6 w-11 rounded-full transition-colors duration-200',
            checked ? 'bg-primary-600' : 'bg-gray-300',
            disabled && 'opacity-50',
          )}
        >
          <span
            className={cn(
              'absolute top-0.5 left-0.5 h-5 w-5 rounded-full bg-white shadow transition-transform duration-200',
              checked && 'translate-x-5',
            )}
          />
        </span>
      </label>
      {(label || description) && (
        <div className="flex flex-col">
          {label && (
            <label htmlFor={switchId} className={cn('text-sm font-medium text-gray-700 cursor-pointer', disabled && 'cursor-not-allowed opacity-50')}>
              {label}
            </label>
          )}
          {description && <p className="text-sm text-gray-500">{description}</p>}
        </div>
      )}
    </div>
  );
}
