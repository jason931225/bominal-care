import CaregiverAppShell from '@/components/CaregiverAppShell';
import Link from 'next/link';

const ALL_MEDICATIONS = [
  {
    clientId: 'c001',
    clientName: '이순자',
    clientEmoji: '👵',
    medications: [
      { name: '암로디핀 5mg', purpose: '고혈압', time: '10:00', taken: false, icon: '🩺' },
      { name: '아리셉트 5mg', purpose: '치매', time: '21:00', taken: false, icon: '🧠' },
      { name: '칼슘+비타민D', purpose: '골다공증', time: '13:00', taken: true, icon: '🦴' },
    ],
  },
  {
    clientId: 'c002',
    clientName: '박영철',
    clientEmoji: '👴',
    medications: [
      { name: '메트포르민 500mg', purpose: '당뇨', time: '08:00', taken: true, icon: '💉' },
      { name: '메트포르민 500mg', purpose: '당뇨', time: '18:00', taken: false, icon: '💉' },
      { name: '세레콕시브 100mg', purpose: '관절염', time: '12:00', taken: true, icon: '🦴' },
    ],
  },
  {
    clientId: 'c003',
    clientName: '최말순',
    clientEmoji: '👵',
    medications: [
      { name: '아스피린 100mg', purpose: '혈전 예방', time: '09:00', taken: false, icon: '💊' },
      { name: '리바스티그민 4.6mg', purpose: '치매', time: '08:00', taken: true, icon: '🧠' },
      { name: '로수바스타틴 10mg', purpose: '고지혈증', time: '21:00', taken: false, icon: '❤️' },
    ],
  },
];

const TIME_SLOTS = ['아침 (~12시)', '점심 (12–18시)', '저녁 (18시~)'];

function getSlot(time: string): string {
  const h = parseInt(time.split(':')[0]);
  if (h < 12) return TIME_SLOTS[0];
  if (h < 18) return TIME_SLOTS[1];
  return TIME_SLOTS[2];
}

export default function MedicationsOverviewPage() {
  const totalMeds = ALL_MEDICATIONS.flatMap((c) => c.medications);
  const takenCount = totalMeds.filter((m) => m.taken).length;
  const pendingCount = totalMeds.length - takenCount;

  // Group by time slot
  const grouped = TIME_SLOTS.map((slot) => ({
    slot,
    entries: ALL_MEDICATIONS.flatMap((client) =>
      client.medications
        .filter((m) => getSlot(m.time) === slot)
        .map((med) => ({ ...med, clientName: client.clientName, clientId: client.clientId, clientEmoji: client.clientEmoji }))
    ),
  })).filter((g) => g.entries.length > 0);

  return (
    <CaregiverAppShell activeTab="tasks" title="투약 일정">
      <div className="px-4 py-4 space-y-5">
        {/* Summary */}
        <div className="grid grid-cols-3 gap-3">
          <div className="card text-center py-4">
            <p className="text-2xl font-bold text-slate-800">{totalMeds.length}</p>
            <p className="text-xs text-slate-500 mt-1">전체 투약</p>
          </div>
          <div className="card text-center py-4">
            <p className="text-2xl font-bold text-green-600">{takenCount}</p>
            <p className="text-xs text-slate-500 mt-1">복용 완료</p>
          </div>
          <div className="card text-center py-4">
            <p className="text-2xl font-bold text-red-500">{pendingCount}</p>
            <p className="text-xs text-slate-500 mt-1">미복용</p>
          </div>
        </div>

        {/* Progress */}
        <div className="card py-3">
          <div className="flex items-center justify-between text-xs text-slate-500 mb-2">
            <span>오늘 투약 진행률</span>
            <span className="font-semibold text-blue-600">{Math.round((takenCount / totalMeds.length) * 100)}%</span>
          </div>
          <div className="h-2.5 bg-slate-100 rounded-full overflow-hidden">
            <div
              className="h-full bg-blue-500 rounded-full transition-all"
              style={{ width: `${Math.round((takenCount / totalMeds.length) * 100)}%` }}
            />
          </div>
        </div>

        {/* By Time Slot */}
        {grouped.map(({ slot, entries }) => (
          <div key={slot}>
            <h2 className="section-title">{slot}</h2>
            <div className="space-y-2">
              {entries.map((entry, idx) => (
                <Link key={idx} href={`/clients/${entry.clientId}/medications`}>
                  <div className={`card flex items-center gap-4 active:scale-98 transition-transform ${
                    !entry.taken ? 'border-amber-200 bg-amber-50' : 'opacity-70'
                  }`}>
                    <div className={`w-10 h-10 rounded-xl flex items-center justify-center flex-shrink-0 text-xl ${
                      entry.taken ? 'bg-green-100' : 'bg-white border border-amber-300'
                    }`}>
                      {entry.taken ? '✅' : entry.icon}
                    </div>
                    <div className="flex-1">
                      <div className="flex items-center gap-2">
                        <span className="text-sm">{entry.clientEmoji}</span>
                        <span className="text-sm font-semibold text-slate-800">{entry.clientName}</span>
                      </div>
                      <p className={`text-sm mt-0.5 ${entry.taken ? 'line-through text-slate-400' : 'text-slate-700'}`}>
                        {entry.name}
                      </p>
                      <p className="text-xs text-slate-400">{entry.purpose} · {entry.time}</p>
                    </div>
                    {!entry.taken && (
                      <span className="badge-warning flex-shrink-0">미복용</span>
                    )}
                  </div>
                </Link>
              ))}
            </div>
          </div>
        ))}

        {/* Per Client */}
        <div>
          <h2 className="section-title">이용자별 투약 현황</h2>
          <div className="space-y-3">
            {ALL_MEDICATIONS.map((client) => {
              const taken = client.medications.filter((m) => m.taken).length;
              const total = client.medications.length;
              const pct = Math.round((taken / total) * 100);
              return (
                <Link key={client.clientId} href={`/clients/${client.clientId}/medications`}>
                  <div className="card flex items-center gap-4 active:scale-98 transition-transform">
                    <span className="text-2xl flex-shrink-0">{client.clientEmoji}</span>
                    <div className="flex-1">
                      <div className="flex items-center justify-between mb-1.5">
                        <span className="text-sm font-semibold text-slate-800">{client.clientName}</span>
                        <span className="text-xs text-slate-500">{taken}/{total} 복용</span>
                      </div>
                      <div className="h-2 bg-slate-100 rounded-full overflow-hidden">
                        <div
                          className={`h-full rounded-full transition-all ${pct === 100 ? 'bg-green-500' : 'bg-blue-500'}`}
                          style={{ width: `${pct}%` }}
                        />
                      </div>
                    </div>
                    <svg className="w-4 h-4 text-slate-300 flex-shrink-0" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24">
                      <path strokeLinecap="round" strokeLinejoin="round" d="M9 5l7 7-7 7" />
                    </svg>
                  </div>
                </Link>
              );
            })}
          </div>
        </div>

        <div className="pb-2" />
      </div>
    </CaregiverAppShell>
  );
}
