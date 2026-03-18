'use client';

import { useSearchParams } from 'next/navigation';
import Link from 'next/link';

const ERROR_MESSAGES: Record<string, string> = {
  Configuration: '서버 설정에 문제가 있습니다.',
  AccessDenied: '접근 권한이 없습니다.',
  Verification: '인증 링크가 만료되었거나 이미 사용되었습니다.',
  Default: '로그인 중 오류가 발생했습니다.',
  CredentialsSignin: '로그인 정보가 올바르지 않습니다.',
};

export default function AuthErrorPage() {
  const searchParams = useSearchParams();
  const errorType = searchParams.get('error') ?? 'Default';
  const message = ERROR_MESSAGES[errorType] ?? ERROR_MESSAGES.Default;

  return (
    <div className="min-h-screen bg-gray-50 flex flex-col items-center justify-center px-4 py-12">
      <div className="w-full max-w-md text-center">
        <div className="bg-white border-2 border-red-200 rounded-2xl p-8">
          <div className="w-14 h-14 bg-red-100 rounded-full flex items-center justify-center mx-auto mb-4">
            <svg className="w-7 h-7 text-red-600" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" d="M12 9v3.75m9-.75a9 9 0 11-18 0 9 9 0 0118 0zm-9 3.75h.008v.008H12v-.008z" />
            </svg>
          </div>
          <h1 className="text-xl font-bold text-gray-900 mb-2">로그인 오류</h1>
          <p className="text-gray-500 text-sm mb-6">{message}</p>
          {errorType !== 'Default' && (
            <p className="text-xs text-gray-400 mb-4">오류 코드: {errorType}</p>
          )}
          <Link
            href="/auth/signin"
            className="inline-block bg-indigo-600 text-white font-semibold text-sm px-6 py-3 rounded-xl hover:bg-indigo-700 transition-colors"
          >
            로그인 페이지로 돌아가기
          </Link>
        </div>
      </div>
    </div>
  );
}
