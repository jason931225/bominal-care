'use client';

import { useState } from 'react';
import FamilyAppShell from '@/components/FamilyAppShell';

const INITIAL_PROFILE = {
  name: '김가족',
  email: 'family@example.com',
  phone: '010-9999-8888',
  relation: '자녀',
  address: '서울특별시 강남구 역삼동 123-45',
  emergencyContact: '010-7777-6666',
  emergencyName: '김형제',
  bio: '어머니의 건강과 케어를 직접 관리하고 있습니다.',
};

const MANAGED_SENIORS = [
  {
    id: 'senior-1',
    name: '김복순',
    relation: '모',
    age: 78,
    grade: '3등급',
    status: 'active',
    caseManager: '최지원',
  },
  {
    id: 'senior-2',
    name: '이정남',
    relation: '부',
    age: 81,
    grade: '2등급',
    status: 'active',
    caseManager: '박담당',
  },
];

const ACTIVITY_LOG = [
  { id: 1, action: '승인 처리 — 물리치료 횟수 증가', date: '2026-03-13', icon: '✅' },
  { id: 2, action: '계약서 확인 — 방문 요양 갱신', date: '2026-03-10', icon: '📄' },
  { id: 3, action: '대리 예약 — 병원 동행 (강남성모병원)', date: '2026-03-08', icon: '📅' },
];

export default function ProfilePage() {
  const [profile, setProfile] = useState(INITIAL_PROFILE);
  const [editing, setEditing] = useState(false);
  const [editForm, setEditForm] = useState(INITIAL_PROFILE);

  const updateEdit = (field: string, value: string) => {
    setEditForm((prev) => ({ ...prev, [field]: value }));
  };

  const handleSave = () => {
    setProfile(editForm);
    setEditing(false);
  };

  const handleCancel = () => {
    setEditForm(profile);
    setEditing(false);
  };

  return (
    <FamilyAppShell>
      <div className="max-w-2xl mx-auto px-4 py-6">
        {/* Header */}
        <div className="flex items-center justify-between mb-6">
          <h1 className="text-2xl font-bold text-gray-900">내 프로필</h1>
          {!editing && (
            <button
              onClick={() => { setEditForm(profile); setEditing(true); }}
              className="px-4 py-2 border border-gray-300 text-gray-700 text-sm font-medium rounded-lg hover:bg-gray-50 transition-colors"
            >
              수정
            </button>
          )}
        </div>

        {/* Profile Card */}
        <div className="bg-white border border-gray-200 rounded-xl p-5 mb-5">
          <div className="flex items-center gap-4 mb-5">
            <div className="w-16 h-16 rounded-full bg-blue-600 text-white text-2xl font-bold flex items-center justify-center flex-shrink-0">
              {profile.name.charAt(0)}
            </div>
            <div>
              <h2 className="text-xl font-bold text-gray-900">{profile.name}</h2>
              <p className="text-sm text-gray-500">{profile.relation} · {profile.email}</p>
              <span className="text-xs bg-blue-50 text-blue-700 px-2 py-0.5 rounded-full border border-blue-200 font-medium mt-1 inline-block">
                가족 보호자
              </span>
            </div>
          </div>

          {!editing ? (
            <div className="space-y-3">
              {[
                { label: '연락처', value: profile.phone },
                { label: '주소', value: profile.address },
                { label: '비상 연락처', value: `${profile.emergencyName} (${profile.emergencyContact})` },
                { label: '소개', value: profile.bio },
              ].map((item) => (
                <div key={item.label} className="flex gap-4">
                  <span className="text-sm text-gray-400 w-24 flex-shrink-0">{item.label}</span>
                  <span className="text-sm text-gray-800 flex-1">{item.value}</span>
                </div>
              ))}
            </div>
          ) : (
            <div className="space-y-4">
              <div className="grid sm:grid-cols-2 gap-4">
                <div>
                  <label className="block text-xs font-medium text-gray-600 mb-1">성명</label>
                  <input type="text" value={editForm.name} onChange={(e) => updateEdit('name', e.target.value)}
                    className="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-blue-500" />
                </div>
                <div>
                  <label className="block text-xs font-medium text-gray-600 mb-1">연락처</label>
                  <input type="tel" value={editForm.phone} onChange={(e) => updateEdit('phone', e.target.value)}
                    className="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-blue-500" />
                </div>
                <div className="sm:col-span-2">
                  <label className="block text-xs font-medium text-gray-600 mb-1">주소</label>
                  <input type="text" value={editForm.address} onChange={(e) => updateEdit('address', e.target.value)}
                    className="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-blue-500" />
                </div>
                <div>
                  <label className="block text-xs font-medium text-gray-600 mb-1">비상 연락처 이름</label>
                  <input type="text" value={editForm.emergencyName} onChange={(e) => updateEdit('emergencyName', e.target.value)}
                    className="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-blue-500" />
                </div>
                <div>
                  <label className="block text-xs font-medium text-gray-600 mb-1">비상 연락처</label>
                  <input type="tel" value={editForm.emergencyContact} onChange={(e) => updateEdit('emergencyContact', e.target.value)}
                    className="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-blue-500" />
                </div>
                <div className="sm:col-span-2">
                  <label className="block text-xs font-medium text-gray-600 mb-1">소개</label>
                  <textarea value={editForm.bio} onChange={(e) => updateEdit('bio', e.target.value)} rows={2}
                    className="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 resize-none" />
                </div>
              </div>
              <div className="flex gap-3">
                <button onClick={handleCancel} className="flex-1 py-2 border border-gray-300 text-gray-700 text-sm font-medium rounded-lg hover:bg-gray-50 transition-colors">
                  취소
                </button>
                <button onClick={handleSave} className="flex-1 py-2 bg-blue-600 text-white text-sm font-semibold rounded-lg hover:bg-blue-700 transition-colors">
                  저장
                </button>
              </div>
            </div>
          )}
        </div>

        {/* Managed Seniors */}
        <div className="bg-white border border-gray-200 rounded-xl p-5 mb-5">
          <h3 className="font-bold text-gray-900 mb-3">관리 중인 어르신</h3>
          <div className="space-y-3">
            {MANAGED_SENIORS.map((senior) => (
              <div key={senior.id} className="flex items-center gap-3 p-3 bg-gray-50 rounded-xl">
                <span className="text-2xl">👴</span>
                <div className="flex-1">
                  <p className="font-semibold text-gray-900 text-sm">
                    {senior.name} ({senior.relation}) · {senior.age}세
                  </p>
                  <p className="text-xs text-gray-500 mt-0.5">
                    {senior.grade} · 케어매니저: {senior.caseManager}
                  </p>
                </div>
                <span className="text-xs font-medium text-green-700 bg-green-50 px-2 py-0.5 rounded-full border border-green-200">
                  관리 중
                </span>
              </div>
            ))}
          </div>
        </div>

        {/* Activity Log */}
        <div className="bg-white border border-gray-200 rounded-xl p-5">
          <h3 className="font-bold text-gray-900 mb-3">최근 활동</h3>
          <div className="space-y-3">
            {ACTIVITY_LOG.map((log) => (
              <div key={log.id} className="flex items-start gap-3">
                <span className="text-xl flex-shrink-0">{log.icon}</span>
                <div>
                  <p className="text-sm text-gray-700">{log.action}</p>
                  <p className="text-xs text-gray-400 mt-0.5">{log.date}</p>
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </FamilyAppShell>
  );
}
