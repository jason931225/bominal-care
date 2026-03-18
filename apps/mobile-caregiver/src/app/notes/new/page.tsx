'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import CaregiverAppShell from '@/components/CaregiverAppShell';

const CLIENTS = [
  { id: 'c001', name: '이순자' },
  { id: 'c002', name: '박영철' },
  { id: 'c003', name: '최말순' },
];

const SERVICES = ['목욕 지원', '식사 지원', '이동 지원', '가사 지원', '투약 보조', '배변 지원', '말벗 서비스', '인지활동', '상처 관리'];

const MOODS = [
  { emoji: '😢', label: '매우 나쁨', value: 1 },
  { emoji: '😕', label: '나쁨', value: 2 },
  { emoji: '😐', label: '보통', value: 3 },
  { emoji: '🙂', label: '좋음', value: 4 },
  { emoji: '😊', label: '매우 좋음', value: 5 },
];

export default function NewNotePage() {
  const router = useRouter();
  const [form, setForm] = useState({
    clientId: '',
    visitDate: new Date().toISOString().split('T')[0],
    startTime: '10:00',
    endTime: '13:00',
    selectedServices: new Set<string>(),
    moodScore: 0,
    content: '',
    specialNotes: '',
    bloodPressure: '',
    pulse: '',
    temperature: '',
    weight: '',
  });
  const [submitting, setSubmitting] = useState(false);

  const update = <K extends keyof typeof form>(key: K, value: (typeof form)[K]) => {
    setForm((prev) => ({ ...prev, [key]: value }));
  };

  const toggleService = (s: string) => {
    setForm((prev) => {
      const next = new Set(prev.selectedServices);
      if (next.has(s)) {
        next.delete(s);
      } else {
        next.add(s);
      }
      return { ...prev, selectedServices: next };
    });
  };

  const isValid = form.clientId && form.content.trim().length > 10;

  const handleSubmit = async () => {
    if (!isValid) return;
    setSubmitting(true);
    await new Promise((r) => setTimeout(r, 1200));
    router.push('/notes');
  };

  return (
    <CaregiverAppShell
      activeTab="tasks"
      title="케어일지 작성"
      showBackButton
      backHref="/notes"
    >
      <div className="px-4 py-4 space-y-5">
        {/* Client & Date */}
        <div className="card space-y-4">
          <div>
            <label className="label">이용자 *</label>
            <select
              className="input-field"
              value={form.clientId}
              onChange={(e) => update('clientId', e.target.value)}
            >
              <option value="">이용자를 선택해 주세요</option>
              {CLIENTS.map((c) => (
                <option key={c.id} value={c.id}>{c.name} 어르신</option>
              ))}
            </select>
          </div>
          <div>
            <label className="label">방문일 *</label>
            <input
              type="date"
              className="input-field"
              value={form.visitDate}
              onChange={(e) => update('visitDate', e.target.value)}
            />
          </div>
          <div className="grid grid-cols-2 gap-3">
            <div>
              <label className="label">시작 시간</label>
              <input
                type="time"
                className="input-field"
                value={form.startTime}
                onChange={(e) => update('startTime', e.target.value)}
              />
            </div>
            <div>
              <label className="label">종료 시간</label>
              <input
                type="time"
                className="input-field"
                value={form.endTime}
                onChange={(e) => update('endTime', e.target.value)}
              />
            </div>
          </div>
        </div>

        {/* Services */}
        <div className="card">
          <label className="label">제공 서비스</label>
          <div className="flex flex-wrap gap-2">
            {SERVICES.map((s) => {
              const active = form.selectedServices.has(s);
              return (
                <button
                  key={s}
                  type="button"
                  onClick={() => toggleService(s)}
                  className={`px-3 py-1.5 rounded-full text-sm font-medium border transition-colors ${
                    active
                      ? 'bg-blue-600 border-blue-600 text-white'
                      : 'bg-white border-slate-200 text-slate-600'
                  }`}
                >
                  {s}
                </button>
              );
            })}
          </div>
        </div>

        {/* Mood */}
        <div className="card">
          <label className="label">이용자 상태</label>
          <div className="flex justify-around">
            {MOODS.map((m) => (
              <button
                key={m.value}
                type="button"
                onClick={() => update('moodScore', m.value)}
                className={`flex flex-col items-center gap-1 p-2 rounded-xl transition-all ${
                  form.moodScore === m.value ? 'bg-blue-50 scale-110' : ''
                }`}
              >
                <span className="text-2xl">{m.emoji}</span>
                <span className="text-xs text-slate-400">{m.label}</span>
              </button>
            ))}
          </div>
        </div>

        {/* Vitals */}
        <div className="card">
          <label className="label">활력징후 (선택)</label>
          <div className="grid grid-cols-2 gap-3">
            <div>
              <label className="text-xs text-slate-500 mb-1 block">혈압 (수축/이완)</label>
              <input
                className="input-field"
                placeholder="128/82"
                value={form.bloodPressure}
                onChange={(e) => update('bloodPressure', e.target.value)}
              />
            </div>
            <div>
              <label className="text-xs text-slate-500 mb-1 block">맥박 (회/분)</label>
              <input
                className="input-field"
                placeholder="72"
                inputMode="numeric"
                value={form.pulse}
                onChange={(e) => update('pulse', e.target.value.replace(/\D/g, ''))}
              />
            </div>
            <div>
              <label className="text-xs text-slate-500 mb-1 block">체온 (°C)</label>
              <input
                className="input-field"
                placeholder="36.5"
                inputMode="decimal"
                value={form.temperature}
                onChange={(e) => update('temperature', e.target.value)}
              />
            </div>
            <div>
              <label className="text-xs text-slate-500 mb-1 block">체중 (kg)</label>
              <input
                className="input-field"
                placeholder="52"
                inputMode="numeric"
                value={form.weight}
                onChange={(e) => update('weight', e.target.value.replace(/\D/g, ''))}
              />
            </div>
          </div>
        </div>

        {/* Main Content */}
        <div className="card">
          <label className="label">케어 내용 *</label>
          <textarea
            className="input-field resize-none"
            rows={6}
            placeholder="오늘 제공한 케어 서비스 내용을 상세히 기록해 주세요.&#10;(최소 10자 이상)"
            value={form.content}
            onChange={(e) => update('content', e.target.value)}
          />
          <p className="text-xs text-slate-400 mt-1 text-right">{form.content.length}자</p>
        </div>

        {/* Special Notes */}
        <div className="card">
          <label className="label">특이사항</label>
          <textarea
            className="input-field resize-none"
            rows={3}
            placeholder="이상 증상, 가족 요청사항, 다음 방문 시 주의사항 등..."
            value={form.specialNotes}
            onChange={(e) => update('specialNotes', e.target.value)}
          />
        </div>

        {/* Submit */}
        <div className="pb-6 space-y-3">
          <button
            type="button"
            onClick={handleSubmit}
            disabled={!isValid || submitting}
            className="btn-primary flex items-center justify-center gap-2"
          >
            {submitting ? (
              <>
                <svg className="w-4 h-4 animate-spin" fill="none" viewBox="0 0 24 24">
                  <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
                  <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
                </svg>
                저장 중...
              </>
            ) : '케어일지 저장'}
          </button>
          <button type="button" onClick={() => router.back()} className="btn-secondary">
            취소
          </button>
        </div>
      </div>
    </CaregiverAppShell>
  );
}
