// 새 예약 — New Appointment Form
// Large-touch-target form for booking a new medical appointment

'use client';

import { useState } from 'react';
import Link from 'next/link';
import SeniorAppShell from '@/components/SeniorAppShell';

const SPECIALTY_OPTIONS = [
  '내과',
  '외과',
  '정형외과',
  '안과',
  '이비인후과',
  '치과',
  '피부과',
  '심장내과',
  '신경과',
  '재활의학과',
  '기타',
];

const TIME_SLOTS = [
  '오전 9:00',
  '오전 9:30',
  '오전 10:00',
  '오전 10:30',
  '오전 11:00',
  '오전 11:30',
  '오후 1:00',
  '오후 1:30',
  '오후 2:00',
  '오후 2:30',
  '오후 3:00',
  '오후 3:30',
  '오후 4:00',
];

interface FormState {
  specialty: string;
  hospital: string;
  doctor: string;
  date: string;
  time: string;
  reason: string;
  notifyCaregiver: boolean;
  notes: string;
}

const INITIAL_FORM: FormState = {
  specialty: '',
  hospital: '',
  doctor: '',
  date: '',
  time: '',
  reason: '',
  notifyCaregiver: true,
  notes: '',
};

function FormField({
  label,
  required,
  children,
  hint,
}: {
  label: string;
  required?: boolean;
  children: React.ReactNode;
  hint?: string;
}) {
  return (
    <div className="mb-4">
      <label className="block text-senior-base font-bold text-gray-800 mb-2">
        {label}
        {required && <span className="text-danger-500 ml-1" aria-label="필수">*</span>}
      </label>
      {hint && <p className="text-senior-sm text-gray-500 mb-2">{hint}</p>}
      {children}
    </div>
  );
}

const inputClass =
  'w-full border-2 border-gray-300 rounded-xl px-4 py-3 text-senior-lg text-gray-900 ' +
  'focus:border-primary-500 focus:outline-none ' +
  'min-h-touch-senior placeholder:text-gray-400';

