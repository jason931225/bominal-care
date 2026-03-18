'use client';
import { createContext, useContext, useState, useCallback, type ReactNode } from 'react';

interface ThemeContextValue {
  readonly textScale: number;
  readonly highContrast: boolean;
  readonly reducedMotion: boolean;
  readonly setTextScale: (scale: number) => void;
  readonly toggleHighContrast: () => void;
  readonly toggleReducedMotion: () => void;
}

const ThemeContext = createContext<ThemeContextValue | null>(null);

interface ThemeProviderProps {
  children: ReactNode;
  defaultTextScale?: number;
}

export function ThemeProvider({ children, defaultTextScale = 1 }: ThemeProviderProps) {
  const [textScale, setTextScale] = useState(defaultTextScale);
  const [highContrast, setHighContrast] = useState(false);
  const [reducedMotion, setReducedMotion] = useState(false);

  const toggleHighContrast = useCallback(() => setHighContrast((prev) => !prev), []);
  const toggleReducedMotion = useCallback(() => setReducedMotion((prev) => !prev), []);

  return (
    <ThemeContext.Provider
      value={{ textScale, highContrast, reducedMotion, setTextScale, toggleHighContrast, toggleReducedMotion }}
    >
      <div
        style={{ '--text-scale': textScale } as React.CSSProperties}
        className={highContrast ? 'high-contrast' : ''}
        data-reduced-motion={reducedMotion}
      >
        {children}
      </div>
    </ThemeContext.Provider>
  );
}

export function useTheme(): ThemeContextValue {
  const ctx = useContext(ThemeContext);
  if (!ctx) throw new Error('useTheme must be used within ThemeProvider');
  return ctx;
}
