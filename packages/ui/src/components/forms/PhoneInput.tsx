'use client';
import { forwardRef, type InputHTMLAttributes, type ChangeEvent } from 'react';
import { cn } from '../../lib/utils';
import { formatPhoneNumber } from '../../lib/utils';

interface PhoneInputProps extends Omit<InputHTMLAttributes<HTMLInputElement>, 'type' | 'onChange'> {
  label?: string;
  error?: string;
  helperText?: string;
  onChange?: (value: string) => void;
}

export const PhoneInput = forwardRef<HTMLInputElement, PhoneInputProps>(
  ({ className, label, error, helperText, id, onChange, value, ...props }, ref) => {
    const inputId = id ?? 'phone-input';

    function handleChange(e: ChangeEvent<HTMLInputElement>) {
      const raw = e.target.value.replace(/\D/g, '').slice(0, 11);
      onChange?.(raw);
    }

    const displayValue = typeof value === 'string' ? formatPhoneNumber(value) : '';

    return (
      <div className="flex flex-col gap-1">
        {label && (
          <label htmlFor={inputId} className="text-sm font-medium text-gray-700">
            {label}
          </label>
        )}
        <input
          ref={ref}
          id={inputId}
          type="tel"
          inputMode="numeric"
          placeholder="010-0000-0000"
          value={displayValue}
          onChange={handleChange}
          className={cn(
            'flex h-10 w-full rounded-md border border-gray-300 bg-white px-3 text-sm text-gray-900',
            'placeholder:text-gray-400 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-transparent',
            'disabled:cursor-not-allowed disabled:opacity-50',
            error && 'border-danger-500 focus:ring-danger-500',
            className,
          )}
          aria-invalid={!!error}
          {...props}
        />
        {error && <p className="text-sm text-danger-500">{error}</p>}
        {!error && helperText && <p className="text-sm text-gray-500">{helperText}</p>}
      </div>
    );
  },
);

PhoneInput.displayName = 'PhoneInput';
