// 기회 상세 — Opportunity Detail
// Full details for a job, volunteering, or community opportunity with application

'use client';

import { useState } from 'react';
import Link from 'next/link';
import SeniorAppShell from '@/components/SeniorAppShell';

const MOCK_OPPORTUNITY = {
  id: 'opp-1',
  type: '일자리' as const,
  title: '노인복지관 안내 도우미',
  organization: '서울 노인 복지관',
  organizationPhone: '02-1234-6000',
  location: '서울 중구 을지로 200',
  schedule: '주 3일 (월·수·금) 오전 9시~1시',
  description: '복지관을 방문하시는 어르신들의 안내 및 프로그램 보조 업무를 담당합니다. 시니어가 시니어를 돕는 따뜻한 일자리입니다.',
  pay: '시간당 12,000원',
  monthlyEstimate: '약 576,000원/월 (주 12시간)',
  deadline: '2026년 3월 31일',
  startDate: '2026년 4월 7일',
  ageLimit: '60세 이상',
  spotsTotal: 3,
  spotsRemaining: 3,
  requirements: [
    '만 60세 이상',
    '기초적인 스마트폰 사용 가능',
    '성실하고 친화력 있는 분',
    '거동이 불편하지 않은 분',
  ],
  benefits: [
    '4대 보험 가입',
    '교통비 별도 지원 (월 5만원)',
    '식사 제공 (근무일 점심)',
    '연 1회 건강검진 지원',
  ],
  workContent: [
    '방문객 안내 및 프로그램 등록 보조',
    '공지사항 게시 및 자료 배부',
    '회의실·프로그램실 준비 지원',
    '간단한 전화 응대',
  ],
};

