'use client';

import Link from 'next/link';
import { useState } from 'react';
import FamilyAppShell from '@/components/FamilyAppShell';

const SERVICE_TYPES = [
  { id: 'visit-care', label: '방문 요양', icon: '🏥', desc: '일상 생활 보조' },
  { id: 'hospital', label: '병원 동행', icon: '🚗', desc: '병원 이동 및 동행' },
  { id: 'pt', label: '물리치료', icon: '🦵', desc: '재활 운동 치료' },
  { id: 'bath', label: '목욕 서비스', icon: '🛁', desc: '방문 목욕 보조' },
  { id: 'meal', label: '식사 배달', icon: '🍱', desc: '도시락 배달 서비스' },
  { id: 'cognitive', label: '인지 프로그램', icon: '🧠', desc: '인지 재활 프로그램' },
];

const PROVIDERS_BY_TYPE: Record<string, { id: string; name: string; org: string }[]> = {
  'visit-care': [
    { id: 'p1', name: '박미영 요양보호사', org: '행복케어 복지센터' },
    { id: 'p2', name: '이순자 요양보호사', org: '행복케어 복지센터' },
  ],
  hospital: [
    { id: 'p3', name: '김선화 사회복지사', org: '나눔케어 서비스' },
  ],
  pt: [
    { id: 'p4', name: '김도현 물리치료사', org: '하이케어 의원' },
  ],
};

