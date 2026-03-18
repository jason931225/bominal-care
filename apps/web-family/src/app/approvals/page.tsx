import Link from 'next/link';
import FamilyAppShell from '@/components/FamilyAppShell';

const APPROVALS = [
  {
    id: 'apv-001',
    title: '물리치료 횟수 증가 승인',
    description: '주 2회 → 주 4회 (낙상 후 재활 목적)',
    requestedBy: '케어매니저 최지원',
    requestedAt: '2026-03-13',
    deadline: '2026-03-15',
    urgency: 'high',
    urgencyLabel: '긴급',
    status: 'pending',
    statusLabel: '승인 대기',
    category: '서비스 변경',
    estimatedCost: '+월 120,000원 (본인 부담)',
  },
  {
    id: 'apv-002',
    title: '보조기기 대여 승인',
    description: '전동 휠체어 렌탈 요청 (월 85,000원)',
    requestedBy: '케어매니저 최지원',
    requestedAt: '2026-03-14',
    deadline: '2026-03-20',
    urgency: 'normal',
    urgencyLabel: '일반',
    status: 'pending',
    statusLabel: '승인 대기',
    category: '보조기기',
    estimatedCost: '월 17,000원 (본인 부담 20%)',
  },
  {
    id: 'apv-003',
    title: '담당 요양보호사 교체 동의',
    description: '이순자 요양보호사 → 박미영 요양보호사 (개인 사정)',
    requestedBy: '행복케어 복지센터',
    requestedAt: '2026-03-08',
    deadline: '2026-03-10',
    urgency: 'normal',
    urgencyLabel: '일반',
    status: 'approved',
    statusLabel: '승인 완료',
    category: '인력 변경',
    estimatedCost: '변동 없음',
  },
  {
    id: 'apv-004',
    title: '응급 입원 동의서',
    description: '2월 14일 응급 입원 처치 사후 동의',
    requestedBy: '강남성모병원 의료팀',
    requestedAt: '2026-02-14',
    deadline: '2026-02-15',
    urgency: 'high',
    urgencyLabel: '긴급',
    status: 'approved',
    statusLabel: '승인 완료',
    category: '의료',
    estimatedCost: '실손보험 청구 예정',
  },
];

const statusStyle: Record<string, string> = {
  pending: 'bg-yellow-50 text-yellow-700 border border-yellow-200',
  approved: 'bg-green-50 text-green-700 border border-green-200',
  rejected: 'bg-red-50 text-red-700 border border-red-200',
};

const urgencyStyle: Record<string, string> = {
  high: 'bg-red-50 text-red-700 border border-red-200',
  normal: 'bg-gray-100 text-gray-600 border border-gray-200',
};

export default function ApprovalsPage() {
  const pendingCount = APPROVALS.filter((a) => a.status === 'pending').length;

  return (
    <FamilyAppShell>
      <div className="max-w-3xl mx-auto px-4 py-6">
        {/* Header */}
        <div className="mb-6">
          <h1 className="text-2xl font-bold text-gray-900">승인 관리</h1>
          <p className="text-sm text-gray-500 mt-1">
            대기 중인 승인 요청
            {pendingCount > 0 && (
              <span className="ml-2 font-bold text-red-600">{pendingCount}건 즉시 처리 필요</span>
            )}
          </p>
        </div>

        {/* Stats */}
        <div className="grid grid-cols-3 gap-3 mb-6">
          <div className="bg-white border border-gray-200 rounded-xl p-3 text-center">
            <p className="text-2xl font-bold text-yellow-600">{pendingCount}</p>
            <p className="text-xs text-gray-500 mt-0.5">대기 중</p>
          </div>
          <div className="bg-white border border-gray-200 rounded-xl p-3 text-center">
            <p className="text-2xl font-bold text-green-600">
              {APPROVALS.filter((a) => a.status === 'approved').length}
            </p>
            <p className="text-xs text-gray-500 mt-0.5">승인 완료</p>
          </div>
          <div className="bg-white border border-gray-200 rounded-xl p-3 text-center">
            <p className="text-2xl font-bold text-gray-600">{APPROVALS.length}</p>
            <p className="text-xs text-gray-500 mt-0.5">전체</p>
          </div>
        </div>

        {/* Pending Approvals */}
        {APPROVALS.filter((a) => a.status === 'pending').length > 0 && (
          <section className="mb-6">
            <h2 className="text-sm font-semibold text-red-600 uppercase tracking-wide mb-3 flex items-center gap-2">
              <span className="w-2 h-2 rounded-full bg-red-500 inline-block" />
              승인 필요
            </h2>
            <div className="space-y-3">
              {APPROVALS.filter((a) => a.status === 'pending').map((approval) => (
                <Link key={approval.id} href={`/approvals/${approval.id}`}>
                  <div className="bg-white border border-yellow-200 rounded-xl p-4 hover:border-yellow-400 transition-colors shadow-sm">
                    <div className="flex items-start justify-between gap-2 mb-2">
                      <div className="flex items-center gap-2 flex-wrap">
                        <span className={`text-xs font-semibold px-2 py-0.5 rounded-full ${urgencyStyle[approval.urgency]}`}>
                          {approval.urgencyLabel}
                        </span>
                        <span className="text-xs text-gray-500">{approval.category}</span>
                      </div>
                      <span className={`text-xs font-semibold px-2 py-0.5 rounded-full flex-shrink-0 ${statusStyle[approval.status]}`}>
                        {approval.statusLabel}
                      </span>
                    </div>
                    <p className="font-semibold text-gray-900">{approval.title}</p>
                    <p className="text-sm text-gray-600 mt-0.5">{approval.description}</p>
                    <div className="flex items-center justify-between mt-3">
                      <div>
                        <p className="text-xs text-gray-400">{approval.requestedBy}</p>
                        <p className="text-xs text-red-600 font-medium mt-0.5">마감: {approval.deadline}</p>
                      </div>
                      <span className="text-sm text-blue-600 font-medium">처리하기 →</span>
                    </div>
                  </div>
                </Link>
              ))}
            </div>
          </section>
        )}

        {/* Completed */}
        <section>
          <h2 className="text-sm font-semibold text-gray-400 uppercase tracking-wide mb-3">
            처리 완료
          </h2>
          <div className="space-y-2">
            {APPROVALS.filter((a) => a.status !== 'pending').map((approval) => (
              <Link key={approval.id} href={`/approvals/${approval.id}`}>
                <div className="bg-gray-50 border border-gray-200 rounded-xl p-4 hover:bg-gray-100 transition-colors">
                  <div className="flex items-start justify-between gap-2">
                    <div className="flex-1">
                      <p className="font-medium text-gray-700">{approval.title}</p>
                      <p className="text-xs text-gray-500 mt-0.5">{approval.requestedBy} · {approval.requestedAt}</p>
                    </div>
                    <span className={`flex-shrink-0 text-xs font-semibold px-2 py-0.5 rounded-full ${statusStyle[approval.status]}`}>
                      {approval.statusLabel}
                    </span>
                  </div>
                </div>
              </Link>
            ))}
          </div>
        </section>
      </div>
    </FamilyAppShell>
  );
}
