'use client';

import InternalAppShell from '@/components/InternalAppShell';
import Link from 'next/link';
import { useState } from 'react';

const REFERRAL_TYPES = ['타 기관 서비스 연계', '시설 입소 의뢰', '병원 이송 의뢰', '전문 서비스 연계', '기타'];

const CLIENTS = ['박순자', '김복동', '이정자', '조길동', '강명순', '윤영희', '홍길자', '장미숙', '이철수', '김영희'];

const KNOWN_ORGS = [
  '강남노인병원', '서울노인요양원', '강남구청 노인복지팀', '역삼동 주민센터', '강남구 치매안심센터',
  '강남세브란스병원', '역삼의원', '강남장기요양센터',
];

export default function NewReferralPage() {
  const [direction, setDirection] = useState<'outgoing' | 'incoming'>('outgoing');

  return (
    <InternalAppShell>
      <div className="space-y-6">
        {/* Breadcrumb */}
        <div className="flex items-center gap-2 text-sm text-gray-500">
          <Link href="/referrals" className="hover:text-blue-600">의뢰 관리</Link>
          <span>/</span>
          <span className="text-gray-900">새 의뢰 생성</span>
        </div>

        <div className="flex items-center justify-between">
          <h1 className="page-title">새 의뢰 생성</h1>
          <Link href="/referrals" className="btn-secondary">취소</Link>
        </div>

        <div className="max-w-2xl space-y-6">
          {/* Direction selector */}
          <div className="card p-5">
            <h2 className="section-title mb-4">의뢰 방향</h2>
            <div className="grid grid-cols-2 gap-3">
              <button
                onClick={() => setDirection('outgoing')}
                className={`p-4 rounded-xl border-2 text-left transition-all ${
                  direction === 'outgoing'
                    ? 'border-blue-500 bg-blue-50'
                    : 'border-gray-200 hover:border-gray-300'
                }`}
              >
                <div className="flex items-center gap-2 mb-1">
                  <svg className="w-5 h-5 text-blue-600" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                    <path strokeLinecap="round" strokeLinejoin="round" d="M17 8l4 4m0 0l-4 4m4-4H3" />
                  </svg>
                  <span className="font-semibold text-gray-900">발신 의뢰</span>
                </div>
                <p className="text-xs text-gray-500">우리 기관 → 타 기관으로 의뢰</p>
              </button>
              <button
                onClick={() => setDirection('incoming')}
                className={`p-4 rounded-xl border-2 text-left transition-all ${
                  direction === 'incoming'
                    ? 'border-green-500 bg-green-50'
                    : 'border-gray-200 hover:border-gray-300'
                }`}
              >
                <div className="flex items-center gap-2 mb-1">
                  <svg className="w-5 h-5 text-green-600" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                    <path strokeLinecap="round" strokeLinejoin="round" d="M7 16l-4-4m0 0l4-4m-4 4h18" />
                  </svg>
                  <span className="font-semibold text-gray-900">수신 의뢰 등록</span>
                </div>
                <p className="text-xs text-gray-500">타 기관 → 우리 기관으로 접수</p>
              </button>
            </div>
          </div>

          {/* Form */}
          <div className="card p-5">
            <h2 className="section-title mb-4">의뢰 정보</h2>
            <div className="space-y-4">
              <div>
                <label className="label">이용자</label>
                <select className="input">
                  <option value="">이용자 선택...</option>
                  {CLIENTS.map(c => <option key={c}>{c}</option>)}
                  <option>신규 이용자 (직접 입력)</option>
                </select>
              </div>

              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="label">
                    {direction === 'outgoing' ? '수신 기관' : '발신 기관'}
                  </label>
                  <input
                    list="orgs"
                    type="text"
                    className="input"
                    placeholder="기관명 입력 또는 선택..."
                  />
                  <datalist id="orgs">
                    {KNOWN_ORGS.map(org => <option key={org} value={org} />)}
                  </datalist>
                </div>
                <div>
                  <label className="label">담당자 연락처</label>
                  <input type="tel" className="input" placeholder="010-0000-0000" />
                </div>
              </div>

              <div>
                <label className="label">의뢰 유형</label>
                <select className="input">
                  <option value="">유형 선택...</option>
                  {REFERRAL_TYPES.map(t => <option key={t}>{t}</option>)}
                </select>
              </div>

              <div>
                <label className="label">의뢰 사유</label>
                <textarea
                  className="input w-full h-24 resize-none"
                  placeholder="의뢰 사유를 상세히 입력하세요..."
                />
              </div>

              <div>
                <label className="label">현재 상태 / 주요 정보</label>
                <textarea
                  className="input w-full h-20 resize-none"
                  placeholder="이용자의 현재 건강 상태 및 전달 필요 사항..."
                />
              </div>

              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="label">희망 시작일</label>
                  <input type="date" className="input" />
                </div>
                <div>
                  <label className="label">긴급도</label>
                  <select className="input">
                    <option>일반</option>
                    <option>우선</option>
                    <option>긴급</option>
                  </select>
                </div>
              </div>

              <div>
                <label className="label">첨부 파일 (선택)</label>
                <div className="border-2 border-dashed border-gray-300 rounded-lg p-4 text-center hover:border-blue-400 transition-colors cursor-pointer">
                  <svg className="w-8 h-8 text-gray-400 mx-auto mb-2" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                    <path strokeLinecap="round" strokeLinejoin="round" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" />
                  </svg>
                  <p className="text-sm text-gray-500">파일을 드래그하거나 클릭하여 업로드</p>
                  <p className="text-xs text-gray-400 mt-1">PDF, JPG, PNG (최대 10MB)</p>
                </div>
              </div>
            </div>
          </div>

          {/* Submit */}
          <div className="flex justify-end gap-3">
            <Link href="/referrals" className="btn-secondary">취소</Link>
            <button className="btn-secondary">임시 저장</button>
            <button className="btn-primary">
              <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                <path strokeLinecap="round" strokeLinejoin="round" d="M17 8l4 4m0 0l-4 4m4-4H3" />
              </svg>
              의뢰 전송
            </button>
          </div>
        </div>
      </div>
    </InternalAppShell>
  );
}
