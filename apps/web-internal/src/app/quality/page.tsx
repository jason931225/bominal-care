import InternalAppShell from '@/components/InternalAppShell';
import Link from 'next/link';

const METRICS = [
  { label: '방문 완료율', value: 94.2, target: 95, unit: '%', color: 'yellow' },
  { label: '이용자 만족도', value: 4.7, target: 4.5, unit: '/5.0', color: 'green' },
  { label: '요양보호사 만족도', value: 4.5, target: 4.0, unit: '/5.0', color: 'green' },
  { label: '사고 발생률', value: 1.2, target: 2.0, unit: '%', color: 'green' },
  { label: '케어 플랜 준수율', value: 91.8, target: 90, unit: '%', color: 'green' },
  { label: '서류 적시 제출률', value: 87.3, target: 90, unit: '%', color: 'yellow' },
];

const MONTHLY_TRENDS = [
  { month: '10월', visits: 398, completed: 374, incidents: 3 },
  { month: '11월', visits: 412, completed: 392, incidents: 2 },
  { month: '12월', visits: 387, completed: 365, incidents: 5 },
  { month: '01월', visits: 421, completed: 401, incidents: 4 },
  { month: '02월', visits: 408, completed: 389, incidents: 3 },
  { month: '03월', visits: 142, completed: 134, incidents: 1 },
];

const RECENT_INCIDENTS = [
  { id: 'INC-001', date: '2026-03-12', client: '이정자', type: '낙상', severity: 'high', status: '처리중' },
  { id: 'INC-002', date: '2026-03-08', client: '박순자', type: '건강 이상', severity: 'medium', status: '처리완료' },
  { id: 'INC-003', date: '2026-02-28', client: '김복동', type: '서비스 불만', severity: 'low', status: '처리완료' },
];

const SEVERITY_BADGE: Record<string, string> = { high: 'badge-red', medium: 'badge-yellow', low: 'badge-blue' };
const SEVERITY_LABEL: Record<string, string> = { high: '높음', medium: '보통', low: '낮음' };
const STATUS_BADGE: Record<string, string> = { '처리중': 'badge-yellow', '처리완료': 'badge-green', '미처리': 'badge-red' };

