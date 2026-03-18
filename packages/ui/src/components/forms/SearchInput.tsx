'use client';
import { forwardRef, useRef, useCallback, type InputHTMLAttributes } from 'react';
import { cn } from '../../lib/utils';

interface SearchInputProps extends Omit<InputHTMLAttributes<HTMLInputElement>, 'onChange' | 'type'> {
  onSearch: (value: string) => void;
  debounceMs?: number;
}

export const SearchInput = forwardRef<HTMLInputElement, SearchInputProps>(
  ({ onSearch, debounceMs = 300, className, placeholder = '검색...', ...props }, ref) => {
    const timerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
    const composingRef = useRef(false);

    const scheduleSearch = useCallback(
      (value: string) => {
        if (timerRef.current) clearTimeout(timerRef.current);
        timerRef.current = setTimeout(() => onSearch(value), debounceMs);
      },
      [onSearch, debounceMs],
    );

    return (
      <div className="relative flex items-center">
        <span className="absolute left-3 text-gray-400 pointer-events-none" aria-hidden="true">
          <svg className="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-4.35-4.35M17 11A6 6 0 1 1 5 11a6 6 0 0 1 12 0z" />
          </svg>
        </span>
        <input
          ref={ref}
          type="search"
          placeholder={placeholder}
          className={cn(
            'flex w-full h-10 rounded-md border border-gray-300 bg-white pl-9 pr-3 text-sm text-gray-900',
            'placeholder:text-gray-400 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-transparent',
            'disabled:cursor-not-allowed disabled:opacity-50',
            className,
          )}
          onChange={(e) => {
            if (!composingRef.current) scheduleSearch(e.target.value);
          }}
          onCompositionStart={() => { composingRef.current = true; }}
          onCompositionEnd={(e) => {
            composingRef.current = false;
            scheduleSearch((e.target as HTMLInputElement).value);
          }}
          {...props}
        />
      </div>
    );
  },
);

SearchInput.displayName = 'SearchInput';
