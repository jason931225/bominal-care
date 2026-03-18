'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import CaregiverAppShell from '@/components/CaregiverAppShell';

const CLIENTS = [
  { id: 'c001', name: '이순자' },
  { id: 'c002', name: '박영철' },
  { id: 'c003', name: '최말순' },
];

const INCIDENT_TYPES = [
  { id: 'fall', label: '낙상', icon: '⚠️', severity: 'high' },
  { id: 'medication_error', label: '투약 오류', icon: '💊', severity: 'high' },
  { id: 'injury', label: '부상/상처', icon: '🩹', severity: 'high' },
  { id: 'behavioral', label: '이상 행동', icon: '🧠', severity: 'medium' },
  { id: 'vital_abnormal', label: '활력징후 이상', icon: '🩺', severity: 'high' },
  { id: 'complaint', label: '이용자 불만', icon: '💬', severity: 'low' },
  { id: 'property', label: '분실/파손', icon: '🔑', severity: 'medium' },
  { id: 'emergency', label: '응급 상황', icon: '🚑', severity: 'critical' },
  { id: 'other', label: '기타', icon: '📋', severity: 'low' },
];

const SEVERITY_CONFIG: Record<string, { label: string; color: string; bgColor: string }> = {
  critical: { label: '즉각 대응 필요', color: 'text-red-700', bgColor: 'bg-red-100 border-red-300' },
  high: { label: '긴급', color: 'text-orange-600', bgColor: 'bg-orange-100 border-orange-300' },
  medium: { label: '보통', color: 'text-amber-600', bgColor: 'bg-amber-100 border-amber-300' },
  low: { label: '낮음', color: 'text-slate-600', bgColor: 'bg-slate-100 border-slate-300' },
};