export default function QualityPage() {
  const maxVisits = Math.max(...MONTHLY_TRENDS.map(m => m.visits));

  return (
    <InternalAppShell>
      <div className="space-y-6">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="page-title">품질 관리</h1>
            <p className="text-sm text-gray-500 mt-1">2026년 3월 기준</p>
          </div>
          <Link href="/quality/incidents" className="btn-secondary">
            사고 관리 보기
          </Link>
        </div>

        {/* KPI grid */}
        <div className="grid grid-cols-3 gap-4">
          {METRICS.map((metric) => {
            const achieved = metric.value >= metric.target;
            const progress = metric.unit === '%'
              ? Math.min((metric.value / 100) * 100, 100)
              : (metric.value / 5) * 100;

            return (
              <div key={metric.label} className="card p-5">
                <div className="flex items-start justify-between mb-3">
                  <p className="text-sm font-medium text-gray-600">{metric.label}</p>
                  <span className={achieved ? 'badge-green' : 'badge-yellow'}>
                    {achieved ? '목표 달성' : '목표 미달'}
                  </span>
                </div>
                <div className="flex items-end gap-1 mb-3">
                  <span className="text-3xl font-bold text-gray-900">{metric.value}</span>
                  <span className="text-sm text-gray-500 mb-1">{metric.unit}</span>
                </div>
                <div className="w-full bg-gray-100 rounded-full h-2">
                  <div
                    className={`h-2 rounded-full ${achieved ? 'bg-green-500' : 'bg-yellow-400'}`}
                    style={{ width: `${progress}%` }}
                  />
                </div>
                <p className="text-xs text-gray-400 mt-1">목표: {metric.target}{metric.unit}</p>
              </div>
            );
          })}
        </div>

        <div className="grid grid-cols-3 gap-6">
          {/* Monthly trend chart */}
          <div className="col-span-2 card p-5">
            <h2 className="section-title mb-5">월별 방문 현황</h2>
            <div className="flex items-end gap-4 h-48">
              {MONTHLY_TRENDS.map((m) => {
                const totalH = (m.visits / maxVisits) * 100;
                const completedH = (m.completed / maxVisits) * 100;
                const rate = Math.round((m.completed / m.visits) * 100);
                return (
                  <div key={m.month} className="flex-1 flex flex-col items-center gap-1">
                    <span className="text-xs text-gray-500">{rate}%</span>
                    <div className="relative w-full flex items-end" style={{ height: '160px' }}>
                      <div
                        className="absolute bottom-0 left-0 right-0 bg-blue-100 rounded-t"
                        style={{ height: `${totalH}%` }}
                      />
                      <div
                        className="absolute bottom-0 left-0 right-0 bg-blue-500 rounded-t"
                        style={{ height: `${completedH}%` }}
                      />
                    </div>
                    <span className="text-xs font-medium text-gray-600">{m.month}</span>
                    <span className="text-xs text-gray-400">{m.visits}건</span>
                  </div>
                );
              })}
            </div>
            <div className="flex items-center gap-4 mt-4">
              <div className="flex items-center gap-1.5">
                <div className="w-3 h-3 rounded bg-blue-100"></div>
                <span className="text-xs text-gray-500">전체 방문</span>
              </div>
              <div className="flex items-center gap-1.5">
                <div className="w-3 h-3 rounded bg-blue-500"></div>
                <span className="text-xs text-gray-500">완료 방문</span>
              </div>
            </div>
          </div>

          {/* Recent incidents */}
          <div className="card">
            <div className="flex items-center justify-between px-5 py-4 border-b border-gray-100">
              <h2 className="section-title">최근 사고</h2>
              <Link href="/quality/incidents" className="text-xs text-blue-600 hover:text-blue-800 font-medium">전체보기</Link>
            </div>
            <ul className="divide-y divide-gray-100">
              {RECENT_INCIDENTS.map((inc) => (
                <li key={inc.id} className="px-5 py-3">
                  <div className="flex items-start justify-between gap-2">
                    <div>
                      <p className="text-sm font-medium text-gray-900">{inc.client}</p>
                      <p className="text-xs text-gray-500">{inc.type} · {inc.date}</p>
                    </div>
                    <div className="flex flex-col items-end gap-1">
                      <span className={STATUS_BADGE[inc.status]}>{inc.status}</span>
                      <span className={SEVERITY_BADGE[inc.severity]}>{SEVERITY_LABEL[inc.severity]}</span>
                    </div>
                  </div>
                </li>
              ))}
            </ul>
            <div className="px-5 py-3 border-t border-gray-100">
              <Link href="/quality/incidents" className="text-xs text-blue-600 hover:text-blue-800 font-medium">
                전체 사고 목록 →
              </Link>
            </div>
          </div>
        </div>

        {/* Improvement areas */}
        <div className="card p-5">
          <h2 className="section-title mb-4">개선 필요 사항</h2>
          <div className="grid grid-cols-2 gap-4">
            {[
              { title: '방문 완료율 개선', desc: '목표 95% 대비 현재 94.2%. 미이행 방문 원인 분석 및 대체 인력 확보 필요.', priority: 'medium' },
              { title: '서류 적시 제출률 개선', desc: '목표 90% 대비 현재 87.3%. 요양보호사 교육 및 제출 리마인더 강화 필요.', priority: 'medium' },
            ].map((item, idx) => (
              <div key={idx} className="bg-yellow-50 border border-yellow-200 rounded-xl p-4">
                <div className="flex items-center gap-2 mb-2">
                  <svg className="w-4 h-4 text-yellow-600" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                    <path strokeLinecap="round" strokeLinejoin="round" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                  </svg>
                  <p className="font-semibold text-gray-900">{item.title}</p>
                </div>
                <p className="text-sm text-gray-700">{item.desc}</p>
              </div>
            ))}
          </div>
        </div>
      </div>
    </InternalAppShell>
  );
}
