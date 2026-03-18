'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import Link from 'next/link';
import ApplicantAppShell from '@/components/ApplicantAppShell';

const MOCK_SUMMARY = {
  identity: { name: '김요양', birth: '1985.03.15', phone: '010-1234-5678', idType: '주민등록증' },
  credentials: [
    { label: '요양보호사 자격증', status: '업로드 완료' },
    { label: '신분증 사본', status: '업로드 완료' },
    { label: '통장 사본', status: '업로드 완료' },
  ],
  regions: ['서울 강남구', '서울 서초구', '서울 송파구'],
  schedule: ['월 오전', '월 오후', '화 오전', '수 오전', '수 오후', '목 오전', '금 오전', '금 오후'],
  services: ['목욕 지원', '식사 지원', '이동 지원', '가사 지원', '말벗 서비스'],
  reference: { name: '박복지', relationship: '이전 고용주', phone: '010-9876-5432' },
};

interface SectionCardProps {
  title: string;
  editHref: string;
  children: React.ReactNode;
}

function SectionCard({ title, editHref, children }: SectionCardProps) {
  return (
    <div className="card">
      <div className="flex items-center justify-between mb-3">
        <h3 className="text-sm font-semibold text-slate-800">{title}</h3>
        <Link href={editHref} className="text-xs text-blue-600 font-medium px-2 py-1 bg-blue-50 rounded-lg active:bg-blue-100">
          수정
        </Link>
      </div>
      {children}
    </div>
  );
}

