// 교통 서비스 — Transport / Rides Service
// Lets seniors book medical transport and community rides

'use client';

import { useState } from 'react';
import Link from 'next/link';
import SeniorAppShell from '@/components/SeniorAppShell';

const RIDE_PROVIDERS = [
  {
    id: 'rp-1',
    name: '복지 셔틀버스',
    type: '노인복지 셔틀',
    description: '병원, 복지관, 마트 정기 운행',
    available: true,
    price: '무료',
    phone: '02-1234-5001',
  },
  {
    id: 'rp-2',
    name: '의료 이송 서비스',
    type: '전문 의료 이송',
    description: '휠체어 탑승 가능, 보조 인력 동반',
    available: true,
    price: '1km당 500원',
    phone: '1588-1234',
  },
  {
    id: 'rp-3',
    name: '카카오 T 바우처',
    type: '바우처 택시',
    description: '장기요양 교통 바우처 사용 가능',
    available: true,
    price: '월 4만원 한도 무료',
    phone: null,
  },
];

const UPCOMING_RIDES = [
  {
    id: 'ride-1',
    date: '2026년 3월 15일',
    time: '오전 9:30',
    destination: '서울 중앙 의원',
    provider: '복지 셔틀버스',
    status: 'confirmed',
  },
];

const SHUTTLE_SCHEDULE = [
  { day: '월·수·금', time: '오전 9:00', route: '주민센터 → 병원 → 복지관' },
  { day: '화·목', time: '오전 10:00', route: '주민센터 → 마트 → 주민센터' },
  { day: '매주 토', time: '오전 11:00', route: '주민센터 → 공원 → 주민센터' },
];

