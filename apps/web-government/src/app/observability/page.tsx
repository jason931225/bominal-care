'use client';

import { useState, useEffect, useCallback } from 'react';
import GovernmentAppShell from '@/components/GovernmentAppShell';

interface DashboardStats {
  byEventType: Array<{ eventType: string; count: number }>;
  bySeverity: Array<{ severity: string; count: number }>;
  total: number;
  unacknowledged: number;
}

interface Signal {
  id: string;
  event_type: string;
  severity: string;
  subject_person_id: string | null;
  actor_user_id: string | null;
  entity_type: string | null;
  entity_id: string | null;
  message: string;
  metadata: unknown;
  acknowledged_at: string | null;
  acknowledged_by: string | null;
  created_at: string;
}

const SEVERITY_BADGE: Record<string, string> = {
  CRITICAL: 'badge-red',
  HIGH: 'badge-red',
  MEDIUM: 'badge-yellow',
  LOW: 'badge-blue',
  INFO: 'badge-gray',
};

const SEVERITY_DOT: Record<string, string> = {
  CRITICAL: 'bg-red-500',
  HIGH: 'bg-red-400',
  MEDIUM: 'bg-yellow-500',
  LOW: 'bg-blue-400',
  INFO: 'bg-gray-400',
};

const EVENT_TYPE_LABEL: Record<string, string> = {
  VITAL_ANOMALY: '활력 징후 이상',
  MISSED_MEDICATION: '복약 누락',
  MISSED_VISIT: '방문 누락',
  FALL_DETECTED: '낙상 감지',
  INACTIVITY: '활동 없음',
  CARE_PLAN_CHANGE: '케어 플랜 변경',
  EMERGENCY_ALERT: '긴급 알림',
  SYSTEM_ERROR: '시스템 오류',
};

const MOCK_STATS: DashboardStats = {
  total: 156,
  unacknowledged: 23,
  bySeverity: [
    { severity: 'CRITICAL', count: 3 },
    { severity: 'HIGH', count: 12 },
    { severity: 'MEDIUM', count: 34 },
    { severity: 'LOW', count: 45 },
    { severity: 'INFO', count: 62 },
  ],
  byEventType: [
    { eventType: 'MISSED_MEDICATION', count: 42 },
    { eventType: 'VITAL_ANOMALY', count: 28 },
    { eventType: 'MISSED_VISIT', count: 22 },
    { eventType: 'INACTIVITY', count: 19 },
    { eventType: 'FALL_DETECTED', count: 15 },
    { eventType: 'CARE_PLAN_CHANGE', count: 14 },
    { eventType: 'EMERGENCY_ALERT', count: 9 },
    { eventType: 'SYSTEM_ERROR', count: 7 },
  ],
};

const MOCK_SIGNALS: Signal[] = [
  {
    id: 'SIG-001',
    event_type: 'FALL_DETECTED',
    severity: 'CRITICAL',
    subject_person_id: 'PER-001',
    actor_user_id: null,
    entity_type: 'sensor',
    entity_id: 'SENSOR-42',
    message: '김복순 어르신 낙상 감지 - 거실 센서',
    metadata: { location: '거실', confidence: 0.95 },
    acknowledged_at: null,
    acknowledged_by: null,
    created_at: '2026-03-15T14:20:00Z',
  },
  {
    id: 'SIG-002',
    event_type: 'MISSED_MEDICATION',
    severity: 'HIGH',
    subject_person_id: 'PER-002',
    actor_user_id: null,
    entity_type: 'medication_event',
    entity_id: 'MED-EVT-100',
    message: '이병수 어르신 오전 혈압약 미복용 (3회 연속)',
    metadata: { medication: '혈압약', consecutive_misses: 3 },
    acknowledged_at: null,
    acknowledged_by: null,
    created_at: '2026-03-15T12:00:00Z',
  },
  {
    id: 'SIG-003',
    event_type: 'VITAL_ANOMALY',
    severity: 'MEDIUM',
    subject_person_id: 'PER-003',
    actor_user_id: null,
    entity_type: 'vital_reading',
    entity_id: 'VIT-201',
    message: '최순자 어르신 혈압 상승 감지 (수축기 160mmHg)',
    metadata: { systolic: 160, diastolic: 95 },
    acknowledged_at: '2026-03-15T11:00:00Z',
    acknowledged_by: 'USR-001',
    created_at: '2026-03-15T10:30:00Z',
  },
  {
    id: 'SIG-004',
    event_type: 'MISSED_VISIT',
    severity: 'HIGH',
    subject_person_id: 'PER-004',
    actor_user_id: 'USR-005',
    entity_type: 'visit',
    entity_id: 'VIS-302',
    message: '박미영 요양보호사 방문 일정 미이행 - 정만복 어르신',
    metadata: { scheduled_at: '2026-03-15T09:00:00Z' },
    acknowledged_at: null,
    acknowledged_by: null,
    created_at: '2026-03-15T09:30:00Z',
  },
  {
    id: 'SIG-005',
    event_type: 'INACTIVITY',
    severity: 'MEDIUM',
    subject_person_id: 'PER-005',
    actor_user_id: null,
    entity_type: 'activity_monitor',
    entity_id: 'ACT-55',
    message: '오영자 어르신 12시간 이상 움직임 미감지',
    metadata: { last_activity: '2026-03-14T22:00:00Z' },
    acknowledged_at: null,
    acknowledged_by: null,
    created_at: '2026-03-15T10:00:00Z',
  },
  {
    id: 'SIG-006',
    event_type: 'CARE_PLAN_CHANGE',
    severity: 'INFO',
    subject_person_id: 'PER-001',
    actor_user_id: 'USR-001',
    entity_type: 'care_plan',
    entity_id: 'CP-012',
    message: '김복순 어르신 케어 플랜 서비스 항목 변경',
    metadata: { change: '방문목욕 추가' },
    acknowledged_at: '2026-03-15T09:00:00Z',
    acknowledged_by: 'USR-001',
    created_at: '2026-03-15T08:45:00Z',
  },
];

