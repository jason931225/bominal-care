import InternalAppShell from '@/components/InternalAppShell';

const COMPLIANCE_CATEGORIES = [
  {
    category: '법정 서류',
    items: [
      { name: '장기요양기관 지정서', status: 'ok', dueDate: '2027-03-01', note: '갱신 완료' },
      { name: '사업자 등록증', status: 'ok', dueDate: '-', note: '유효' },
      { name: '시설 배상책임보험', status: 'ok', dueDate: '2026-12-31', note: '가입 완료' },
      { name: '개인정보 처리방침 공개', status: 'ok', dueDate: '-', note: '홈페이지 게시 완료' },
    ],
  },
  {
    category: '요양보호사 관리',
    items: [
      { name: '자격증 사본 보관 (전 직원)', status: 'warning', dueDate: '2026-03-31', note: '3명 미제출' },
      { name: '건강검진 결과 보관', status: 'warning', dueDate: '2026-04-30', note: '5명 기한 만료 예정' },
      { name: '범죄경력 조회서', status: 'ok', dueDate: '-', note: '전 직원 완료' },
      { name: '교육 이수 확인 (연 8시간)', status: 'ok', dueDate: '2026-12-31', note: '완료율 94%' },
    ],
  },
  {
    category: '이용자 관리',
    items: [
      { name: '장기요양인정서 보관', status: 'ok', dueDate: '-', note: '전 이용자 완료' },
      { name: '표준장기요양이용계약서', status: 'warning', dueDate: '2026-03-31', note: '2명 서명 미완료' },
      { name: '개인정보 수집 동의서', status: 'ok', dueDate: '-', note: '전 이용자 완료' },
      { name: '케어 플랜 (6개월 갱신)', status: 'ok', dueDate: '2026-07-10', note: '유효' },
    ],
  },
  {
    category: '기관 운영',
    items: [
      { name: '급여제공 기록지 보관', status: 'ok', dueDate: '-', note: '정상 보관' },
      { name: '월별 청구 적정성 확인', status: 'ok', dueDate: '-', note: '3월 청구 완료' },
      { name: '현지조사 대비 서류 점검', status: 'error', dueDate: '2026-03-27', note: 'D-12 점검 예정' },
      { name: '운영위원회 회의록', status: 'ok', dueDate: '-', note: '분기 1회 완료' },
    ],
  },
];

const STATUS_CONFIG = {
  ok: { badge: 'badge-green', label: '정상', icon: '✓' },
  warning: { badge: 'badge-yellow', label: '주의', icon: '!' },
  error: { badge: 'badge-red', label: '위반', icon: '✗' },
};

