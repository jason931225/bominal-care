import InternalAppShell from '@/components/InternalAppShell';

const REPORTS = [
  {
    category: '운영 보고서',
    items: [
      { name: '월별 급여 제공 현황', period: '2026년 2월', generated: '2026-03-05', status: '완료', format: 'Excel' },
      { name: '분기별 운영 실적', period: '2025년 4분기', generated: '2026-01-10', status: '완료', format: 'PDF' },
      { name: '연간 운영 결과 보고서', period: '2025년', generated: '2026-02-15', status: '완료', format: 'PDF' },
    ],
  },
  {
    category: '품질 보고서',
    items: [
      { name: '이용자 만족도 조사 결과', period: '2026년 1분기', generated: '2026-03-10', status: '완료', format: 'PDF' },
      { name: '사고 현황 및 조치 결과', period: '2026년 2월', generated: '2026-03-05', status: '완료', format: 'PDF' },
      { name: '케어 플랜 이행률 분석', period: '2026년 2월', generated: '2026-03-08', status: '완료', format: 'Excel' },
    ],
  },
  {
    category: '인사 보고서',
    items: [
      { name: '요양보호사 근무 현황', period: '2026년 2월', generated: '2026-03-03', status: '완료', format: 'Excel' },
      { name: '방문 이행률 개인별 현황', period: '2026년 2월', generated: '2026-03-03', status: '완료', format: 'Excel' },
      { name: '교육 이수 현황', period: '2025년', generated: '2026-01-10', status: '완료', format: 'PDF' },
    ],
  },
  {
    category: '재무 보고서',
    items: [
      { name: '장기요양급여 청구 현황', period: '2026년 2월', generated: '2026-03-05', status: '완료', format: 'Excel' },
      { name: '기관 수익 현황', period: '2026년 2월', generated: '2026-03-10', status: '완료', format: 'Excel' },
    ],
  },
];

const MONTHLY_STATS = [
  { label: '방문 완료', value: '389', unit: '건', change: '+4.1%', up: true },
  { label: '방문 완료율', value: '95.3', unit: '%', change: '+1.1%p', up: true },
  { label: '신규 이용자', value: '3', unit: '명', change: '-1명', up: false },
  { label: '이탈 이용자', value: '1', unit: '명', change: '없음', up: true },
  { label: '신규 사고', value: '2', unit: '건', change: '-1건', up: true },
  { label: '급여 청구액', value: '28.4', unit: '백만원', change: '+3.2%', up: true },
];

export default function ReportsPage() {
  return (
    <InternalAppShell>
      <div className="space-y-6">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="page-title">보고서 및 분석</h1>
            <p className="text-sm text-gray-500 mt-1">2026년 3월 15일 기준</p>
          </div>
          <button className="btn-primary">
            <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M12 4v16m8-8H4" />
            </svg>
            보고서 생성
          </button>
        </div>

        {/* Last month stats */}
        <div className="card p-5">
          <h2 className="section-title mb-4">지난달 (2026년 2월) 주요 지표</h2>
          <div className="grid grid-cols-6 gap-4">
            {MONTHLY_STATS.map((stat) => (
              <div key={stat.label} className="text-center">
                <p className="text-xs text-gray-500 mb-1">{stat.label}</p>
                <div className="flex items-end justify-center gap-0.5">
                  <span className="text-2xl font-bold text-gray-900">{stat.value}</span>
                  <span className="text-sm text-gray-500 mb-0.5">{stat.unit}</span>
                </div>
                <p className={`text-xs font-medium mt-1 ${stat.up ? 'text-green-600' : 'text-red-600'}`}>
                  {stat.change}
                </p>
              </div>
            ))}
          </div>
        </div>

        {/* Report generator */}
        <div className="card p-5">
          <h2 className="section-title mb-4">보고서 빠른 생성</h2>
          <div className="grid grid-cols-4 gap-3">
            {[
              { name: '월별 방문 현황', icon: '📊', desc: '이번달 방문 완료/미이행 현황' },
              { name: '이용자 현황', icon: '👤', desc: '활성/대기/종결 이용자 목록' },
              { name: '요양보호사 현황', icon: '👩', desc: '근무 시간 및 방문 이행률' },
              { name: '사고/이상 현황', icon: '🔔', desc: '이번달 사고 및 조치 현황' },
            ].map((report) => (
              <button
                key={report.name}
                className="card p-4 text-left hover:border-blue-300 hover:shadow-md transition-all group"
              >
                <div className="text-2xl mb-2">{report.icon}</div>
                <p className="text-sm font-semibold text-gray-900 group-hover:text-blue-700">{report.name}</p>
                <p className="text-xs text-gray-500 mt-0.5">{report.desc}</p>
              </button>
            ))}
          </div>
        </div>

        {/* Report library */}
        <div className="space-y-4">
          {REPORTS.map((cat) => (
            <div key={cat.category} className="card overflow-hidden">
              <div className="px-5 py-3 bg-gray-50 border-b border-gray-200">
                <h2 className="font-semibold text-gray-900">{cat.category}</h2>
              </div>
              <div className="divide-y divide-gray-100">
                {cat.items.map((report, idx) => (
                  <div key={idx} className="flex items-center gap-4 px-5 py-3 hover:bg-gray-50 transition-colors">
                    <div className="w-9 h-9 rounded-lg bg-blue-50 flex items-center justify-center flex-shrink-0">
                      <svg className="w-5 h-5 text-blue-600" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                        <path strokeLinecap="round" strokeLinejoin="round" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                      </svg>
                    </div>
                    <div className="flex-1">
                      <p className="text-sm font-medium text-gray-900">{report.name}</p>
                      <p className="text-xs text-gray-500">기간: {report.period}</p>
                    </div>
                    <div className="text-right">
                      <p className="text-xs text-gray-500">{report.generated}</p>
                      <span className="text-xs font-medium text-gray-600 bg-gray-100 px-2 py-0.5 rounded">{report.format}</span>
                    </div>
                    <span className="badge-green">{report.status}</span>
                    <div className="flex items-center gap-2 ml-2">
                      <button className="text-xs text-blue-600 hover:text-blue-800 font-medium">보기</button>
                      <button className="text-xs text-gray-500 hover:text-gray-700 font-medium">다운로드</button>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          ))}
        </div>
      </div>
    </InternalAppShell>
  );
}
