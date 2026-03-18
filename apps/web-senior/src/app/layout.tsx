import type { Metadata, Viewport } from 'next';
import { AuthSessionProvider } from '@bominal-senior/auth';
import './globals.css';

export const metadata: Metadata = {
  title: '시니어 포털 | Senior Portal',
  description: '대한민국 시니어를 위한 종합 건강·생활 포털',
  keywords: ['시니어', '노인', '건강', '복지', '케어', '의료'],
  authors: [{ name: '시니어케어' }],
  manifest: '/manifest.json',
};

export const viewport: Viewport = {
  width: 'device-width',
  initialScale: 1,
  maximumScale: 1,
  themeColor: '#4f46e5',
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="ko" className="text-scale-senior">
      <head>
        <link rel="preconnect" href="https://cdn.jsdelivr.net" />
        <link
          rel="stylesheet"
          href="https://cdn.jsdelivr.net/gh/orioncactus/pretendard@v1.3.9/dist/web/variable/pretendardvariable-dynamic-subset.min.css"
        />
      </head>
      <body className="font-sans bg-gray-50 text-gray-900 min-h-screen">
        <AuthSessionProvider>{children}</AuthSessionProvider>
      </body>
    </html>
  );
}
