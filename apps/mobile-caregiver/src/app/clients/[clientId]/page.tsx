import Link from 'next/link';
import CaregiverAppShell from '@/components/CaregiverAppShell';

const CLIENT_DATA: Record<string, {
  id: string; name: string; age: number; gender: string; birthDate: string;
  address: string; addressDetail: string; phone: string;
  careLevel: number; primaryDiagnosis: string[]; allergies: string[];
  emergencyContact: { name: string; relationship: string; phone: string };
  caregiverNote: string; emoji: string;
  recentVitals: { date: string; bp: string; pulse: number; temp: number; weight: number };
  medications: { name: string; time: string; dose: string }[];
  upcomingVisits: { date: string; time: string; services: string[] }[];
}> = {
  c001: {
    id: 'c001',
    name: '이순자',
    age: 82,
    gender: '여',
    birthDate: '1944.05.12',
    address: '서울 강남구 대치동 123-45',
    addressDetail: '현대아파트 101동 1502호',
    phone: '010-2345-6789',
    careLevel: 3,
    primaryDiagnosis: ['고혈압', '경증 치매', '골다공증'],
    allergies: ['페니실린', '조개류'],
    emergencyContact: { name: '이민준', relationship: '아들', phone: '010-9876-5432' },
    caregiverNote: '낙상 위험이 높음. 화장실 이동 시 반드시 동행. 혈압약(암로디핀 5mg)은 아침 10시에 복용. 인지 저하로 인해 지시 반복 필요.',
    emoji: '👵',
    recentVitals: { date: '2026-03-13', bp: '128/82', pulse: 74, temp: 36.5, weight: 52 },
    medications: [
      { name: '암로디핀 5mg', time: '10:00', dose: '1정' },
      { name: '아리셉트 5mg', time: '21:00', dose: '1정' },
      { name: '칼슘 + 비타민D', time: '13:00', dose: '1정' },
    ],
    upcomingVisits: [
      { date: '3/15', time: '10:00–13:00', services: ['목욕', '식사'] },
      { date: '3/17', time: '10:00–13:00', services: ['목욕', '식사'] },
      { date: '3/19', time: '10:00–13:00', services: ['목욕', '식사'] },
    ],
  },
};

const CARE_LEVEL_LABELS: Record<number, { label: string; color: string }> = {
  1: { label: '1등급', color: 'bg-slate-100 text-slate-600' },
  2: { label: '2등급', color: 'bg-yellow-100 text-yellow-700' },
  3: { label: '3등급', color: 'bg-orange-100 text-orange-700' },
  4: { label: '4등급', color: 'bg-red-100 text-red-700' },
  5: { label: '5등급', color: 'bg-purple-100 text-purple-700' },
};

interface Props {
  params: Promise<{ clientId: string }>;
}

