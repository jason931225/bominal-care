'use client';

import Link from 'next/link';
import { useState } from 'react';
import FamilyAppShell from '@/components/FamilyAppShell';

const STEPS = ['신청자 정보', '어르신 정보', '신청 유형', '서류 제출', '확인'];

const APPLICATION_TYPES = [
  {
    id: 'renewal',
    label: '갱신 신청',
    desc: '현재 인정 유효기간 만료 전 갱신',
    icon: '🔄',
  },
  {
    id: 'upgrade',
    label: '등급 변경 신청',
    desc: '상태 악화로 인한 상위 등급 신청',
    icon: '⬆️',
  },
  {
    id: 'new',
    label: '신규 신청',
    desc: '처음으로 장기요양 인정 신청',
    icon: '📝',
  },
];

const REQUIRED_DOCS = [
  { id: 'id', label: '신청인 신분증 사본', required: true },
  { id: 'medical', label: '의사 소견서 (발급 6개월 이내)', required: true },
  { id: 'power', label: '가족관계증명서 (대리 신청 시)', required: true },
  { id: 'resident', label: '주민등록등본 (최근 3개월)', required: false },
];

export default function EligibilityApplyPage() {
  const [step, setStep] = useState(0);
  const [appType, setAppType] = useState('renewal');
  const [uploadedDocs, setUploadedDocs] = useState<string[]>([]);
  const [submitted, setSubmitted] = useState(false);

  const [form, setForm] = useState({
    applicantName: '김가족',
    applicantPhone: '010-9999-8888',
    applicantRelation: '자녀',
    seniorName: '김복순',
    seniorBirth: '1948-05-12',
    seniorResidentId: '480512-*******',
    seniorAddress: '서울특별시 강남구 테헤란로 123',
    symptoms: '',
  });

  const updateForm = (field: string, value: string) => {
    setForm((prev) => ({ ...prev, [field]: value }));
  };

  const toggleDoc = (docId: string) => {
    setUploadedDocs((prev) =>
      prev.includes(docId) ? prev.filter((d) => d !== docId) : [...prev, docId]
    );
  };

  if (submitted) {
    return (
      <FamilyAppShell>
        <div className="max-w-lg mx-auto px-4 py-16 text-center">
          <span className="text-6xl block mb-4">✅</span>
          <h2 className="text-2xl font-bold text-gray-900 mb-2">신청 완료</h2>
          <p className="text-gray-500 mb-2">장기요양 인정 신청이 접수되었습니다</p>
          <div className="bg-gray-50 rounded-xl p-4 text-sm text-left space-y-2 mb-6">
            <p><span className="text-gray-500">접수번호:</span> <span className="font-semibold">APP-2026-031500</span></p>
            <p><span className="text-gray-500">신청 유형:</span> <span className="font-semibold">갱신 신청</span></p>
            <p><span className="text-gray-500">처리 예상:</span> <span className="font-semibold">영업일 30일 이내</span></p>
          </div>
          <p className="text-sm text-gray-500 mb-6">
            국민건강보험공단에서 담당자가 방문 조사를 진행합니다. 연락을 기다려 주세요.
          </p>
          <Link
            href="/eligibility"
            className="inline-flex items-center gap-2 px-5 py-2.5 bg-blue-600 text-white rounded-lg font-medium hover:bg-blue-700 transition-colors"
          >
            수급 현황으로 돌아가기
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
          <Link href="/eligibility" className="hover:text-blue-600">수급 자격</Link>
          <span>/</span>
          <span className="text-gray-900 font-medium">인정 신청</span>
        </nav>

        <h1 className="text-2xl font-bold text-gray-900 mb-6">장기요양 인정 신청</h1>

        {/* Step Indicator */}
        <div className="flex items-center gap-1 mb-6 overflow-x-auto pb-1">
          {STEPS.map((s, i) => (
            <div key={s} className="flex items-center gap-1 flex-shrink-0">
              <div className={`w-6 h-6 rounded-full flex items-center justify-center text-xs font-bold ${
                step > i ? 'bg-blue-600 text-white' :
                step === i ? 'bg-blue-600 text-white ring-2 ring-blue-200' :
                'bg-gray-200 text-gray-500'
              }`}>
                {step > i ? '✓' : i + 1}
              </div>
              <span className={`text-xs ${step >= i ? 'text-blue-600 font-medium' : 'text-gray-400'}`}>
                {s}
              </span>
              {i < STEPS.length - 1 && <div className={`w-4 h-0.5 ${step > i ? 'bg-blue-400' : 'bg-gray-200'}`} />}
            </div>
          ))}
        </div>

        {/* Step 0: Applicant Info */}
        {step === 0 && (
          <div className="space-y-4">
            <h2 className="font-bold text-gray-900">신청인 정보</h2>
            <div className="grid sm:grid-cols-2 gap-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1.5">성명</label>
                <input type="text" value={form.applicantName} onChange={(e) => updateForm('applicantName', e.target.value)}
                  className="w-full px-3 py-2.5 border border-gray-300 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-blue-500" />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1.5">연락처</label>
                <input type="tel" value={form.applicantPhone} onChange={(e) => updateForm('applicantPhone', e.target.value)}
                  className="w-full px-3 py-2.5 border border-gray-300 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-blue-500" />
              </div>
              <div className="sm:col-span-2">
                <label className="block text-sm font-medium text-gray-700 mb-1.5">어르신과의 관계</label>
                <select value={form.applicantRelation} onChange={(e) => updateForm('applicantRelation', e.target.value)}
                  className="w-full px-3 py-2.5 border border-gray-300 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white">
                  {['자녀', '배우자', '형제자매', '기타 가족', '법정 후견인'].map((r) => (
                    <option key={r}>{r}</option>
                  ))}
                </select>
              </div>
            </div>
          </div>
        )}

        {/* Step 1: Senior Info */}
        {step === 1 && (
          <div className="space-y-4">
            <h2 className="font-bold text-gray-900">수급 대상자(어르신) 정보</h2>
            <div className="grid sm:grid-cols-2 gap-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1.5">성명</label>
                <input type="text" value={form.seniorName} onChange={(e) => updateForm('seniorName', e.target.value)}
                  className="w-full px-3 py-2.5 border border-gray-300 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-blue-500" />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1.5">생년월일</label>
                <input type="date" value={form.seniorBirth} onChange={(e) => updateForm('seniorBirth', e.target.value)}
                  className="w-full px-3 py-2.5 border border-gray-300 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-blue-500" />
              </div>
              <div className="sm:col-span-2">
                <label className="block text-sm font-medium text-gray-700 mb-1.5">주민등록번호</label>
                <input type="text" value={form.seniorResidentId} onChange={(e) => updateForm('seniorResidentId', e.target.value)}
                  placeholder="000000-0000000"
                  className="w-full px-3 py-2.5 border border-gray-300 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-blue-500" />
              </div>
              <div className="sm:col-span-2">
                <label className="block text-sm font-medium text-gray-700 mb-1.5">주소</label>
                <input type="text" value={form.seniorAddress} onChange={(e) => updateForm('seniorAddress', e.target.value)}
                  className="w-full px-3 py-2.5 border border-gray-300 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-blue-500" />
              </div>
            </div>
          </div>
        )}

        {/* Step 2: App Type */}
        {step === 2 && (
          <div className="space-y-4">
            <h2 className="font-bold text-gray-900">신청 유형 선택</h2>
            {APPLICATION_TYPES.map((type) => (
              <button
                key={type.id}
                onClick={() => setAppType(type.id)}
                className={`w-full flex items-start gap-4 p-4 rounded-xl border-2 text-left transition-colors ${
                  appType === type.id ? 'border-blue-500 bg-blue-50' : 'border-gray-200 bg-white hover:border-blue-300'
                }`}
              >
                <span className="text-3xl flex-shrink-0">{type.icon}</span>
                <div>
                  <p className="font-semibold text-gray-900">{type.label}</p>
                  <p className="text-sm text-gray-500 mt-0.5">{type.desc}</p>
                </div>
                {appType === type.id && <span className="ml-auto text-blue-600 font-bold">✓</span>}
              </button>
            ))}
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1.5">증상 및 현 상태 (선택)</label>
              <textarea
                value={form.symptoms}
                onChange={(e) => updateForm('symptoms', e.target.value)}
                rows={3}
                placeholder="현재 어르신의 상태나 신청 사유를 간략히 적어주세요..."
                className="w-full px-3 py-2 border border-gray-300 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 resize-none"
              />
            </div>
          </div>
        )}

        {/* Step 3: Documents */}
        {step === 3 && (
          <div className="space-y-4">
            <h2 className="font-bold text-gray-900">서류 제출</h2>
            <p className="text-sm text-gray-500">필수 서류를 업로드하거나 제출 예정으로 표시하세요</p>
            <div className="space-y-3">
              {REQUIRED_DOCS.map((doc) => (
                <div key={doc.id} className="flex items-center justify-between p-4 bg-white border border-gray-200 rounded-xl">
                  <div className="flex items-center gap-3">
                    <span className="text-xl">📄</span>
                    <div>
                      <p className="text-sm font-medium text-gray-800">{doc.label}</p>
                      <p className="text-xs text-gray-500">{doc.required ? '필수' : '선택'}</p>
                    </div>
                  </div>
                  <button
                    onClick={() => toggleDoc(doc.id)}
                    className={`px-3 py-1.5 rounded-lg text-xs font-semibold transition-colors ${
                      uploadedDocs.includes(doc.id)
                        ? 'bg-green-100 text-green-700 border border-green-300'
                        : 'bg-gray-100 text-gray-600 border border-gray-300 hover:bg-blue-50'
                    }`}
                  >
                    {uploadedDocs.includes(doc.id) ? '✓ 제출 완료' : '업로드'}
                  </button>
                </div>
              ))}
            </div>
          </div>
        )}

        {/* Step 4: Confirm */}
        {step === 4 && (
          <div className="space-y-4">
            <h2 className="font-bold text-gray-900">신청 내용 확인</h2>
            <div className="bg-blue-50 border border-blue-200 rounded-xl p-5 space-y-3 text-sm">
              <div className="flex justify-between"><span className="text-blue-600">신청인</span><span className="font-medium">{form.applicantName} ({form.applicantRelation})</span></div>
              <div className="flex justify-between"><span className="text-blue-600">대상자</span><span className="font-medium">{form.seniorName}</span></div>
              <div className="flex justify-between"><span className="text-blue-600">신청 유형</span><span className="font-medium">{APPLICATION_TYPES.find((t) => t.id === appType)?.label}</span></div>
              <div className="flex justify-between"><span className="text-blue-600">제출 서류</span><span className="font-medium">{uploadedDocs.length}건</span></div>
            </div>
            <div className="bg-amber-50 border border-amber-200 rounded-xl p-3">
              <p className="text-xs text-amber-700">
                제출된 서류는 국민건강보험공단으로 전달됩니다. 방문 조사 일정은 7~14일 내 연락 예정입니다.
              </p>
            </div>
          </div>
        )}

        {/* Navigation */}
        <div className="flex gap-3 mt-8">
          {step > 0 && (
            <button
              onClick={() => setStep(step - 1)}
              className="flex-1 py-3 border border-gray-300 text-gray-700 font-semibold rounded-xl hover:bg-gray-50 transition-colors"
            >
              이전
            </button>
          )}
          <button
            onClick={() => step < STEPS.length - 1 ? setStep(step + 1) : setSubmitted(true)}
            className="flex-1 py-3 bg-blue-600 text-white font-semibold rounded-xl hover:bg-blue-700 transition-colors"
          >
            {step === STEPS.length - 1 ? '신청 제출' : '다음'}
          </button>
        </div>
      </div>
    </FamilyAppShell>
  );
}