export default function OpportunityDetailPage() {
  const [applied, setApplied] = useState(false);
  const [name, setName] = useState('');
  const [phone, setPhone] = useState('');
  const [showForm, setShowForm] = useState(false);

  const opp = MOCK_OPPORTUNITY;

  function handleApply(e: React.FormEvent) {
    e.preventDefault();
    if (name && phone) {
      setApplied(true);
    }
  }

  return (
    <SeniorAppShell>
      <div className="page-content">
        {/* Back */}
        <Link
          href="/opportunities"
          className="inline-flex items-center gap-2 text-primary-600 font-medium text-senior-base mb-5 min-h-touch"
        >
          <svg className="w-5 h-5" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24" aria-hidden="true">
            <path strokeLinecap="round" strokeLinejoin="round" d="M15 19l-7-7 7-7" />
          </svg>
          목록으로
        </Link>

        {/* Header */}
        <div className="senior-card mb-4">
          <span className="bg-primary-100 text-primary-700 text-senior-sm font-bold px-2.5 py-1 rounded-full">
            💼 {opp.type}
          </span>
          <h1 className="text-senior-2xl font-bold text-gray-900 mt-2 mb-1">{opp.title}</h1>
          <p className="text-senior-base text-gray-600 mb-3">{opp.organization} · {opp.location}</p>
          <p className="text-senior-base text-gray-700 leading-relaxed">{opp.description}</p>
        </div>

        {/* Key info */}
        <section className="senior-card mb-4" aria-labelledby="key-info-heading">
          <h2 id="key-info-heading" className="text-senior-lg font-bold text-gray-800 mb-3">핵심 정보</h2>
          <div className="space-y-2">
            {[
              { icon: '🕐', label: '일정', value: opp.schedule },
              { icon: '💰', label: '급여', value: `${opp.pay} (${opp.monthlyEstimate})` },
              { icon: '📅', label: '시작일', value: opp.startDate },
              { icon: '⏰', label: '모집 마감', value: opp.deadline },
              { icon: '👤', label: '연령', value: opp.ageLimit },
              { icon: '👥', label: '모집 인원', value: `${opp.spotsTotal}명 (잔여: ${opp.spotsRemaining}명)` },
            ].map(({ icon, label, value }) => (
              <div key={label} className="flex items-start gap-3">
                <span className="w-6 text-center flex-shrink-0" aria-hidden="true">{icon}</span>
                <div className="flex-1 flex justify-between">
                  <span className="text-senior-sm text-gray-500 w-20 flex-shrink-0">{label}</span>
                  <span className="text-senior-base font-semibold text-gray-800 text-right">{value}</span>
                </div>
              </div>
            ))}
          </div>
        </section>

        {/* Requirements */}
        <section className="senior-card mb-4" aria-labelledby="requirements-heading">
          <h2 id="requirements-heading" className="text-senior-lg font-bold text-gray-800 mb-2">지원 자격</h2>
          <ul className="space-y-1">
            {opp.requirements.map((r, i) => (
              <li key={i} className="flex items-start gap-2 text-senior-base text-gray-700">
                <span className="text-primary-500 flex-shrink-0 mt-0.5" aria-hidden="true">•</span>
                {r}
              </li>
            ))}
          </ul>
        </section>

        {/* Work content */}
        <section className="senior-card mb-4" aria-labelledby="work-content-heading">
          <h2 id="work-content-heading" className="text-senior-lg font-bold text-gray-800 mb-2">주요 업무</h2>
          <ul className="space-y-1">
            {opp.workContent.map((w, i) => (
              <li key={i} className="flex items-start gap-2 text-senior-base text-gray-700">
                <span className="text-secondary-500 flex-shrink-0 mt-0.5" aria-hidden="true">✓</span>
                {w}
              </li>
            ))}
          </ul>
        </section>

        {/* Benefits */}
        <section className="senior-card mb-5" aria-labelledby="benefits-heading">
          <h2 id="benefits-heading" className="text-senior-lg font-bold text-gray-800 mb-2">복리후생</h2>
          <ul className="space-y-1">
            {opp.benefits.map((b, i) => (
              <li key={i} className="flex items-start gap-2 text-senior-base text-gray-700">
                <span className="text-success-500 flex-shrink-0 mt-0.5" aria-hidden="true">✓</span>
                {b}
              </li>
            ))}
          </ul>
        </section>

        {/* Application section */}
        {applied ? (
          <div className="senior-card text-center py-4 mb-4">
            <p className="text-3xl mb-2" aria-hidden="true">✅</p>
            <p className="text-senior-xl font-bold text-success-700">지원 완료!</p>
            <p className="text-senior-base text-gray-600 mt-1">담당자가 3일 내 연락드립니다.</p>
          </div>
        ) : showForm ? (
          <div className="senior-card mb-4">
            <h2 className="text-senior-lg font-bold text-gray-800 mb-4">지원서 작성</h2>
            <form onSubmit={handleApply} className="space-y-4">
              <div>
                <label className="block text-senior-base font-bold text-gray-700 mb-1">이름 <span className="text-danger-500">*</span></label>
                <input
                  type="text"
                  value={name}
                  onChange={(e) => setName(e.target.value)}
                  placeholder="본인 이름"
                  required
                  className="w-full border-2 border-gray-300 rounded-xl px-4 py-3 text-senior-lg focus:border-primary-500 focus:outline-none min-h-touch-senior"
                />
              </div>
              <div>
                <label className="block text-senior-base font-bold text-gray-700 mb-1">연락처 <span className="text-danger-500">*</span></label>
                <input
                  type="tel"
                  value={phone}
                  onChange={(e) => setPhone(e.target.value)}
                  placeholder="010-0000-0000"
                  required
                  className="w-full border-2 border-gray-300 rounded-xl px-4 py-3 text-senior-lg focus:border-primary-500 focus:outline-none min-h-touch-senior"
                />
              </div>
              <button type="submit" className="senior-btn-primary w-full">지원하기</button>
              <button type="button" onClick={() => setShowForm(false)} className="w-full text-center text-senior-base text-gray-500 py-2 min-h-touch">취소</button>
            </form>
          </div>
        ) : (
          <div className="space-y-3">
            <button onClick={() => setShowForm(true)} className="senior-btn-primary w-full">
              지원하기
            </button>
            <a href={`tel:${opp.organizationPhone}`} className="senior-btn-secondary w-full">
              📞 담당자에게 전화하기
            </a>
          </div>
        )}
      </div>
    </SeniorAppShell>
  );
}
