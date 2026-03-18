'use client';

import { ErrorPage } from '@bominal-senior/ui';

export default function GlobalError({
  error,
  reset,
}: {
  error: Error & { digest?: string };
  reset: () => void;
}) {
  return (
    <html lang="ko">
      <body>
        <ErrorPage error={error} reset={reset} />
      </body>
    </html>
  );
}