export default function BookForSeniorPage() {
  const [step, setStep] = useState(1);
  const [serviceType, setServiceType] = useState('');
  const [providerId, setProviderId] = useState('');
  const [date, setDate] = useState('');
  const [time, setTime] = useState('');
  const [note, setNote] = useState('');
  const [submitted, setSubmitted] = useState(false);

  const providers = PROVIDERS_BY_TYPE[serviceType] ?? [];
  const selectedServiceLabel = SERVICE_TYPES.find((s) => s.id === serviceType)?.label ?? '';
  const selectedProvider = providers.find((p) => p.id === providerId);

  const handleSubmit = () => {
    setSubmitted(true);
  };

  if (submitted) {
    return (
      <FamilyAppShell>
        <div className="max-w-lg mx-auto px-4 py-16 text-center">
          <span className="text-6xl block mb-4">✅</span>
          <h2 className="text-2xl font-bold text-gray-900 mb-2">예약 완료</h2>
          <p className="text-gray-500 mb-2">김복순 어머님을 대신하여 예약이 완료되었습니다</p>
          <div className="bg-gray-50 rounded-xl p-4 text-left mb-6 text-sm space-y-2">
            <p><span className="text-gray-500">서비스:</span> <span className="font-medium">{selectedServiceLabel}</span></p>
            <p><span className="text-gray-500">제공자:</span> <span className="font-medium">{selectedProvider?.name}</span></p>
            <p><span className="text-gray-500">일시:</span> <span className="font-medium">{date} {time}</span></p>
          </div>
          <Link
            href="/help-senior"
            className="inline-flex items-center gap-2 px-5 py-2.5 bg-blue-600 text-white rounded-lg font-medium hover:bg-blue-700 transition-colors"
          >
            대리 서비스 홈으로
          </Link>
        </div>
      </FamilyAppShell>
    );
  }

  return (
    <FamilyAppShell>
      <div className="max-w-2xl mx-auto px-4 py-6">
        {/* Breadcrumb */}
        <nav className="flex items-center gap-2 text-sm text-gray-500 mb-4">
          <Link href="/help-senior" className="hover:text-blue-600">대리 서비스</Link>
          <span>/</span>
          <span className="text-gray-900 font-medium">서비스 예약</span>
        </nav>

        <h1 className="text-2xl font-bold text-gray-900 mb-2">서비스 예약 대리</h1>
        <p className="text-sm text-gray-500 mb-6">김복순 어머님을 대신하여 예약합니다</p>

        {/* Step Indicator */}
        <div className="flex items-center gap-2 mb-6">
          {[1, 2, 3].map((s) => (
            <div key={s} className="flex items-center gap-2">
              <div className={`w-7 h-7 rounded-full flex items-center justify-center text-xs font-bold ${
                step >= s ? 'bg-blue-600 text-white' : 'bg-gray-200 text-gray-500'
              }`}>
                {s}
              </div>
              <span className={`text-xs ${step >= s ? 'text-blue-600 font-medium' : 'text-gray-400'}`}>
                {s === 1 ? '서비스 선택' : s === 2 ? '일정 선택' : '확인'}
              </span>
              {s < 3 && <div className={`flex-1 h-0.5 w-8 ${step > s ? 'bg-blue-400' : 'bg-gray-200'}`} />}
            </div>
          ))}
        </div>

        {/* Step 1: Service Type */}
        {step === 1 && (
          <div>
            <h2 className="font-bold text-gray-900 mb-4">1단계: 서비스 유형 선택</h2>
            <div className="grid sm:grid-cols-2 gap-3 mb-6">
              {SERVICE_TYPES.map((svc) => (
                <button
                  key={svc.id}
                  onClick={() => setServiceType(svc.id)}
                  className={`p-4 rounded-xl border-2 text-left transition-colors ${
                    serviceType === svc.id
                      ? 'border-blue-500 bg-blue-50'
                      : 'border-gray-200 bg-white hover:border-blue-300'
                  }`}
                >
                  <span className="text-2xl block mb-1">{svc.icon}</span>
                  <p className="font-semibold text-gray-900 text-sm">{svc.label}</p>
                  <p className="text-xs text-gray-500 mt-0.5">{svc.desc}</p>
                </button>
              ))}
            </div>
            <button
              onClick={() => setStep(2)}
              disabled={!serviceType}
              className="w-full py-3 bg-blue-600 text-white font-semibold rounded-xl hover:bg-blue-700 transition-colors disabled:bg-gray-200 disabled:text-gray-400 disabled:cursor-not-allowed"
            >
              다음 단계
            </button>
          </div>
        )}

        {/* Step 2: Date/Time + Provider */}
        {step === 2 && (
          <div>
            <h2 className="font-bold text-gray-900 mb-4">2단계: 일정 및 제공자 선택</h2>

            <div className="space-y-4 mb-6">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1.5">제공자 선택</label>
                <div className="space-y-2">
                  {providers.length > 0 ? providers.map((p) => (
                    <button
                      key={p.id}
                      onClick={() => setProviderId(p.id)}
                      className={`w-full flex items-center gap-3 p-3 rounded-xl border-2 text-left transition-colors ${
                        providerId === p.id ? 'border-blue-500 bg-blue-50' : 'border-gray-200 bg-white hover:border-blue-300'
                      }`}
                    >
                      <span className="text-2xl">👤</span>
                      <div>
                        <p className="font-medium text-gray-900 text-sm">{p.name}</p>
                        <p className="text-xs text-gray-500">{p.org}</p>
                      </div>
                      {providerId === p.id && <span className="ml-auto text-blue-600">✓</span>}
                    </button>
                  )) : (
                    <p className="text-sm text-gray-500 p-3 bg-gray-50 rounded-xl">
                      연결된 제공자가 없습니다. 매칭 요청을 먼저 완료해 주세요.
                    </p>
                  )}
                </div>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1.5">날짜</label>
                <input
                  type="date"
                  value={date}
                  onChange={(e) => setDate(e.target.value)}
                  min="2026-03-15"
                  className="w-full px-3 py-2.5 border border-gray-300 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1.5">시간</label>
                <select
                  value={time}
                  onChange={(e) => setTime(e.target.value)}
                  className="w-full px-3 py-2.5 border border-gray-300 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white"
                >
                  <option value="">시간 선택</option>
                  {['오전 8:00', '오전 9:00', '오전 10:00', '오전 11:00', '오후 1:00', '오후 2:00', '오후 3:00', '오후 4:00'].map((t) => (
                    <option key={t} value={t}>{t}</option>
                  ))}
                </select>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1.5">특이사항 (선택)</label>
                <textarea
                  value={note}
                  onChange={(e) => setNote(e.target.value)}
                  rows={2}
                  placeholder="전달할 사항을 입력하세요..."
                  className="w-full px-3 py-2 border border-gray-300 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 resize-none"
                />
              </div>
            </div>

            <div className="flex gap-3">
              <button onClick={() => setStep(1)} className="flex-1 py-3 border border-gray-300 text-gray-700 font-semibold rounded-xl hover:bg-gray-50 transition-colors">
                이전
              </button>
              <button
                onClick={() => setStep(3)}
                disabled={!providerId || !date || !time}
                className="flex-1 py-3 bg-blue-600 text-white font-semibold rounded-xl hover:bg-blue-700 transition-colors disabled:bg-gray-200 disabled:text-gray-400 disabled:cursor-not-allowed"
              >
                다음 단계
              </button>
            </div>
          </div>
        )}

        {/* Step 3: Confirm */}
        {step === 3 && (
          <div>
            <h2 className="font-bold text-gray-900 mb-4">3단계: 예약 확인</h2>

            <div className="bg-blue-50 border border-blue-200 rounded-xl p-5 mb-6">
              <h3 className="font-bold text-blue-900 mb-4">예약 요약</h3>
              <div className="space-y-3 text-sm">
                <div className="flex justify-between">
                  <span className="text-blue-600">대상자</span>
                  <span className="font-semibold text-blue-900">김복순 어머님</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-blue-600">서비스</span>
                  <span className="font-semibold text-blue-900">{selectedServiceLabel}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-blue-600">제공자</span>
                  <span className="font-semibold text-blue-900">{selectedProvider?.name}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-blue-600">일시</span>
                  <span className="font-semibold text-blue-900">{date} {time}</span>
                </div>
                {note && (
                  <div className="pt-2 border-t border-blue-200">
                    <span className="text-blue-600 block mb-1">특이사항</span>
                    <span className="text-blue-900">{note}</span>
                  </div>
                )}
              </div>
            </div>

            <div className="bg-amber-50 border border-amber-200 rounded-xl p-3 mb-5">
              <p className="text-xs text-amber-700">
                대리 예약임을 확인합니다. 이 예약은 김복순 어머님과 제공자에게 알림이 전송됩니다.
              </p>
            </div>

            <div className="flex gap-3">
              <button onClick={() => setStep(2)} className="flex-1 py-3 border border-gray-300 text-gray-700 font-semibold rounded-xl hover:bg-gray-50 transition-colors">
                수정
              </button>
              <button onClick={handleSubmit} className="flex-1 py-3 bg-blue-600 text-white font-semibold rounded-xl hover:bg-blue-700 transition-colors">
                예약 완료
              </button>
            </div>
          </div>
        )}
      </div>
    </FamilyAppShell>
  );
}
