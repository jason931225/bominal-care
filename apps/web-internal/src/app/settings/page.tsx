'use client';

import InternalAppShell from '@/components/InternalAppShell';
import { useState } from 'react';

type Tab = 'org' | 'users' | 'notifications' | 'billing' | 'integration';

const TABS: { id: Tab; label: string }[] = [
  { id: 'org', label: '기관 정보' },
  { id: 'users', label: '사용자 관리' },
  { id: 'notifications', label: '알림 설정' },
  { id: 'billing', label: '청구 설정' },
  { id: 'integration', label: '연동 관리' },
];

const USERS = [
  { name: '김관리자', email: 'kim@happycare.co.kr', role: '시설장', status: '활성', lastLogin: '2026-03-15' },
  { name: '이팀장', email: 'lee@happycare.co.kr', role: '팀장', status: '활성', lastLogin: '2026-03-15' },
  { name: '박담당', email: 'park@happycare.co.kr', role: '담당자', status: '활성', lastLogin: '2026-03-14' },
  { name: '최직원', email: 'choi@happycare.co.kr', role: '담당자', status: '비활성', lastLogin: '2026-02-20' },
];

function OrgSettings() {
  return (
    <div className="space-y-6">
      <div className="card p-6">
        <h3 className="section-title mb-5">기관 기본 정보</h3>
        <div className="grid grid-cols-2 gap-5">
          <div>
            <label className="label">기관명</label>
            <input type="text" className="input" defaultValue="행복노인복지센터" />
          </div>
          <div>
            <label className="label">사업자 등록번호</label>
            <input type="text" className="input" defaultValue="123-45-67890" />
          </div>
          <div>
            <label className="label">대표자명</label>
            <input type="text" className="input" defaultValue="김복지" />
          </div>
          <div>
            <label className="label">대표 전화</label>
            <input type="tel" className="input" defaultValue="02-1234-5678" />
          </div>
          <div className="col-span-2">
            <label className="label">주소</label>
            <input type="text" className="input" defaultValue="서울특별시 강남구 역삼동 123-45 복지빌딩 3층" />
          </div>
          <div>
            <label className="label">장기요양기관 기호</label>
            <input type="text" className="input" defaultValue="A12345678" />
          </div>
          <div>
            <label className="label">지정 유효기간</label>
            <input type="text" className="input" defaultValue="2027-03-01" />
          </div>
        </div>
      </div>

      <div className="card p-6">
        <h3 className="section-title mb-5">제공 서비스</h3>
        <div className="space-y-3">
          {['방문요양', '방문목욕', '방문간호'].map((svc) => (
            <label key={svc} className="flex items-center gap-3 cursor-pointer">
              <input
                type="checkbox"
                className="w-4 h-4 text-blue-600 rounded"
                defaultChecked={svc !== '방문간호'}
              />
              <span className="text-sm text-gray-800">{svc}</span>
            </label>
          ))}
        </div>
      </div>

      <div className="flex justify-end gap-3">
        <button className="btn-secondary">취소</button>
        <button className="btn-primary">저장</button>
      </div>
    </div>
  );
}