export default function NewAppointmentPage() {
  const [form, setForm] = useState<FormState>(INITIAL_FORM);
  const [submitted, setSubmitted] = useState(false);
  const [errors, setErrors] = useState<Partial<Record<keyof FormState, string>>>({});

  function update<K extends keyof FormState>(field: K, value: FormState[K]) {
    setForm((prev) => ({ ...prev, [field]: value }));
    setErrors((prev) => ({ ...prev, [field]: undefined }));
  }

  function validate(): boolean {
    const newErrors: Partial<Record<keyof FormState, string>> = {};
    if (!form.specialty) newErrors.specialty = '진료과를 선택해 주세요.';
    if (!form.hospital) newErrors.hospital = '병원명을 입력해 주세요.';
    if (!form.date) newErrors.date = '날짜를 선택해 주세요.';
    if (!form.time) newErrors.time = '시간을 선택해 주세요.';
    if (!form.reason) newErrors.reason = '방문 이유를 입력해 주세요.';
    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  }

  function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    if (validate()) {
      // In production: POST to API, then redirect
      setSubmitted(true);
    }
  }

  if (submitted) {
    return (
      <SeniorAppShell>
        <div className="page-content flex flex-col items-center justify-center min-h-[60vh] text-center">
          <div className="text-6xl mb-4" aria-hidden="true">✅</div>
          <h1 className="text-senior-2xl font-bold text-gray-900 mb-2">예약이 완료됐습니다!</h1>
          <p className="text-senior-lg text-gray-600 mb-2">{form.hospital}</p>
          <p className="text-senior-base text-gray-500 mb-8">{form.date} {form.time}</p>
          {form.notifyCaregiver && (
            <p className="text-senior-sm text-success-700 bg-success-50 rounded-xl px-4 py-2 mb-6">
              보호자에게 알림이 전송됐습니다.
            </p>
          )}
          <Link href="/appointments" className="senior-btn-primary">
            예약 목록으로 돌아가기
          </Link>
        </div>
      </SeniorAppShell>
    );
  }

  return (
    <SeniorAppShell>
      <div className="page-content">
        <Link
          href="/appointments"
          className="inline-flex items-center gap-2 text-primary-600 font-medium text-senior-base mb-5 min-h-touch"
        >
          <svg className="w-5 h-5" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24" aria-hidden="true">
            <path strokeLinecap="round" strokeLinejoin="round" d="M15 19l-7-7 7-7" />
          </svg>
          돌아가기
        </Link>

        <h1 className="text-senior-2xl font-bold text-gray-900 mb-6">새 진료 예약</h1>

        <form onSubmit={handleSubmit} noValidate>
          {/* Specialty */}
          <FormField label="진료과" required>
            <select
              value={form.specialty}
              onChange={(e) => update('specialty', e.target.value)}
              className={`${inputClass} ${errors.specialty ? 'border-danger-500' : ''}`}
              aria-invalid={!!errors.specialty}
            >
              <option value="">진료과를 선택하세요</option>
              {SPECIALTY_OPTIONS.map((s) => (
                <option key={s} value={s}>{s}</option>
              ))}
            </select>
            {errors.specialty && (
              <p className="mt-1 text-senior-sm text-danger-600" role="alert">{errors.specialty}</p>
            )}
          </FormField>

          {/* Hospital */}
          <FormField label="병원명" required>
            <input
              type="text"
              value={form.hospital}
              onChange={(e) => update('hospital', e.target.value)}
              placeholder="예: 서울 중앙 의원"
              className={`${inputClass} ${errors.hospital ? 'border-danger-500' : ''}`}
              aria-invalid={!!errors.hospital}
            />
            {errors.hospital && (
              <p className="mt-1 text-senior-sm text-danger-600" role="alert">{errors.hospital}</p>
            )}
          </FormField>

          {/* Doctor (optional) */}
          <FormField label="담당 의사 (선택)">
            <input
              type="text"
              value={form.doctor}
              onChange={(e) => update('doctor', e.target.value)}
              placeholder="예: 김민준 원장"
              className={inputClass}
            />
          </FormField>

          {/* Date */}
          <FormField label="날짜" required>
            <input
              type="date"
              value={form.date}
              onChange={(e) => update('date', e.target.value)}
              min={new Date().toISOString().split('T')[0]}
              className={`${inputClass} ${errors.date ? 'border-danger-500' : ''}`}
              aria-invalid={!!errors.date}
            />
            {errors.date && (
              <p className="mt-1 text-senior-sm text-danger-600" role="alert">{errors.date}</p>
            )}
          </FormField>

          {/* Time */}
          <FormField label="시간" required>
            <div className="grid grid-cols-3 gap-2">
              {TIME_SLOTS.map((slot) => (
                <button
                  key={slot}
                  type="button"
                  onClick={() => update('time', slot)}
                  className={`py-2.5 rounded-xl text-senior-sm font-semibold border-2 transition-colors min-h-touch
                    ${form.time === slot
                      ? 'bg-primary-600 border-primary-600 text-white'
                      : 'bg-white border-gray-300 text-gray-700 hover:border-primary-400'
                    }`}
                >
                  {slot}
                </button>
              ))}
            </div>
            {errors.time && (
              <p className="mt-1 text-senior-sm text-danger-600" role="alert">{errors.time}</p>
            )}
          </FormField>

          {/* Reason */}
          <FormField label="방문 이유" required hint="예: 혈압 관리, 당뇨 검진, 무릎 통증">
            <textarea
              value={form.reason}
              onChange={(e) => update('reason', e.target.value)}
              rows={3}
              placeholder="방문 이유를 간단히 적어주세요"
              className={`${inputClass} h-auto resize-none leading-relaxed ${errors.reason ? 'border-danger-500' : ''}`}
              aria-invalid={!!errors.reason}
            />
            {errors.reason && (
              <p className="mt-1 text-senior-sm text-danger-600" role="alert">{errors.reason}</p>
            )}
          </FormField>

          {/* Notes */}
          <FormField label="추가 메모 (선택)">
            <textarea
              value={form.notes}
              onChange={(e) => update('notes', e.target.value)}
              rows={2}
              placeholder="특별한 준비사항이나 메모"
              className="w-full border-2 border-gray-300 rounded-xl px-4 py-3 text-senior-lg text-gray-900 focus:border-primary-500 focus:outline-none h-auto resize-none leading-relaxed"
            />
          </FormField>

          {/* Notify caregiver */}
          <div className="senior-card mb-6 flex items-center gap-4">
            <div className="flex-1">
              <p className="text-senior-base font-bold text-gray-800">보호자에게 알리기</p>
              <p className="text-senior-sm text-gray-500">예약 확인 후 보호자에게 알림 전송</p>
            </div>
            <button
              type="button"
              role="switch"
              aria-checked={form.notifyCaregiver}
              onClick={() => update('notifyCaregiver', !form.notifyCaregiver)}
              className={`relative inline-flex w-14 h-8 rounded-full transition-colors duration-200 flex-shrink-0
                ${form.notifyCaregiver ? 'bg-primary-600' : 'bg-gray-300'}`}
            >
              <span
                className={`absolute top-1 w-6 h-6 rounded-full bg-white shadow transition-transform duration-200
                  ${form.notifyCaregiver ? 'translate-x-7' : 'translate-x-1'}`}
              />
            </button>
          </div>

          {/* Submit */}
          <button type="submit" className="senior-btn-primary w-full text-senior-xl py-4">
            예약 신청하기
          </button>
        </form>
      </div>
    </SeniorAppShell>
  );
}