export default function RidesPage() {
  const [showBooking, setShowBooking] = useState(false);
  const [destination, setDestination] = useState('');
  const [rideDate, setRideDate] = useState('');
  const [rideTime, setRideTime] = useState('');
  const [submitted, setSubmitted] = useState(false);

  function handleBook(e: React.FormEvent) {
    e.preventDefault();
    if (destination && rideDate && rideTime) {
      setSubmitted(true);
    }
  }

  return (
    <SeniorAppShell>
      <div className="page-content">
        <Link
          href="/services"
          className="inline-flex items-center gap-2 text-primary-600 font-medium text-senior-base mb-5 min-h-touch"
        >
          <svg className="w-5 h-5" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24" aria-hidden="true">
            <path strokeLinecap="round" strokeLinejoin="round" d="M15 19l-7-7 7-7" />
          </svg>
          서비스로
        </Link>

        <h1 className="text-senior-2xl font-bold text-gray-900 mb-5">교통 서비스 🚐</h1>

        {/* Upcoming rides */}
        {UPCOMING_RIDES.length > 0 && (
          <section className="mb-5" aria-labelledby="upcoming-rides-heading">
            <h2 id="upcoming-rides-heading" className="senior-section-title">예정된 이동</h2>
            {UPCOMING_RIDES.map((ride) => (
              <div key={ride.id} className="bg-primary-50 border-2 border-primary-300 rounded-2xl p-4 flex items-center gap-3">
                <span className="text-3xl" aria-hidden="true">🚐</span>
                <div className="flex-1">
                  <p className="text-senior-base font-bold text-primary-800">{ride.destination}</p>
                  <p className="text-senior-base text-primary-700">{ride.date} {ride.time}</p>
                  <p className="text-senior-sm text-gray-500">{ride.provider}</p>
                </div>
                <span className="bg-success-50 text-success-700 text-senior-sm font-bold px-3 py-1 rounded-full">
                  확정
                </span>
              </div>
            ))}
          </section>
        )}

        {/* Book a ride */}
        {!submitted ? (
          <section className="senior-card mb-5" aria-labelledby="book-ride-heading">
            <h2 id="book-ride-heading" className="text-senior-lg font-bold text-gray-800 mb-3">이동 예약하기</h2>
            {!showBooking ? (
              <button
                onClick={() => setShowBooking(true)}
                className="senior-btn-primary w-full"
              >
                + 이동 예약
              </button>
            ) : (
              <form onSubmit={handleBook} className="space-y-4">
                <div>
                  <label className="block text-senior-base font-bold text-gray-700 mb-1">목적지 <span className="text-danger-500">*</span></label>
                  <input
                    type="text"
                    value={destination}
                    onChange={(e) => setDestination(e.target.value)}
                    placeholder="예: 서울 중앙 의원"
                    required
                    className="w-full border-2 border-gray-300 rounded-xl px-4 py-3 text-senior-lg focus:border-primary-500 focus:outline-none min-h-touch-senior"
                  />
                </div>
                <div>
                  <label className="block text-senior-base font-bold text-gray-700 mb-1">날짜 <span className="text-danger-500">*</span></label>
                  <input
                    type="date"
                    value={rideDate}
                    onChange={(e) => setRideDate(e.target.value)}
                    required
                    min={new Date().toISOString().split('T')[0]}
                    className="w-full border-2 border-gray-300 rounded-xl px-4 py-3 text-senior-lg focus:border-primary-500 focus:outline-none min-h-touch-senior"
                  />
                </div>
                <div>
                  <label className="block text-senior-base font-bold text-gray-700 mb-1">출발 시간 <span className="text-danger-500">*</span></label>
                  <input
                    type="time"
                    value={rideTime}
                    onChange={(e) => setRideTime(e.target.value)}
                    required
                    className="w-full border-2 border-gray-300 rounded-xl px-4 py-3 text-senior-lg focus:border-primary-500 focus:outline-none min-h-touch-senior"
                  />
                </div>
                <button type="submit" className="senior-btn-primary w-full">예약 신청</button>
                <button type="button" onClick={() => setShowBooking(false)} className="w-full text-center text-senior-base text-gray-500 py-2 min-h-touch">취소</button>
              </form>
            )}
          </section>
        ) : (
          <div className="senior-card mb-5 text-center py-4">
            <p className="text-3xl mb-2" aria-hidden="true">✅</p>
            <p className="text-senior-lg font-bold text-success-700">이동 예약이 접수됐습니다!</p>
            <p className="text-senior-sm text-gray-500 mt-1">확인 후 연락드리겠습니다.</p>
            <button onClick={() => { setSubmitted(false); setShowBooking(false); setDestination(''); setRideDate(''); setRideTime(''); }} className="senior-btn-secondary mt-4">
              새 예약하기
            </button>
          </div>
        )}

        {/* Shuttle schedule */}
        <section className="senior-card mb-5" aria-labelledby="shuttle-heading">
          <h2 id="shuttle-heading" className="text-senior-lg font-bold text-gray-800 mb-3">복지 셔틀 정기 운행 일정</h2>
          <div className="space-y-3">
            {SHUTTLE_SCHEDULE.map((s, i) => (
              <div key={i} className="flex items-start gap-3">
                <span className="bg-primary-100 text-primary-700 text-senior-sm font-bold px-2 py-1 rounded-lg min-w-[80px] text-center flex-shrink-0">{s.day}</span>
                <div>
                  <p className="text-senior-base font-semibold text-gray-800">{s.time}</p>
                  <p className="text-senior-sm text-gray-500">{s.route}</p>
                </div>
              </div>
            ))}
          </div>
        </section>

        {/* Providers */}
        <section aria-labelledby="providers-heading">
          <h2 id="providers-heading" className="senior-section-title">이동 서비스 제공기관</h2>
          <div className="space-y-3">
            {RIDE_PROVIDERS.map((p) => (
              <div key={p.id} className="senior-card">
                <div className="flex items-start justify-between mb-2">
                  <div>
                    <p className="text-senior-lg font-bold text-gray-900">{p.name}</p>
                    <p className="text-senior-sm text-gray-500">{p.type}</p>
                  </div>
                  <span className="bg-success-50 text-success-700 text-senior-sm font-semibold px-2.5 py-1 rounded-full">
                    이용 가능
                  </span>
                </div>
                <p className="text-senior-sm text-gray-600 mb-2">{p.description}</p>
                <div className="flex items-center justify-between">
                  <span className="text-senior-base font-bold text-primary-700">💰 {p.price}</span>
                  {p.phone && (
                    <a href={`tel:${p.phone}`} className="text-senior-sm text-primary-600 font-medium min-h-touch flex items-center">
                      📞 {p.phone}
                    </a>
                  )}
                </div>
              </div>
            ))}
          </div>
        </section>
      </div>
    </SeniorAppShell>
  );
}
