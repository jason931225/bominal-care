import type { Metadata } from 'next';
import { AuthSessionProvider } from '@bominal-senior/auth';
import './globals.css';

export const metadata: Metadata = {
  title: '내부 관리 | Internal Portal',
  description: '요양기관 및 서비스 제공자를 위한 포털',
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="ko">
      <head>
        <link rel="preconnect" href="https://cdn.jsdelivr.net" />
        <link
          rel="stylesheet"
          href="https://cdn.jsdelivr.net/gh/orioncactus/pretendard@v1.3.9/dist/web/variable/pretendardvariable-dynamic-subset.min.css"
        />
      </head>
      <body className="font-sans">
        <AuthSessionProvider>{children}</AuthSessionProvider>
      </body>
    </html>
  );
}
