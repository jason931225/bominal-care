'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import ApplicantAppShell from '@/components/ApplicantAppShell';

interface UploadedFile {
  name: string;
  size: string;
  status: 'uploading' | 'done' | 'error';
}

interface CredentialSection {
  id: string;
  label: string;
  required: boolean;
  description: string;
  file: UploadedFile | null;
}

export default function CredentialsPage() {
  const router = useRouter();
  const [credentials, setCredentials] = useState<CredentialSection[]>([
    {
      id: 'caregiverCert',
      label: '요양보호사 자격증',
      required: true,
      description: '요양보호사 국가자격증 사본 (앞면)',
      file: null,
    },
    {
      id: 'idCard',
      label: '신분증 사본',
      required: true,
      description: '주민등록증 또는 운전면허증 앞면',
      file: null,
    },
    {
      id: 'bankBook',
      label: '통장 사본',
      required: true,
      description: '수당 지급용 본인 명의 통장',
      file: null,
    },
    {
      id: 'healthCheck',
      label: '건강진단서',
      required: false,
      description: '3개월 이내 발급 (보건소 또는 병원)',
      file: null,
    },
    {
      id: 'criminalRecord',
      label: '범죄경력조회서',
      required: false,
      description: '경찰청 발급 6개월 이내',
      file: null,
    },
  ]);

  const handleFileSelect = (id: string, fileName: string) => {
    const sizeKb = Math.floor(Math.random() * 800 + 200);
    const sizeLabel = sizeKb > 999 ? `${(sizeKb / 1024).toFixed(1)}MB` : `${sizeKb}KB`;

    setCredentials((prev) =>
      prev.map((c) =>
        c.id === id
          ? { ...c, file: { name: fileName, size: sizeLabel, status: 'uploading' as const } }
          : c
      )
    );

    // Simulate upload completion
    setTimeout(() => {
      setCredentials((prev) =>
        prev.map((c) =>
          c.id === id && c.file
            ? { ...c, file: { ...c.file, status: 'done' as const } }
            : c
        )
      );
    }, 1200);
  };

  const handleRemove = (id: string) => {
    setCredentials((prev) =>
      prev.map((c) => (c.id === id ? { ...c, file: null } : c))
    );
  };

  const requiredDone = credentials
    .filter((c) => c.required)
    .every((c) => c.file?.status === 'done');

  return (
    <ApplicantAppShell currentStep={2} title="자격증 및 서류 업로드">
      <div className="px-4 py-6 space-y-4">
        <div>
          <h2 className="text-lg font-bold text-slate-900 mb-1">서류를 업로드해 주세요</h2>
          <p className="text-sm text-slate-500">JPG, PNG, PDF 형식 · 파일당 최대 10MB</p>
        </div>

        {credentials.map((cred) => (
          <div key={cred.id} className="card">
            <div className="flex items-start justify-between mb-3">
              <div>
                <div className="flex items-center gap-2">
                  <span className="text-sm font-semibold text-slate-800">{cred.label}</span>
                  {cred.required ? (
                    <span className="text-xs font-medium text-red-500 bg-red-50 px-1.5 py-0.5 rounded">필수</span>
                  ) : (
                    <span className="text-xs font-medium text-slate-400 bg-slate-100 px-1.5 py-0.5 rounded">선택</span>
                  )}
                </div>
                <p className="text-xs text-slate-400 mt-0.5">{cred.description}</p>
              </div>
            </div>

            {cred.file ? (
              <div
                className={`flex items-center gap-3 p-3 rounded-xl border ${
                  cred.file.status === 'done'
                    ? 'border-green-200 bg-green-50'
                    : cred.file.status === 'uploading'
                    ? 'border-blue-200 bg-blue-50'
                    : 'border-red-200 bg-red-50'
                }`}
              >
                {cred.file.status === 'uploading' ? (
                  <svg className="w-5 h-5 text-blue-600 animate-spin flex-shrink-0" fill="none" viewBox="0 0 24 24">
                    <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
                    <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
                  </svg>
                ) : cred.file.status === 'done' ? (
                  <svg className="w-5 h-5 text-green-600 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                    <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clipRule="evenodd" />
                  </svg>
                ) : (
                  <svg className="w-5 h-5 text-red-600 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                    <path fillRule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clipRule="evenodd" />
                  </svg>
                )}
                <div className="flex-1 min-w-0">
                  <p className="text-sm font-medium text-slate-800 truncate">{cred.file.name}</p>
                  <p className="text-xs text-slate-500">
                    {cred.file.status === 'uploading' ? '업로드 중...' : cred.file.size}
                  </p>
                </div>
                {cred.file.status === 'done' && (
                  <button
                    type="button"
                    onClick={() => handleRemove(cred.id)}
                    className="p-1.5 rounded-lg active:bg-red-100 transition-colors"
                    aria-label="삭제"
                  >
                    <svg className="w-4 h-4 text-slate-400" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24">
                      <path strokeLinecap="round" strokeLinejoin="round" d="M6 18L18 6M6 6l12 12" />
                    </svg>
                  </button>
                )}
              </div>
            ) : (
              <label className="block cursor-pointer">
                <input
                  type="file"
                  accept=".jpg,.jpeg,.png,.pdf"
                  className="hidden"
                  onChange={(e) => {
                    const file = e.target.files?.[0];
                    if (file) handleFileSelect(cred.id, file.name);
                  }}
                />
                <div className="flex items-center justify-center gap-2 py-4 border-2 border-dashed border-slate-200 rounded-xl bg-slate-50 active:bg-slate-100 transition-colors">
                  <svg className="w-5 h-5 text-slate-400" fill="none" stroke="currentColor" strokeWidth={1.8} viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12" />
                  </svg>
                  <span className="text-sm font-medium text-slate-500">파일 선택 또는 촬영</span>
                </div>
              </label>
            )}
          </div>
        ))}

        <div className="bg-amber-50 rounded-xl p-4 border border-amber-200">
          <p className="text-xs text-amber-700 leading-relaxed">
            <strong>안내:</strong> 업로드된 서류는 검토 후 즉시 삭제됩니다.
            서류 위변조 시 법적 책임이 발생할 수 있습니다.
          </p>
        </div>

        <div className="pb-6">
          <button
            type="button"
            onClick={() => router.push('/apply/service-region')}
            disabled={!requiredDone}
            className="btn-primary"
          >
            다음 단계로
          </button>
        </div>
      </div>
    </ApplicantAppShell>
  );
}
