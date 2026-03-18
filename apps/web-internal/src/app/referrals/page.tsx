import InternalAppShell from '@/components/InternalAppShell';
import Link from 'next/link';

const REFERRALS = [
  {
    id: 'REF-001',
    direction: 'outgoing',
    client: '박순자',
    fromOrg: '행복노인복지센터',
    toOrg: '강남노인병원',
    reason: '낙상 후 재활 치료 필요',
    referredAt: '2026-03-12',
    status: '수락됨',
    notes: '3월 15일 첫 방문 예정',
  },
  {
    id: 'REF-002',
    direction: 'incoming',
    client: '서영자',
    fromOrg: '강남구청 노인복지팀',
    toOrg: '행복노인복지센터',
    reason: '신규 방문요양 서비스 신청',
    referredAt: '2026-03-11',
    status: '검토중',
    notes: '',
  },
  {
    id: 'REF-003',
    direction: 'outgoing',
    client: '홍만수',
    fromOrg: '행복노인복지센터',
    toOrg: '서울노인요양원',
    reason: '건강 악화로 시설 입소 권고',
    referredAt: '2026-03-08',
    status: '처리중',
    notes: '가족 동의 확인 중',
  },
  {
    id: 'REF-004',
    direction: 'incoming',
    client: '이연순',
    fromOrg: '역삼동 주민센터',
    toOrg: '행복노인복지센터',
    reason: '독거노인 방문요양 연계',
    referredAt: '2026-03-05',
    status: '완료',
    notes: '3/10 서비스 개시',
  },
  {
    id: 'REF-005',
    direction: 'incoming',
    client: '김정순',
    fromOrg: '강남구 치매안심센터',
    toOrg: '행복노인복지센터',
    reason: '치매 진단 후 재가서비스 연계',
    referredAt: '2026-03-01',
    status: '완료',
    notes: '서비스 개시 완료',
  },
];

const STATUS_BADGE: Record<string, string> = {
  '수락됨': 'badge-green',
  '검토중': 'badge-yellow',
  '처리중': 'badge-blue',
  '완료': 'badge-gray',
  '거절됨': 'badge-red',
};

export default function ReferralsPage() {
  const incoming = REFERRALS.filter(r => r.direction === 'incoming');
  const outgoing = REFERRALS.filter(r => r.direction === 'outgoing');

  return (
    <InternalAppShell>
      <div className="space-y-6">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="page-title">의뢰 관리</h1>
            <p className="text-sm text-gray-500 mt-1">수신 {incoming.length}건 · 발신 {outgoing.length}건</p>
          </div>
          <Link href="/referrals/new" className="btn-primary">
            <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M12 4v16m8-8H4" />
            </svg>
            새 의뢰 생성
          </Link>
        </div>

        {/* Stats */}
        <div className="grid grid-cols-4 gap-4">
          <div className="stat-card">
            <p className="text-sm font-medium text-gray-500">수신 의뢰</p>
            <p className="text-3xl font-bold text-gray-900 mt-1">{incoming.length}</p>
          </div>
          <div className="stat-card">
            <p className="text-sm font-medium text-gray-500">발신 의뢰</p>
            <p className="text-3xl font-bold text-gray-900 mt-1">{outgoing.length}</p>
          </div>
          <div className="stat-card">
            <p className="text-sm font-medium text-gray-500">검토 필요</p>
            <p className="text-3xl font-bold text-yellow-600 mt-1">{REFERRALS.filter(r => r.status === '검토중').length}</p>
          </div>
          <div className="stat-card">
            <p className="text-sm font-medium text-gray-500">이번달 완료</p>
            <p className="text-3xl font-bold text-green-600 mt-1">{REFERRALS.filter(r => r.status === '완료').length}</p>
          </div>
        </div>

        {/* Two tabs: incoming / outgoing */}
        <div className="space-y-4">
          {/* Incoming */}
          <div className="card">
            <div className="flex items-center justify-between px-5 py-4 border-b border-gray-100">
              <div className="flex items-center gap-2">
                <div className="w-2 h-2 rounded-full bg-green-500"></div>
                <h2 className="section-title">수신 의뢰</h2>
                <span className="badge-green">{incoming.length}건</span>
              </div>
            </div>
            <div className="overflow-x-auto">
              <table className="w-full">
                <thead className="bg-gray-50">
                  <tr>
                    <th className="table-header">의뢰 번호</th>
                    <th className="table-header">이용자</th>
                    <th className="table-header">발신 기관</th>
                    <th className="table-header">사유</th>
                    <th className="table-header">의뢰일</th>
                    <th className="table-header">상태</th>
                    <th className="table-header">비고</th>
                    <th className="table-header"></th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-gray-100">
                  {incoming.map((ref) => (
                    <tr key={ref.id} className="hover:bg-gray-50 transition-colors">
                      <td className="table-cell font-mono text-xs text-gray-600">{ref.id}</td>
                      <td className="table-cell font-medium">{ref.client}</td>
                      <td className="table-cell">{ref.fromOrg}</td>
                      <td className="table-cell max-w-xs">
                        <p className="text-xs text-gray-600 truncate">{ref.reason}</p>
                      </td>
                      <td className="table-cell text-gray-500">{ref.referredAt}</td>
                      <td className="table-cell"><span className={STATUS_BADGE[ref.status]}>{ref.status}</span></td>
                      <td className="table-cell text-xs text-gray-500">{ref.notes || '-'}</td>
                      <td className="table-cell">
                        <button className="text-blue-600 hover:text-blue-800 text-xs font-medium">처리</button>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>

          {/* Outgoing */}
          <div className="card">
            <div className="flex items-center justify-between px-5 py-4 border-b border-gray-100">
              <div className="flex items-center gap-2">
                <div className="w-2 h-2 rounded-full bg-blue-500"></div>
                <h2 className="section-title">발신 의뢰</h2>
                <span className="badge-blue">{outgoing.length}건</span>
              </div>
            </div>
            <div className="overflow-x-auto">
              <table className="w-full">
                <thead className="bg-gray-50">
                  <tr>
                    <th className="table-header">의뢰 번호</th>
                    <th className="table-header">이용자</th>
                    <th className="table-header">수신 기관</th>
                    <th className="table-header">사유</th>
                    <th className="table-header">의뢰일</th>
                    <th className="table-header">상태</th>
                    <th className="table-header">비고</th>
                    <th className="table-header"></th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-gray-100">
                  {outgoing.map((ref) => (
                    <tr key={ref.id} className="hover:bg-gray-50 transition-colors">
                      <td className="table-cell font-mono text-xs text-gray-600">{ref.id}</td>
                      <td className="table-cell font-medium">{ref.client}</td>
                      <td className="table-cell">{ref.toOrg}</td>
                      <td className="table-cell max-w-xs">
                        <p className="text-xs text-gray-600 truncate">{ref.reason}</p>
                      </td>
                      <td className="table-cell text-gray-500">{ref.referredAt}</td>
                      <td className="table-cell"><span className={STATUS_BADGE[ref.status]}>{ref.status}</span></td>
                      <td className="table-cell text-xs text-gray-500">{ref.notes || '-'}</td>
                      <td className="table-cell">
                        <button className="text-blue-600 hover:text-blue-800 text-xs font-medium">추적</button>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </div>
    </InternalAppShell>
  );
}
