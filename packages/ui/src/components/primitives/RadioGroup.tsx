'use client';
import { type ChangeEvent } from 'react';
import { cn } from '../../lib/utils';

interface RadioOption {
  value: string;
  label: string;
  disabled?: boolean;
}

interface RadioGroupProps {
  name: string;
  label?: string;
  options: RadioOption[];
  value?: string;
  onChange?: (value: string) => void;
  className?: string;
}

export function RadioGroup({ name, label, options, value, onChange, className }: RadioGroupProps) {
  function handleChange(e: ChangeEvent<HTMLInputElement>) {
    onChange?.(e.target.value);
  }

  return (
    <fieldset className={cn('flex flex-col gap-2', className)}>
      {label && <legend className="text-sm font-medium text-gray-700 mb-1">{label}</legend>}
      {options.map((opt) => (
        <label
          key={opt.value}
          className={cn(
            'flex items-center gap-2 cursor-pointer',
            opt.disabled && 'cursor-not-allowed opacity-50',
          )}
        >
          <input
            type="radio"
            name={name}
            value={opt.value}
            checked={value === opt.value}
            onChange={handleChange}
            disabled={opt.disabled}
            className="h-4 w-4 border-gray-300 text-primary-600 focus:ring-2 focus:ring-primary-500"
          />
          <span className="text-sm text-gray-700">{opt.label}</span>
        </label>
      ))}
    </fieldset>
  );
}
