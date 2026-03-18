'use client';

import Link from 'next/link';
import { useState } from 'react';
import FamilyAppShell from '@/components/FamilyAppShell';

const APPROVAL = {
  id: 'apv-001',
  title: '물리치료 횟수 증가 승인',
  description: '주 2회 → 주 4회 (낙상 후 재활 목적)',
  requestedBy: '케어매니저 최지원',
  requestedAt: '2026-03-13 오후 2:30',
  deadline: '2026-03-15',
  urgency: 'high',
  urgencyLabel: '긴급',
  status: 'pending',
  category: '서비스 변경',
  estimatedCost: '+월 120,000원 (본인 부담)',
  rationale:
    '지난 2월 14일 낙상 사고 이후 어르신의 하지 근력 저하가 관찰되고 있습니다. 현행 주 2회 물리치료로는 재활 목표 달성이 어렵다고 판단하여 주 4회로 증가를 요청드립니다. 3개월간 집중 재활 후 재평가 예정입니다.',
  supportingDocs: [
    { name: '낙상 사고 보고서 (2026.02.14)', icon: '📄' },
    { name: '물리치료사 소견서', icon: '📋' },
    { name: '주치의 권고 의견서', icon: '🏥' },
  ],
  costBreakdown: [
    { label: '현행 월 비용 (주 2회)', amount: '90,000원' },
    { label: '변경 후 월 비용 (주 4회)', amount: '210,000원' },
    { label: '본인 부담 증가분 (20%)', amount: '+24,000원/월' },
    { label: '공단 추가 부담', amount: '+96,000원/월' },
  ],
};