const SEVERITY_STAT_COLORS: Record<string, string> = {
  CRITICAL: 'bg-red-50 text-red-700 border-l-4 border-l-red-500',
  HIGH: 'bg-orange-50 text-orange-700 border-l-4 border-l-orange-500',
  MEDIUM: 'bg-yellow-50 text-yellow-700 border-l-4 border-l-yellow-400',
  LOW: 'bg-blue-50 text-blue-700 border-l-4 border-l-blue-400',
  INFO: 'bg-gray-50 text-gray-700 border-l-4 border-l-gray-400',
};

function formatTimestamp(iso: string): string {
  const date = new Date(iso);
  const yyyy = date.getFullYear();
  const mm = String(date.getMonth() + 1).padStart(2, '0');
  const dd = String(date.getDate()).padStart(2, '0');
  const hh = String(date.getHours()).padStart(2, '0');
  const min = String(date.getMinutes()).padStart(2, '0');
  return `${yyyy}-${mm}-${dd} ${hh}:${min}`;
}

export default function ObservabilityPage() {
  const [stats, setStats] = useState<DashboardStats>(MOCK_STATS);
  const [signals, setSignals] = useState<Signal[]>([]);
  const [loading, setLoading] = useState(true);

  const fetchData = useCallback(async () => {
    setLoading(true);
    try {
      const response = await fetch('/api/observability');
      const result = await response.json();
      if (result.success && result.data !== null) {
        setStats(result.data.stats);
        setSignals(result.data.signals);
      } else {
        setStats(MOCK_STATS);
        setSignals(MOCK_SIGNALS);
      }
    } catch {
      setStats(MOCK_STATS);
      setSignals(MOCK_SIGNALS);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchData();
  }, [fetchData]);

  return (
    <GovernmentAppShell>
      <div className="space-y-6">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="page-title">모니터링</h1>
            <p className="text-sm text-gray-500 mt-1">실시간 시그널 모니터링 대시보드</p>
          </div>
          <div className="flex gap-2">
            <button onClick={fetchData} className="btn-secondary">
              <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                <path strokeLinecap="round" strokeLinejoin="round" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
              </svg>
              새로고침
            </button>
          </div>
        </div>

        {loading ? (
          <div className="flex items-center justify-center py-16">
            <p className="text-sm text-gray-500">로딩 중...</p>
          </div>
        ) : (
          <>
            {/* Summary Stats */}
            <div className="grid grid-cols-4 gap-4">
              <div className="stat-card">
                <p className="text-sm font-medium text-gray-500">전체 시그널</p>
                <p className="text-3xl font-bold text-gray-900 mt-1">{stats.total}</p>
              </div>
              <div className="stat-card border-l-4 border-l-red-500">
                <p className="text-sm font-medium text-gray-500">미확인</p>
                <p className="text-3xl font-bold text-red-600 mt-1">{stats.unacknowledged}</p>
              </div>
              <div className="stat-card border-l-4 border-l-red-400">
                <p className="text-sm font-medium text-gray-500">긴급 / 높음</p>
                <p className="text-3xl font-bold text-orange-600 mt-1">
                  {stats.bySeverity
                    .filter((s) => s.severity === 'CRITICAL' || s.severity === 'HIGH')
                    .reduce((sum, s) => sum + s.count, 0)}
                </p>
              </div>
              <div className="stat-card border-l-4 border-l-blue-400">
                <p className="text-sm font-medium text-gray-500">보통 이하</p>
                <p className="text-3xl font-bold text-blue-600 mt-1">
                  {stats.bySeverity
                    .filter((s) => s.severity !== 'CRITICAL' && s.severity !== 'HIGH')
                    .reduce((sum, s) => sum + s.count, 0)}
                </p>
              </div>
            </div>

            <div className="grid grid-cols-3 gap-6">
              {/* By Severity */}
              <div className="card">
                <div className="px-5 py-4 border-b border-slate-100">
                  <h2 className="section-title">심각도별 분류</h2>
                </div>
                <div className="p-4 space-y-2">
                  {stats.bySeverity.map((item) => (
                    <div
                      key={item.severity}
                      className={`flex items-center justify-between p-3 rounded-lg ${SEVERITY_STAT_COLORS[item.severity] ?? 'bg-gray-50'}`}
                    >
                      <span className="text-sm font-medium">{item.severity}</span>
                      <span className="text-lg font-bold">{item.count}</span>
                    </div>
                  ))}
                </div>
              </div>

              {/* By Event Type */}
              <div className="col-span-2 card">
                <div className="px-5 py-4 border-b border-slate-100">
                  <h2 className="section-title">이벤트 유형별 분류</h2>
                </div>
                <div className="p-4">
                  <div className="grid grid-cols-2 gap-3">
                    {stats.byEventType.map((item) => (
                      <div key={item.eventType} className="flex items-center justify-between p-3 bg-slate-50 rounded-lg">
                        <span className="text-sm text-gray-700">
                          {EVENT_TYPE_LABEL[item.eventType] ?? item.eventType}
                        </span>
                        <span className="text-sm font-bold text-gray-900">{item.count}</span>
                      </div>
                    ))}
                  </div>
                </div>
              </div>
            </div>

            {/* Recent Signals */}
            <div className="card overflow-hidden">
              <div className="flex items-center justify-between px-5 py-4 border-b border-slate-100">
                <h2 className="section-title">최근 시그널</h2>
                <span className="text-xs text-gray-500">미확인 {signals.filter((s) => s.acknowledged_at === null).length}건</span>
              </div>
              <div className="divide-y divide-slate-100">
                {signals.map((signal) => (
                  <div
                    key={signal.id}
                    className={`flex items-start gap-4 px-5 py-4 hover:bg-slate-50 transition-colors ${
                      signal.acknowledged_at === null ? 'bg-white' : 'bg-slate-25 opacity-75'
                    }`}
                  >
                    <div className={`w-2.5 h-2.5 rounded-full flex-shrink-0 mt-1.5 ${SEVERITY_DOT[signal.severity] ?? 'bg-gray-400'}`} />
                    <div className="flex-1 min-w-0">
                      <div className="flex items-center gap-2 mb-1">
                        <span className={SEVERITY_BADGE[signal.severity] ?? 'badge-gray'}>
                          {signal.severity}
                        </span>
                        <span className="text-xs text-gray-500 bg-gray-100 px-2 py-0.5 rounded-full">
                          {EVENT_TYPE_LABEL[signal.event_type] ?? signal.event_type}
                        </span>
                        {signal.acknowledged_at !== null && (
                          <span className="text-xs text-green-600 font-medium">확인됨</span>
                        )}
                      </div>
                      <p className="text-sm font-medium text-gray-900">{signal.message}</p>
                      <div className="flex items-center gap-3 mt-1 text-xs text-gray-400">
                        <span>{formatTimestamp(signal.created_at)}</span>
                        {signal.entity_id && <span>{signal.entity_id}</span>}
                      </div>
                    </div>
                    {signal.acknowledged_at === null && (
                      <button className="flex-shrink-0 text-xs text-indigo-600 hover:text-indigo-800 font-medium px-3 py-1 border border-indigo-200 rounded-lg hover:bg-indigo-50 transition-colors">
                        확인
                      </button>
                    )}
                  </div>
                ))}
              </div>
            </div>
          </>
        )}
      </div>
    </GovernmentAppShell>
  );
}
