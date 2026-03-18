import Link from 'next/link';
import CaregiverAppShell from '@/components/CaregiverAppShell';

const NOTES = [
  {
    id: 'n001',
    type: 'daily',
    clientName: '이순자',
    clientId: 'c001',
    date: '2026-03-13',
    title: '목욕 지원 및 식사 보조 완료',
    preview: '오전 10시 방문. 입욕 보조 40분 완료. 점심 죽으로 준비. 혈압약 복용 확인. 전반적으로 컨디션 양호하나 무릎 통증 호소.',
    mood: '🙂',
    services: ['목욕 지원', '식사 지원'],
  },
  {
    id: 'n002',
    type: 'daily',
    clientName: '박영철',
    clientId: 'c002',
    date: '2026-03-12',
    title: '가사 지원 및 투약 관리',
    preview: '오후 방문. 청소 및 세탁 완료. 혈당 체크 결과 148mg/dL (정상 범위). 저녁 식사 준비 완료. 어르신 기분 좋음.',
    mood: '😊',
    services: ['가사 지원', '투약 보조'],
  },
  {
    id: 'n003',
    type: 'incident',
    clientName: '이순자',
    clientId: 'c001',
    date: '2026-03-10',
    title: '낙상 사고 보고서',
    preview: '오전 11시경 화장실 이동 중 미끄러짐 발생. 즉시 부축하여 낙상은 방지됨. 경미한 타박상 없음. 보호자 연락 완료.',
    mood: null,
    services: [],
  },
  {
    id: 'n004',
    type: 'daily',
    clientName: '최말순',
    clientId: 'c003',
    date: '2026-03-11',
    title: '신체 케어 및 위생 관리',
    preview: '오전 방문. 체위 변경 3회 수행. 욕창 예방 위해 등과 발뒤꿈치 확인 — 이상 없음. 구강 청결 완료.',
    mood: '😐',
    services: ['목욕 지원', '이동 지원'],
  },
];

export default function NotesPage() {
  const dailyNotes = NOTES.filter((n) => n.type === 'daily');
  const incidentNotes = NOTES.filter((n) => n.type === 'incident');

  return (
    <CaregiverAppShell activeTab="tasks" title="케어일지">
      <div className="px-4 py-4 space-y-5">
        {/* Actions */}
        <div className="grid grid-cols-2 gap-3">
          <Link href="/notes/new">
            <div className="bg-blue-600 rounded-2xl p-4 flex items-center gap-3 active:scale-95 transition-transform">
              <span className="text-2xl">📝</span>
              <div>
                <p className="text-white font-semibold text-sm">케어일지 작성</p>
                <p className="text-blue-200 text-xs">일일 케어 기록</p>
              </div>
            </div>
          </Link>
          <Link href="/notes/incident">
            <div className="bg-red-500 rounded-2xl p-4 flex items-center gap-3 active:scale-95 transition-transform">
              <span className="text-2xl">🚨</span>
              <div>
                <p className="text-white font-semibold text-sm">사고 보고서</p>
                <p className="text-red-200 text-xs">사고 발생 시 즉시</p>
              </div>
            </div>
          </Link>
        </div>

        {/* Stats */}
        <div className="grid grid-cols-3 gap-3">
          <div className="card text-center py-3">
            <p className="text-xl font-bold text-blue-600">{dailyNotes.length}</p>
            <p className="text-xs text-slate-500 mt-0.5">이번 달 일지</p>
          </div>
          <div className="card text-center py-3">
            <p className="text-xl font-bold text-red-500">{incidentNotes.length}</p>
            <p className="text-xs text-slate-500 mt-0.5">사고 보고</p>
          </div>
          <div className="card text-center py-3">
            <p className="text-xl font-bold text-green-600">3</p>
            <p className="text-xs text-slate-500 mt-0.5">담당 이용자</p>
          </div>
        </div>

        {/* Incident Alert */}
        {incidentNotes.length > 0 && (
          <div className="bg-red-50 border border-red-200 rounded-2xl p-4">
            <div className="flex items-center gap-2 mb-2">
              <span className="text-red-500 font-bold text-sm">⚠️ 사고 보고서</span>
              <span className="badge-danger">{incidentNotes.length}건</span>
            </div>
            {incidentNotes.map((note) => (
              <div key={note.id} className="mt-2">
                <p className="text-sm font-medium text-red-700">{note.title}</p>
                <p className="text-xs text-red-500">{note.clientName} · {note.date}</p>
              </div>
            ))}
          </div>
        )}

        {/* Daily Notes */}
        <div>
          <h2 className="section-title">케어일지</h2>
          <div className="space-y-3">
            {NOTES.map((note) => (
              <div key={note.id} className={`card active:scale-98 transition-transform ${
                note.type === 'incident' ? 'border-red-200 bg-red-50' : ''
              }`}>
                <div className="flex items-start gap-3">
                  <div className={`w-10 h-10 rounded-xl flex items-center justify-center flex-shrink-0 ${
                    note.type === 'incident' ? 'bg-red-100' : 'bg-blue-50'
                  }`}>
                    <span className="text-xl">{note.type === 'incident' ? '🚨' : '📝'}</span>
                  </div>
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2 mb-0.5">
                      <span className="text-xs font-medium text-slate-500">{note.clientName}</span>
                      <span className="text-xs text-slate-300">·</span>
                      <span className="text-xs text-slate-400">{note.date}</span>
                      {note.mood && <span className="text-sm">{note.mood}</span>}
                    </div>
                    <p className={`text-sm font-semibold ${note.type === 'incident' ? 'text-red-700' : 'text-slate-800'}`}>
                      {note.title}
                    </p>
                    <p className="text-xs text-slate-500 mt-1 leading-relaxed line-clamp-2">{note.preview}</p>
                    {note.services.length > 0 && (
                      <div className="flex gap-1 mt-2 flex-wrap">
                        {note.services.map((s) => (
                          <span key={s} className="text-xs bg-slate-100 text-slate-500 px-2 py-0.5 rounded-full">{s}</span>
                        ))}
                      </div>
                    )}
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>

        <div className="pb-2" />
      </div>
    </CaregiverAppShell>
  );
}
