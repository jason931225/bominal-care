'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import CaregiverAppShell from '@/components/CaregiverAppShell';

const DAYS = ['월', '화', '수', '목', '금', '토', '일'];
const TIMES = [
  { id: 'morning', label: '오전', sub: '08:00 – 12:00' },
  { id: 'afternoon', label: '오후', sub: '12:00 – 18:00' },
  { id: 'evening', label: '저녁', sub: '18:00 – 22:00' },
];

const REGIONS: Record<string, string[]> = {
  서울: ['강남구', '강동구', '강북구', '강서구', '관악구', '광진구', '구로구', '노원구', '도봉구', '동대문구', '동작구', '마포구', '서대문구', '서초구', '성동구', '성북구', '송파구', '양천구', '영등포구', '용산구', '은평구', '종로구', '중구', '중랑구'],
};

type SlotKey = string;

const INITIAL_SLOTS: SlotKey[] = [
  '월_morning', '월_afternoon', '화_morning', '수_morning', '수_afternoon',
  '목_morning', '금_morning', '금_afternoon',
];

const INITIAL_REGIONS = new Set(['강남구', '서초구', '송파구']);

export default function AvailabilityPage() {
  const router = useRouter();
  const [slots, setSlots] = useState<Set<SlotKey>>(new Set(INITIAL_SLOTS));
  const [regions, setRegions] = useState<Set<string>>(new Set(INITIAL_REGIONS));
  const [travelTime, setTravelTime] = useState(30);
  const [saving, setSaving] = useState(false);
  const [saved, setSaved] = useState(false);
  const [activeCity] = useState('서울');

  const toggleSlot = (day: string, time: string) => {
    const key = `${day}_${time}`;
    setSlots((prev) => {
      const next = new Set(prev);
      if (next.has(key)) {
        next.delete(key);
      } else {
        next.add(key);
      }
      return next;
    });
  };

  const toggleRegion = (r: string) => {
    setRegions((prev) => {
      const next = new Set(prev);
      if (next.has(r)) {
        next.delete(r);
      } else {
        next.add(r);
      }
      return next;
    });
  };

  const handleSave = async () => {
    setSaving(true);
    await new Promise((r) => setTimeout(r, 1000));
    setSaving(false);
    setSaved(true);
    setTimeout(() => {
      router.push('/profile');
    }, 1200);
  };

  return (
    <CaregiverAppShell
      activeTab="profile"
      title="가능 시간 설정"
      showBackButton
      backHref="/profile"
    >
      <div className="px-4 py-6 space-y-6">
        {saved && (
          <div className="fixed top-16 left-4 right-4 z-50 bg-green-500 text-white rounded-2xl p-4 flex items-center gap-3 shadow-lg">
            <svg className="w-5 h-5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
              <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clipRule="evenodd" />
            </svg>
            <p className="font-semibold">저장 완료! 이동 중...</p>
          </div>
        )}

        {/* Schedule Section */}
        <div>
          <h2 className="text-base font-bold text-slate-900 mb-1">근무 가능 요일/시간</h2>
          <p className="text-sm text-slate-500 mb-4">가능한 시간대를 모두 선택해 주세요.</p>

          <div className="overflow-x-auto no-scrollbar -mx-4 px-4">
            <div className="min-w-max">
              <div className="grid gap-1 mb-1" style={{ gridTemplateColumns: '56px repeat(7, 1fr)' }}>
                <div />
                {DAYS.map((day) => (
                  <div key={day} className={`text-center text-xs font-semibold py-1 ${day === '토' ? 'text-blue-600' : day === '일' ? 'text-red-500' : 'text-slate-600'}`}>
                    {day}
                  </div>
                ))}
              </div>
              {TIMES.map((time) => (
                <div key={time.id} className="grid gap-1 mb-1" style={{ gridTemplateColumns: '56px repeat(7, 1fr)' }}>
                  <div className="flex flex-col justify-center">
                    <span className="text-xs font-medium text-slate-600">{time.label}</span>
                    <span className="text-xs text-slate-400 leading-tight">{time.sub}</span>
                  </div>
                  {DAYS.map((day) => {
                    const key = `${day}_${time.id}`;
                    const active = slots.has(key);
                    return (
                      <button
                        key={day}
                        type="button"
                        onClick={() => toggleSlot(day, time.id)}
                        className={`h-12 rounded-lg transition-colors text-xs font-medium ${
                          active ? 'bg-blue-600 text-white' : 'bg-slate-100 text-slate-400'
                        }`}
                      >
                        {active ? '✓' : ''}
                      </button>
                    );
                  })}
                </div>
              ))}
            </div>
          </div>

          <div className="mt-3 flex items-center justify-between">
            <span className="text-sm text-slate-600">
              <span className="font-semibold text-blue-600">{slots.size}개</span> 시간대 선택됨
            </span>
            <button
              type="button"
              onClick={() => setSlots(new Set())}
              className="text-xs text-slate-400 underline"
            >
              전체 초기화
            </button>
          </div>
        </div>

        {/* Region Section */}
        <div>
          <h2 className="text-base font-bold text-slate-900 mb-1">활동 가능 지역</h2>
          <p className="text-sm text-slate-500 mb-4">근무 가능한 지역을 선택해 주세요.</p>

          <div className="grid grid-cols-4 gap-2">
            {REGIONS[activeCity].slice(0, 20).map((r) => {
              const active = regions.has(r);
              return (
                <button
                  key={r}
                  type="button"
                  onClick={() => toggleRegion(r)}
                  className={`py-2.5 px-1 rounded-xl text-xs font-medium transition-colors ${
                    active ? 'bg-blue-600 text-white' : 'bg-slate-100 text-slate-600'
                  }`}
                >
                  {r}
                </button>
              );
            })}
          </div>

          <div className="mt-3 flex items-center justify-between">
            <span className="text-sm text-slate-600">
              <span className="font-semibold text-blue-600">{regions.size}개</span> 지역 선택됨
            </span>
          </div>
        </div>

        {/* Travel Time */}
        <div className="card">
          <div className="flex items-center justify-between mb-3">
            <h3 className="text-sm font-semibold text-slate-800">최대 이동 가능 시간</h3>
            <span className="text-sm font-bold text-blue-600">{travelTime}분</span>
          </div>
          <input
            type="range"
            min={10}
            max={90}
            step={10}
            value={travelTime}
            onChange={(e) => setTravelTime(Number(e.target.value))}
            className="w-full accent-blue-600"
          />
          <div className="flex justify-between text-xs text-slate-400 mt-1">
            <span>10분</span>
            <span>90분</span>
          </div>
        </div>

        {/* Save */}
        <div className="pb-6 space-y-3">
          <button
            type="button"
            onClick={handleSave}
            disabled={slots.size === 0 || regions.size === 0 || saving}
            className="btn-primary flex items-center justify-center gap-2"
          >
            {saving ? (
              <>
                <svg className="w-4 h-4 animate-spin" fill="none" viewBox="0 0 24 24">
                  <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
                  <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
                </svg>
                저장 중...
              </>
            ) : '변경 사항 저장'}
          </button>
          <button type="button" onClick={() => router.back()} className="btn-secondary">
            취소
          </button>
        </div>
      </div>
    </CaregiverAppShell>
  );
}
