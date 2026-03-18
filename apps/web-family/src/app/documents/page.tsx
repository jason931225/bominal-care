import Link from 'next/link';
import FamilyAppShell from '@/components/FamilyAppShell';

const DOCUMENTS = [
  {
    id: 'doc-001',
    category: '계약서',
    title: '방문 요양 서비스 계약서',
    description: '행복케어 복지센터 · 2026.01.01 ~ 2026.03.31',
    date: '2026-01-01',
    status: 'active',
    statusLabel: '유효',
    statusColor: 'green',
    icon: '📄',
    size: '1.2 MB',
    signed: true,
  },
  {
    id: 'doc-002',
    category: '동의서',
    title: '개인정보 수집·이용 동의서',
    description: '케어 서비스 전반에 걸친 개인정보 처리 동의',
    date: '2026-01-01',
    status: 'active',
    statusLabel: '유효',
    statusColor: 'green',
    icon: '✍️',
    size: '0.5 MB',
    signed: true,
  },
  {
    id: 'doc-003',
    category: '케어 계획서',
    title: '2026년 1분기 케어 계획서',
    description: '케어매니저 최지원 작성 · 가족 확인 완료',
    date: '2026-01-05',
    status: 'confirmed',
    statusLabel: '확인 완료',
    statusColor: 'blue',
    icon: '📋',
    size: '0.8 MB',
    signed: true,
  },
  {
    id: 'doc-004',
    category: '평가 보고서',
    title: '2월 월간 케어 보고서',
    description: '방문 기록, 건강 관찰 요약, 복약 이행률',
    date: '2026-03-05',
    status: 'new',
    statusLabel: '신규',
    statusColor: 'yellow',
    icon: '📊',
    size: '2.1 MB',
    signed: false,
  },
  {
    id: 'doc-005',
    category: '계약서',
    title: '계약 갱신 동의서 (미서명)',
    description: '2026.04.01 ~ 2026.06.30 갱신 계약',
    date: '2026-03-13',
    status: 'pending',
    statusLabel: '서명 필요',
    statusColor: 'red',
    icon: '⚠️',
    size: '1.1 MB',
    signed: false,
  },
  {
    id: 'doc-006',
    category: '진단서',
    title: '장기요양 인정 진단서',
    description: '보건복지부 발급 · 3등급 인정',
    date: '2025-08-20',
    status: 'active',
    statusLabel: '유효',
    statusColor: 'green',
    icon: '🏥',
    size: '0.7 MB',
    signed: false,
  },
];

const CATEGORIES = ['전체', '계약서', '동의서', '케어 계획서', '평가 보고서', '진단서'];

const statusBadge: Record<string, string> = {
  green: 'bg-green-50 text-green-700 border border-green-200',
  blue: 'bg-blue-50 text-blue-700 border border-blue-200',
  yellow: 'bg-yellow-50 text-yellow-700 border border-yellow-200',
  red: 'bg-red-50 text-red-700 border border-red-200',
};

export default function DocumentsPage() {
  const pendingDocs = DOCUMENTS.filter((d) => d.status === 'pending').length;

  return (
    <FamilyAppShell>
      <div className="max-w-3xl mx-auto px-4 py-6">
        {/* Header */}
        <div className="mb-6">
          <h1 className="text-2xl font-bold text-gray-900">문서 관리</h1>
          <p className="text-sm text-gray-500 mt-1">계약서, 동의서, 케어 보고서 등 모든 문서</p>
        </div>

        {/* Alert: signature needed */}
        {pendingDocs > 0 && (
          <div className="bg-red-50 border border-red-200 rounded-xl p-4 flex items-start gap-3 mb-5">
            <span className="text-xl flex-shrink-0">⚠️</span>
            <div>
              <p className="font-semibold text-red-800">서명이 필요한 문서가 {pendingDocs}건 있습니다</p>
              <p className="text-sm text-red-600 mt-0.5">계약 만료 전 서명을 완료해 주세요</p>
            </div>
          </div>
        )}

        {/* Category Filters */}
        <div className="flex gap-2 overflow-x-auto pb-2 mb-5">
          {CATEGORIES.map((cat) => (
            <button
              key={cat}
              className={`flex-shrink-0 px-3 py-1.5 rounded-full text-sm font-medium border transition-colors ${
                cat === '전체'
                  ? 'bg-blue-600 text-white border-blue-600'
                  : 'bg-white text-gray-600 border-gray-200 hover:border-blue-300'
              }`}
            >
              {cat}
            </button>
          ))}
        </div>

        {/* Documents Grid */}
        <div className="space-y-3">
          {DOCUMENTS.map((doc) => (
            <Link key={doc.id} href={`/documents/${doc.id}`}>
              <div className={`bg-white border rounded-xl p-4 hover:border-blue-300 transition-colors cursor-pointer ${
                doc.status === 'pending' ? 'border-red-200' : 'border-gray-200'
              }`}>
                <div className="flex items-start gap-3">
                  <span className="text-3xl flex-shrink-0">{doc.icon}</span>
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2 flex-wrap mb-1">
                      <span className="text-xs text-gray-400 font-medium">{doc.category}</span>
                      <span className={`text-xs font-semibold px-2 py-0.5 rounded-full ${statusBadge[doc.statusColor]}`}>
                        {doc.statusLabel}
                      </span>
                      {doc.signed && (
                        <span className="text-xs text-green-600 bg-green-50 px-2 py-0.5 rounded-full border border-green-200">
                          서명 완료
                        </span>
                      )}
                    </div>
                    <p className="font-semibold text-gray-900 text-sm">{doc.title}</p>
                    <p className="text-xs text-gray-500 mt-0.5">{doc.description}</p>
                    <div className="flex items-center gap-3 mt-2">
                      <span className="text-xs text-gray-400">{doc.date}</span>
                      <span className="text-xs text-gray-400">{doc.size}</span>
                    </div>
                  </div>
                  <div className="flex-shrink-0 flex flex-col items-end gap-2">
                    <span className="text-blue-500 text-sm">→</span>
                  </div>
                </div>
              </div>
            </Link>
          ))}
        </div>
      </div>
    </FamilyAppShell>
  );
}
