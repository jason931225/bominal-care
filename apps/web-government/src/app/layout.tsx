import type { Metadata } from 'next';
import { AuthSessionProvider } from '@bominal-senior/auth';
import './globals.css';

export const metadata: Metadata = {
  title: '정부 포털 | Government Portal',
  description: '지방자치단체 및 정부 기관을 위한 포털',
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
