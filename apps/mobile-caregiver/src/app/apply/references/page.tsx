'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import ApplicantAppShell from '@/components/ApplicantAppShell';

interface Reference {
  id: string;
  name: string;
  relationship: string;
  phone: string;
  organization: string;
}

const EMPTY_REF = (): Reference => ({
  id: Math.random().toString(36).slice(2),
  name: '',
  relationship: '',
  phone: '',
  organization: '',
});

const RELATIONSHIPS = ['이전 고용주', '동료 요양보호사', '의료기관 관계자', '지인', '기타'];

export default function ReferencesPage() {
  const router = useRouter();
  const [refs, setRefs] = useState<Reference[]>([EMPTY_REF()]);
  const [expanded, setExpanded] = useState<string>(refs[0].id);

  const updateRef = (id: string, field: keyof Reference, value: string) => {
    setRefs((prev) =>
      prev.map((r) => (r.id === id ? { ...r, [field]: value } : r))
    );
  };

  const addRef = () => {
    if (refs.length >= 3) return;
    const newRef = EMPTY_REF();
    setRefs((prev) => [...prev, newRef]);
    setExpanded(newRef.id);
  };

  const removeRef = (id: string) => {
    if (refs.length === 1) return;
    setRefs((prev) => prev.filter((r) => r.id !== id));
  };

  const formatPhone = (value: string) => {
    const digits = value.replace(/\D/g, '').slice(0, 11);
    if (digits.length <= 3) return digits;
    if (digits.length <= 7) return `${digits.slice(0, 3)}-${digits.slice(3)}`;
    return `${digits.slice(0, 3)}-${digits.slice(3, 7)}-${digits.slice(7)}`;
  };

  const isRefValid = (r: Reference) => r.name && r.phone.length >= 12 && r.relationship;

  return (
    <ApplicantAppShell currentStep={6} title="추천인 정보">
      <div className="px-4 py-6 space-y-4">
        <div>
          <h2 className="text-lg font-bold text-slate-900 mb-1">추천인을 입력해 주세요</h2>
          <p className="text-sm text-slate-500">선택사항이며, 최대 3명까지 등록할 수 있습니다.</p>
        </div>

        {/* Info notice */}
        <div className="bg-blue-50 rounded-xl p-4 border border-blue-100">
          <p className="text-xs text-blue-700 leading-relaxed">
            추천인이 있으면 심사가 빨라질 수 있습니다. 추천인에게는 연락하여 확인 과정이 진행됩니다.
          </p>
        </div>

        {/* Reference Cards */}
        <div className="space-y-3">
          {refs.map((ref, idx) => {
            const isOpen = expanded === ref.id;
            const isValid = isRefValid(ref);
            return (
              <div key={ref.id} className="card">
                {/* Accordion Header */}
                <button
                  type="button"
                  onClick={() => setExpanded(isOpen ? '' : ref.id)}
                  className="w-full flex items-center justify-between"
                >
                  <div className="flex items-center gap-3">
                    <div className={`w-8 h-8 rounded-full flex items-center justify-center text-sm font-bold ${
                      isValid ? 'bg-green-100 text-green-700' : 'bg-slate-100 text-slate-500'
                    }`}>
                      {idx + 1}
                    </div>
                    <div className="text-left">
                      <p className={`text-sm font-semibold ${ref.name ? 'text-slate-800' : 'text-slate-400'}`}>
                        {ref.name || `추천인 ${idx + 1}`}
                      </p>
                      {ref.relationship && (
                        <p className="text-xs text-slate-400">{ref.relationship}</p>
                      )}
                    </div>
                  </div>
                  <div className="flex items-center gap-2">
                    {isValid && (
                      <span className="badge-success">완료</span>
                    )}
                    <svg
                      className={`w-4 h-4 text-slate-400 transition-transform ${isOpen ? 'rotate-180' : ''}`}
                      fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24"
                    >
                      <path strokeLinecap="round" strokeLinejoin="round" d="M19 9l-7 7-7-7" />
                    </svg>
                  </div>
                </button>

                {/* Accordion Body */}
                {isOpen && (
                  <div className="mt-4 space-y-3 pt-4 border-t border-slate-100">
                    <div>
                      <label className="label">이름 *</label>
                      <input className="input-field" type="text" placeholder="홍길동" value={ref.name}
                        onChange={(e) => updateRef(ref.id, 'name', e.target.value)} />
                    </div>
                    <div>
                      <label className="label">관계 *</label>
                      <select className="input-field" value={ref.relationship}
                        onChange={(e) => updateRef(ref.id, 'relationship', e.target.value)}>
                        <option value="">선택해주세요</option>
                        {RELATIONSHIPS.map((r) => <option key={r} value={r}>{r}</option>)}
                      </select>
                    </div>
                    <div>
                      <label className="label">연락처 *</label>
                      <input className="input-field" type="tel" placeholder="010-0000-0000" inputMode="tel"
                        value={ref.phone}
                        onChange={(e) => updateRef(ref.id, 'phone', formatPhone(e.target.value))} />
                    </div>
                    <div>
                      <label className="label">소속 기관</label>
                      <input className="input-field" type="text" placeholder="요양원, 병원, 복지관 등"
                        value={ref.organization}
                        onChange={(e) => updateRef(ref.id, 'organization', e.target.value)} />
                    </div>

                    {refs.length > 1 && (
                      <button type="button" onClick={() => removeRef(ref.id)}
                        className="btn-danger text-sm py-3">
                        이 추천인 삭제
                      </button>
                    )}
                  </div>
                )}
              </div>
            );
          })}
        </div>

        {/* Add Reference */}
        {refs.length < 3 && (
          <button type="button" onClick={addRef}
            className="w-full flex items-center justify-center gap-2 py-4 border-2 border-dashed border-slate-200 rounded-2xl text-slate-500 font-medium text-sm active:bg-slate-50">
            <svg className="w-5 h-5" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" d="M12 4v16m8-8H4" />
            </svg>
            추천인 추가 ({refs.length}/3)
          </button>
        )}

        <div className="pb-6 space-y-3">
          <button type="button" onClick={() => router.push('/apply/review')} className="btn-primary">
            다음 단계로
          </button>
          <button type="button" onClick={() => router.push('/apply/review')}
            className="w-full text-center text-sm text-slate-400 py-2">
            추천인 없이 계속하기
          </button>
        </div>
      </div>
    </ApplicantAppShell>
  );
}
