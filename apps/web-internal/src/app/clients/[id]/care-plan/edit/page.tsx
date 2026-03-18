'use client';

import InternalAppShell from '@/components/InternalAppShell';
import Link from 'next/link';
import { use, useState } from 'react';

const INITIAL_GOALS = [
  '일상생활 독립성 유지 및 향상',
  '만성질환(고혈압, 당뇨) 관리 지원',
  '사회적 고립 예방 및 정서적 안정 지원',
  '낙상 예방 및 안전한 이동 지원',
];

export default function EditCarePlanPage({ params }: { params: Promise<{ id: string }> }) {
  const { id } = use(params);
  const [goals, setGoals] = useState<string[]>(INITIAL_GOALS);
  const [newGoal, setNewGoal] = useState('');
  const [healthNotes, setHealthNotes] = useState(
    '고혈압 약 (아침 복용), 당뇨 약 (식후 복용). 저혈당 증상 관찰 필요. 혈압 160 이상 시 즉시 보고.'
  );
  const [restrictions, setRestrictions] = useState(
    '격렬한 운동 금지. 계단 이동 시 반드시 지원. 혼자 외출 불가.'
  );
  const [emergency, setEmergency] = useState(
    '박민수 (아들) 010-9876-5432 → 강남세브란스병원 응급실 02-1234-5678'
  );

  const addGoal = () => {
    if (newGoal.trim()) {
      setGoals([...goals, newGoal.trim()]);
      setNewGoal('');
    }
  };

  const removeGoal = (idx: number) => {
    setGoals(goals.filter((_, i) => i !== idx));
  };

  return (
    <InternalAppShell>
      <div className="space-y-6">
        {/* Breadcrumb */}
        <div className="flex items-center gap-2 text-sm text-gray-500">
          <Link href="/clients" className="hover:text-blue-600">이용자 관리</Link>
          <span>/</span>
          <Link href={`/clients/${id}`} className="hover:text-blue-600">박순자</Link>
          <span>/</span>
          <Link href={`/clients/${id}/care-plan`} className="hover:text-blue-600">케어 플랜</Link>
          <span>/</span>
          <span className="text-gray-900">수정</span>
        </div>

        <div className="flex items-center justify-between">
          <h1 className="page-title">케어 플랜 수정</h1>
          <div className="flex gap-2">
            <Link href={`/clients/${id}/care-plan`} className="btn-secondary">취소</Link>
            <button className="btn-primary">
              <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                <path strokeLinecap="round" strokeLinejoin="round" d="M5 13l4 4L19 7" />
              </svg>
              저장
            </button>
          </div>
        </div>

        <div className="grid grid-cols-2 gap-6">
          {/* Left column */}
          <div className="space-y-6">
            {/* Client & plan basics */}
            <div className="card p-5">
              <h2 className="section-title mb-4">기본 정보</h2>
              <div className="space-y-4">
                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <label className="label">이용자</label>
                    <input type="text" className="input bg-gray-50" value="박순자" readOnly />
                  </div>
                  <div>
                    <label className="label">장기요양 등급</label>
                    <select className="input">
                      <option>2등급</option>
                      <option>1등급</option>
                      <option>3등급</option>
                      <option>4등급</option>
                      <option>5등급</option>
                    </select>
                  </div>
                </div>
                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <label className="label">다음 검토 일자</label>
                    <input type="date" className="input" defaultValue="2026-07-10" />
                  </div>
                  <div>
                    <label className="label">플랜 상태</label>
                    <select className="input">
                      <option>유효</option>
                      <option>검토중</option>
                      <option>만료</option>
                    </select>
                  </div>
                </div>
              </div>
            </div>

            {/* Services */}
            <div className="card p-5">
              <h2 className="section-title mb-4">서비스 계획</h2>
              <div className="space-y-4">
                {[
                  { category: '신체활동 지원', frequency: '주 5회', duration: '3시간/회' },
                  { category: '인지활동 지원', frequency: '주 3회', duration: '30분/회' },
                  { category: '가사활동 지원', frequency: '주 2회', duration: '1시간/회' },
                  { category: '건강 관리', frequency: '매 방문', duration: '15분/회' },
                ].map((svc, idx) => (
                  <div key={idx} className="border border-gray-200 rounded-lg p-4">
                    <div className="grid grid-cols-3 gap-3">
                      <div>
                        <label className="label">서비스 유형</label>
                        <input type="text" className="input" defaultValue={svc.category} />
                      </div>
                      <div>
                        <label className="label">빈도</label>
                        <input type="text" className="input" defaultValue={svc.frequency} />
                      </div>
                      <div>
                        <label className="label">시간</label>
                        <input type="text" className="input" defaultValue={svc.duration} />
                      </div>
                    </div>
                  </div>
                ))}
                <button className="w-full py-2 border-2 border-dashed border-gray-300 rounded-lg text-sm text-gray-500 hover:border-blue-400 hover:text-blue-600 transition-colors">
                  + 서비스 항목 추가
                </button>
              </div>
            </div>
          </div>

          {/* Right column */}
          <div className="space-y-6">
            {/* Goals */}
            <div className="card p-5">
              <h2 className="section-title mb-4">케어 목표</h2>
              <ul className="space-y-2 mb-3">
                {goals.map((goal, idx) => (
                  <li key={idx} className="flex items-center gap-2 bg-blue-50 rounded-lg px-3 py-2">
                    <span className="w-5 h-5 rounded-full bg-blue-600 text-white text-xs font-bold flex items-center justify-center flex-shrink-0">
                      {idx + 1}
                    </span>
                    <span className="flex-1 text-sm text-gray-800">{goal}</span>
                    <button
                      onClick={() => removeGoal(idx)}
                      className="text-gray-400 hover:text-red-600 transition-colors"
                    >
                      <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                        <path strokeLinecap="round" strokeLinejoin="round" d="M6 18L18 6M6 6l12 12" />
                      </svg>
                    </button>
                  </li>
                ))}
              </ul>
              <div className="flex gap-2">
                <input
                  type="text"
                  className="input flex-1"
                  placeholder="새 목표 입력..."
                  value={newGoal}
                  onChange={(e) => setNewGoal(e.target.value)}
                  onKeyDown={(e) => e.key === 'Enter' && addGoal()}
                />
                <button onClick={addGoal} className="btn-primary px-3">추가</button>
              </div>
            </div>

            {/* Health notes */}
            <div className="card p-5">
              <h2 className="section-title mb-3">건강 특이사항</h2>
              <textarea
                className="input w-full h-24 resize-none"
                value={healthNotes}
                onChange={(e) => setHealthNotes(e.target.value)}
                placeholder="건강 관련 특이사항을 입력하세요..."
              />
            </div>

            {/* Restrictions */}
            <div className="card p-5">
              <h2 className="section-title mb-3">제한 사항</h2>
              <textarea
                className="input w-full h-20 resize-none"
                value={restrictions}
                onChange={(e) => setRestrictions(e.target.value)}
                placeholder="활동 제한 사항을 입력하세요..."
              />
            </div>

            {/* Emergency */}
            <div className="card p-5 border-l-4 border-l-red-500">
              <h2 className="section-title text-red-700 mb-3">응급 연락처</h2>
              <textarea
                className="input w-full h-20 resize-none"
                value={emergency}
                onChange={(e) => setEmergency(e.target.value)}
                placeholder="응급 연락처 및 이송 병원을 입력하세요..."
              />
            </div>
          </div>
        </div>

        {/* Save footer */}
        <div className="flex justify-end gap-2 pt-2 border-t border-gray-200">
          <Link href={`/clients/${id}/care-plan`} className="btn-secondary">취소</Link>
          <button className="btn-primary">
            <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M5 13l4 4L19 7" />
            </svg>
            케어 플랜 저장
          </button>
        </div>
      </div>
    </InternalAppShell>
  );
}
