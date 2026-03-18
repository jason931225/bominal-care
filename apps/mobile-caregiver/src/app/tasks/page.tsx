import Link from 'next/link';
import CaregiverAppShell from '@/components/CaregiverAppShell';

const TASKS = [
  {
    id: 't001',
    title: '이순자 어르신 혈압약 복용 확인',
    clientName: '이순자',
    clientId: 'c001',
    dueTime: '10:00',
    category: 'medication',
    priority: 'high',
    completed: false,
    description: '암로디핀 5mg 1정 오전 복용 확인 및 기록',
  },
  {
    id: 't002',
    title: '이순자 어르신 입욕 지원',
    clientName: '이순자',
    clientId: 'c001',
    dueTime: '10:30',
    category: 'care',
    priority: 'high',
    completed: false,
    description: '미끄럼 방지 매트 사용. 30~45분 소요 예상.',
  },
  {
    id: 't003',
    title: '박영철 어르신 혈당 체크',
    clientName: '박영철',
    clientId: 'c002',
    dueTime: '14:30',
    category: 'medical',
    priority: 'high',
    completed: false,
    description: '식전 혈당 측정 후 기록. 200 이상 시 가족 연락.',
  },
  {
    id: 't004',
    title: '이순자 어르신 케어일지 작성',
    clientName: '이순자',
    clientId: 'c001',
    dueTime: '13:00',
    category: 'documentation',
    priority: 'medium',
    completed: true,
    description: '방문 중 제공한 서비스 및 이용자 상태 기록',
  },
  {
    id: 't005',
    title: '최말순 어르신 방문 준비',
    clientName: '최말순',
    clientId: 'c003',
    dueTime: '내일',
    category: 'schedule',
    priority: 'low',
    completed: false,
    description: '케어플랜 확인 및 필요 물품 준비',
  },
];

const CATEGORY_CONFIG: Record<string, { icon: string; label: string; bg: string; text: string }> = {
  medication: { icon: '💊', label: '투약', bg: 'bg-purple-100', text: 'text-purple-700' },
  care: { icon: '🤝', label: '케어', bg: 'bg-blue-100', text: 'text-blue-700' },
  medical: { icon: '🩺', label: '의료', bg: 'bg-red-100', text: 'text-red-700' },
  documentation: { icon: '📝', label: '기록', bg: 'bg-slate-100', text: 'text-slate-600' },
  schedule: { icon: '📅', label: '일정', bg: 'bg-green-100', text: 'text-green-700' },
};

const PRIORITY_CONFIG: Record<string, { label: string; dot: string }> = {
  high: { label: '긴급', dot: 'bg-red-500' },
  medium: { label: '보통', dot: 'bg-amber-400' },
  low: { label: '낮음', dot: 'bg-slate-300' },
};

export default function TasksPage() {
  const pending = TASKS.filter((t) => !t.completed);
  const completed = TASKS.filter((t) => t.completed);

  return (
    <CaregiverAppShell activeTab="tasks" title="업무">
      <div className="px-4 py-4 space-y-5">
        {/* Summary */}
        <div className="grid grid-cols-3 gap-3">
          <div className="card text-center py-4">
            <p className="text-2xl font-bold text-slate-800">{TASKS.length}</p>
            <p className="text-xs text-slate-500 mt-1">전체 업무</p>
          </div>
          <div className="card text-center py-4">
            <p className="text-2xl font-bold text-red-500">{pending.length}</p>
            <p className="text-xs text-slate-500 mt-1">미완료</p>
          </div>
          <div className="card text-center py-4">
            <p className="text-2xl font-bold text-green-600">{completed.length}</p>
            <p className="text-xs text-slate-500 mt-1">완료</p>
          </div>
        </div>

        {/* Pending Tasks */}
        {pending.length > 0 && (
          <div>
            <h2 className="section-title">오늘 할 일 ({pending.length})</h2>
            <div className="space-y-2">
              {pending.map((task) => {
                const cat = CATEGORY_CONFIG[task.category];
                const pri = PRIORITY_CONFIG[task.priority];
                return (
                  <Link key={task.id} href={`/tasks/${task.id}`}>
                    <div className="card flex items-start gap-4 active:scale-98 transition-transform">
                      <div className={`w-10 h-10 rounded-xl flex items-center justify-center flex-shrink-0 ${cat.bg}`}>
                        <span className="text-xl">{cat.icon}</span>
                      </div>
                      <div className="flex-1 min-w-0">
                        <div className="flex items-start justify-between gap-2">
                          <p className="text-sm font-semibold text-slate-800 leading-snug">{task.title}</p>
                          <div className="flex items-center gap-1 flex-shrink-0">
                            <div className={`w-2 h-2 rounded-full ${pri.dot}`} />
                            <span className="text-xs text-slate-500">{pri.label}</span>
                          </div>
                        </div>
                        <div className="flex items-center gap-2 mt-1">
                          <span className={`text-xs font-medium px-1.5 py-0.5 rounded ${cat.bg} ${cat.text}`}>{cat.label}</span>
                          <span className="text-xs text-slate-400">{task.clientName} · {task.dueTime}</span>
                        </div>
                      </div>
                    </div>
                  </Link>
                );
              })}
            </div>
          </div>
        )}

        {/* Completed Tasks */}
        {completed.length > 0 && (
          <div>
            <h2 className="section-title text-slate-400">완료 ({completed.length})</h2>
            <div className="space-y-2">
              {completed.map((task) => {
                const cat = CATEGORY_CONFIG[task.category];
                return (
                  <div key={task.id} className="card flex items-start gap-4 opacity-60">
                    <div className={`w-10 h-10 rounded-xl flex items-center justify-center flex-shrink-0 ${cat.bg}`}>
                      <svg className="w-5 h-5 text-green-600" fill="currentColor" viewBox="0 0 20 20">
                        <path fillRule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clipRule="evenodd" />
                      </svg>
                    </div>
                    <div className="flex-1 min-w-0">
                      <p className="text-sm text-slate-500 line-through">{task.title}</p>
                      <span className="text-xs text-slate-400">{task.clientName} · {task.dueTime}</span>
                    </div>
                  </div>
                );
              })}
            </div>
          </div>
        )}

        {/* Quick Links */}
        <div>
          <h2 className="section-title">빠른 실행</h2>
          <div className="grid grid-cols-2 gap-3">
            <Link href="/notes/new">
              <div className="card flex items-center gap-3 active:scale-95 transition-transform">
                <span className="text-2xl">📝</span>
                <span className="text-sm font-medium text-slate-700">케어일지 작성</span>
              </div>
            </Link>
            <Link href="/notes/incident">
              <div className="card flex items-center gap-3 active:scale-95 transition-transform">
                <span className="text-2xl">🚨</span>
                <span className="text-sm font-medium text-slate-700">사고 보고서</span>
              </div>
            </Link>
          </div>
        </div>

        <div className="pb-2" />
      </div>
    </CaregiverAppShell>
  );
}
