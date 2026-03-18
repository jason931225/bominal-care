'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import Link from 'next/link';
import CaregiverAppShell from '@/components/CaregiverAppShell';

const TASK_DATA: Record<string, {
  id: string; title: string; clientName: string; clientId: string;
  dueTime: string; category: string; priority: string; completed: boolean;
  description: string; instructions: string[];
  checklistItems: { id: string; label: string; required: boolean }[];
}> = {
  t001: {
    id: 't001',
    title: '이순자 어르신 혈압약 복용 확인',
    clientName: '이순자',
    clientId: 'c001',
    dueTime: '10:00',
    category: 'medication',
    priority: 'high',
    completed: false,
    description: '암로디핀 5mg 1정 오전 복용 확인 및 기록',
    instructions: [
      '약 봉투 및 약통에서 오늘 복용분 확인',
      '충분한 물(200ml)과 함께 복용하도록 지원',
      '복용 후 이상반응(어지러움, 부종) 여부 확인',
      '복용 완료 후 투약 기록 작성',
    ],
    checklistItems: [
      { id: 'c1', label: '약 복용 전 어르신 상태 확인', required: true },
      { id: 'c2', label: '암로디핀 5mg 1정 복용 완료', required: true },
      { id: 'c3', label: '복용 후 이상반응 없음 확인', required: true },
      { id: 'c4', label: '투약 기록 입력 완료', required: true },
    ],
  },
};

export default function TaskDetailPage({ params: _params }: { params: Promise<{ taskId: string }> }) {
  const router = useRouter();
  const task = TASK_DATA.t001;

  const [completedChecks, setCompletedChecks] = useState<Set<string>>(new Set());
  const [memo, setMemo] = useState('');
  const [saving, setSaving] = useState(false);

  const toggleCheck = (id: string) => {
    setCompletedChecks((prev) => {
      const next = new Set(prev);
      if (next.has(id)) {
        next.delete(id);
      } else {
        next.add(id);
      }
      return next;
    });
  };

  const requiredDone = task.checklistItems
    .filter((i) => i.required)
    .every((i) => completedChecks.has(i.id));

  const handleComplete = async () => {
    if (!requiredDone) return;
    setSaving(true);
    await new Promise((r) => setTimeout(r, 1000));
    router.push('/tasks');
  };

  const PRIORITY_CONFIG: Record<string, { label: string; color: string }> = {
    high: { label: '긴급', color: 'badge-danger' },
    medium: { label: '보통', color: 'badge-warning' },
    low: { label: '낮음', color: 'badge-info' },
  };

  const CATEGORY_CONFIG: Record<string, { icon: string; label: string }> = {
    medication: { icon: '💊', label: '투약' },
    care: { icon: '🤝', label: '케어' },
    medical: { icon: '🩺', label: '의료' },
    documentation: { icon: '📝', label: '기록' },
    schedule: { icon: '📅', label: '일정' },
  };

  const priorityInfo = PRIORITY_CONFIG[task.priority];
  const catInfo = CATEGORY_CONFIG[task.category];

  return (
    <CaregiverAppShell
      activeTab="tasks"
      title="업무 상세"
      showBackButton
      backHref="/tasks"
    >
      <div className="px-4 py-4 space-y-4">
        {/* Task Header */}
        <div className="card">
          <div className="flex items-start gap-3 mb-3">
            <span className="text-3xl">{catInfo.icon}</span>
            <div className="flex-1">
              <div className="flex flex-wrap gap-2 mb-1">
                <span className={priorityInfo.color}>{priorityInfo.label}</span>
                <span className="badge-info">{catInfo.label}</span>
              </div>
              <h2 className="text-base font-bold text-slate-900">{task.title}</h2>
            </div>
          </div>
          <div className="space-y-2 pt-3 border-t border-slate-100">
            <div className="flex items-center gap-2 text-sm">
              <svg className="w-4 h-4 text-slate-400" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
              </svg>
              <Link href={`/clients/${task.clientId}`} className="text-blue-600 font-medium">{task.clientName} 어르신</Link>
            </div>
            <div className="flex items-center gap-2 text-sm">
              <svg className="w-4 h-4 text-slate-400" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              <span className="text-slate-700">오늘 {task.dueTime} 예정</span>
            </div>
          </div>
          <p className="text-sm text-slate-600 mt-3 leading-relaxed">{task.description}</p>
        </div>

        {/* Instructions */}
        <div className="card">
          <h3 className="section-title">수행 방법</h3>
          <ol className="space-y-2">
            {task.instructions.map((inst, idx) => (
              <li key={idx} className="flex items-start gap-3">
                <div className="w-5 h-5 rounded-full bg-blue-100 flex items-center justify-center flex-shrink-0 mt-0.5">
                  <span className="text-xs font-bold text-blue-600">{idx + 1}</span>
                </div>
                <span className="text-sm text-slate-700 leading-relaxed">{inst}</span>
              </li>
            ))}
          </ol>
        </div>

        {/* Checklist */}
        <div className="card">
          <div className="flex items-center justify-between mb-3">
            <h3 className="section-title mb-0">완료 체크리스트</h3>
            <span className="text-xs text-slate-400">
              {completedChecks.size}/{task.checklistItems.length}
            </span>
          </div>
          <div className="space-y-3">
            {task.checklistItems.map((item) => {
              const done = completedChecks.has(item.id);
              return (
                <button
                  key={item.id}
                  type="button"
                  onClick={() => toggleCheck(item.id)}
                  className="w-full flex items-center gap-3 text-left"
                >
                  <div className={`w-6 h-6 rounded-md border-2 flex items-center justify-center flex-shrink-0 transition-colors ${
                    done ? 'border-green-500 bg-green-500' : 'border-slate-300'
                  }`}>
                    {done && (
                      <svg className="w-3.5 h-3.5 text-white" fill="currentColor" viewBox="0 0 20 20">
                        <path fillRule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clipRule="evenodd" />
                      </svg>
                    )}
                  </div>
                  <span className={`text-sm flex-1 ${done ? 'line-through text-slate-400' : 'text-slate-700'}`}>
                    {item.label}
                  </span>
                  {item.required && !done && (
                    <span className="text-xs text-red-500 font-medium">필수</span>
                  )}
                </button>
              );
            })}
          </div>
        </div>

        {/* Memo */}
        <div className="card">
          <h3 className="section-title">수행 메모</h3>
          <textarea
            className="input-field resize-none"
            rows={3}
            placeholder="특이사항이 있으면 기록해 주세요..."
            value={memo}
            onChange={(e) => setMemo(e.target.value)}
          />
        </div>

        {/* Action */}
        <div className="pb-6 space-y-3">
          {!requiredDone && (
            <p className="text-xs text-center text-red-500">필수 체크리스트를 모두 완료해 주세요.</p>
          )}
          <button
            type="button"
            onClick={handleComplete}
            disabled={!requiredDone || saving}
            className="btn-primary flex items-center justify-center gap-2"
          >
            {saving ? (
              <>
                <svg className="w-4 h-4 animate-spin" fill="none" viewBox="0 0 24 24">
                  <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
                  <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
                </svg>
                저장 중...
              </>
            ) : '업무 완료 처리'}
          </button>
        </div>
      </div>
    </CaregiverAppShell>
  );
}
