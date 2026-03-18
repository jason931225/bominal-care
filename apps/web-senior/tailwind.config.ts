import type { Config } from 'tailwindcss';

const config: Config = {
  content: [
    './src/pages/**/*.{js,ts,jsx,tsx,mdx}',
    './src/components/**/*.{js,ts,jsx,tsx,mdx}',
    './src/app/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  theme: {
    extend: {
      fontFamily: {
        sans: [
          'Pretendard Variable',
          'Pretendard',
          '-apple-system',
          'BlinkMacSystemFont',
          'system-ui',
          'Roboto',
          'sans-serif',
        ],
      },
      colors: {
        primary: {
          50: '#eef2ff',
          100: '#e0e7ff',
          200: '#c7d2fe',
          300: '#a5b4fc',
          400: '#818cf8',
          500: '#6366f1',
          600: '#4f46e5',
          700: '#4338ca',
          800: '#3730a3',
          900: '#312e81',
          950: '#1e1b4b',
        },
        secondary: {
          50: '#f0fdfa',
          100: '#ccfbf1',
          200: '#99f6e4',
          300: '#5eead4',
          400: '#2dd4bf',
          500: '#14b8a6',
          600: '#0d9488',
          700: '#0f766e',
          800: '#115e59',
          900: '#134e4a',
          950: '#042f2e',
        },
        success: { 50: '#f0fdf4', 500: '#22c55e', 700: '#15803d' },
        warning: { 50: '#fffbeb', 500: '#f59e0b', 700: '#b45309' },
        danger: { 50: '#fef2f2', 500: '#ef4444', 700: '#b91c1c' },
        info: { 50: '#eff6ff', 500: '#3b82f6', 700: '#1d4ed8' },
      },
      fontSize: {
        'senior-sm': ['calc(0.875rem * var(--text-scale, 1))', { lineHeight: '1.5' }],
        'senior-base': ['calc(1rem * var(--text-scale, 1))', { lineHeight: '1.6' }],
        'senior-lg': ['calc(1.125rem * var(--text-scale, 1))', { lineHeight: '1.6' }],
        'senior-xl': ['calc(1.25rem * var(--text-scale, 1))', { lineHeight: '1.5' }],
        'senior-2xl': ['calc(1.5rem * var(--text-scale, 1))', { lineHeight: '1.4' }],
        'senior-3xl': ['calc(1.875rem * var(--text-scale, 1))', { lineHeight: '1.3' }],
      },
      spacing: {
        touch: '44px',
        'touch-senior': '56px',
      },
      minHeight: {
        touch: '44px',
        'touch-senior': '56px',
      },
      minWidth: {
        touch: '44px',
        'touch-senior': '56px',
      },
    },
  },
  plugins: [],
};

export default config;
