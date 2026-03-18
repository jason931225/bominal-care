import Link from 'next/link';
import CaregiverAppShell from '@/components/CaregiverAppShell';

const PROFILE = {
  name: '김요양',
  age: 38,
  phone: '010-1234-5678',
  email: 'caregiver@example.com',
  certNumber: 'CA-2018-048291',
  certDate: '2018-03-15',
  employedDate: '2022-06-01',
  status: '활동 중',
  rating: 4.9,
  reviewCount: 127,
  completedVisits: 1842,
  totalHoursThisMonth: 68.5,
  earningsThisMonth: '1,043,255원',
  regions: ['강남구', '서초구', '송파구'],
  services: ['목욕 지원', '식사 지원', '가사 지원', '투약 보조', '말벗 서비스'],
  schedule: ['월 오전', '월 오후', '화 오전', '수 오전', '수 오후', '목 오전', '금 오전', '금 오후'],
};

const MENU_ITEMS = [
  { icon: '🗓️', label: '가능 시간 설정', href: '/profile/availability', badge: null },
  { icon: '📋', label: '케어일지 기록', href: '/notes', badge: null },
  { icon: '🔔', label: '알림 설정', href: '/notifications', badge: null },
  { icon: '📞', label: '고객센터 문의', href: '#', badge: null },
  { icon: '❓', label: '도움말 / FAQ', href: '#', badge: null },
  { icon: '📄', label: '개인정보처리방침', href: '#', badge: null },
];

export default function ProfilePage() {
  return (
    <CaregiverAppShell activeTab="profile" title="내 정보">
      <div className="px-4 py-4 space-y-5">
        {/* Profile Header */}
        <div className="card">
          <div className="flex items-start gap-4 mb-4">
            <div className="w-16 h-16 bg-blue-100 rounded-2xl flex items-center justify-center text-3xl flex-shrink-0">
              👩‍⚕️
            </div>
            <div className="flex-1">
              <div className="flex items-center gap-2">
                <h2 className="text-xl font-bold text-slate-900">{PROFILE.name}</h2>
                <span className="badge-success">{PROFILE.status}</span>
              </div>
              <p className="text-sm text-slate-500 mt-0.5">{PROFILE.age}세 · 요양보호사</p>
              <p className="text-sm text-slate-500">{PROFILE.phone}</p>
            </div>
          </div>
          <div className="grid grid-cols-3 gap-3 pt-4 border-t border-slate-100">
            <div className="text-center">
              <p className="text-xl font-bold text-amber-500">{PROFILE.rating}</p>
              <p className="text-xs text-slate-400 mt-0.5">평점 ⭐</p>
            </div>
            <div className="text-center">
              <p className="text-xl font-bold text-blue-600">{PROFILE.completedVisits}</p>
              <p className="text-xs text-slate-400 mt-0.5">완료 방문</p>
            </div>
            <div className="text-center">
              <p className="text-xl font-bold text-slate-700">{PROFILE.reviewCount}</p>
              <p className="text-xs text-slate-400 mt-0.5">리뷰</p>
            </div>
          </div>
        </div>

        {/* This Month */}
        <div className="card">
          <h3 className="section-title">이번 달 현황</h3>
          <div className="space-y-3">
            <div className="flex items-center justify-between py-2 border-b border-slate-100">
              <span className="text-sm text-slate-600">근무 시간</span>
              <span className="text-sm font-bold text-slate-800">{PROFILE.totalHoursThisMonth}시간</span>
            </div>
            <div className="flex items-center justify-between py-2">
              <span className="text-sm text-slate-600">예상 수당</span>
              <span className="text-sm font-bold text-green-600">{PROFILE.earningsThisMonth}</span>
            </div>
          </div>
        </div>

        {/* Certificate */}
        <div className="card">
          <h3 className="section-title">자격 정보</h3>
          <div className="space-y-2.5">
            <div className="flex items-center justify-between">
              <span className="text-sm text-slate-500">자격증 번호</span>
              <span className="text-sm font-medium text-slate-800">{PROFILE.certNumber}</span>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-sm text-slate-500">취득일</span>
              <span className="text-sm font-medium text-slate-800">{PROFILE.certDate}</span>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-sm text-slate-500">입사일</span>
              <span className="text-sm font-medium text-slate-800">{PROFILE.employedDate}</span>
            </div>
          </div>
        </div>

        {/* Service Areas */}
        <div className="card">
          <div className="flex items-center justify-between mb-3">
            <h3 className="section-title mb-0">활동 지역</h3>
            <Link href="/profile/availability" className="text-xs text-blue-600 font-medium">수정</Link>
          </div>
          <div className="flex flex-wrap gap-2">
            {PROFILE.regions.map((r) => (
              <span key={r} className="badge-info">{r}</span>
            ))}
          </div>
        </div>

        {/* Schedule */}
        <div className="card">
          <div className="flex items-center justify-between mb-3">
            <h3 className="section-title mb-0">근무 가능 시간</h3>
            <Link href="/profile/availability" className="text-xs text-blue-600 font-medium">수정</Link>
          </div>
          <div className="flex flex-wrap gap-2">
            {PROFILE.schedule.map((s) => (
              <span key={s} className="badge-info">{s}</span>
            ))}
          </div>
        </div>

        {/* Services */}
        <div className="card">
          <h3 className="section-title">제공 서비스</h3>
          <div className="flex flex-wrap gap-2">
            {PROFILE.services.map((s) => (
              <span key={s} className="badge-info">{s}</span>
            ))}
          </div>
        </div>

        {/* Menu */}
        <div className="card">
          <h3 className="section-title">설정 및 지원</h3>
          <div className="space-y-0">
            {MENU_ITEMS.map((item, idx) => (
              <Link key={idx} href={item.href}>
                <div className="flex items-center gap-3 py-3.5 border-b border-slate-100 last:border-0 active:bg-slate-50 -mx-4 px-4 transition-colors">
                  <span className="text-xl">{item.icon}</span>
                  <span className="text-sm text-slate-700 flex-1">{item.label}</span>
                  {item.badge && (
                    <span className="badge-danger">{item.badge}</span>
                  )}
                  <svg className="w-4 h-4 text-slate-300" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" d="M9 5l7 7-7 7" />
                  </svg>
                </div>
              </Link>
            ))}
          </div>
        </div>

        {/* Logout */}
        <div className="pb-6">
          <button type="button" className="btn-danger py-3 text-sm">
            로그아웃
          </button>
          <p className="text-xs text-center text-slate-300 mt-4">
            버전 1.0.0 · © 2026 시니어케어
          </p>
        </div>
      </div>
    </CaregiverAppShell>
  );
}