export default function IncidentReportPage() {
  const router = useRouter();
  const [form, setForm] = useState({
    clientId: '',
    incidentType: '',
    incidentDate: new Date().toISOString().split('T')[0],
    incidentTime: '',
    location: '',
    description: '',
    immediateAction: '',
    familyNotified: false,
    familyNotifiedTime: '',
    medicalAttention: false,
    medicalDetail: '',
    witnesses: '',
    followUp: '',
  });
  const [submitting, setSubmitting] = useState(false);

  const update = <K extends keyof typeof form>(key: K, value: (typeof form)[K]) => {
    setForm((prev) => ({ ...prev, [key]: value }));
  };

  const selectedType = INCIDENT_TYPES.find((t) => t.id === form.incidentType);
  const severity = selectedType ? SEVERITY_CONFIG[INCIDENT_TYPES.find((t) => t.id === form.incidentType)?.severity ?? 'low'] : null;

  const isValid = form.clientId && form.incidentType && form.description.trim().length > 20 && form.immediateAction.trim().length > 5;

  const handleSubmit = async () => {
    if (!isValid) return;
    setSubmitting(true);
    await new Promise((r) => setTimeout(r, 1500));
    router.push('/notes');
  };

  return (
    <CaregiverAppShell
      activeTab="tasks"
      title="사고 보고서"
      showBackButton
      backHref="/notes"
    >
      <div className="px-4 py-4 space-y-5">
        {/* Warning Header */}
        <div className="bg-red-50 border border-red-200 rounded-2xl p-4">
          <div className="flex items-start gap-3">
            <span className="text-2xl flex-shrink-0">🚨</span>
            <div>
              <p className="text-sm font-bold text-red-700">사고 발생 시 즉시 보고하세요</p>
              <p className="text-xs text-red-500 mt-0.5 leading-relaxed">
                응급 상황이라면 먼저 119에 신고하고, 보호자에게 연락한 후 보고서를 작성하세요.
              </p>
            </div>
          </div>
          <a href="tel:119" className="mt-3 flex items-center justify-center gap-2 bg-red-600 text-white font-bold text-sm py-3 rounded-xl active:scale-95 transition-transform">
            <svg className="w-5 h-5" fill="none" stroke="currentColor" strokeWidth={2.5} viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z" />
            </svg>
            119 응급 신고
          </a>
        </div>

        {/* Basic Info */}
        <div className="card space-y-4">
          <h3 className="section-title">기본 정보</h3>
          <div>
            <label className="label">이용자 *</label>
            <select className="input-field" value={form.clientId} onChange={(e) => update('clientId', e.target.value)}>
              <option value="">선택</option>
              {CLIENTS.map((c) => <option key={c.id} value={c.id}>{c.name} 어르신</option>)}
            </select>
          </div>
          <div className="grid grid-cols-2 gap-3">
            <div>
              <label className="label">사고 날짜 *</label>
              <input type="date" className="input-field" value={form.incidentDate} onChange={(e) => update('incidentDate', e.target.value)} />
            </div>
            <div>
              <label className="label">사고 시간 *</label>
              <input type="time" className="input-field" value={form.incidentTime} onChange={(e) => update('incidentTime', e.target.value)} />
            </div>
          </div>
          <div>
            <label className="label">사고 장소 *</label>
            <input className="input-field" placeholder="예: 화장실, 침실, 거실..." value={form.location} onChange={(e) => update('location', e.target.value)} />
          </div>
        </div>

        {/* Incident Type */}
        <div className="card">
          <h3 className="section-title">사고 유형 *</h3>
          <div className="grid grid-cols-3 gap-2">
            {INCIDENT_TYPES.map((type) => {
              const isSelected = form.incidentType === type.id;
              const sev = SEVERITY_CONFIG[type.severity];
              return (
                <button
                  key={type.id}
                  type="button"
                  onClick={() => update('incidentType', type.id)}
                  className={`p-3 rounded-xl border text-center transition-colors ${
                    isSelected
                      ? `${sev.bgColor} border-current`
                      : 'bg-white border-slate-200'
                  }`}
                >
                  <div className="text-xl mb-1">{type.icon}</div>
                  <p className={`text-xs font-medium ${isSelected ? sev.color : 'text-slate-600'}`}>{type.label}</p>
                </button>
              );
            })}
          </div>
          {severity && selectedType && (
            <div className={`mt-3 p-3 rounded-xl border ${severity.bgColor}`}>
              <p className={`text-xs font-semibold ${severity.color}`}>
                {selectedType.icon} {selectedType.label} — {severity.label}
              </p>
            </div>
          )}
        </div>

        {/* Description */}
        <div className="card space-y-4">
          <div>
            <label className="label">사고 경위 *</label>
            <textarea
              className="input-field resize-none"
              rows={5}
              placeholder="언제, 어디서, 어떻게 사고가 발생했는지 상세히 기술해 주세요. (20자 이상)"
              value={form.description}
              onChange={(e) => update('description', e.target.value)}
            />
          </div>
          <div>
            <label className="label">즉각 조치 내용 *</label>
            <textarea
              className="input-field resize-none"
              rows={3}
              placeholder="사고 발생 후 즉시 취한 조치를 기술해 주세요."
              value={form.immediateAction}
              onChange={(e) => update('immediateAction', e.target.value)}
            />
          </div>
        </div>

        {/* Notifications */}
        <div className="card space-y-4">
          <h3 className="section-title">연락 및 후속 조치</h3>

          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-slate-800">보호자 연락 완료</p>
              <p className="text-xs text-slate-400">사고 후 보호자에게 연락했나요?</p>
            </div>
            <button
              type="button"
              onClick={() => update('familyNotified', !form.familyNotified)}
              className={`relative w-12 h-6 rounded-full transition-colors ${form.familyNotified ? 'bg-blue-600' : 'bg-slate-200'}`}
            >
              <div className={`absolute top-0.5 w-5 h-5 bg-white rounded-full shadow transition-transform ${form.familyNotified ? 'translate-x-6' : 'translate-x-0.5'}`} />
            </button>
          </div>

          {form.familyNotified && (
            <div>
              <label className="label">연락 시간</label>
              <input type="time" className="input-field" value={form.familyNotifiedTime} onChange={(e) => update('familyNotifiedTime', e.target.value)} />
            </div>
          )}

          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-slate-800">의료 조치 필요</p>
              <p className="text-xs text-slate-400">병원 방문 또는 치료가 필요한가요?</p>
            </div>
            <button
              type="button"
              onClick={() => update('medicalAttention', !form.medicalAttention)}
              className={`relative w-12 h-6 rounded-full transition-colors ${form.medicalAttention ? 'bg-red-500' : 'bg-slate-200'}`}
            >
              <div className={`absolute top-0.5 w-5 h-5 bg-white rounded-full shadow transition-transform ${form.medicalAttention ? 'translate-x-6' : 'translate-x-0.5'}`} />
            </button>
          </div>

          {form.medicalAttention && (
            <div>
              <label className="label">의료 조치 내용</label>
              <textarea className="input-field resize-none" rows={2} placeholder="처치 내용, 병원 방문 여부 등..." value={form.medicalDetail} onChange={(e) => update('medicalDetail', e.target.value)} />
            </div>
          )}

          <div>
            <label className="label">목격자</label>
            <input className="input-field" placeholder="목격자 이름 및 관계 (없으면 '없음')" value={form.witnesses} onChange={(e) => update('witnesses', e.target.value)} />
          </div>

          <div>
            <label className="label">향후 조치 계획</label>
            <textarea className="input-field resize-none" rows={3} placeholder="재발 방지를 위한 계획, 추가 조치 사항..." value={form.followUp} onChange={(e) => update('followUp', e.target.value)} />
          </div>
        </div>

        {/* Submit */}
        <div className="pb-6 space-y-3">
          {!isValid && (
            <p className="text-xs text-center text-red-500">필수 항목을 모두 입력해 주세요.</p>
          )}
          <button
            type="button"
            onClick={handleSubmit}
            disabled={!isValid || submitting}
            className="w-full bg-red-600 text-white font-semibold py-4 px-6 rounded-2xl text-base active:scale-95 transition-transform disabled:opacity-50 flex items-center justify-center gap-2"
          >
            {submitting ? (
              <>
                <svg className="w-4 h-4 animate-spin" fill="none" viewBox="0 0 24 24">
                  <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
                  <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
                </svg>
                제출 중...
              </>
            ) : '사고 보고서 제출'}
          </button>
          <button type="button" onClick={() => router.back()} className="btn-secondary">취소</button>
        </div>
      </div>
    </CaregiverAppShell>
  );
}
