import Link from 'next/link';
import FamilyAppShell from '@/components/FamilyAppShell';

const DOCUMENT = {
  id: 'doc-001',
  category: '계약서',
  title: '방문 요양 서비스 계약서',
  status: 'active',
  statusLabel: '유효',
  date: '2026-01-01',
  expiresAt: '2026-03-31',
  signed: true,
  signedAt: '2026-01-01',
  signedBy: '김가족 (가족 대리인)',
  provider: '행복케어 복지센터',
  size: '1.2 MB',
  pages: 8,
  summary: [
    { label: '계약자', value: '김복순 (위임: 김가족)' },
    { label: '서비스 제공자', value: '행복케어 복지센터' },
    { label: '서비스 유형', value: '방문 요양 (재가급여)' },
    { label: '계약 기간', value: '2026.01.01 ~ 2026.03.31' },
    { label: '서비스 횟수', value: '주 5회 (월~금) 오전 8~11시' },
    { label: '월 청구 금액', value: '약 487,000원 (본인 부담 20%)' },
  ],
  keyTerms: [
    '서비스 변경 시 최소 7일 전 통보 필요',
    '천재지변, 질병 등 불가피한 사유 시 당일 취소 가능',
    '계약 해지 시 1개월 전 통보',
    '개인정보는 케어 서비스 목적 외 사용 불가',
    '분쟁 발생 시 관할 법원은 서울중앙지방법원',
  ],
};

export default function DocumentDetailPage({
  params: _params,
}: {
  params: Promise<{ id: string }>;
}) {
  return (
    <FamilyAppShell>
      <div className="max-w-2xl mx-auto px-4 py-6">
        {/* Breadcrumb */}
        <nav className="flex items-center gap-2 text-sm text-gray-500 mb-4">
          <Link href="/documents" className="hover:text-blue-600">문서 관리</Link>
          <span>/</span>
          <span className="text-gray-900 font-medium">{DOCUMENT.title}</span>
        </nav>

        {/* Header Card */}
        <div className="bg-white border border-gray-200 rounded-xl p-5 mb-4">
          <div className="flex items-start gap-4">
            <span className="text-4xl">📄</span>
            <div className="flex-1">
              <div className="flex items-center gap-2 flex-wrap mb-1">
                <span className="text-xs text-gray-500 font-medium">{DOCUMENT.category}</span>
                <span className="text-xs font-semibold px-2 py-0.5 rounded-full bg-green-50 text-green-700 border border-green-200">
                  {DOCUMENT.statusLabel}
                </span>
              </div>
              <h1 className="text-lg font-bold text-gray-900">{DOCUMENT.title}</h1>
              <p className="text-sm text-gray-500 mt-1">
                {DOCUMENT.provider} · {DOCUMENT.pages}페이지 · {DOCUMENT.size}
              </p>
              {DOCUMENT.signed && (
                <div className="flex items-center gap-1.5 mt-2">
                  <span className="text-green-500">✓</span>
                  <span className="text-xs text-green-700 font-medium">
                    {DOCUMENT.signedAt} {DOCUMENT.signedBy} 서명 완료
                  </span>
                </div>
              )}
            </div>
          </div>
        </div>

        {/* Contract Summary */}
        <div className="bg-white border border-gray-200 rounded-xl p-5 mb-4">
          <h2 className="font-bold text-gray-900 mb-3">계약 요약</h2>
          <div className="space-y-3">
            {DOCUMENT.summary.map((item) => (
              <div key={item.label} className="flex justify-between text-sm gap-3">
                <span className="text-gray-500 flex-shrink-0">{item.label}</span>
                <span className="font-medium text-gray-800 text-right">{item.value}</span>
              </div>
            ))}
          </div>
        </div>

        {/* Key Terms */}
        <div className="bg-white border border-gray-200 rounded-xl p-5 mb-4">
          <h2 className="font-bold text-gray-900 mb-3">주요 조항</h2>
          <ul className="space-y-2">
            {DOCUMENT.keyTerms.map((term) => (
              <li key={term} className="flex items-start gap-2 text-sm">
                <span className="text-blue-400 flex-shrink-0 mt-0.5">•</span>
                <span className="text-gray-700">{term}</span>
              </li>
            ))}
          </ul>
        </div>

        {/* Expiry Warning */}
        <div className="bg-amber-50 border border-amber-200 rounded-xl p-4 mb-6">
          <div className="flex items-center gap-2">
            <span>⏰</span>
            <p className="text-sm font-semibold text-amber-800">
              계약 만료일: {DOCUMENT.expiresAt} — 16일 후 만료 예정
            </p>
          </div>
          <p className="text-xs text-amber-600 mt-1 ml-6">갱신 절차를 진행하려면 담당 케어매니저에게 연락하세요</p>
        </div>

        {/* PDF Viewer Placeholder */}
        <div className="bg-gray-100 border-2 border-dashed border-gray-300 rounded-xl p-8 text-center mb-6">
          <span className="text-4xl block mb-2">📑</span>
          <p className="font-semibold text-gray-700">문서 미리보기</p>
          <p className="text-sm text-gray-500 mt-1 mb-4">PDF 뷰어 — {DOCUMENT.pages}페이지 계약서</p>
          <button className="px-4 py-2 bg-gray-200 text-gray-700 text-sm font-medium rounded-lg hover:bg-gray-300 transition-colors">
            PDF 열기
          </button>
        </div>

        {/* Actions */}
        <div className="flex gap-3">
          <button className="flex-1 py-3 border border-gray-300 text-gray-700 text-sm font-semibold rounded-xl hover:bg-gray-50 transition-colors">
            다운로드
          </button>
          <button className="flex-1 py-3 bg-blue-600 text-white text-sm font-semibold rounded-xl hover:bg-blue-700 transition-colors">
            공유
          </button>
        </div>
      </div>
    </FamilyAppShell>
  );
}
