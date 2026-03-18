// 복용 기록 — Medication Log / History
// Chronological log of medication intake with adherence tracking

'use client';

import { useState } from 'react';
import Link from 'next/link';
import SeniorAppShell from '@/components/SeniorAppShell';

type TakenStatus = 'taken' | 'missed' | 'late';

interface LogEntry {
  id: string;
  date: string;
  timeLabel: string;
  medications: string[];
  status: TakenStatus;
  takenAt?: string;
  note?: string;
}

const LOG_DATA: LogEntry[] = [
  // Today
  { id: 'log-1', date: '2026년 3월 15일', timeLabel: '아침 식전', medications: ['오메프라졸 20mg'], status: 'taken', takenAt: '오전 7:52' },
  { id: 'log-2', date: '2026년 3월 15일', timeLabel: '아침 식후', medications: ['암로디핀 5mg', '메트포르민 500mg'], status: 'taken', takenAt: '오전 8:10' },
  { id: 'log-3', date: '2026년 3월 15일', timeLabel: '점심 식후', medications: ['아스피린 100mg', '메트포르민 500mg'], status: 'missed' },
  // Yesterday
  { id: 'log-4', date: '2026년 3월 14일', timeLabel: '아침 식전', medications: ['오메프라졸 20mg'], status: 'taken', takenAt: '오전 8:05' },
  { id: 'log-5', date: '2026년 3월 14일', timeLabel: '아침 식후', medications: ['암로디핀 5mg', '메트포르민 500mg'], status: 'taken', takenAt: '오전 8:30' },
  { id: 'log-6', date: '2026년 3월 14일', timeLabel: '점심 식후', medications: ['아스피린 100mg', '메트포르민 500mg'], status: 'taken', takenAt: '오후 12:45' },
  { id: 'log-7', date: '2026년 3월 14일', timeLabel: '저녁 식후', medications: ['메트포르민 500mg'], status: 'late', takenAt: '오후 9:20', note: '저녁 식사가 늦어 지연 복용' },
  // 3 days ago
  { id: 'log-8', date: '2026년 3월 13일', timeLabel: '아침 식전', medications: ['오메프라졸 20mg'], status: 'taken', takenAt: '오전 7:48' },
  { id: 'log-9', date: '2026년 3월 13일', timeLabel: '아침 식후', medications: ['암로디핀 5mg', '메트포르민 500mg'], status: 'taken', takenAt: '오전 8:15' },
  { id: 'log-10', date: '2026년 3월 13일', timeLabel: '점심 식후', medications: ['아스피린 100mg', '메트포르민 500mg'], status: 'missed', note: '외출로 인해 복용 누락' },
  { id: 'log-11', date: '2026년 3월 13일', timeLabel: '저녁 식후', medications: ['메트포르민 500mg'], status: 'taken', takenAt: '오후 7:05' },
];

const STATUS_CONFIG: Record<TakenStatus, { label: string; bg: string; text: string; icon: string }> = {
  taken: { label: '복용 완료', bg: 'bg-success-50', text: 'text-success-700', icon: '✅' },
  missed: { label: '복용 누락', bg: 'bg-danger-50', text: 'text-danger-700', icon: '❌' },
  late: { label: '지연 복용', bg: 'bg-warning-50', text: 'text-warning-700', icon: '⚠️' },
};

// Group entries by date
function groupByDate(entries: LogEntry[]): Map<string, LogEntry[]> {
  return entries.reduce((acc, entry) => {
    const existing = acc.get(entry.date) ?? [];
    return new Map(acc).set(entry.date, [...existing, entry]);
  }, new Map<string, LogEntry[]>());
}

// Adherence rate calculation for past 7 days
function calcAdherence(entries: LogEntry[]): number {
  const total = entries.length;
  if (total === 0) return 100;
  const taken = entries.filter((e) => e.status === 'taken' || e.status === 'late').length;
  return Math.round((taken / total) * 100);
}

