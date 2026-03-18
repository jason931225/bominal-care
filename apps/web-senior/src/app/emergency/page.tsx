// 긴급 연락 — Emergency Contacts & Quick Dial
// Critical page: large touch targets, prominent phone numbers, clear layout

import SeniorAppShell from '@/components/SeniorAppShell';

// Emergency and personal contacts — always available
const EMERGENCY_CONTACTS = [
  {
    id: 'ec-1',
    name: '119 응급',
    subtitle: '화재·구조·응급 구급',
    phone: '119',
    color: 'bg-danger-500 hover:bg-danger-700',
    textColor: 'text-white',
    icon: '🚑',
    priority: 1,
  },
  {
    id: 'ec-2',
    name: '112 경찰',
    subtitle: '사건·사고·범죄 신고',
    phone: '112',
    color: 'bg-blue-600 hover:bg-blue-800',
    textColor: 'text-white',
    icon: '🚔',
    priority: 2,
  },
  {
    id: 'ec-3',
    name: '129 복지 위기',
    subtitle: '자살 예방 · 복지 긴급 상담',
    phone: '129',
    color: 'bg-secondary-600 hover:bg-secondary-700',
    textColor: 'text-white',
    icon: '💙',
    priority: 3,
  },
];

const PERSONAL_CONTACTS = [
  {
    id: 'pc-1',
    name: '큰딸 — 김지은',
    relation: '가족',
    phone: '010-2345-6789',
    icon: '👩',
  },
  {
    id: 'pc-2',
    name: '큰아들 — 김민수',
    relation: '가족',
    phone: '010-3456-7890',
    icon: '👨',
  },
  {
    id: 'pc-3',
    name: '케어매니저 이수진',
    relation: '케어',
    phone: '010-9876-5432',
    icon: '👩‍⚕️',
  },
  {
    id: 'pc-4',
    name: '담당의 김민준',
    relation: '의료',
    phone: '02-1234-5678',
    icon: '👨‍⚕️',
  },
  {
    id: 'pc-5',
    name: '이웃 박순자',
    relation: '이웃',
    phone: '010-4567-8901',
    icon: '🏠',
  },
];

const HEALTH_INFO = [
  { label: '혈액형', value: 'A형' },
  { label: '알레르기', value: '페니실린 계열 항생제' },
  { label: '만성 질환', value: '고혈압, 제2형 당뇨' },
  { label: '현재 복용 약', value: '암로디핀, 메트포르민, 아스피린' },
  { label: '병원', value: '서울 중앙 의원 (02-1234-5678)' },
];

export default function EmergencyPage() {
  return (
    <SeniorAppShell>
      <div className="page-content">
        <h1 className="text-senior-2xl font-bold text-gray-900 mb-2">긴급 연락</h1>
        <p className="text-senior-base text-gray-500 mb-5">아래 버튼을 누르면 바로 전화가 연결됩니다</p>

        {/* Emergency call buttons — extra large */}
        <section aria-labelledby="emergency-numbers-heading" className="mb-6">
          <h2 id="emergency-numbers-heading" className="senior-section-title">긴급 신고 전화</h2>
          <div className="space-y-3">
            {EMERGENCY_CONTACTS.map((contact) => (
              <a
                key={contact.id}
                href={`tel:${contact.phone}`}
                className={`${contact.color} ${contact.textColor} rounded-2xl px-5 py-4 flex items-center gap-4 shadow-md transition-colors active:scale-[0.98]`}
                aria-label={`${contact.name} ${contact.phone} 전화 연결`}
              >
                <span className="text-4xl flex-shrink-0" aria-hidden="true">{contact.icon}</span>
                <div className="flex-1">
                  <p className="text-2xl font-black tracking-wide">{contact.phone}</p>
                  <p className="text-senior-lg font-bold">{contact.name}</p>
                  <p className="text-senior-sm opacity-80">{contact.subtitle}</p>
                </div>
                <svg className="w-8 h-8 opacity-80 flex-shrink-0" fill="none" stroke="currentColor" strokeWidth={2.5} viewBox="0 0 24 24" aria-hidden="true">
                  <path strokeLinecap="round" strokeLinejoin="round" d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.948V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z" />
                </svg>
              </a>
            ))}
          </div>
        </section>

        {/* Personal emergency contacts */}
        <section aria-labelledby="personal-contacts-heading" className="mb-6">
          <h2 id="personal-contacts-heading" className="senior-section-title">개인 비상 연락처</h2>
          <div className="space-y-2">
            {PERSONAL_CONTACTS.map((contact) => (
              <a
                key={contact.id}
                href={`tel:${contact.phone}`}
                className="senior-card flex items-center gap-4 hover:shadow-md active:scale-[0.99] transition-all"
                aria-label={`${contact.name} ${contact.phone}`}
              >
                <span className="text-3xl flex-shrink-0" aria-hidden="true">{contact.icon}</span>
                <div className="flex-1">
                  <p className="text-senior-lg font-bold text-gray-900">{contact.name}</p>
                  <p className="text-senior-base text-primary-600 font-medium">{contact.phone}</p>
                  <span className="text-senior-sm text-gray-500">{contact.relation}</span>
                </div>
                <div className="w-10 h-10 rounded-full bg-primary-50 flex items-center justify-center flex-shrink-0" aria-hidden="true">
                  <svg className="w-5 h-5 text-primary-600" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.948V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z" />
                  </svg>
                </div>
              </a>
            ))}
          </div>
        </section>

        {/* Health information card for paramedics */}
        <section aria-labelledby="health-info-heading" className="mb-6">
          <h2 id="health-info-heading" className="senior-section-title">의료 정보 (응급 대응용)</h2>
          <div className="senior-card bg-danger-50 border-danger-200 border-2">
            <p className="text-senior-sm text-danger-700 font-bold mb-3">
              🚑 이 정보를 구급대원에게 보여주세요
            </p>
            <div className="space-y-2">
              {HEALTH_INFO.map(({ label, value }) => (
                <div key={label} className="flex items-start gap-2">
                  <span className="text-senior-sm text-danger-600 font-semibold w-24 flex-shrink-0">{label}</span>
                  <span className="text-senior-base font-bold text-danger-800">{value}</span>
                </div>
              ))}
            </div>
          </div>
        </section>
      </div>
    </SeniorAppShell>
  );
}
