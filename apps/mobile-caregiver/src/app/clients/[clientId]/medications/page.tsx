'use client';

import { useState } from 'react';
import CaregiverAppShell from '@/components/CaregiverAppShell';

const MEDICATIONS = [
  {
    id: 'm001',
    name: '암로디핀 5mg',
    purpose: '고혈압 치료',
    times: [{ time: '10:00', label: '아침', taken: false }],
    dosage: '1정 (경구)',
    withFood: false,
    sideEffects: ['부종', '두통', '어지러움'],
    prescribedBy: '서울내과 김의사',
    refillDate: '2026-04-01',
    color: 'bg-orange-50 border-orange-200',
    iconColor: 'text-orange-500',
  },
  {
    id: 'm002',
    name: '아리셉트 5mg',
    purpose: '치매 진행 억제',
    times: [{ time: '21:00', label: '저녁', taken: false }],
    dosage: '1정 (경구)',
    withFood: true,
    sideEffects: ['구역감', '식욕 저하', '설사'],
    prescribedBy: '강남 신경과 이의사',
    refillDate: '2026-03-25',
    color: 'bg-purple-50 border-purple-200',
    iconColor: 'text-purple-500',
  },
  {
    id: 'm003',
    name: '칼슘 + 비타민D',
    purpose: '골다공증 예방',
    times: [{ time: '13:00', label: '점심', taken: true }],
    dosage: '1정 (경구)',
    withFood: true,
    sideEffects: ['변비'],
    prescribedBy: '일반 처방',
    refillDate: '2026-05-01',
    color: 'bg-yellow-50 border-yellow-200',
    iconColor: 'text-yellow-500',
  },
];

