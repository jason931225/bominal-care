import type { Metadata } from 'next';
import { AuthSessionProvider } from '@bominal-senior/auth';
import './globals.css';

export const metadata: Metadata = {
  title: '가족 포털 | Family Portal',
  description: '가족 및 보호자를 위한 시니어 케어 포털',
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="ko">
      <head>
        {/* Pretendard Variable font — best Korean web font for readability */}
        <link rel="preconnect" href="https://cdn.jsdelivr.net" />
        <link
          rel="stylesheet"
          href="https://cdn.jsdelivr.net/gh/orioncactus/pretendard@v1.3.9/dist/web/variable/pretendardvariable-dynamic-subset.min.css"
        />
      </head>
      <body className="font-sans bg-gray-50 text-gray-900 antialiased">
        <AuthSessionProvider>{children}</AuthSessionProvider>
      </body>
    </html>
  );
}
