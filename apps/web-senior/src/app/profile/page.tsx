// 내 프로필 — User Profile
// Personal information, health baseline, and family contacts

import Link from 'next/link';
import SeniorAppShell from '@/components/SeniorAppShell';

const USER_PROFILE = {
  name: '김영자',
  nameEn: 'Kim Young-ja',
  birthDate: '1950년 5월 12일',
  age: 75,
  gender: '여성',
  phone: '010-1234-5678',
  address: '서울특별시 강남구 대치동 123-45 행복 아파트 302호',
  insuranceNumber: '500512-2xxxxxx',
  ltcLevel: '장기요양 3등급',
  ltcExpiry: '2027년 6월 30일',
  joinDate: '2025년 11월 1일',
  photo: null,
};

const HEALTH_BASELINE = [
  { label: '혈액형', value: 'A형 (RH+)' },
  { label: '키 / 체중', value: '155cm / 58kg' },
  { label: '만성 질환', value: '고혈압, 제2형 당뇨, 퇴행성 관절염' },
  { label: '알레르기', value: '페니실린 계열 항생제' },
  { label: '과거 수술', value: '백내장 수술 (2022년 우측), 무릎 주사치료 (2024년)' },
];

const FAMILY_CONTACTS = [
  { name: '김지은 (큰딸)', phone: '010-2345-6789', relation: '딸', isEmergency: true },
  { name: '김민수 (큰아들)', phone: '010-3456-7890', relation: '아들', isEmergency: true },
  { name: '이민철 (사위)', phone: '010-5678-9012', relation: '사위', isEmergency: false },
];

const MENU_ITEMS = [
  { href: '/consent', icon: '🔒', label: '동의 관리', description: '개인정보 및 서비스 동의' },
  { href: '/notifications', icon: '🔔', label: '알림 설정', description: '알림 수신 설정' },
  { href: '/settings', icon: '⚙️', label: '앱 설정', description: '글자 크기, 색상 대비' },
  { href: '/emergency', icon: '🚨', label: '긴급 연락처', description: '비상 연락처 관리' },
];

export default function ProfilePage() {
  return (
    <SeniorAppShell>
      <div className="page-content">
        {/* Profile header */}
        <div className="senior-card mb-5">
          <div className="flex items-center gap-4 mb-4">
            <div className="w-20 h-20 rounded-full bg-primary-100 border-4 border-primary-200 flex items-center justify-center text-4xl flex-shrink-0" aria-label="프로필 사진">
              👵
            </div>
            <div>
              <h1 className="text-senior-2xl font-bold text-gray-900">{USER_PROFILE.name}</h1>
              <p className="text-senior-base text-gray-600">{USER_PROFILE.age}세 · {USER_PROFILE.gender}</p>
              <p className="text-senior-sm text-gray-500">{USER_PROFILE.birthDate}</p>
            </div>
          </div>

          {/* LTC badge */}
          <div className="bg-secondary-50 border border-secondary-200 rounded-xl p-3 flex items-center gap-3">
            <span className="text-2xl" aria-hidden="true">🏥</span>
            <div>
              <p className="text-senior-base font-bold text-secondary-700">{USER_PROFILE.ltcLevel}</p>
              <p className="text-senior-sm text-secondary-600">유효기간: {USER_PROFILE.ltcExpiry}</p>
            </div>
          </div>
        </div>

        {/* Personal information */}
        <section className="senior-card mb-4" aria-labelledby="personal-info-heading">
          <div className="flex items-center justify-between mb-3">
            <h2 id="personal-info-heading" className="text-senior-lg font-bold text-gray-800">기본 정보</h2>
            <button className="text-senior-sm text-primary-600 font-medium min-h-touch flex items-center">
              수정
            </button>
          </div>
          <div className="space-y-2">
            {[
              { label: '연락처', value: USER_PROFILE.phone },
              { label: '주소', value: USER_PROFILE.address },
              { label: '건강보험번호', value: USER_PROFILE.insuranceNumber },
              { label: '가입일', value: USER_PROFILE.joinDate },
            ].map(({ label, value }) => (
              <div key={label} className="flex items-start gap-3">
                <span className="text-senior-sm text-gray-500 w-24 flex-shrink-0 pt-0.5">{label}</span>
                <span className="text-senior-base text-gray-800 flex-1">{value}</span>
              </div>
            ))}
          </div>
        </section>

        {/* Health baseline */}
        <section className="senior-card mb-4" aria-labelledby="health-info-heading">
          <div className="flex items-center justify-between mb-3">
            <h2 id="health-info-heading" className="text-senior-lg font-bold text-gray-800">건강 기본 정보</h2>
            <Link href="/medical-history" className="text-senior-sm text-primary-600 font-medium min-h-touch flex items-center">
              전체 기록
            </Link>
          </div>
          <div className="space-y-2">
            {HEALTH_BASELINE.map(({ label, value }) => (
              <div key={label} className="flex items-start gap-3">
                <span className="text-senior-sm text-gray-500 w-24 flex-shrink-0 pt-0.5">{label}</span>
                <span className="text-senior-base text-gray-800 flex-1">{value}</span>
              </div>
            ))}
          </div>
        </section>

        {/* Family contacts */}
        <section className="senior-card mb-4" aria-labelledby="family-heading">
          <h2 id="family-heading" className="text-senior-lg font-bold text-gray-800 mb-3">가족 및 보호자</h2>
          <div className="space-y-3">
            {FAMILY_CONTACTS.map((contact) => (
              <div key={contact.name} className="flex items-center gap-3">
                <span className="text-2xl" aria-hidden="true">
                  {contact.relation === '딸' ? '👩' : contact.relation === '아들' ? '👨' : '👤'}
                </span>
                <div className="flex-1">
                  <p className="text-senior-base font-semibold text-gray-800">{contact.name}</p>
                  <a href={`tel:${contact.phone}`} className="text-senior-sm text-primary-600 font-medium">
                    {contact.phone}
                  </a>
                </div>
                {contact.isEmergency && (
                  <span className="bg-danger-50 text-danger-600 text-senior-sm font-bold px-2 py-0.5 rounded-full">
                    비상
                  </span>
                )}
              </div>
            ))}
          </div>
        </section>

        {/* Account menu */}
        <section aria-labelledby="menu-heading">
          <h2 id="menu-heading" className="senior-section-title">계정 설정</h2>
          <div className="space-y-2">
            {MENU_ITEMS.map((item) => (
              <Link
                key={item.href}
                href={item.href}
                className="senior-card flex items-center gap-4 hover:shadow-md active:scale-[0.99] transition-all"
              >
                <span className="text-2xl" aria-hidden="true">{item.icon}</span>
                <div className="flex-1">
                  <p className="text-senior-base font-bold text-gray-800">{item.label}</p>
                  <p className="text-senior-sm text-gray-500">{item.description}</p>
                </div>
                <svg className="w-5 h-5 text-gray-400 flex-shrink-0" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24" aria-hidden="true">
                  <path strokeLinecap="round" strokeLinejoin="round" d="M9 5l7 7-7 7" />
                </svg>
              </Link>
            ))}
          </div>
        </section>

        {/* Logout */}
        <button className="w-full mt-5 py-3 text-center text-senior-base text-danger-600 font-semibold min-h-touch border-2 border-danger-200 rounded-2xl hover:bg-danger-50 transition-colors">
          로그아웃
        </button>
      </div>
    </SeniorAppShell>
  );
}
