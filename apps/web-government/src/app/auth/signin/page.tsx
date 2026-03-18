'use client';

import { useState } from 'react';
import { signIn } from 'next-auth/react';

const APP_NAME = '정부 관리 포털';

const DEMO_USERS = [
  {
    name: '정민호',
    email: 'minho.jung@gov.kr',
    role: 'GOVERNMENT_REVIEWER',
    roleLabel: '정부 검토자',
    description: '서울특별시 강남구 노인복지과',
  },
] as const;

const THEME = {
  gradient: 'from-slate-700 to-slate-800',
  badge: 'bg-indigo-100 text-indigo-700',
  cardHover: 'hover:border-indigo-300 hover:shadow-indigo-100',
  button: 'bg-indigo-600',
  spinner: 'border-indigo-600',
  hoverButton: 'hover:bg-indigo-700',
} as const;

export default function SignInPage() {
  const [signingInEmail, setSigningInEmail] = useState<string | null>(null);

  function handleSignIn(email: string) {
    setSigningInEmail(email);
    signIn('demo-login', { email, callbackUrl: '/' });
  }

  return (
    <div className="min-h-screen bg-gray-50 flex flex-col items-center justify-center px-4 py-12">
      <div className="w-full max-w-md">
        {/* Header */}
        <div className={`bg-gradient-to-br ${THEME.gradient} rounded-2xl p-6 text-white text-center mb-8`}>
          <h1 className="text-2xl font-bold">{APP_NAME}</h1>
          <p className="text-slate-300 mt-1 text-sm">데모 로그인</p>
        </div>

        {/* Description */}
        <p className="text-center text-gray-500 text-sm mb-6">
          아래 계정을 선택하여 로그인하세요.
        </p>

        {/* Demo user cards */}
        <div className="space-y-3">
          {DEMO_USERS.map((user) => {
            const isLoading = signingInEmail === user.email;
            return (
              <button
                key={user.email}
                type="button"
                disabled={signingInEmail !== null}
                onClick={() => handleSignIn(user.email)}
                className={`w-full bg-white border-2 border-gray-200 rounded-2xl p-5 text-left transition-all ${THEME.cardHover} disabled:opacity-60 disabled:cursor-not-allowed`}
              >
                <div className="flex items-center gap-4">
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2 mb-1">
                      <span className="text-lg font-bold text-gray-900">{user.name}</span>
                      <span className={`text-xs font-semibold px-2 py-0.5 rounded-full ${THEME.badge}`}>
                        {user.roleLabel}
                      </span>
                    </div>
                    <p className="text-sm text-gray-500 truncate">{user.email}</p>
                    <p className="text-xs text-gray-400 mt-1">{user.description}</p>
                  </div>
                  {isLoading ? (
                    <div className={`w-6 h-6 border-2 ${THEME.spinner} border-t-transparent rounded-full animate-spin`} />
                  ) : (
                    <svg className="w-5 h-5 text-gray-400" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24">
                      <path strokeLinecap="round" strokeLinejoin="round" d="M9 5l7 7-7 7" />
                    </svg>
                  )}
                </div>
              </button>
            );
          })}
        </div>
      </div>
    </div>
  );
}