export default function MedicationLogPage() {
  const [filterStatus, setFilterStatus] = useState<TakenStatus | 'all'>('all');

  const filtered = filterStatus === 'all'
    ? LOG_DATA
    : LOG_DATA.filter((e) => e.status === filterStatus);

  const grouped = groupByDate(filtered);
  const adherence = calcAdherence(LOG_DATA);

  return (
    <SeniorAppShell>
      <div className="page-content">
        {/* Header */}
        <Link
          href="/medications"
          className="inline-flex items-center gap-2 text-primary-600 font-medium text-senior-base mb-5 min-h-touch"
        >
          <svg className="w-5 h-5" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24" aria-hidden="true">
            <path strokeLinecap="round" strokeLinejoin="round" d="M15 19l-7-7 7-7" />
          </svg>
          약물 목록으로
        </Link>

        <h1 className="text-senior-2xl font-bold text-gray-900 mb-5">복용 기록</h1>

        {/* Adherence summary */}
        <div className="senior-card mb-5">
          <h2 className="text-senior-base font-bold text-gray-700 mb-3">최근 7일 복용 이행률</h2>
          <div className="flex items-center gap-4">
            <div className="relative w-20 h-20 flex-shrink-0">
              {/* Circular progress approximation using a ring */}
              <svg className="w-20 h-20 -rotate-90" viewBox="0 0 80 80" aria-hidden="true">
                <circle cx="40" cy="40" r="30" fill="none" stroke="#e5e7eb" strokeWidth="8" />
                <circle
                  cx="40" cy="40" r="30"
                  fill="none"
                  stroke={adherence >= 80 ? '#22c55e' : adherence >= 60 ? '#f59e0b' : '#ef4444'}
                  strokeWidth="8"
                  strokeDasharray={`${2 * Math.PI * 30}`}
                  strokeDashoffset={`${2 * Math.PI * 30 * (1 - adherence / 100)}`}
                  strokeLinecap="round"
                />
              </svg>
              <span className="absolute inset-0 flex items-center justify-center text-senior-lg font-bold text-gray-800">
                {adherence}%
              </span>
            </div>
            <div>
              <p className={`text-senior-xl font-bold ${adherence >= 80 ? 'text-success-600' : 'text-warning-600'}`}>
                {adherence >= 80 ? '양호' : '개선 필요'}
              </p>
              <p className="text-senior-sm text-gray-500">복용 누락 시 의사나 케어매니저에게 알려주세요.</p>
            </div>
          </div>
        </div>

        {/* Filter tabs */}
        <div className="flex gap-2 mb-5 overflow-x-auto pb-1" role="group" aria-label="복용 기록 필터">
          {(['all', 'taken', 'missed', 'late'] as const).map((f) => (
            <button
              key={f}
              onClick={() => setFilterStatus(f)}
              className={`flex-shrink-0 px-4 py-2 rounded-xl text-senior-sm font-semibold border-2 transition-colors min-h-touch
                ${filterStatus === f
                  ? 'bg-primary-600 border-primary-600 text-white'
                  : 'bg-white border-gray-300 text-gray-700 hover:border-primary-400'
                }`}
            >
              {f === 'all' ? '전체' : STATUS_CONFIG[f].label}
            </button>
          ))}
        </div>

        {/* Log entries grouped by date */}
        {Array.from(grouped.entries()).map(([date, entries]) => (
          <section key={date} className="mb-5" aria-labelledby={`date-${date}`}>
            <h2 id={`date-${date}`} className="text-senior-base font-bold text-gray-600 mb-2 px-1">
              {date}
            </h2>
            <div className="space-y-2">
              {entries.map((entry) => {
                const config = STATUS_CONFIG[entry.status];
                return (
                  <div key={entry.id} className={`${config.bg} rounded-2xl p-4`}>
                    <div className="flex items-start justify-between mb-1">
                      <div className="flex items-center gap-2">
                        <span aria-hidden="true">{config.icon}</span>
                        <p className={`text-senior-base font-bold ${config.text}`}>{entry.timeLabel}</p>
                      </div>
                      <span className={`text-senior-sm font-semibold ${config.text}`}>
                        {entry.takenAt ?? (entry.status === 'missed' ? '미복용' : '')}
                      </span>
                    </div>
                    <p className="text-senior-sm text-gray-600 ml-7">{entry.medications.join(', ')}</p>
                    {entry.note && (
                      <p className="text-senior-sm text-gray-500 ml-7 mt-1 italic">{entry.note}</p>
                    )}
                  </div>
                );
              })}
            </div>
          </section>
        ))}

        {filtered.length === 0 && (
          <div className="text-center py-16 text-gray-400">
            <p className="text-senior-lg">해당 기록이 없습니다.</p>
          </div>
        )}
      </div>
    </SeniorAppShell>
  );
}
