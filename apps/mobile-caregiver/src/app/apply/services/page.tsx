'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import ApplicantAppShell from '@/components/ApplicantAppShell';

interface ServiceOption {
  id: string;
  category: string;
  label: string;
  description: string;
  icon: string;
  requiresCert: boolean;
}

const SERVICES: ServiceOption[] = [
  { id: 'bathing', category: '신체지원', label: '목욕 지원', description: '입욕, 샤워, 구강청결 등 위생관리', icon: '🛁', requiresCert: false },
  { id: 'feeding', category: '신체지원', label: '식사 지원', description: '식사 준비, 섭취 보조, 영양관리', icon: '🍱', requiresCert: false },
  { id: 'mobility', category: '신체지원', label: '이동 지원', description: '체위변경, 보행보조, 이동 도움', icon: '🚶', requiresCert: false },
  { id: 'excretion', category: '신체지원', label: '배변 지원', description: '화장실 이용, 기저귀 교환', icon: '🚿', requiresCert: false },
  { id: 'housework', category: '가사지원', label: '가사 지원', description: '청소, 세탁, 장보기, 식사 준비', icon: '🏠', requiresCert: false },
  { id: 'companion', category: '인지활동', label: '말벗 서비스', description: '대화 상대, 정서 지원, 취미활동 보조', icon: '💬', requiresCert: false },
  { id: 'cognitive', category: '인지활동', label: '인지활동 프로그램', description: '치매예방 활동, 인지자극 훈련', icon: '🧩', requiresCert: false },
  { id: 'medical', category: '의료지원', label: '투약 보조', description: '처방약 복용 관리 및 기록', icon: '💊', requiresCert: true },
  { id: 'wound', category: '의료지원', label: '상처 관리', description: '단순 상처 소독 및 처치', icon: '🩹', requiresCert: true },
  { id: 'dementia', category: '전문케어', label: '치매 전문 돌봄', description: '치매 노인 특화 케어서비스', icon: '🧠', requiresCert: true },
  { id: 'palliative', category: '전문케어', label: '호스피스 완화케어', description: '임종기 환자 돌봄 지원', icon: '🌸', requiresCert: true },
  { id: 'hospital', category: '병원동행', label: '병원 동행 서비스', description: '외래진료, 검사, 치료 동행', icon: '🏥', requiresCert: false },
];

const CATEGORIES = ['신체지원', '가사지원', '인지활동', '의료지원', '전문케어', '병원동행'];

export default function ServicesPage() {
  const router = useRouter();
  const [selected, setSelected] = useState<Set<string>>(new Set(['bathing', 'feeding', 'housework']));

  const toggle = (id: string) => {
    setSelected((prev) => {
      const next = new Set(prev);
      if (next.has(id)) {
        next.delete(id);
      } else {
        next.add(id);
      }
      return next;
    });
  };

  return (
    <ApplicantAppShell currentStep={5} title="제공 가능 서비스">
      <div className="px-4 py-6 space-y-6">
        <div>
          <h2 className="text-lg font-bold text-slate-900 mb-1">제공 가능한 서비스를 선택해 주세요</h2>
          <p className="text-sm text-slate-500">여러 항목을 선택할 수 있으며, 전문 자격이 필요한 항목은 표시됩니다.</p>
        </div>

        {/* Selected count */}
        <div className="flex items-center gap-2">
          <span className="badge-info">{selected.size}개 선택됨</span>
          {selected.size > 0 && (
            <button
              type="button"
              onClick={() => setSelected(new Set())}
              className="text-xs text-slate-400 underline"
            >
              전체 해제
            </button>
          )}
        </div>

        {CATEGORIES.map((cat) => {
          const items = SERVICES.filter((s) => s.category === cat);
          return (
            <div key={cat}>
              <h3 className="text-sm font-bold text-slate-500 uppercase tracking-wide mb-3">{cat}</h3>
              <div className="space-y-2">
                {items.map((svc) => {
                  const isOn = selected.has(svc.id);
                  return (
                    <button
                      key={svc.id}
                      type="button"
                      onClick={() => toggle(svc.id)}
                      className={`w-full flex items-center gap-4 p-4 rounded-2xl border text-left transition-colors active:scale-98 ${
                        isOn
                          ? 'border-blue-500 bg-blue-50'
                          : 'border-slate-200 bg-white'
                      }`}
                    >
                      <span className="text-2xl flex-shrink-0">{svc.icon}</span>
                      <div className="flex-1 min-w-0">
                        <div className="flex items-center gap-2">
                          <span className={`text-sm font-semibold ${isOn ? 'text-blue-700' : 'text-slate-800'}`}>
                            {svc.label}
                          </span>
                          {svc.requiresCert && (
                            <span className="text-xs bg-amber-100 text-amber-700 px-1.5 py-0.5 rounded font-medium">
                              자격필요
                            </span>
                          )}
                        </div>
                        <p className="text-xs text-slate-400 mt-0.5 leading-relaxed">{svc.description}</p>
                      </div>
                      <div className={`w-6 h-6 rounded-full border-2 flex items-center justify-center flex-shrink-0 transition-colors ${
                        isOn ? 'border-blue-600 bg-blue-600' : 'border-slate-300 bg-white'
                      }`}>
                        {isOn && (
                          <svg className="w-3.5 h-3.5 text-white" fill="currentColor" viewBox="0 0 20 20">
                            <path fillRule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clipRule="evenodd" />
                          </svg>
                        )}
                      </div>
                    </button>
                  );
                })}
              </div>
            </div>
          );
        })}

        <div className="pb-6">
          <button
            type="button"
            onClick={() => router.push('/apply/references')}
            disabled={selected.size === 0}
            className="btn-primary"
          >
            다음 단계로
          </button>
        </div>
      </div>
    </ApplicantAppShell>
  );
}
