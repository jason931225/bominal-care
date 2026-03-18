'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import Link from 'next/link';
import CaregiverAppShell from '@/components/CaregiverAppShell';

const MOCK_VISIT = {
  id: 'v001',
  clientName: '이순자',
  address: '서울 강남구 대치동 123-45',
  scheduledTime: '10:00',
  services: ['목욕 지원', '식사 지원', '배변 지원'],
};

interface Props {
  params: Promise<{ visitId: string }>;
}

export default function CheckinPage({ params: _params }: Props) {
  const router = useRouter();
  const [locationStatus, setLocationStatus] = useState<'idle' | 'locating' | 'success' | 'error'>('idle');
  const [location, setLocation] = useState<{ lat: number; lng: number; accuracy: number } | null>(null);
  const [checkinTime] = useState(() => {
    const now = new Date();
    return `${now.getHours().toString().padStart(2, '0')}:${now.getMinutes().toString().padStart(2, '0')}`;
  });
  const [note, setNote] = useState('');
  const [submitting, setSubmitting] = useState(false);

  const handleGetLocation = () => {
    setLocationStatus('locating');
    if (typeof window !== 'undefined' && navigator.geolocation) {
      navigator.geolocation.getCurrentPosition(
        (pos) => {
          setLocation({
            lat: pos.coords.latitude,
            lng: pos.coords.longitude,
            accuracy: Math.round(pos.coords.accuracy),
          });
          setLocationStatus('success');
        },
        () => {
          // Fallback mock location for demo
          setLocation({ lat: 37.4994, lng: 127.0616, accuracy: 15 });
          setLocationStatus('success');
        },
        { timeout: 8000 }
      );
    } else {
      setLocation({ lat: 37.4994, lng: 127.0616, accuracy: 15 });
      setLocationStatus('success');
    }
  };

  const handleCheckin = async () => {
    if (locationStatus !== 'success') return;
    setSubmitting(true);
    await new Promise((r) => setTimeout(r, 1200));
    router.push(`/schedule/${MOCK_VISIT.id}`);
  };

  return (
    <CaregiverAppShell
      activeTab="schedule"
      title="체크인"
      showBackButton
      backHref={`/schedule/${MOCK_VISIT.id}`}
    >
      <div className="px-4 py-6 space-y-5">
        {/* Visit Summary */}
        <div className="card bg-blue-50 border-blue-200">
          <div className="flex items-center gap-3 mb-3">
            <div className="w-10 h-10 bg-blue-100 rounded-xl flex items-center justify-center text-xl">👵</div>
            <div>
              <p className="text-sm font-bold text-blue-800">{MOCK_VISIT.clientName} 어르신</p>
              <p className="text-xs text-blue-600">{MOCK_VISIT.address}</p>
            </div>
          </div>
          <div className="flex flex-wrap gap-1.5">
            {MOCK_VISIT.services.map((s) => (
              <span key={s} className="text-xs bg-blue-100 text-blue-700 px-2.5 py-1 rounded-full font-medium">{s}</span>
            ))}
          </div>
        </div>

        {/* Check-in Time */}
        <div className="card">
          <h3 className="section-title">체크인 시간</h3>
          <div className="flex items-center justify-between">
            <div>
              <p className="text-3xl font-bold text-slate-900">{checkinTime}</p>
              <p className="text-xs text-slate-400 mt-1">2026년 3월 15일</p>
            </div>
            <div className="text-right">
              <p className="text-sm text-slate-500">예정 시간</p>
              <p className="text-lg font-semibold text-slate-700">{MOCK_VISIT.scheduledTime}</p>
              {checkinTime === MOCK_VISIT.scheduledTime ? (
                <span className="badge-success mt-1">정시</span>
              ) : (
                <span className="badge-info mt-1">
                  {checkinTime > MOCK_VISIT.scheduledTime ? '지연' : '조기'}
                </span>
              )}
            </div>
          </div>
        </div>

        {/* Location Verification */}
        <div className="card">
          <h3 className="section-title">위치 확인</h3>
          <p className="text-xs text-slate-500 mb-4">
            방문지 위치를 확인하기 위해 현재 위치가 필요합니다.
          </p>

          {locationStatus === 'idle' && (
            <button type="button" onClick={handleGetLocation} className="btn-primary">
              현재 위치 확인하기
            </button>
          )}

          {locationStatus === 'locating' && (
            <div className="flex flex-col items-center py-6 gap-3">
              <svg className="w-8 h-8 text-blue-600 animate-spin" fill="none" viewBox="0 0 24 24">
                <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
                <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
              </svg>
              <p className="text-sm text-slate-500">위치 확인 중...</p>
            </div>
          )}

          {locationStatus === 'success' && location && (
            <div>
              <div className="bg-green-50 rounded-xl p-4 border border-green-200 mb-3">
                <div className="flex items-center gap-2 mb-2">
                  <svg className="w-5 h-5 text-green-600" fill="currentColor" viewBox="0 0 20 20">
                    <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clipRule="evenodd" />
                  </svg>
                  <p className="text-sm font-semibold text-green-700">위치 확인 완료</p>
                </div>
                <p className="text-xs text-green-600">
                  위도 {location.lat.toFixed(4)}, 경도 {location.lng.toFixed(4)}
                </p>
                <p className="text-xs text-green-600 mt-0.5">정확도 ±{location.accuracy}m</p>
              </div>

              {/* Mock Map Placeholder */}
              <div className="h-40 bg-slate-100 rounded-xl overflow-hidden relative">
                <div className="absolute inset-0 flex items-center justify-center">
                  <div className="text-center">
                    <div className="text-3xl mb-1">📍</div>
                    <p className="text-xs text-slate-500">지도 미리보기</p>
                    <p className="text-xs text-slate-400">{MOCK_VISIT.address}</p>
                  </div>
                </div>
                <div className="absolute bottom-2 right-2">
                  <span className="bg-green-500 text-white text-xs font-medium px-2 py-1 rounded-full">
                    방문지 근처
                  </span>
                </div>
              </div>
            </div>
          )}

          {locationStatus === 'error' && (
            <div>
              <div className="bg-red-50 rounded-xl p-3 border border-red-200 mb-3">
                <p className="text-sm text-red-700">위치를 확인할 수 없습니다. 설정에서 위치 접근을 허용해 주세요.</p>
              </div>
              <button type="button" onClick={handleGetLocation} className="btn-secondary">
                다시 시도
              </button>
            </div>
          )}
        </div>

        {/* Optional note */}
        <div className="card">
          <h3 className="section-title">도착 메모 (선택)</h3>
          <textarea
            className="input-field resize-none"
            rows={3}
            placeholder="특이사항이 있으면 입력해 주세요..."
            value={note}
            onChange={(e) => setNote(e.target.value)}
          />
        </div>

        {/* Submit */}
        <div className="pb-6 space-y-3">
          <button
            type="button"
            onClick={handleCheckin}
            disabled={locationStatus !== 'success' || submitting}
            className="btn-primary flex items-center justify-center gap-2"
          >
            {submitting ? (
              <>
                <svg className="w-4 h-4 animate-spin" fill="none" viewBox="0 0 24 24">
                  <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
                  <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
                </svg>
                처리 중...
              </>
            ) : '체크인 완료'}
          </button>
          <Link href={`/schedule/${MOCK_VISIT.id}`} className="btn-secondary block text-center">
            취소
          </Link>
        </div>
      </div>
    </CaregiverAppShell>
  );
}
