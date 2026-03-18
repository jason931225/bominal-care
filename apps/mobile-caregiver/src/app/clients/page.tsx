import Link from 'next/link';
import CaregiverAppShell from '@/components/CaregiverAppShell';

const CLIENTS = [
  {
    id: 'c001',
    name: '이순자',
    age: 82,
    gender: '여',
    address: '강남구 대치동',
    careLevel: 3,
    primaryDiagnosis: ['고혈압', '경증 치매'],
    lastVisit: '2026-03-13',
    nextVisit: '2026-03-15 10:00',
    status: 'active',
    emergencyPhone: '010-9876-5432',
    emoji: '👵',
  },
  {
    id: 'c002',
    name: '박영철',
    age: 76,
    gender: '남',
    address: '강남구 역삼동',
    careLevel: 2,
    primaryDiagnosis: ['당뇨', '관절염'],
    lastVisit: '2026-03-12',
    nextVisit: '2026-03-15 14:30',
    status: 'active',
    emergencyPhone: '010-3456-7890',
    emoji: '👴',
  },
  {
    id: 'c003',
    name: '최말순',
    age: 88,
    gender: '여',
    address: '서초구 반포동',
    careLevel: 4,
    primaryDiagnosis: ['뇌졸중 후유증', '욕창'],
    lastVisit: '2026-03-11',
    nextVisit: '2026-03-16 09:00',
    status: 'active',
    emergencyPhone: '010-5678-9012',
    emoji: '👵',
  },
];

const CARE_LEVEL_COLORS: Record<number, string> = {
  1: 'bg-slate-100 text-slate-600',
  2: 'bg-yellow-100 text-yellow-700',
  3: 'bg-orange-100 text-orange-700',
  4: 'bg-red-100 text-red-700',
  5: 'bg-purple-100 text-purple-700',
};

export default function ClientsPage() {
  return (
    <CaregiverAppShell activeTab="clients" title="이용자">
      <div className="px-4 py-4 space-y-4">
        {/* Summary */}
        <div className="grid grid-cols-3 gap-3">
          <div className="card text-center py-4">
            <p className="text-2xl font-bold text-blue-600">{CLIENTS.length}</p>
            <p className="text-xs text-slate-500 mt-1">담당 이용자</p>
          </div>
          <div className="card text-center py-4">
            <p className="text-2xl font-bold text-green-600">2</p>
            <p className="text-xs text-slate-500 mt-1">오늘 방문</p>
          </div>
          <div className="card text-center py-4">
            <p className="text-2xl font-bold text-amber-500">1</p>
            <p className="text-xs text-slate-500 mt-1">주의 필요</p>
          </div>
        </div>

        {/* Client List */}
        <div className="space-y-3">
          {CLIENTS.map((client) => (
            <Link key={client.id} href={`/clients/${client.id}`}>
              <div className="card active:scale-98 transition-transform">
                <div className="flex items-start gap-4">
                  {/* Avatar */}
                  <div className="w-14 h-14 bg-slate-100 rounded-2xl flex items-center justify-center text-2xl flex-shrink-0">
                    {client.emoji}
                  </div>

                  {/* Info */}
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2 mb-1">
                      <h3 className="text-base font-bold text-slate-900">{client.name}</h3>
                      <span className={`text-xs font-semibold px-2 py-0.5 rounded-full ${CARE_LEVEL_COLORS[client.careLevel]}`}>
                        {client.careLevel}등급
                      </span>
                    </div>
                    <p className="text-sm text-slate-500">{client.age}세 {client.gender}성 · {client.address}</p>
                    <div className="flex flex-wrap gap-1 mt-1.5">
                      {client.primaryDiagnosis.map((d) => (
                        <span key={d} className="text-xs bg-slate-100 text-slate-600 px-2 py-0.5 rounded-full">{d}</span>
                      ))}
                    </div>
                    <div className="flex items-center gap-3 mt-2">
                      <div className="flex items-center gap-1">
                        <svg className="w-3.5 h-3.5 text-slate-400" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24">
                          <path strokeLinecap="round" strokeLinejoin="round" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
                        </svg>
                        <span className="text-xs text-slate-400">다음: {client.nextVisit}</span>
                      </div>
                    </div>
                  </div>

                  {/* Arrow */}
                  <svg className="w-5 h-5 text-slate-300 flex-shrink-0 mt-1" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" d="M9 5l7 7-7 7" />
                  </svg>
                </div>
              </div>
            </Link>
          ))}
        </div>
      </div>
    </CaregiverAppShell>
  );
}
