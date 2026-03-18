export const defaultLocale = 'ko' as const;
export const locales = ['ko', 'en'] as const;
export type Locale = (typeof locales)[number];
