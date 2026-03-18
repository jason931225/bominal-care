'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import Link from 'next/link';
import CaregiverAppShell from '@/components/CaregiverAppShell';

const MOCK_VISIT = {
  id: 'v001',
  clientName: '이순자',
  checkinTime: '10:03',
  scheduledEnd: '13:00',
  services: ['목욕 지원', '식사 지원', '배변 지원'],
  careItems: [
    { id: 'c1', label: '혈압약 복용 확인', required: true },
    { id: 'c2', label: '입욕 보조 (30분)', required: true },
    { id: 'c3', label: '점심 식사 준비 및 보조', required: true },
    { id: 'c4', label: '낙상 위험 평가 기록', required: false },
    { id: 'c5', label: '주변 환경 정리', required: false },
  ],
};

type VitalKey = 'bloodPressureSys' | 'bloodPressureDia' | 'pulse' | 'temperature' | 'weight';

export default function CheckoutPage({ params: _params }: { params: Promise<{ visitId: string }> }) {
  const router = useRouter();
  const [checkoutTime] = useState(() => {
    const now = new Date();
    return `${now.getHours().toString().padStart(2, '0')}:${now.getMinutes().toString().padStart(2, '0')}`;
  });
  const [completedItems, setCompletedItems] = useState<Set<string>>(new Set());
  const [mood, setMood] = useState<number | null>(null);
  const [vitals, setVitals] = useState<Record<VitalKey, string>>({
    bloodPressureSys: '',
    bloodPressureDia: '',
    pulse: '',
    temperature: '',
    weight: '',
  });
  const [careNote, setCareNote] = useState('');
  const [specialNote, setSpecialNote] = useState('');
  const [submitting, setSubmitting] = useState(false);

  const toggleItem = (id: string) => {
    setCompletedItems((prev) => {
      const next = new Set(prev);
      if (next.has(id)) {
        next.delete(id);
      } else {
        next.add(id);
      }
      return next;
    });
  };

  const requiredDone = MOCK_VISIT.careItems
    .filter((i) => i.required)
    .every((i) => completedItems.has(i.id));

  const handleCheckout = async () => {
    if (!requiredDone) return;
    setSubmitting(true);
    await new Promise((r) => setTimeout(r, 1500));
    router.push('/schedule');
  };

  const checkinHour = parseInt(MOCK_VISIT.checkinTime.split(':')[0]);
  const checkinMin = parseInt(MOCK_VISIT.checkinTime.split(':')[1]);
  const checkoutHour = parseInt(checkoutTime.split(':')[0]);
  const checkoutMin = parseInt(checkoutTime.split(':')[1]);
  const durationMin = (checkoutHour * 60 + checkoutMin) - (checkinHour * 60 + checkinMin);
  const durationLabel = `${Math.floor(durationMin / 60)}시간 ${durationMin % 60}분`;

  const MOODS = ['😢', '😕', '😐', '🙂', '😊'];

  return (
    <CaregiverAppShell
      activeTab="schedule"
      title="체크아웃"
      showBackButton
      backHref={`/schedule/${MOCK_VISIT.id}`}
    >
      <div className="px-4 py-6 space-y-5">
        {/* Summary */}
        <div className="card bg-indigo-50 border-indigo-200">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-xs text-indigo-500 font-medium">방문 완료</p>
              <p className="text-base font-bold text-indigo-800 mt-0.5">{MOCK_VISIT.clientName} 어르신</p>
            </div>
            <div className="text-right">
              <p className="text-2xl font-bold text-indigo-700">{durationLabel}</p>
              <p className="text-xs text-indigo-500">{MOCK_VISIT.checkinTime} – {checkoutTime}</p>
            </div>
          </div>
        </div>

        {/* Care Checklist */}
        <div className="card">
          <div className="flex items-center justify-between mb-3">
            <h3 className="section-title mb-0">케어 수행 확인 *</h3>
            <span className="text-xs text-slate-400">
              {completedItems.size}/{MOCK_VISIT.careItems.length}
            </span>
          </div>
          <div className="space-y-3">
            {MOCK_VISIT.careItems.map((item) => {
              const done = completedItems.has(item.id);
              return (
                <button
                  key={item.id}
                  type="button"
                  onClick={() => toggleItem(item.id)}
                  className="w-full flex items-center gap-3 text-left"
                >
                  <div className={`w-6 h-6 rounded-md border-2 flex items-center justify-center flex-shrink-0 transition-colors ${
                    done ? 'border-green-500 bg-green-500' : 'border-slate-300 bg-white'
                  }`}>
                    {done && (
                      <svg className="w-3.5 h-3.5 text-white" fill="currentColor" viewBox="0 0 20 20">
                        <path fillRule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clipRule="evenodd" />
                      </svg>
                    )}
                  </div>
                  <span className={`text-sm flex-1 ${done ? 'line-through text-slate-400' : 'text-slate-700'}`}>
                    {item.label}
                  </span>
                  {item.required && !done && (
                    <span className="text-xs text-red-500 font-medium flex-shrink-0">필수</span>
                  )}
                </button>
              );
            })}
          </div>
        </div>

        {/* Client Mood */}
        <div className="card">
          <h3 className="section-title">이용자 상태</h3>
          <div className="flex justify-between px-2">
            {MOODS.map((emoji, idx) => (
              <button
                key={idx}
                type="button"
                onClick={() => setMood(idx)}
                className={`flex flex-col items-center gap-1 p-2 rounded-xl transition-colors ${
                  mood === idx ? 'bg-blue-50 scale-110' : 'active:bg-slate-50'
                }`}
              >
                <span className="text-3xl">{emoji}</span>
                <span className="text-xs text-slate-500">
                  {['매우 나쁨', '나쁨', '보통', '좋음', '매우 좋음'][idx]}
                </span>
              </button>
            ))}
          </div>
        </div>

        {/* Vitals */}
        <div className="card">
          <h3 className="section-title">활력징후 (선택)</h3>
          <div className="space-y-3">
            <div className="grid grid-cols-2 gap-2">
              <div>
                <label className="label">수축기 혈압</label>
                <div className="relative">
                  <input className="input-field pr-10" type="text" placeholder="120" inputMode="numeric"
                    value={vitals.bloodPressureSys}
                    onChange={(e) => setVitals((v) => ({ ...v, bloodPressureSys: e.target.value.replace(/\D/g, '') }))} />
                  <span className="absolute right-3 top-1/2 -translate-y-1/2 text-xs text-slate-400">mmHg</span>
                </div>
              </div>
              <div>
                <label className="label">이완기 혈압</label>
                <div className="relative">
                  <input className="input-field pr-10" type="text" placeholder="80" inputMode="numeric"
                    value={vitals.bloodPressureDia}
                    onChange={(e) => setVitals((v) => ({ ...v, bloodPressureDia: e.target.value.replace(/\D/g, '') }))} />
                  <span className="absolute right-3 top-1/2 -translate-y-1/2 text-xs text-slate-400">mmHg</span>
                </div>
              </div>
            </div>
            <div className="grid grid-cols-3 gap-2">
              <div>
                <label className="label">맥박</label>
                <div className="relative">
                  <input className="input-field pr-6" type="text" placeholder="72" inputMode="numeric"
                    value={vitals.pulse}
                    onChange={(e) => setVitals((v) => ({ ...v, pulse: e.target.value.replace(/\D/g, '') }))} />
                  <span className="absolute right-2 top-1/2 -translate-y-1/2 text-xs text-slate-400">회</span>
                </div>
              </div>
              <div>
                <label className="label">체온</label>
                <div className="relative">
                  <input className="input-field pr-4" type="text" placeholder="36.5" inputMode="decimal"
                    value={vitals.temperature}
                    onChange={(e) => setVitals((v) => ({ ...v, temperature: e.target.value }))} />
                  <span className="absolute right-2 top-1/2 -translate-y-1/2 text-xs text-slate-400">°C</span>
                </div>
              </div>
              <div>
                <label className="label">체중</label>
                <div className="relative">
                  <input className="input-field pr-5" type="text" placeholder="58" inputMode="numeric"
                    value={vitals.weight}
                    onChange={(e) => setVitals((v) => ({ ...v, weight: e.target.value.replace(/\D/g, '') }))} />
                  <span className="absolute right-2 top-1/2 -translate-y-1/2 text-xs text-slate-400">kg</span>
                </div>
              </div>
            </div>
          </div>
        </div>

        {/* Care Note */}
        <div className="card">
          <h3 className="section-title">케어 일지</h3>
          <textarea
            className="input-field resize-none"
            rows={4}
            placeholder="오늘 제공한 서비스 내용과 이용자 상태를 기록해 주세요..."
            value={careNote}
            onChange={(e) => setCareNote(e.target.value)}
          />
        </div>

        {/* Special Notes */}
        <div className="card">
          <h3 className="section-title">특이사항</h3>
          <textarea
            className="input-field resize-none"
            rows={3}
            placeholder="이상 증상, 가족 전달사항 등을 기록해 주세요..."
            value={specialNote}
            onChange={(e) => setSpecialNote(e.target.value)}
          />
          <Link href="/notes/incident" className="mt-2 flex items-center gap-1.5 text-sm text-red-600 font-medium">
            <svg className="w-4 h-4" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
            </svg>
            사고가 발생했나요? 사고 보고서 작성
          </Link>
        </div>

        {/* Submit */}
        <div className="pb-6 space-y-3">
          {!requiredDone && (
            <p className="text-xs text-red-500 text-center">필수 케어 항목을 모두 완료해 주세요.</p>
          )}
          <button
            type="button"
            onClick={handleCheckout}
            disabled={!requiredDone || submitting}
            className="btn-primary flex items-center justify-center gap-2"
          >
            {submitting ? (
              <>
                <svg className="w-4 h-4 animate-spin" fill="none" viewBox="0 0 24 24">
                  <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
                  <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
                </svg>
                저장 중...
              </>
            ) : '체크아웃 완료'}
          </button>
          <Link href={`/schedule/${MOCK_VISIT.id}`} className="btn-secondary block text-center">
            취소
          </Link>
        </div>
      </div>
    </CaregiverAppShell>
  );
}