export default async function ClientProfilePage({ params }: Props) {
  const { clientId } = await params;
  const client = CLIENT_DATA[clientId] ?? CLIENT_DATA.c001;
  const levelInfo = CARE_LEVEL_LABELS[client.careLevel];

  return (
    <CaregiverAppShell
      activeTab="clients"
      title={`${client.name} 어르신`}
      showBackButton
      backHref="/clients"
    >
      <div className="px-4 py-4 space-y-4">
        {/* Header Card */}
        <div className="card">
          <div className="flex items-start gap-4 mb-4">
            <div className="w-16 h-16 bg-slate-100 rounded-2xl flex items-center justify-center text-3xl flex-shrink-0">
              {client.emoji}
            </div>
            <div className="flex-1">
              <div className="flex items-center gap-2 flex-wrap">
                <h2 className="text-xl font-bold text-slate-900">{client.name}</h2>
                <span className={`text-xs font-semibold px-2.5 py-1 rounded-full ${levelInfo.color}`}>
                  {levelInfo.label}
                </span>
              </div>
              <p className="text-sm text-slate-500 mt-0.5">{client.age}세 {client.gender}성 · {client.birthDate}</p>
              <a href={`tel:${client.phone}`} className="text-sm text-blue-600 font-medium mt-1 block">
                {client.phone}
              </a>
            </div>
          </div>

          <div className="space-y-2.5 pt-4 border-t border-slate-100">
            <div className="flex items-start gap-2.5">
              <svg className="w-4 h-4 text-slate-400 mt-0.5 flex-shrink-0" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" />
                <path strokeLinecap="round" strokeLinejoin="round" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z" />
              </svg>
              <div>
                <p className="text-sm text-slate-700">{client.address}</p>
                <p className="text-xs text-slate-400">{client.addressDetail}</p>
              </div>
            </div>
          </div>
        </div>

        {/* Quick Actions */}
        <div className="grid grid-cols-3 gap-3">
          <Link href={`/clients/${client.id}/care-plan`}>
            <div className="card flex flex-col items-center gap-2 py-4 active:scale-95 transition-transform text-center">
              <span className="text-2xl">📋</span>
              <span className="text-xs font-medium text-slate-700">케어플랜</span>
            </div>
          </Link>
          <Link href={`/clients/${client.id}/medications`}>
            <div className="card flex flex-col items-center gap-2 py-4 active:scale-95 transition-transform text-center">
              <span className="text-2xl">💊</span>
              <span className="text-xs font-medium text-slate-700">투약관리</span>
            </div>
          </Link>
          <Link href="/notes/new">
            <div className="card flex flex-col items-center gap-2 py-4 active:scale-95 transition-transform text-center">
              <span className="text-2xl">📝</span>
              <span className="text-xs font-medium text-slate-700">케어일지</span>
            </div>
          </Link>
        </div>

        {/* Diagnosis & Allergies */}
        <div className="card">
          <h3 className="section-title">진단 / 알레르기</h3>
          <div className="mb-3">
            <p className="text-xs font-medium text-slate-500 mb-2">진단명</p>
            <div className="flex flex-wrap gap-2">
              {client.primaryDiagnosis.map((d) => (
                <span key={d} className="badge-warning">{d}</span>
              ))}
            </div>
          </div>
          <div>
            <p className="text-xs font-medium text-slate-500 mb-2">알레르기</p>
            <div className="flex flex-wrap gap-2">
              {client.allergies.map((a) => (
                <span key={a} className="badge-danger">{a}</span>
              ))}
            </div>
          </div>
        </div>

        {/* Caregiver Note */}
        <div className="card">
          <h3 className="section-title">담당 보호사 메모</h3>
          <p className="text-sm text-slate-700 leading-relaxed bg-amber-50 rounded-xl p-3 border border-amber-100">
            ⚠️ {client.caregiverNote}
          </p>
        </div>

        {/* Recent Vitals */}
        <div className="card">
          <div className="flex items-center justify-between mb-3">
            <h3 className="section-title mb-0">최근 활력징후</h3>
            <span className="text-xs text-slate-400">{client.recentVitals.date}</span>
          </div>
          <div className="grid grid-cols-4 gap-3">
            {[
              { label: '혈압', value: client.recentVitals.bp, unit: '' },
              { label: '맥박', value: String(client.recentVitals.pulse), unit: '회' },
              { label: '체온', value: String(client.recentVitals.temp), unit: '°C' },
              { label: '체중', value: String(client.recentVitals.weight), unit: 'kg' },
            ].map((v) => (
              <div key={v.label} className="bg-slate-50 rounded-xl p-3 text-center">
                <p className="text-xs text-slate-400 mb-1">{v.label}</p>
                <p className="text-sm font-bold text-slate-800">{v.value}</p>
                {v.unit && <p className="text-xs text-slate-400">{v.unit}</p>}
              </div>
            ))}
          </div>
        </div>

        {/* Medications */}
        <div className="card">
          <div className="flex items-center justify-between mb-3">
            <h3 className="section-title mb-0">복용 약물</h3>
            <Link href={`/clients/${client.id}/medications`} className="text-xs text-blue-600 font-medium">
              전체 보기
            </Link>
          </div>
          <div className="space-y-2">
            {client.medications.map((med) => (
              <div key={med.name} className="flex items-center justify-between py-2 border-b border-slate-100 last:border-0">
                <div className="flex items-center gap-2">
                  <span className="text-base">💊</span>
                  <span className="text-sm font-medium text-slate-800">{med.name}</span>
                </div>
                <div className="text-right">
                  <p className="text-xs font-medium text-blue-600">{med.time}</p>
                  <p className="text-xs text-slate-400">{med.dose}</p>
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Emergency Contact */}
        <div className="card">
          <h3 className="section-title">비상 연락처</h3>
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-semibold text-slate-800">{client.emergencyContact.name}</p>
              <p className="text-xs text-slate-400">{client.emergencyContact.relationship}</p>
            </div>
            <a
              href={`tel:${client.emergencyContact.phone}`}
              className="flex items-center gap-2 bg-red-50 text-red-600 font-semibold text-sm px-4 py-2 rounded-xl active:scale-95 transition-transform"
            >
              <svg className="w-4 h-4" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z" />
              </svg>
              {client.emergencyContact.phone}
            </a>
          </div>
        </div>

        {/* Upcoming Visits */}
        <div className="card">
          <h3 className="section-title">예정 방문</h3>
          <div className="space-y-2">
            {client.upcomingVisits.map((v, idx) => (
              <div key={idx} className="flex items-center justify-between py-2 border-b border-slate-100 last:border-0">
                <div className="flex items-center gap-3">
                  <div className="w-10 h-10 bg-blue-50 rounded-xl flex items-center justify-center">
                    <span className="text-xs font-bold text-blue-600">{v.date}</span>
                  </div>
                  <span className="text-sm text-slate-700">{v.time}</span>
                </div>
                <div className="flex gap-1">
                  {v.services.map((s) => (
                    <span key={s} className="text-xs bg-slate-100 text-slate-600 px-2 py-0.5 rounded-full">{s}</span>
                  ))}
                </div>
              </div>
            ))}
          </div>
        </div>

        <div className="pb-4" />
      </div>
    </CaregiverAppShell>
  );
}
