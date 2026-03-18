import Link from 'next/link';
import FamilyAppShell from '@/components/FamilyAppShell';

const MATCH_REQUESTS = [
  {
    id: 'req-001',
    title: '방문 요양 요청 #001',
    createdAt: '2026-03-10',
    status: 'matched',
    statusLabel: '매칭 완료',
    statusColor: 'green',
    serviceType: '방문 요양',
    schedule: '주 5회 (월~금) 오전 8시',
    recommendations: 3,
    selected: '박미영 요양보호사',
  },
  {
    id: 'req-002',
    title: '물리치료 서비스 요청 #002',
    createdAt: '2026-03-13',
    status: 'reviewing',
    statusLabel: '추천 검토 중',
    statusColor: 'blue',
    serviceType: '물리치료',
    schedule: '주 4회 (월/화/목/금) 오후 2시',
    recommendations: 5,
    selected: null,
  },
  {
    id: 'req-003',
    title: '인지 프로그램 요청 #003',
    createdAt: '2026-03-14',
    status: 'pending',
    statusLabel: '매칭 대기',
    statusColor: 'yellow',
    serviceType: '인지 재활',
    schedule: '주 3회 (화/목/토)',
    recommendations: 0,
    selected: null,
  },
];

const statusBadge: Record<string, string> = {
  green: 'bg-green-50 text-green-700 border border-green-200',
  blue: 'bg-blue-50 text-blue-700 border border-blue-200',
  yellow: 'bg-yellow-50 text-yellow-700 border border-yellow-200',
};

export default function MatchingPage() {
  return (
    <FamilyAppShell>
      <div className="max-w-3xl mx-auto px-4 py-6">
        {/* Header */}
        <div className="flex items-start justify-between mb-6">
          <div>
            <h1 className="text-2xl font-bold text-gray-900">매칭 요청</h1>
            <p className="text-sm text-gray-500 mt-1">서비스 제공자 매칭 요청 목록</p>
          </div>
          <Link
            href="/matching/new"
            className="flex-shrink-0 px-4 py-2 bg-blue-600 text-white text-sm font-semibold rounded-lg hover:bg-blue-700 transition-colors"
          >
            + 새 요청
          </Link>
        </div>

        {/* Summary Cards */}
        <div className="grid grid-cols-3 gap-3 mb-6">
          {[
            { label: '전체 요청', value: MATCH_REQUESTS.length, color: 'gray' },
            { label: '매칭 완료', value: MATCH_REQUESTS.filter((r) => r.status === 'matched').length, color: 'green' },
            { label: '검토 중', value: MATCH_REQUESTS.filter((r) => r.status !== 'matched').length, color: 'blue' },
          ].map((stat) => (
            <div key={stat.label} className="bg-white border border-gray-200 rounded-xl p-3 text-center">
              <p className="text-2xl font-bold text-gray-900">{stat.value}</p>
              <p className="text-xs text-gray-500 mt-0.5">{stat.label}</p>
            </div>
          ))}
        </div>

        {/* Requests List */}
        <div className="space-y-3">
          {MATCH_REQUESTS.map((req) => (
            <div key={req.id} className="bg-white border border-gray-200 rounded-xl p-5 hover:border-blue-300 transition-colors">
              <div className="flex items-start justify-between gap-3 mb-3">
                <div>
                  <h3 className="font-semibold text-gray-900">{req.title}</h3>
                  <p className="text-xs text-gray-400 mt-0.5">생성일: {req.createdAt}</p>
                </div>
                <span className={`flex-shrink-0 text-xs font-semibold px-2.5 py-1 rounded-full ${statusBadge[req.statusColor]}`}>
                  {req.statusLabel}
                </span>
              </div>

              <div className="grid grid-cols-2 gap-3 mb-4">
                <div className="bg-gray-50 rounded-lg p-3">
                  <p className="text-xs text-gray-400 mb-0.5">서비스 유형</p>
                  <p className="text-sm font-medium text-gray-800">{req.serviceType}</p>
                </div>
                <div className="bg-gray-50 rounded-lg p-3">
                  <p className="text-xs text-gray-400 mb-0.5">스케줄</p>
                  <p className="text-sm font-medium text-gray-800">{req.schedule}</p>
                </div>
              </div>

              {req.selected && (
                <div className="bg-green-50 border border-green-200 rounded-lg p-3 mb-3">
                  <p className="text-xs text-green-600 font-medium">선택된 제공자</p>
                  <p className="text-sm font-semibold text-green-800 mt-0.5">{req.selected}</p>
                </div>
              )}

              <div className="flex items-center justify-between">
                <span className="text-sm text-gray-500">
                  추천 제공자: <span className="font-semibold text-gray-800">{req.recommendations}명</span>
                </span>
                <Link
                  href={`/matching/${req.id}`}
                  className="text-sm text-blue-600 font-medium hover:underline"
                >
                  상세 보기 →
                </Link>
              </div>
            </div>
          ))}
        </div>

        {/* Create New CTA */}
        <div className="mt-6 border-2 border-dashed border-gray-200 rounded-xl p-6 text-center hover:border-blue-300 transition-colors cursor-pointer">
          <span className="text-3xl mb-2 block">🔍</span>
          <p className="font-semibold text-gray-700">새 매칭 요청 만들기</p>
          <p className="text-sm text-gray-500 mt-1 mb-4">필요한 서비스 유형, 일정, 조건을 입력하세요</p>
          <Link
            href="/matching/new"
            className="inline-flex items-center gap-2 px-5 py-2 bg-blue-600 text-white text-sm font-semibold rounded-lg hover:bg-blue-700 transition-colors"
          >
            매칭 요청 시작
          </Link>
        </div>
      </div>
    </FamilyAppShell>
  );
}
