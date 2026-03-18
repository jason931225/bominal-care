import Link from 'next/link';
import FamilyAppShell from '@/components/FamilyAppShell';

const EMERGENCY_NUMBER = {
  label: '긴급 신고',
  number: '119',
  description: '화재, 구급, 구조 신고 (24시간)',
  color: 'red',
};

const HOSPITAL_CONTACT = {
  label: '담당 병원',
  name: '강남성모병원 내과',
  number: '02-1234-5678',
  doctor: '박의사',
  description: '담당 주치의 연결',
  color: 'blue',
};

const CAREGIVER_CONTACT = {
  label: '담당 요양보호사',
  name: '박미영 요양보호사',
  number: '010-1234-5678',
  org: '행복케어 복지센터',
  description: '현재 담당 케어 제공자',
  color: 'green',
};

const FAMILY_MEMBERS = [
  { name: '김가족 (본인)', relation: '자녀', number: '010-9876-5432', primary: true },
  { name: '김형제', relation: '자녀', number: '010-5555-1234', primary: false },
  { name: '박배우자', relation: '며느리', number: '010-3333-7890', primary: false },
];

const dialButtonStyle: Record<string, string> = {
  red: 'bg-red-600 hover:bg-red-700 text-white',
  blue: 'bg-blue-600 hover:bg-blue-700 text-white',
  green: 'bg-green-600 hover:bg-green-700 text-white',
};

const cardBorderStyle: Record<string, string> = {
  red: 'border-red-200 bg-red-50',
  blue: 'border-blue-200 bg-blue-50',
  green: 'border-green-200 bg-green-50',
};

export default function EmergencyContactsPage() {
  return (
    <FamilyAppShell>
      <div className="max-w-3xl mx-auto px-4 py-6">
        {/* Breadcrumb */}
        <nav className="flex items-center gap-2 text-sm text-gray-500 mb-4">
          <Link href="/help-senior" className="hover:text-blue-600">대리 서비스</Link>
          <span>/</span>
          <span className="text-gray-900 font-medium">긴급 연락처</span>
        </nav>

        {/* Header */}
        <div className="mb-6">
          <h1 className="text-2xl font-bold text-gray-900">긴급 연락처</h1>
          <p className="text-sm text-gray-500 mt-1">
            김복순 어머님 관련 긴급 연락처 목록입니다
          </p>
        </div>

        {/* Emergency Warning */}
        <div className="bg-red-50 border border-red-200 rounded-xl p-4 flex items-start gap-3 mb-6">
          <span className="text-2xl flex-shrink-0">🚨</span>
          <div>
            <p className="text-sm font-semibold text-red-800">응급 상황 시 즉시 119에 연락하세요</p>
            <p className="text-sm text-red-700 mt-0.5">
              의식 불명, 호흡 곤란, 낙상 사고 등 응급 상황에는 119에 먼저 연락한 후
              담당 요양보호사 및 가족에게 알려주세요.
            </p>
          </div>
        </div>

        {/* Emergency 119 */}
        <div className={`border-2 rounded-xl p-5 mb-4 ${cardBorderStyle[EMERGENCY_NUMBER.color]}`}>
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-4">
              <div className="w-14 h-14 bg-red-100 rounded-full flex items-center justify-center">
                <span className="text-3xl">🚑</span>
              </div>
              <div>
                <p className="text-xs font-semibold text-red-600 uppercase tracking-wide">{EMERGENCY_NUMBER.label}</p>
                <p className="text-3xl font-black text-red-700">{EMERGENCY_NUMBER.number}</p>
                <p className="text-xs text-red-600 mt-0.5">{EMERGENCY_NUMBER.description}</p>
              </div>
            </div>
            <a
              href={`tel:${EMERGENCY_NUMBER.number}`}
              className={`flex items-center gap-2 px-6 py-3 rounded-xl text-lg font-bold transition-colors shadow-md ${dialButtonStyle[EMERGENCY_NUMBER.color]}`}
            >
              <span>📞</span>
              전화 걸기
            </a>
          </div>
        </div>

        {/* Hospital Contact */}
        <div className={`border rounded-xl p-5 mb-4 ${cardBorderStyle[HOSPITAL_CONTACT.color]}`}>
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-4">
              <div className="w-12 h-12 bg-blue-100 rounded-full flex items-center justify-center">
                <span className="text-2xl">🏥</span>
              </div>
              <div>
                <p className="text-xs font-semibold text-blue-600 uppercase tracking-wide">{HOSPITAL_CONTACT.label}</p>
                <p className="text-lg font-bold text-gray-900">{HOSPITAL_CONTACT.name}</p>
                <p className="text-sm text-gray-600">담당의: {HOSPITAL_CONTACT.doctor} · {HOSPITAL_CONTACT.number}</p>
              </div>
            </div>
            <a
              href={`tel:${HOSPITAL_CONTACT.number}`}
              className={`flex items-center gap-2 px-5 py-2.5 rounded-xl text-sm font-bold transition-colors ${dialButtonStyle[HOSPITAL_CONTACT.color]}`}
            >
              <span>📞</span>
              전화 걸기
            </a>
          </div>
        </div>

        {/* Caregiver Contact */}
        <div className={`border rounded-xl p-5 mb-8 ${cardBorderStyle[CAREGIVER_CONTACT.color]}`}>
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-4">
              <div className="w-12 h-12 bg-green-100 rounded-full flex items-center justify-center">
                <span className="text-2xl">👩‍⚕️</span>
              </div>
              <div>
                <p className="text-xs font-semibold text-green-600 uppercase tracking-wide">{CAREGIVER_CONTACT.label}</p>
                <p className="text-lg font-bold text-gray-900">{CAREGIVER_CONTACT.name}</p>
                <p className="text-sm text-gray-600">{CAREGIVER_CONTACT.org} · {CAREGIVER_CONTACT.number}</p>
              </div>
            </div>
            <a
              href={`tel:${CAREGIVER_CONTACT.number}`}
              className={`flex items-center gap-2 px-5 py-2.5 rounded-xl text-sm font-bold transition-colors ${dialButtonStyle[CAREGIVER_CONTACT.color]}`}
            >
              <span>📞</span>
              전화 걸기
            </a>
          </div>
        </div>

        {/* Family Members */}
        <div>
          <h2 className="text-lg font-bold text-gray-900 mb-4">가족 연락처</h2>
          <div className="bg-white border border-gray-200 rounded-xl divide-y divide-gray-100">
            {FAMILY_MEMBERS.map((member) => (
              <div key={member.number} className="flex items-center justify-between p-4">
                <div className="flex items-center gap-3">
                  <div className="w-10 h-10 bg-gray-100 rounded-full flex items-center justify-center">
                    <span className="text-lg">👤</span>
                  </div>
                  <div>
                    <div className="flex items-center gap-2">
                      <p className="font-semibold text-gray-900 text-sm">{member.name}</p>
                      {member.primary && (
                        <span className="text-xs bg-blue-100 text-blue-700 px-2 py-0.5 rounded-full font-medium">
                          주 연락처
                        </span>
                      )}
                    </div>
                    <p className="text-xs text-gray-500">{member.relation} · {member.number}</p>
                  </div>
                </div>
                <a
                  href={`tel:${member.number}`}
                  className="flex items-center gap-1.5 px-4 py-2 bg-gray-100 hover:bg-gray-200 text-gray-700 rounded-lg text-sm font-medium transition-colors"
                >
                  <span>📞</span>
                  전화
                </a>
              </div>
            ))}
          </div>
        </div>
      </div>
    </FamilyAppShell>
  );
}
