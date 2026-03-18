'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import ApplicantAppShell from '@/components/ApplicantAppShell';

const REGIONS: Record<string, string[]> = {
  서울: ['강남구', '강동구', '강북구', '강서구', '관악구', '광진구', '구로구', '금천구', '노원구', '도봉구', '동대문구', '동작구', '마포구', '서대문구', '서초구', '성동구', '성북구', '송파구', '양천구', '영등포구', '용산구', '은평구', '종로구', '중구', '중랑구'],
  경기: ['수원시', '성남시', '고양시', '용인시', '부천시', '안산시', '안양시', '남양주시', '화성시', '의정부시', '광명시', '파주시', '광주시', '시흥시', '김포시', '군포시', '하남시', '오산시', '양주시', '구리시'],
  인천: ['중구', '동구', '미추홀구', '연수구', '남동구', '부평구', '계양구', '서구', '강화군', '옹진군'],
  부산: ['중구', '서구', '동구', '영도구', '부산진구', '동래구', '남구', '북구', '해운대구', '사하구', '금정구', '강서구', '연제구', '수영구', '사상구', '기장군'],
  대구: ['중구', '동구', '서구', '남구', '북구', '수성구', '달서구', '달성군'],
};

export default function ServiceRegionPage() {
  const router = useRouter();
  const [selectedCity, setSelectedCity] = useState('서울');
  const [selectedDistricts, setSelectedDistricts] = useState<Set<string>>(new Set());
  const [travelTime, setTravelTime] = useState(30);

  const toggleDistrict = (district: string) => {
    setSelectedDistricts((prev) => {
      const next = new Set(prev);
      if (next.has(district)) {
        next.delete(district);
      } else {
        next.add(district);
      }
      return next;
    });
  };

  const selectAllInCity = () => {
    const districts = REGIONS[selectedCity] ?? [];
    setSelectedDistricts(new Set(districts));
  };

  const clearAll = () => {
    setSelectedDistricts(new Set());
  };

  const selectedCount = selectedDistricts.size;

  return (
    <ApplicantAppShell currentStep={3} title="서비스 가능 지역">
      <div className="flex flex-col h-full">
        <div className="px-4 pt-6 pb-4">
          <h2 className="text-lg font-bold text-slate-900 mb-1">활동 가능한 지역을 선택해 주세요</h2>
          <p className="text-sm text-slate-500">최소 1개 이상의 구/군을 선택해 주세요.</p>
        </div>

        {/* City Tabs */}
        <div className="px-4 overflow-x-auto no-scrollbar mb-4">
          <div className="flex gap-2 w-max">
            {Object.keys(REGIONS).map((city) => (
              <button
                key={city}
                type="button"
                onClick={() => setSelectedCity(city)}
                className={`px-4 py-2 rounded-full text-sm font-medium whitespace-nowrap transition-colors ${
                  selectedCity === city
                    ? 'bg-blue-600 text-white'
                    : 'bg-slate-100 text-slate-600'
                }`}
              >
                {city}
              </button>
            ))}
          </div>
        </div>

        {/* Actions */}
        <div className="px-4 flex items-center justify-between mb-3">
          <span className="text-sm text-slate-600">
            {selectedCount > 0 ? (
              <span className="text-blue-600 font-semibold">{selectedCount}개</span>
            ) : '0개'} 선택됨
          </span>
          <div className="flex gap-2">
            <button type="button" onClick={selectAllInCity} className="text-xs text-blue-600 font-medium px-3 py-1.5 bg-blue-50 rounded-lg active:bg-blue-100">
              전체선택
            </button>
            <button type="button" onClick={clearAll} className="text-xs text-slate-500 font-medium px-3 py-1.5 bg-slate-100 rounded-lg active:bg-slate-200">
              초기화
            </button>
          </div>
        </div>

        {/* Districts Grid */}
        <div className="px-4 grid grid-cols-3 gap-2 overflow-y-auto flex-1 pb-4">
          {(REGIONS[selectedCity] ?? []).map((district) => {
            const active = selectedDistricts.has(district);
            return (
              <button
                key={district}
                type="button"
                onClick={() => toggleDistrict(district)}
                className={`py-3 px-2 rounded-xl text-sm font-medium transition-colors ${
                  active
                    ? 'bg-blue-600 text-white shadow-sm'
                    : 'bg-white text-slate-700 border border-slate-200'
                }`}
              >
                {district}
              </button>
            );
          })}
        </div>

        {/* Travel Time */}
        <div className="px-4 py-4 bg-white border-t border-slate-100">
          <div className="flex items-center justify-between mb-3">
            <label className="text-sm font-semibold text-slate-800">최대 이동 가능 시간</label>
            <span className="text-sm font-bold text-blue-600">{travelTime}분</span>
          </div>
          <input
            type="range"
            min={10}
            max={90}
            step={10}
            value={travelTime}
            onChange={(e) => setTravelTime(Number(e.target.value))}
            className="w-full accent-blue-600"
          />
          <div className="flex justify-between text-xs text-slate-400 mt-1">
            <span>10분</span>
            <span>90분</span>
          </div>
        </div>

        {/* Next */}
        <div className="px-4 py-4 bg-white border-t border-slate-100">
          <button
            type="button"
            onClick={() => router.push('/apply/schedule')}
            disabled={selectedCount === 0}
            className="btn-primary"
          >
            다음 단계로
          </button>
        </div>
      </div>
    </ApplicantAppShell>
  );
}