export default function ApprovalDetailPage({
  params: _params,
}: {
  params: Promise<{ id: string }>;
}) {
  const [decision, setDecision] = useState<'approve' | 'reject' | null>(null);
  const [note, setNote] = useState('');
  const [submitted, setSubmitted] = useState(false);

  const handleSubmit = () => {
    if (!decision) return;
    setSubmitted(true);
  };

  if (submitted) {
    return (
      <FamilyAppShell>
        <div className="max-w-lg mx-auto px-4 py-16 text-center">
          <span className="text-6xl block mb-4">{decision === 'approve' ? '✅' : '❌'}</span>
          <h2 className="text-2xl font-bold text-gray-900 mb-2">
            {decision === 'approve' ? '승인 완료' : '거부 완료'}
          </h2>
          <p className="text-gray-500 mb-6">
            {decision === 'approve'
              ? '케어매니저에게 승인 결과가 전달되었습니다'
              : '케어매니저에게 거부 사유가 전달되었습니다'}
          </p>
          <Link
            href="/approvals"
            className="inline-flex items-center gap-2 px-5 py-2.5 bg-blue-600 text-white rounded-lg font-medium hover:bg-blue-700 transition-colors"
          >
            승인 목록으로 돌아가기
          </Link>
        </div>
      </FamilyAppShell>
    );
  }

  return (
    <FamilyAppShell>
      <div className="max-w-2xl mx-auto px-4 py-6">
        {/* Breadcrumb */}
        <nav className="flex items-center gap-2 text-sm text-gray-500 mb-4">
          <Link href="/approvals" className="hover:text-blue-600">승인 관리</Link>
          <span>/</span>
          <span className="text-gray-900 font-medium">승인 요청 상세</span>
        </nav>

        {/* Urgency Banner */}
        <div className="bg-red-50 border border-red-200 rounded-xl p-3 flex items-center gap-2 mb-5">
          <span>🚨</span>
          <p className="text-sm font-semibold text-red-700">긴급 · 마감일: {APPROVAL.deadline} 까지 처리 필요</p>
        </div>

        {/* Header */}
        <div className="bg-white border border-gray-200 rounded-xl p-5 mb-4">
          <div className="flex items-center gap-2 mb-2">
            <span className="text-xs font-semibold px-2 py-0.5 rounded-full bg-red-50 text-red-700 border border-red-200">
              {APPROVAL.urgencyLabel}
            </span>
            <span className="text-xs text-gray-500">{APPROVAL.category}</span>
          </div>
          <h1 className="text-xl font-bold text-gray-900">{APPROVAL.title}</h1>
          <p className="text-sm text-gray-600 mt-1">{APPROVAL.description}</p>
          <p className="text-xs text-gray-400 mt-2">{APPROVAL.requestedBy} · {APPROVAL.requestedAt}</p>
        </div>

        {/* Rationale */}
        <div className="bg-white border border-gray-200 rounded-xl p-5 mb-4">
          <h2 className="font-bold text-gray-900 mb-2">요청 사유</h2>
          <p className="text-sm text-gray-700 leading-relaxed">{APPROVAL.rationale}</p>
        </div>

        {/* Cost Impact */}
        <div className="bg-white border border-gray-200 rounded-xl p-5 mb-4">
          <h2 className="font-bold text-gray-900 mb-3">비용 변동</h2>
          <div className="space-y-2">
            {APPROVAL.costBreakdown.map((item) => (
              <div key={item.label} className="flex justify-between text-sm">
                <span className="text-gray-600">{item.label}</span>
                <span className={`font-semibold ${item.amount.startsWith('+') ? 'text-red-600' : 'text-gray-800'}`}>
                  {item.amount}
                </span>
              </div>
            ))}
          </div>
        </div>

        {/* Supporting Documents */}
        <div className="bg-white border border-gray-200 rounded-xl p-5 mb-4">
          <h2 className="font-bold text-gray-900 mb-3">첨부 문서</h2>
          <div className="space-y-2">
            {APPROVAL.supportingDocs.map((doc) => (
              <div key={doc.name} className="flex items-center gap-2 p-2.5 bg-gray-50 rounded-lg hover:bg-gray-100 cursor-pointer">
                <span>{doc.icon}</span>
                <span className="text-sm text-blue-600 hover:underline">{doc.name}</span>
              </div>
            ))}
          </div>
        </div>

        {/* Decision */}
        <div className="bg-white border border-gray-200 rounded-xl p-5 mb-5">
          <h2 className="font-bold text-gray-900 mb-4">결정</h2>
          <div className="grid grid-cols-2 gap-3 mb-4">
            <button
              onClick={() => setDecision('approve')}
              className={`py-3 rounded-xl font-semibold text-sm border-2 transition-colors ${
                decision === 'approve'
                  ? 'bg-green-600 text-white border-green-600'
                  : 'bg-white text-green-700 border-green-300 hover:bg-green-50'
              }`}
            >
              ✓ 승인
            </button>
            <button
              onClick={() => setDecision('reject')}
              className={`py-3 rounded-xl font-semibold text-sm border-2 transition-colors ${
                decision === 'reject'
                  ? 'bg-red-600 text-white border-red-600'
                  : 'bg-white text-red-700 border-red-300 hover:bg-red-50'
              }`}
            >
              ✗ 거부
            </button>
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1.5">
              메모 (선택사항)
            </label>
            <textarea
              value={note}
              onChange={(e) => setNote(e.target.value)}
              rows={3}
              placeholder={
                decision === 'reject'
                  ? '거부 사유를 입력해 주세요...'
                  : '케어매니저에게 전달할 메시지를 입력하세요...'
              }
              className="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent resize-none"
            />
          </div>
        </div>

        <button
          onClick={handleSubmit}
          disabled={!decision}
          className={`w-full py-3 rounded-xl font-semibold text-sm transition-colors ${
            decision
              ? decision === 'approve'
                ? 'bg-green-600 text-white hover:bg-green-700'
                : 'bg-red-600 text-white hover:bg-red-700'
              : 'bg-gray-200 text-gray-400 cursor-not-allowed'
          }`}
        >
          {decision === 'approve' ? '승인 제출' : decision === 'reject' ? '거부 제출' : '결정을 선택하세요'}
        </button>
      </div>
    </FamilyAppShell>
  );
}