export default function MedicationsPage({ params: _params }: { params: Promise<{ clientId: string }> }) {
  const [takenMap, setTakenMap] = useState<Record<string, boolean>>(
    Object.fromEntries(MEDICATIONS.flatMap((m) => m.times.map((t) => [`${m.id}_${t.time}`, t.taken])))
  );
  const [expandedId, setExpandedId] = useState<string | null>(null);

  const toggleTaken = (medId: string, time: string) => {
    const key = `${medId}_${time}`;
    setTakenMap((prev) => ({ ...prev, [key]: !prev[key] }));
  };

  const todayDate = '2026년 3월 15일';

  return (
    <CaregiverAppShell
      activeTab="clients"
      title="이순자 투약 관리"
      showBackButton
      backHref="/clients/c001"
    >
      <div className="px-4 py-4 space-y-4">
        {/* Date Header */}
        <div className="flex items-center justify-between">
          <h2 className="text-sm font-semibold text-slate-700">{todayDate} 투약 현황</h2>
          <span className="badge-info">
            {Object.values(takenMap).filter(Boolean).length}/{Object.values(takenMap).length} 완료
          </span>
        </div>

        {/* Progress Bar */}
        <div className="card py-3">
          <div className="flex items-center justify-between text-xs text-slate-500 mb-2">
            <span>오늘 투약 진행률</span>
            <span className="font-semibold text-blue-600">
              {Math.round((Object.values(takenMap).filter(Boolean).length / Object.values(takenMap).length) * 100)}%
            </span>
          </div>
          <div className="h-2 bg-slate-100 rounded-full overflow-hidden">
            <div
              className="h-full bg-blue-500 rounded-full transition-all duration-500"
              style={{
                width: `${Math.round((Object.values(takenMap).filter(Boolean).length / Object.values(takenMap).length) * 100)}%`,
              }}
            />
          </div>
        </div>

        {/* Medication Cards */}
        {MEDICATIONS.map((med) => {
          const isExpanded = expandedId === med.id;
          const isAllTaken = med.times.every((t) => takenMap[`${med.id}_${t.time}`]);

          return (
            <div key={med.id} className={`card border ${med.color}`}>
              {/* Header */}
              <div className="flex items-start justify-between mb-3">
                <div className="flex items-start gap-3">
                  <span className={`text-2xl mt-0.5 ${med.iconColor}`}>💊</span>
                  <div>
                    <p className="text-base font-bold text-slate-800">{med.name}</p>
                    <p className="text-xs text-slate-500">{med.purpose}</p>
                    <p className="text-xs text-slate-400 mt-0.5">{med.dosage} · {med.withFood ? '식후 복용' : '식전 복용'}</p>
                  </div>
                </div>
                {isAllTaken && <span className="badge-success flex-shrink-0">완료</span>}
              </div>

              {/* Time Slots */}
              <div className="flex gap-2 flex-wrap mb-3">
                {med.times.map((slot) => {
                  const key = `${med.id}_${slot.time}`;
                  const taken = takenMap[key];
                  return (
                    <button
                      key={slot.time}
                      type="button"
                      onClick={() => toggleTaken(med.id, slot.time)}
                      className={`flex items-center gap-2 px-3 py-2 rounded-xl border text-sm font-medium transition-colors ${
                        taken
                          ? 'bg-green-500 border-green-500 text-white'
                          : 'bg-white border-slate-300 text-slate-600 active:bg-slate-50'
                      }`}
                    >
                      {taken ? (
                        <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                          <path fillRule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clipRule="evenodd" />
                        </svg>
                      ) : (
                        <svg className="w-4 h-4" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24">
                          <path strokeLinecap="round" strokeLinejoin="round" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
                        </svg>
                      )}
                      {slot.label} {slot.time}
                    </button>
                  );
                })}
              </div>

              {/* Expand Toggle */}
              <button
                type="button"
                onClick={() => setExpandedId(isExpanded ? null : med.id)}
                className="flex items-center gap-1 text-xs text-slate-400 font-medium"
              >
                상세 정보
                <svg className={`w-3.5 h-3.5 transition-transform ${isExpanded ? 'rotate-180' : ''}`} fill="none" stroke="currentColor" strokeWidth={2.5} viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" d="M19 9l-7 7-7-7" />
                </svg>
              </button>

              {/* Expanded */}
              {isExpanded && (
                <div className="mt-3 pt-3 border-t border-slate-200 space-y-2">
                  <div>
                    <p className="text-xs font-medium text-slate-500 mb-1">주의 부작용</p>
                    <div className="flex flex-wrap gap-1">
                      {med.sideEffects.map((s) => (
                        <span key={s} className="text-xs bg-red-100 text-red-600 px-2 py-0.5 rounded-full">{s}</span>
                      ))}
                    </div>
                  </div>
                  <div className="flex justify-between text-xs">
                    <span className="text-slate-500">처방 의사</span>
                    <span className="text-slate-700 font-medium">{med.prescribedBy}</span>
                  </div>
                  <div className="flex justify-between text-xs">
                    <span className="text-slate-500">리필 예정일</span>
                    <span className={`font-medium ${med.refillDate < '2026-04-01' ? 'text-red-600' : 'text-slate-700'}`}>
                      {med.refillDate}
                    </span>
                  </div>
                </div>
              )}
            </div>
          );
        })}

        {/* Allergy Warning */}
        <div className="card bg-red-50 border-red-200">
          <h3 className="text-sm font-semibold text-red-700 mb-2">⚠️ 알레르기 주의</h3>
          <div className="flex gap-2 flex-wrap">
            {['페니실린', '조개류'].map((a) => (
              <span key={a} className="badge-danger">{a}</span>
            ))}
          </div>
        </div>

        {/* Note */}
        <div className="card">
          <h3 className="section-title">투약 메모</h3>
          <textarea
            className="input-field resize-none"
            rows={3}
            placeholder="오늘 투약 특이사항을 기록해 주세요..."
          />
          <button type="button" className="mt-2 btn-primary py-3 text-sm">
            기록 저장
          </button>
        </div>

        <div className="pb-4" />
      </div>
    </CaregiverAppShell>
  );
}
