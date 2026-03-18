'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import ApplicantAppShell from '@/components/ApplicantAppShell';

const DAYS = ['월', '화', '수', '목', '금', '토', '일'];
const TIMES = [
  { id: 'morning', label: '오전', sub: '08:00 – 12:00' },
  { id: 'afternoon', label: '오후', sub: '12:00 – 18:00' },
  { id: 'evening', label: '저녁', sub: '18:00 – 22:00' },
  { id: 'night', label: '야간', sub: '22:00 – 06:00' },
];

type SlotKey = `${string}_${string}`;

export default function SchedulePage() {
  const router = useRouter();
  const [selected, setSelected] = useState<Set<SlotKey>>(new Set());
  const [minHours, setMinHours] = useState(4);
  const [maxHours, setMaxHours] = useState(8);

  const toggleSlot = (day: string, time: string) => {
    const key: SlotKey = `${day}_${time}`;
    setSelected((prev) => {
      const next = new Set(prev);
      if (next.has(key)) {
        next.delete(key);
      } else {
        next.add(key);
      }
      return next;
    });
  };

  const isSelected = (day: string, time: string) => selected.has(`${day}_${time}` as SlotKey);

  const selectAll = () => {
    const all = new Set<SlotKey>();
    DAYS.forEach((d) => TIMES.forEach((t) => all.add(`${d}_${t.id}` as SlotKey)));
    setSelected(all);
  };

  const clearAll = () => setSelected(new Set());

  const selectedCount = selected.size;

  return (
    <ApplicantAppShell currentStep={4} title="가능 시간 설정">
      <div className="px-4 py-6 space-y-6">
        <div>
          <h2 className="text-lg font-bold text-slate-900 mb-1">근무 가능한 요일·시간을 선택해 주세요</h2>
          <p className="text-sm text-slate-500">해당하는 칸을 탭하여 선택하세요.</p>
        </div>

        {/* Actions */}
        <div className="flex items-center justify-between">
          <span className="text-sm text-slate-600">
            {selectedCount > 0 ? (
              <span className="text-blue-600 font-semibold">{selectedCount}개</span>
            ) : '0개'} 선택됨
          </span>
          <div className="flex gap-2">
            <button type="button" onClick={selectAll} className="text-xs text-blue-600 font-medium px-3 py-1.5 bg-blue-50 rounded-lg">
              전체선택
            </button>
            <button type="button" onClick={clearAll} className="text-xs text-slate-500 font-medium px-3 py-1.5 bg-slate-100 rounded-lg">
              초기화
            </button>
          </div>
        </div>

        {/* Schedule Grid */}
        <div className="overflow-x-auto no-scrollbar -mx-4 px-4">
          <div className="min-w-max">
            {/* Header row */}
            <div className="grid gap-1 mb-1" style={{ gridTemplateColumns: '60px repeat(7, 1fr)' }}>
              <div />
              {DAYS.map((day) => (
                <div key={day} className={`text-center text-xs font-semibold py-1 ${day === '토' ? 'text-blue-600' : day === '일' ? 'text-red-500' : 'text-slate-600'}`}>
                  {day}
                </div>
              ))}
            </div>

            {/* Time rows */}
            {TIMES.map((time) => (
              <div key={time.id} className="grid gap-1 mb-1" style={{ gridTemplateColumns: '60px repeat(7, 1fr)' }}>
                <div className="flex flex-col justify-center pr-2">
                  <span className="text-xs font-medium text-slate-700">{time.label}</span>
                  <span className="text-xs text-slate-400 leading-tight">{time.sub}</span>
                </div>
                {DAYS.map((day) => {
                  const active = isSelected(day, time.id);
                  return (
                    <button
                      key={day}
                      type="button"
                      onClick={() => toggleSlot(day, time.id)}
                      className={`h-12 rounded-lg transition-colors text-xs font-medium ${
                        active
                          ? 'bg-blue-600 text-white shadow-sm'
                          : 'bg-slate-100 text-slate-400'
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

        {/* Hours preference */}
        <div className="card space-y-4">
          <h3 className="text-sm font-semibold text-slate-800">1일 선호 근무시간</h3>
          <div>
            <div className="flex justify-between text-sm mb-2">
              <span className="text-slate-600">최소</span>
              <span className="font-bold text-blue-600">{minHours}시간</span>
            </div>
            <input
              type="range"
              min={2}
              max={maxHours}
              step={1}
              value={minHours}
              onChange={(e) => setMinHours(Number(e.target.value))}
              className="w-full accent-blue-600"
            />
          </div>
          <div>
            <div className="flex justify-between text-sm mb-2">
              <span className="text-slate-600">최대</span>
              <span className="font-bold text-blue-600">{maxHours}시간</span>
            </div>
            <input
              type="range"
              min={minHours}
              max={12}
              step={1}
              value={maxHours}
              onChange={(e) => setMaxHours(Number(e.target.value))}
              className="w-full accent-blue-600"
            />
          </div>
        </div>

        <div className="pb-6">
          <button
            type="button"
            onClick={() => router.push('/apply/services')}
            disabled={selectedCount === 0}
            className="btn-primary"
          >
            다음 단계로
          </button>
        </div>
      </div>
    </ApplicantAppShell>
  );
}
