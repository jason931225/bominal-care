import Link from 'next/link';
import FamilyAppShell from '@/components/FamilyAppShell';

const PAYMENT = {
  id: 'pay-001',
  date: '2026-03-10',
  service: '방문 요양 서비스',
  provider: '박미영 요양보호사',
  providerOrg: '행복케어 복지센터',
  period: '2026년 2월 (2026.02.01 ~ 02.28)',
  total: 487000,
  copay: 97400,
  insurance: 389600,
  status: 'paid',
  statusLabel: '결제 완료',
  method: '자동이체 (국민은행 123-456-789012)',
  receiptNo: 'RCP-2026-031042',
  items: [
    { desc: '방문 요양 (월~금)', sessions: 20, unitPrice: 22000, amount: 440000 },
    { desc: '토요일 방문 추가', sessions: 4, unitPrice: 11750, amount: 47000 },
  ],
  gradeInfo: {
    grade: '3등급',
    monthlyLimit: 1341000,
    usedAmount: 487000,
    usageRate: 36,
  },
};

function formatKRW(amount: number) {
  return amount.toLocaleString('ko-KR') + '원';
}

export default function PaymentDetailPage({
  params: _params,
}: {
  params: Promise<{ id: string }>;
}) {
  return (
    <FamilyAppShell>
      <div className="max-w-2xl mx-auto px-4 py-6">
        {/* Breadcrumb */}
        <nav className="flex items-center gap-2 text-sm text-gray-500 mb-4">
          <Link href="/payments" className="hover:text-blue-600">결제 내역</Link>
          <span>/</span>
          <span className="text-gray-900 font-medium">결제 상세</span>
        </nav>

        {/* Status Banner */}
        <div className="bg-green-50 border border-green-200 rounded-xl p-4 flex items-center gap-3 mb-5">
          <span className="text-3xl">✅</span>
          <div>
            <p className="font-bold text-green-800">결제 완료</p>
            <p className="text-sm text-green-600">{PAYMENT.date} 처리 · {PAYMENT.method}</p>
          </div>
        </div>

        {/* Receipt Header */}
        <div className="bg-white border border-gray-200 rounded-xl p-5 mb-4">
          <div className="flex items-start justify-between mb-4">
            <h1 className="text-lg font-bold text-gray-900">{PAYMENT.service}</h1>
            <span className="text-xs text-gray-400">{PAYMENT.receiptNo}</span>
          </div>

          <div className="space-y-2 text-sm mb-4">
            <div className="flex justify-between">
              <span className="text-gray-500">서비스 제공자</span>
              <span className="font-medium text-gray-800">{PAYMENT.provider}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-500">소속 기관</span>
              <span className="font-medium text-gray-800">{PAYMENT.providerOrg}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-500">서비스 기간</span>
              <span className="font-medium text-gray-800">{PAYMENT.period}</span>
            </div>
          </div>
        </div>

        {/* Itemized Bill */}
        <div className="bg-white border border-gray-200 rounded-xl p-5 mb-4">
          <h2 className="font-bold text-gray-900 mb-3">청구 내역</h2>
          <div className="space-y-2 mb-4">
            {PAYMENT.items.map((item) => (
              <div key={item.desc} className="flex items-center justify-between text-sm py-2 border-b border-gray-100 last:border-0">
                <div>
                  <p className="font-medium text-gray-800">{item.desc}</p>
                  <p className="text-xs text-gray-400">{item.sessions}회 × {formatKRW(item.unitPrice)}</p>
                </div>
                <span className="font-semibold text-gray-900">{formatKRW(item.amount)}</span>
              </div>
            ))}
          </div>

          {/* Totals */}
          <div className="bg-gray-50 rounded-lg p-3 space-y-2">
            <div className="flex justify-between text-sm">
              <span className="text-gray-600">서비스 합계</span>
              <span className="font-medium">{formatKRW(PAYMENT.total)}</span>
            </div>
            <div className="flex justify-between text-sm">
              <span className="text-gray-600">공단 부담 (80%)</span>
              <span className="font-medium text-blue-600">- {formatKRW(PAYMENT.insurance)}</span>
            </div>
            <div className="flex justify-between text-sm border-t border-gray-200 pt-2 mt-2">
              <span className="font-bold text-gray-900">본인 부담 (20%)</span>
              <span className="font-bold text-lg text-gray-900">{formatKRW(PAYMENT.copay)}</span>
            </div>
          </div>
        </div>

        {/* Grade Info */}
        <div className="bg-white border border-gray-200 rounded-xl p-5 mb-6">
          <h2 className="font-bold text-gray-900 mb-3">급여 한도 현황</h2>
          <div className="flex items-center justify-between text-sm mb-2">
            <span className="text-gray-500">노인장기요양 {PAYMENT.gradeInfo.grade}</span>
            <span className="text-gray-800 font-medium">
              {formatKRW(PAYMENT.gradeInfo.usedAmount)} / {formatKRW(PAYMENT.gradeInfo.monthlyLimit)}
            </span>
          </div>
          <div className="w-full bg-gray-100 rounded-full h-2.5 mb-1">
            <div
              className="bg-blue-500 h-2.5 rounded-full"
              style={{ width: `${PAYMENT.gradeInfo.usageRate}%` }}
            />
          </div>
          <p className="text-xs text-gray-400">
            이 기간 급여 한도 {PAYMENT.gradeInfo.usageRate}% 사용
          </p>
        </div>

        {/* Actions */}
        <div className="flex gap-3">
          <button className="flex-1 py-3 border border-gray-300 text-gray-700 text-sm font-semibold rounded-xl hover:bg-gray-50 transition-colors">
            영수증 다운로드
          </button>
          <button className="flex-1 py-3 bg-blue-600 text-white text-sm font-semibold rounded-xl hover:bg-blue-700 transition-colors">
            문의하기
          </button>
        </div>
      </div>
    </FamilyAppShell>
  );
}