export default function CompliancePage() {
  const allItems = COMPLIANCE_CATEGORIES.flatMap(c => c.items);
  const okCount = allItems.filter(i => i.status === 'ok').length;
  const warningCount = allItems.filter(i => i.status === 'warning').length;
  const errorCount = allItems.filter(i => i.status === 'error').length;
  const total = allItems.length;

  return (
    <InternalAppShell>
      <div className="space-y-6">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="page-title">규정 준수 관리</h1>
            <p className="text-sm text-gray-500 mt-1">총 {total}개 항목 점검</p>
          </div>
          <div className="flex gap-2">
            <button className="btn-secondary">
              <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                <path strokeLinecap="round" strokeLinejoin="round" d="M17 17h2a2 2 0 002-2v-4a2 2 0 00-2-2H5a2 2 0 00-2 2v4a2 2 0 002 2h2m2 4h6a2 2 0 002-2v-4a2 2 0 00-2-2H9a2 2 0 00-2 2v4a2 2 0 002 2zm8-12V5a2 2 0 00-2-2H9a2 2 0 00-2 2v4h10z" />
              </svg>
              점검표 출력
            </button>
          </div>
        </div>

        {/* Summary */}
        <div className="grid grid-cols-4 gap-4">
          <div className="stat-card">
            <p className="text-sm font-medium text-gray-500">전체 항목</p>
            <p className="text-3xl font-bold text-gray-900 mt-1">{total}</p>
          </div>
          <div className="stat-card border-l-4 border-l-green-500">
            <p className="text-sm font-medium text-gray-500">정상</p>
            <p className="text-3xl font-bold text-green-600 mt-1">{okCount}</p>
          </div>
          <div className="stat-card border-l-4 border-l-yellow-400">
            <p className="text-sm font-medium text-gray-500">주의</p>
            <p className="text-3xl font-bold text-yellow-600 mt-1">{warningCount}</p>
          </div>
          <div className="stat-card border-l-4 border-l-red-500">
            <p className="text-sm font-medium text-gray-500">위반/긴급</p>
            <p className="text-3xl font-bold text-red-600 mt-1">{errorCount}</p>
          </div>
        </div>

        {/* Overall compliance bar */}
        <div className="card p-5">
          <div className="flex items-center justify-between mb-3">
            <h2 className="section-title">전체 준수율</h2>
            <span className="text-2xl font-bold text-gray-900">{Math.round((okCount / total) * 100)}%</span>
          </div>
          <div className="w-full bg-gray-100 rounded-full h-4 overflow-hidden">
            <div className="h-full flex">
              <div className="bg-green-500" style={{ width: `${(okCount / total) * 100}%` }} />
              <div className="bg-yellow-400" style={{ width: `${(warningCount / total) * 100}%` }} />
              <div className="bg-red-500" style={{ width: `${(errorCount / total) * 100}%` }} />
            </div>
          </div>
          <div className="flex items-center gap-6 mt-3">
            <div className="flex items-center gap-1.5"><div className="w-3 h-3 rounded bg-green-500" /><span className="text-xs text-gray-500">정상</span></div>
            <div className="flex items-center gap-1.5"><div className="w-3 h-3 rounded bg-yellow-400" /><span className="text-xs text-gray-500">주의</span></div>
            <div className="flex items-center gap-1.5"><div className="w-3 h-3 rounded bg-red-500" /><span className="text-xs text-gray-500">위반</span></div>
          </div>
        </div>

        {/* Checklist by category */}
        <div className="space-y-4">
          {COMPLIANCE_CATEGORIES.map((cat) => (
            <div key={cat.category} className="card overflow-hidden">
              <div className="px-5 py-3 bg-gray-50 border-b border-gray-200">
                <div className="flex items-center justify-between">
                  <h2 className="font-semibold text-gray-900">{cat.category}</h2>
                  <div className="flex items-center gap-2">
                    <span className="text-xs text-gray-500">
                      {cat.items.filter(i => i.status === 'ok').length}/{cat.items.length} 정상
                    </span>
                  </div>
                </div>
              </div>
              <div className="divide-y divide-gray-100">
                {cat.items.map((item, idx) => {
                  const config = STATUS_CONFIG[item.status as keyof typeof STATUS_CONFIG];
                  return (
                    <div key={idx} className="flex items-center gap-4 px-5 py-3 hover:bg-gray-50 transition-colors">
                      <div className={`w-7 h-7 rounded-full flex items-center justify-center text-sm font-bold flex-shrink-0 ${
                        item.status === 'ok' ? 'bg-green-100 text-green-700' :
                        item.status === 'warning' ? 'bg-yellow-100 text-yellow-700' :
                        'bg-red-100 text-red-700'
                      }`}>
                        {config.icon}
                      </div>
                      <div className="flex-1">
                        <p className="text-sm font-medium text-gray-900">{item.name}</p>
                        <p className="text-xs text-gray-500">{item.note}</p>
                      </div>
                      <div className="text-right">
                        <span className={config.badge}>{config.label}</span>
                        {item.dueDate !== '-' && (
                          <p className="text-xs text-gray-400 mt-1">만료: {item.dueDate}</p>
                        )}
                      </div>
                      <button className="text-gray-400 hover:text-blue-600 transition-colors">
                        <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                          <path strokeLinecap="round" strokeLinejoin="round" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                        </svg>
                      </button>
                    </div>
                  );
                })}
              </div>
            </div>
          ))}
        </div>

        {/* Upcoming deadlines */}
        <div className="card p-5">
          <h2 className="section-title mb-4">임박한 기한</h2>
          <div className="space-y-3">
            {[
              { item: '요양보호사 자격증 사본 제출', deadline: '2026-03-31', days: 16, severity: 'yellow' },
              { item: '현지조사 대비 서류 점검', deadline: '2026-03-27', days: 12, severity: 'red' },
              { item: '표준장기요양이용계약서 서명', deadline: '2026-03-31', days: 16, severity: 'yellow' },
              { item: '요양보호사 건강검진 결과 제출', deadline: '2026-04-30', days: 46, severity: 'blue' },
            ].map((d, idx) => (
              <div key={idx} className={`flex items-center justify-between p-3 rounded-lg border ${
                d.severity === 'red' ? 'bg-red-50 border-red-200' :
                d.severity === 'yellow' ? 'bg-yellow-50 border-yellow-200' :
                'bg-blue-50 border-blue-200'
              }`}>
                <div>
                  <p className="text-sm font-medium text-gray-900">{d.item}</p>
                  <p className="text-xs text-gray-500">기한: {d.deadline}</p>
                </div>
                <span className={`text-sm font-bold px-3 py-1 rounded-full ${
                  d.severity === 'red' ? 'bg-red-100 text-red-700' :
                  d.severity === 'yellow' ? 'bg-yellow-100 text-yellow-700' :
                  'bg-blue-100 text-blue-700'
                }`}>
                  D-{d.days}
                </span>
              </div>
            ))}
          </div>
        </div>
      </div>
    </InternalAppShell>
  );
}