export default function ReviewPage() {
  const router = useRouter();
  const [agreed, setAgreed] = useState({ terms: false, privacy: false, thirdParty: false });
  const [submitting, setSubmitting] = useState(false);

  const allAgreed = agreed.terms && agreed.privacy && agreed.thirdParty;

  const handleSubmit = async () => {
    if (!allAgreed) return;
    setSubmitting(true);
    await new Promise((r) => setTimeout(r, 1500));
    router.push('/apply/status');
  };

  return (
    <ApplicantAppShell currentStep={7} title="최종 검토">
      <div className="px-4 py-6 space-y-4">
        <div>
          <h2 className="text-lg font-bold text-slate-900 mb-1">신청 내용을 확인해 주세요</h2>
          <p className="text-sm text-slate-500">내용이 맞다면 하단에서 동의 후 제출해 주세요.</p>
        </div>

        {/* Identity */}
        <SectionCard title="본인 인증" editHref="/apply/identity">
          <div className="space-y-2">
            <div className="flex justify-between text-sm">
              <span className="text-slate-500">이름</span>
              <span className="font-medium text-slate-800">{MOCK_SUMMARY.identity.name}</span>
            </div>
            <div className="flex justify-between text-sm">
              <span className="text-slate-500">생년월일</span>
              <span className="font-medium text-slate-800">{MOCK_SUMMARY.identity.birth}</span>
            </div>
            <div className="flex justify-between text-sm">
              <span className="text-slate-500">휴대폰</span>
              <span className="font-medium text-slate-800">{MOCK_SUMMARY.identity.phone}</span>
            </div>
            <div className="flex justify-between text-sm">
              <span className="text-slate-500">신분증</span>
              <span className="font-medium text-slate-800">{MOCK_SUMMARY.identity.idType}</span>
            </div>
          </div>
        </SectionCard>

        {/* Credentials */}
        <SectionCard title="업로드 서류" editHref="/apply/credentials">
          <div className="space-y-2">
            {MOCK_SUMMARY.credentials.map((c) => (
              <div key={c.label} className="flex items-center justify-between text-sm">
                <span className="text-slate-600">{c.label}</span>
                <span className="badge-success">{c.status}</span>
              </div>
            ))}
          </div>
        </SectionCard>

        {/* Regions */}
        <SectionCard title="서비스 지역" editHref="/apply/service-region">
          <div className="flex flex-wrap gap-2">
            {MOCK_SUMMARY.regions.map((r) => (
              <span key={r} className="badge-info">{r}</span>
            ))}
          </div>
        </SectionCard>

        {/* Schedule */}
        <SectionCard title="가능 시간" editHref="/apply/schedule">
          <div className="flex flex-wrap gap-2">
            {MOCK_SUMMARY.schedule.map((s) => (
              <span key={s} className="badge-info">{s}</span>
            ))}
          </div>
        </SectionCard>

        {/* Services */}
        <SectionCard title="제공 서비스" editHref="/apply/services">
          <div className="flex flex-wrap gap-2">
            {MOCK_SUMMARY.services.map((s) => (
              <span key={s} className="badge-info">{s}</span>
            ))}
          </div>
        </SectionCard>

        {/* Reference */}
        <SectionCard title="추천인" editHref="/apply/references">
          <div className="space-y-1.5">
            <div className="flex justify-between text-sm">
              <span className="text-slate-500">이름</span>
              <span className="font-medium text-slate-800">{MOCK_SUMMARY.reference.name}</span>
            </div>
            <div className="flex justify-between text-sm">
              <span className="text-slate-500">관계</span>
              <span className="font-medium text-slate-800">{MOCK_SUMMARY.reference.relationship}</span>
            </div>
            <div className="flex justify-between text-sm">
              <span className="text-slate-500">연락처</span>
              <span className="font-medium text-slate-800">{MOCK_SUMMARY.reference.phone}</span>
            </div>
          </div>
        </SectionCard>

        {/* Agreements */}
        <div className="card space-y-3">
          <h3 className="text-sm font-semibold text-slate-800 mb-1">약관 동의</h3>

          {/* All agree */}
          <button
            type="button"
            onClick={() => {
              const all = !allAgreed;
              setAgreed({ terms: all, privacy: all, thirdParty: all });
            }}
            className="flex items-center gap-3 w-full py-2 border-b border-slate-100"
          >
            <div className={`w-5 h-5 rounded border-2 flex items-center justify-center flex-shrink-0 ${
              allAgreed ? 'border-blue-600 bg-blue-600' : 'border-slate-300'
            }`}>
              {allAgreed && (
                <svg className="w-3 h-3 text-white" fill="currentColor" viewBox="0 0 20 20">
                  <path fillRule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clipRule="evenodd" />
                </svg>
              )}
            </div>
            <span className="text-sm font-semibold text-slate-800">전체 동의</span>
          </button>

          {([
            { key: 'terms', label: '이용약관 동의 (필수)' },
            { key: 'privacy', label: '개인정보 수집·이용 동의 (필수)' },
            { key: 'thirdParty', label: '개인정보 제3자 제공 동의 (필수)' },
          ] as const).map((item) => (
            <button
              key={item.key}
              type="button"
              onClick={() => setAgreed((prev) => ({ ...prev, [item.key]: !prev[item.key] }))}
              className="flex items-center gap-3 w-full"
            >
              <div className={`w-5 h-5 rounded border-2 flex items-center justify-center flex-shrink-0 ${
                agreed[item.key] ? 'border-blue-600 bg-blue-600' : 'border-slate-300'
              }`}>
                {agreed[item.key] && (
                  <svg className="w-3 h-3 text-white" fill="currentColor" viewBox="0 0 20 20">
                    <path fillRule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clipRule="evenodd" />
                  </svg>
                )}
              </div>
              <span className="text-sm text-slate-700 text-left">{item.label}</span>
            </button>
          ))}
        </div>

        {/* Submit */}
        <div className="pb-6">
          <button
            type="button"
            onClick={handleSubmit}
            disabled={!allAgreed || submitting}
            className="btn-primary flex items-center justify-center gap-2"
          >
            {submitting ? (
              <>
                <svg className="w-5 h-5 animate-spin" fill="none" viewBox="0 0 24 24">
                  <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
                  <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
                </svg>
                제출 중...
              </>
            ) : '신청서 제출하기'}
          </button>
        </div>
      </div>
    </ApplicantAppShell>
  );
}
