import Link from 'next/link';
import FamilyAppShell from '@/components/FamilyAppShell';

const PAYMENTS = [
  {
    id: 'pay-001',
    date: '2026-03-10',
    service: '방문 요양 서비스',
    provider: '박미영 요양보호사',
    period: '2026년 2월',
    total: 487000,
    copay: 97400,
    insurance: 389600,
    status: 'paid',
    statusLabel: '결제 완료',
    method: '자동이체 (국민은행)',
  },
  {
    id: 'pay-002',
    date: '2026-02-10',
    service: '방문 요양 서비스',
    provider: '박미영 요양보호사',
    period: '2026년 1월',
    total: 512000,
    copay: 102400,
    insurance: 409600,
    status: 'paid',
    statusLabel: '결제 완료',
    method: '자동이체 (국민은행)',
  },
  {
    id: 'pay-003',
    date: '2026-03-13',
    service: '물리치료',
    provider: '김도현 물리치료사',
    period: '2026년 3월 1~13일',
    total: 135000,
    copay: 27000,
    insurance: 108000,
    status: 'pending',
    statusLabel: '청구 예정',
    method: '-',
  },
];

const statusBadge: Record<string, string> = {
  paid: 'bg-green-50 text-green-700 border border-green-200',
  pending: 'bg-yellow-50 text-yellow-700 border border-yellow-200',
  failed: 'bg-red-50 text-red-700 border border-red-200',
};

function formatKRW(amount: number) {
  return amount.toLocaleString('ko-KR') + '원';
}

export default function PaymentsPage() {
  const totalPending = PAYMENTS.filter((p) => p.status === 'pending').reduce((s, p) => s + p.copay, 0);

  return (
    <FamilyAppShell>
      <div className="max-w-3xl mx-auto px-4 py-6">
        {/* Header */}
        <div className="mb-6">
          <h1 className="text-2xl font-bold text-gray-900">결제 내역</h1>
          <p className="text-sm text-gray-500 mt-1">케어 서비스 이용에 따른 결제 이력</p>
        </div>

        {/* Summary Cards */}
        <div className="grid grid-cols-2 sm:grid-cols-3 gap-3 mb-6">
          <div className="bg-white border border-gray-200 rounded-xl p-4">
            <p className="text-xs text-gray-500 mb-1">이번 달 본인 부담</p>
            <p className="text-xl font-bold text-gray-900">{formatKRW(97400)}</p>
            <p className="text-xs text-gray-400 mt-0.5">3월 청구 예정 포함</p>
          </div>
          <div className="bg-white border border-gray-200 rounded-xl p-4">
            <p className="text-xs text-gray-500 mb-1">청구 예정</p>
            <p className="text-xl font-bold text-yellow-600">{formatKRW(totalPending)}</p>
            <p className="text-xs text-gray-400 mt-0.5">자동 결제 예정</p>
          </div>
          <div className="bg-white border border-gray-200 rounded-xl p-4 sm:col-span-1 col-span-2">
            <p className="text-xs text-gray-500 mb-1">공단 지원 합계</p>
            <p className="text-xl font-bold text-blue-600">
              {formatKRW(PAYMENTS.filter((p) => p.status === 'paid').reduce((s, p) => s + p.insurance, 0))}
            </p>
            <p className="text-xs text-gray-400 mt-0.5">2026년 누적</p>
          </div>
        </div>

        {/* Payments List */}
        <div className="space-y-3">
          {PAYMENTS.map((payment) => (
            <Link key={payment.id} href={`/payments/${payment.id}`}>
              <div className="bg-white border border-gray-200 rounded-xl p-4 hover:border-blue-300 transition-colors cursor-pointer">
                <div className="flex items-start justify-between gap-3 mb-3">
                  <div>
                    <p className="font-semibold text-gray-900">{payment.service}</p>
                    <p className="text-xs text-gray-500 mt-0.5">{payment.provider} · {payment.period}</p>
                  </div>
                  <span className={`flex-shrink-0 text-xs font-semibold px-2.5 py-1 rounded-full ${statusBadge[payment.status]}`}>
                    {payment.statusLabel}
                  </span>
                </div>

                <div className="grid grid-cols-3 gap-2 text-sm mb-3">
                  <div>
                    <p className="text-xs text-gray-400">총액</p>
                    <p className="font-medium text-gray-800">{formatKRW(payment.total)}</p>
                  </div>
                  <div>
                    <p className="text-xs text-gray-400">본인 부담</p>
                    <p className="font-bold text-blue-700">{formatKRW(payment.copay)}</p>
                  </div>
                  <div>
                    <p className="text-xs text-gray-400">공단 부담</p>
                    <p className="font-medium text-gray-600">{formatKRW(payment.insurance)}</p>
                  </div>
                </div>

                <div className="flex items-center justify-between">
                  <p className="text-xs text-gray-400">{payment.date} · {payment.method}</p>
                  <span className="text-xs text-blue-600">상세 보기 →</span>
                </div>
              </div>
            </Link>
          ))}
        </div>

        {/* Payment Method Section */}
        <div className="mt-6 bg-white border border-gray-200 rounded-xl p-4">
          <h2 className="font-semibold text-gray-800 mb-3">결제 수단</h2>
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-3">
              <span className="text-2xl">🏦</span>
              <div>
                <p className="text-sm font-medium text-gray-800">국민은행 자동이체</p>
                <p className="text-xs text-gray-500">계좌: 123-456-789012 (김가족)</p>
              </div>
            </div>
            <button className="text-sm text-blue-600 hover:underline">변경</button>
          </div>
        </div>
      </div>
    </FamilyAppShell>
  );
}
