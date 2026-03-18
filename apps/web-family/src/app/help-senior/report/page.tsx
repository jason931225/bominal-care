'use client';

import Link from 'next/link';
import { useState } from 'react';
import FamilyAppShell from '@/components/FamilyAppShell';

const REPORT_TYPES = [
  { id: 'weekly', label: '주간 케어 보고서', description: '한 주간의 방문 요양, 건강 상태, 특이사항 요약' },
  { id: 'monthly', label: '월간 케어 보고서', description: '한 달간의 전반적인 건강 추이 및 서비스 이용 현황' },
  { id: 'medication', label: '복약 이행 보고서', description: '처방약 복용 현황 및 부작용 관찰 보고' },
  { id: 'health', label: '건강 상태 보고서', description: '활력 징후, 인지 기능, 생활 활동 능력 평가' },
  { id: 'custom', label: '맞춤 보고서', description: '원하시는 내용을 직접 지정하여 요청' },
];

export default function CareReportRequestPage() {
  const [reportType, setReportType] = useState('');
  const [startDate, setStartDate] = useState('');
  const [endDate, setEndDate] = useState('');
  const [notes, setNotes] = useState('');
  const [submitted, setSubmitted] = useState(false);

  const selectedReport = REPORT_TYPES.find((r) => r.id === reportType);

  const canSubmit = reportType !== '' && startDate !== '' && endDate !== '';

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!canSubmit) return;
    setSubmitted(true);
  };

  if (submitted) {
    return (
      <FamilyAppShell>
        <div className="max-w-lg mx-auto px-4 py-16 text-center">
          <span className="text-6xl block mb-4">📋</span>
          <h2 className="text-2xl font-bold text-gray-900 mb-2">보고서 요청 완료</h2>
          <p className="text-gray-500 mb-2">
            케어매니저에게 보고서 요청이 전달되었습니다
          </p>
          <div className="bg-gray-50 rounded-xl p-4 text-left mb-6 text-sm space-y-2">
            <p>
              <span className="text-gray-500">보고서 유형:</span>{' '}
              <span className="font-medium">{selectedReport?.label}</span>
            </p>
            <p>
              <span className="text-gray-500">기간:</span>{' '}
              <span className="font-medium">{startDate} ~ {endDate}</span>
            </p>
            {notes && (
              <p>
                <span className="text-gray-500">요청사항:</span>{' '}
                <span className="font-medium">{notes}</span>
              </p>
            )}
          </div>
          <p className="text-xs text-gray-400 mb-6">
            보고서는 보통 2~3 영업일 이내에 작성됩니다. 완료 시 알림을 보내드립니다.
          </p>
          <Link
            href="/help-senior"
            className="inline-flex items-center gap-2 px-5 py-2.5 bg-blue-600 text-white rounded-lg font-medium hover:bg-blue-700 transition-colors"
          >
            대리 서비스 홈으로
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
          <Link href="/help-senior" className="hover:text-blue-600">대리 서비스</Link>
          <span>/</span>
          <span className="text-gray-900 font-medium">케어 리포트 요청</span>
        </nav>

        {/* Header */}
        <div className="mb-6">
          <h1 className="text-2xl font-bold text-gray-900">케어 리포트 요청</h1>
          <p className="text-sm text-gray-500 mt-1">
            김복순 어머님의 케어 보고서를 요청합니다
          </p>
        </div>

        <form onSubmit={handleSubmit}>
          {/* Report Type Selector */}
          <div className="mb-6">
            <label className="block text-sm font-semibold text-gray-900 mb-3">
              보고서 유형 선택
            </label>
            <div className="space-y-2">
              {REPORT_TYPES.map((type) => (
                <button
                  key={type.id}
                  type="button"
                  onClick={() => setReportType(type.id)}
                  className={`w-full text-left p-4 rounded-xl border-2 transition-colors ${
                    reportType === type.id
                      ? 'border-purple-500 bg-purple-50'
                      : 'border-gray-200 bg-white hover:border-purple-300'
                  }`}
                >
                  <p className="font-semibold text-gray-900 text-sm">{type.label}</p>
                  <p className="text-xs text-gray-500 mt-0.5">{type.description}</p>
                </button>
              ))}
            </div>
          </div>

          {/* Date Range */}
          <div className="mb-6">
            <label className="block text-sm font-semibold text-gray-900 mb-3">
              보고 기간
            </label>
            <div className="grid grid-cols-2 gap-3">
              <div>
                <label className="block text-xs text-gray-500 mb-1">시작일</label>
                <input
                  type="date"
                  value={startDate}
                  onChange={(e) => setStartDate(e.target.value)}
                  className="w-full px-3 py-2.5 border border-gray-300 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-purple-500"
                />
              </div>
              <div>
                <label className="block text-xs text-gray-500 mb-1">종료일</label>
                <input
                  type="date"
                  value={endDate}
                  onChange={(e) => setEndDate(e.target.value)}
                  className="w-full px-3 py-2.5 border border-gray-300 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-purple-500"
                />
              </div>
            </div>
          </div>

          {/* Notes */}
          <div className="mb-6">
            <label className="block text-sm font-semibold text-gray-900 mb-3">
              추가 요청사항 (선택)
            </label>
            <textarea
              value={notes}
              onChange={(e) => setNotes(e.target.value)}
              rows={4}
              placeholder="보고서에 포함하고 싶은 내용이나 특별히 확인하고 싶은 사항을 입력하세요..."
              className="w-full px-3 py-2.5 border border-gray-300 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-purple-500 resize-none"
            />
          </div>

          {/* Info notice */}
          <div className="bg-purple-50 border border-purple-200 rounded-xl p-3 mb-6">
            <p className="text-xs text-purple-700">
              요청된 보고서는 담당 케어매니저(최지원)에게 전달됩니다.
              보고서 작성 완료 시 알림을 통해 안내드립니다.
            </p>
          </div>

          {/* Submit */}
          <button
            type="submit"
            disabled={!canSubmit}
            className="w-full py-3 bg-purple-600 text-white font-semibold rounded-xl hover:bg-purple-700 transition-colors disabled:bg-gray-200 disabled:text-gray-400 disabled:cursor-not-allowed"
          >
            보고서 요청하기
          </button>
        </form>
      </div>
    </FamilyAppShell>
  );
}
