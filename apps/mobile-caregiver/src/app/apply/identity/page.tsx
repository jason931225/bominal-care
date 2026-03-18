'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import ApplicantAppShell from '@/components/ApplicantAppShell';

export default function IdentityPage() {
  const router = useRouter();
  const [form, setForm] = useState({
    name: '',
    birthDate: '',
    phone: '',
    verificationCode: '',
    idType: 'resident', // 'resident' | 'driver' | 'passport'
  });
  const [codeSent, setCodeSent] = useState(false);
  const [verified, setVerified] = useState(false);

  const handleChange = (field: string, value: string) => {
    setForm((prev) => ({ ...prev, [field]: value }));
  };

  const handleSendCode = () => {
    if (!form.phone || form.phone.length < 10) return;
    setCodeSent(true);
  };

  const handleVerifyCode = () => {
    if (form.verificationCode.length === 6) {
      setVerified(true);
    }
  };

  const handleNext = () => {
    if (verified) {
      router.push('/apply/credentials');
    }
  };

  const formatPhone = (value: string) => {
    const digits = value.replace(/\D/g, '').slice(0, 11);
    if (digits.length <= 3) return digits;
    if (digits.length <= 7) return `${digits.slice(0, 3)}-${digits.slice(3)}`;
    return `${digits.slice(0, 3)}-${digits.slice(3, 7)}-${digits.slice(7)}`;
  };

  const formatBirth = (value: string) => {
    const digits = value.replace(/\D/g, '').slice(0, 8);
    if (digits.length <= 4) return digits;
    if (digits.length <= 6) return `${digits.slice(0, 4)}.${digits.slice(4)}`;
    return `${digits.slice(0, 4)}.${digits.slice(4, 6)}.${digits.slice(6)}`;
  };

  const isFormValid = form.name && form.birthDate.length === 10 && verified;

  return (
    <ApplicantAppShell currentStep={1} title="본인 인증">
      <div className="px-4 py-6 space-y-6">
        <div>
          <h2 className="text-lg font-bold text-slate-900 mb-1">본인 정보를 입력해 주세요</h2>
          <p className="text-sm text-slate-500">입력하신 정보는 안전하게 암호화되어 보관됩니다.</p>
        </div>

        {/* Name */}
        <div>
          <label className="label">이름 *</label>
          <input
            className="input-field"
            type="text"
            placeholder="홍길동"
            value={form.name}
            onChange={(e) => handleChange('name', e.target.value)}
          />
        </div>

        {/* Birth Date */}
        <div>
          <label className="label">생년월일 *</label>
          <input
            className="input-field"
            type="text"
            placeholder="1985.03.15"
            inputMode="numeric"
            value={form.birthDate}
            onChange={(e) => handleChange('birthDate', formatBirth(e.target.value))}
          />
        </div>

        {/* ID Type */}
        <div>
          <label className="label">신분증 종류 *</label>
          <div className="grid grid-cols-3 gap-2">
            {[
              { value: 'resident', label: '주민등록증' },
              { value: 'driver', label: '운전면허증' },
              { value: 'passport', label: '여권' },
            ].map((opt) => (
              <button
                key={opt.value}
                type="button"
                onClick={() => handleChange('idType', opt.value)}
                className={`py-3 px-2 rounded-xl border text-sm font-medium transition-colors ${
                  form.idType === opt.value
                    ? 'border-blue-600 bg-blue-50 text-blue-700'
                    : 'border-slate-200 bg-white text-slate-600'
                }`}
              >
                {opt.label}
              </button>
            ))}
          </div>
        </div>

        {/* Phone */}
        <div>
          <label className="label">휴대폰 번호 *</label>
          <div className="flex gap-2">
            <input
              className="input-field flex-1"
              type="tel"
              placeholder="010-0000-0000"
              inputMode="tel"
              value={form.phone}
              onChange={(e) => handleChange('phone', formatPhone(e.target.value))}
            />
            <button
              type="button"
              onClick={handleSendCode}
              disabled={form.phone.length < 13}
              className="flex-shrink-0 px-4 py-3.5 bg-blue-600 text-white text-sm font-semibold rounded-xl disabled:opacity-40 active:scale-95 transition-all"
            >
              {codeSent ? '재전송' : '인증요청'}
            </button>
          </div>
          {codeSent && !verified && (
            <p className="text-xs text-blue-600 mt-1.5">
              인증번호가 발송되었습니다. (유효시간 3분)
            </p>
          )}
        </div>

        {/* Verification Code */}
        {codeSent && !verified && (
          <div>
            <label className="label">인증번호 *</label>
            <div className="flex gap-2">
              <input
                className="input-field flex-1"
                type="text"
                placeholder="6자리 입력"
                inputMode="numeric"
                maxLength={6}
                value={form.verificationCode}
                onChange={(e) => handleChange('verificationCode', e.target.value.replace(/\D/g, '').slice(0, 6))}
              />
              <button
                type="button"
                onClick={handleVerifyCode}
                disabled={form.verificationCode.length !== 6}
                className="flex-shrink-0 px-4 py-3.5 bg-slate-700 text-white text-sm font-semibold rounded-xl disabled:opacity-40 active:scale-95 transition-all"
              >
                확인
              </button>
            </div>
          </div>
        )}

        {/* Verified Badge */}
        {verified && (
          <div className="flex items-center gap-2 p-3.5 bg-green-50 rounded-xl border border-green-200">
            <svg className="w-5 h-5 text-green-600 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
              <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clipRule="evenodd" />
            </svg>
            <p className="text-sm text-green-700 font-medium">본인 인증이 완료되었습니다.</p>
          </div>
        )}

        {/* Privacy Notice */}
        <div className="bg-slate-50 rounded-xl p-4">
          <p className="text-xs text-slate-500 leading-relaxed">
            개인정보는 요양보호사 등록 및 서비스 제공 목적으로만 사용되며,
            관계 법령에 따라 안전하게 관리됩니다.
            자세한 내용은 <span className="text-blue-600 underline">개인정보처리방침</span>을 확인해 주세요.
          </p>
        </div>

        {/* Next Button */}
        <div className="pb-6">
          <button
            type="button"
            onClick={handleNext}
            disabled={!isFormValid}
            className="btn-primary"
          >
            다음 단계로
          </button>
        </div>
      </div>
    </ApplicantAppShell>
  );
}