function UsersSettings() {
  return (
    <div className="space-y-4">
      <div className="flex justify-between items-center">
        <p className="text-sm text-gray-600">총 {USERS.length}명의 사용자</p>
        <button className="btn-primary text-sm">
          <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
            <path strokeLinecap="round" strokeLinejoin="round" d="M12 4v16m8-8H4" />
          </svg>
          사용자 초대
        </button>
      </div>
      <div className="card overflow-hidden">
        <table className="w-full">
          <thead className="bg-gray-50 border-b border-gray-200">
            <tr>
              <th className="table-header">이름</th>
              <th className="table-header">이메일</th>
              <th className="table-header">역할</th>
              <th className="table-header">마지막 로그인</th>
              <th className="table-header">상태</th>
              <th className="table-header"></th>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-100">
            {USERS.map((user) => (
              <tr key={user.name} className="hover:bg-gray-50">
                <td className="table-cell">
                  <div className="flex items-center gap-2">
                    <div className="w-8 h-8 bg-blue-100 rounded-full flex items-center justify-center">
                      <span className="text-xs font-semibold text-blue-700">{user.name[0]}</span>
                    </div>
                    <span className="font-medium">{user.name}</span>
                  </div>
                </td>
                <td className="table-cell text-gray-500">{user.email}</td>
                <td className="table-cell">
                  <span className="badge-blue">{user.role}</span>
                </td>
                <td className="table-cell text-gray-500">{user.lastLogin}</td>
                <td className="table-cell">
                  <span className={user.status === '활성' ? 'badge-green' : 'badge-gray'}>{user.status}</span>
                </td>
                <td className="table-cell">
                  <button className="text-blue-600 hover:text-blue-800 text-xs font-medium">수정</button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}

function NotificationSettings() {
  return (
    <div className="card p-6">
      <h3 className="section-title mb-5">알림 설정</h3>
      <div className="space-y-5">
        {[
          { category: '일정 관련', items: ['방문 시작 30분 전 알림', '미이행 방문 알림', '일정 충돌 감지'] },
          { category: '이용자 관련', items: ['건강 이상 보고', '케어 플랜 갱신 알림', '이용자 불만 접수'] },
          { category: '인력 관련', items: ['요양보호사 지각/결근 알림', '자격증 만료 사전 알림', '신규 채용 신청 접수'] },
          { category: '규정/청구 관련', items: ['서류 기한 만료 알림', '청구 오류 감지', '현지조사 일정 알림'] },
        ].map((group) => (
          <div key={group.category}>
            <h4 className="text-sm font-semibold text-gray-700 mb-3">{group.category}</h4>
            <div className="space-y-2 pl-2">
              {group.items.map((item) => (
                <label key={item} className="flex items-center justify-between cursor-pointer">
                  <span className="text-sm text-gray-700">{item}</span>
                  <div className="flex items-center gap-1 bg-blue-100 rounded-full p-0.5 w-10 justify-end">
                    <div className="w-4 h-4 bg-blue-600 rounded-full" />
                  </div>
                </label>
              ))}
            </div>
          </div>
        ))}
        <div className="flex justify-end gap-3 pt-2">
          <button className="btn-secondary">초기화</button>
          <button className="btn-primary">저장</button>
        </div>
      </div>
    </div>
  );
}

function BillingSettings() {
  return (
    <div className="space-y-6">
      <div className="card p-6">
        <h3 className="section-title mb-5">청구 기본 설정</h3>
        <div className="grid grid-cols-2 gap-5">
          <div>
            <label className="label">청구 마감일</label>
            <select className="input">
              <option>매월 15일</option>
              <option>매월 말일</option>
            </select>
          </div>
          <div>
            <label className="label">기본 청구 형식</label>
            <select className="input">
              <option>장기요양 전산청구</option>
              <option>수기 청구</option>
            </select>
          </div>
          <div>
            <label className="label">계좌번호</label>
            <input type="text" className="input" defaultValue="국민은행 123-456-789012" />
          </div>
          <div>
            <label className="label">예금주</label>
            <input type="text" className="input" defaultValue="행복노인복지센터" />
          </div>
        </div>
      </div>
      <div className="flex justify-end gap-3">
        <button className="btn-secondary">취소</button>
        <button className="btn-primary">저장</button>
      </div>
    </div>
  );
}

function IntegrationSettings() {
  return (
    <div className="card p-6">
      <h3 className="section-title mb-5">외부 시스템 연동</h3>
      <div className="space-y-4">
        {[
          { name: '장기요양 전산청구 시스템 (NHIS)', status: '연결됨', lastSync: '2026-03-15 09:00' },
          { name: '카카오 알림톡', status: '연결됨', lastSync: '2026-03-15 11:30' },
          { name: '전자서명 시스템', status: '미연결', lastSync: '-' },
          { name: '회계 프로그램 연동', status: '미연결', lastSync: '-' },
        ].map((item, idx) => (
          <div key={idx} className="flex items-center justify-between p-4 border border-gray-200 rounded-xl">
            <div>
              <p className="font-medium text-gray-900">{item.name}</p>
              <p className="text-xs text-gray-500 mt-0.5">마지막 동기화: {item.lastSync}</p>
            </div>
            <div className="flex items-center gap-3">
              <span className={item.status === '연결됨' ? 'badge-green' : 'badge-gray'}>{item.status}</span>
              <button className="btn-secondary text-xs py-1.5">
                {item.status === '연결됨' ? '설정' : '연결'}
              </button>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}

export default function SettingsPage() {
  const [activeTab, setActiveTab] = useState<Tab>('org');

  return (
    <InternalAppShell>
      <div className="space-y-6">
        <h1 className="page-title">기관 설정</h1>

        {/* Tabs */}
        <div className="border-b border-gray-200">
          <nav className="flex gap-1">
            {TABS.map((tab) => (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id)}
                className={`px-4 py-2.5 text-sm font-medium border-b-2 -mb-px transition-colors ${
                  activeTab === tab.id
                    ? 'border-blue-600 text-blue-700'
                    : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                }`}
              >
                {tab.label}
              </button>
            ))}
          </nav>
        </div>

        {/* Tab content */}
        <div>
          {activeTab === 'org' && <OrgSettings />}
          {activeTab === 'users' && <UsersSettings />}
          {activeTab === 'notifications' && <NotificationSettings />}
          {activeTab === 'billing' && <BillingSettings />}
          {activeTab === 'integration' && <IntegrationSettings />}
        </div>
      </div>
    </InternalAppShell>
  );
}
