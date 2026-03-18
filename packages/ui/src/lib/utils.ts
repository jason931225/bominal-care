import { clsx, type ClassValue } from 'clsx';
import { twMerge } from 'tailwind-merge';

export function cn(...inputs: ClassValue[]): string {
  return twMerge(clsx(inputs));
}

export function formatDate(date: Date | string, locale: string = 'ko'): string {
  const d = typeof date === 'string' ? new Date(date) : date;
  if (locale === 'ko') {
    const year = d.getFullYear();
    const month = String(d.getMonth() + 1).padStart(2, '0');
    const day = String(d.getDate()).padStart(2, '0');
    return `${year}년 ${month}월 ${day}일`;
  }
  return d.toLocaleDateString(locale);
}

export function formatCurrency(amount: number, currency: string = 'KRW'): string {
  if (currency === 'KRW') {
    return new Intl.NumberFormat('ko-KR', { style: 'currency', currency: 'KRW' }).format(amount);
  }
  return new Intl.NumberFormat('ko-KR', { style: 'currency', currency }).format(amount);
}

export function formatTime(date: Date | string, locale: string = 'ko'): string {
  const d = typeof date === 'string' ? new Date(date) : date;
  if (locale === 'ko') {
    return d.toLocaleTimeString('ko-KR', { hour: 'numeric', minute: '2-digit', hour12: true });
  }
  return d.toLocaleTimeString(locale, { hour: 'numeric', minute: '2-digit', hour12: true });
}

export function formatPhoneNumber(phone: string): string {
  const digits = phone.replace(/\D/g, '');
  if (digits.length === 11 && digits.startsWith('010')) {
    return `${digits.slice(0, 3)}-${digits.slice(3, 7)}-${digits.slice(7)}`;
  }
  if (digits.length === 10) {
    return `${digits.slice(0, 3)}-${digits.slice(3, 6)}-${digits.slice(6)}`;
  }
  return phone;
}
