'use client';

import { useState, useEffect, useCallback } from 'react';
import GovernmentAppShell from '@/components/GovernmentAppShell';

interface AuditLogEntry {
  id: string;
  user_id: string | null;
  action: string;
  entity_type: string | null;
  entity_id: string | null;
  old_value: unknown;
  new_value: unknown;
  ip_address: string | null;
  user_agent: string | null;
  created_at: string;
}

interface AuditApiResponse {
  success: boolean;
  data: AuditLogEntry[] | null;
  error: string | null;
  meta: { total: number; page: number; limit: number; totalPages: number } | null;
}

const ACTION_BADGE: Record<string, string> = {
  CREATE: 'badge-green',
  UPDATE: 'badge-blue',
  DELETE: 'badge-red',
  LOGIN: 'badge-yellow',
  LOGOUT: 'badge-gray',
  APPROVE: 'badge-green',
  REJECT: 'badge-red',
  VIEW: 'badge-gray',
};

const MOCK_LOGS: AuditLogEntry[] = [
  {
    id: 'AL-001',
    user_id: 'USR-001',
    action: 'CREATE',
    entity_type: 'eligibility_case',
    entity_id: 'EC-2026-006',
    old_value: null,
    new_value: { applicant: '정만복', type: '장기요양 수급 신청' },
    ip_address: '192.168.1.100',
    user_agent: 'Mozilla/5.0',
    created_at: '2026-03-15T14:32:00Z',
  },
  {
    id: 'AL-002',
    user_id: 'USR-002',
    action: 'APPROVE',
    entity_type: 'eligibility_case',
    entity_id: 'EC-2026-004',
    old_value: { status: '심사중' },
    new_value: { status: '승인' },
    ip_address: '192.168.1.101',
    user_agent: 'Mozilla/5.0',
    created_at: '2026-03-15T11:20:00Z',
  },
  {
    id: 'AL-003',
    user_id: 'USR-001',
    action: 'UPDATE',
    entity_type: 'provider_organization',
    entity_id: 'P003',
    old_value: { status: '정상' },
    new_value: { status: '주의' },
    ip_address: '192.168.1.100',
    user_agent: 'Mozilla/5.0',
    created_at: '2026-03-15T10:15:00Z',
  },
  {
    id: 'AL-004',
    user_id: 'USR-003',
    action: 'LOGIN',
    entity_type: 'session',
    entity_id: null,
    old_value: null,
    new_value: null,
    ip_address: '10.0.0.50',
    user_agent: 'Mozilla/5.0',
    created_at: '2026-03-15T09:00:00Z',
  },
  {
    id: 'AL-005',
    user_id: 'USR-002',
    action: 'REJECT',
    entity_type: 'eligibility_case',
    entity_id: 'EC-2026-003',
    old_value: { status: '심사중' },
    new_value: { status: '반려', reason: '서류 미비' },
    ip_address: '192.168.1.101',
    user_agent: 'Mozilla/5.0',
    created_at: '2026-03-14T16:45:00Z',
  },
  {
    id: 'AL-006',
    user_id: 'USR-001',
    action: 'CREATE',
    entity_type: 'program',
    entity_id: 'PRG-007',
    old_value: null,
    new_value: { name: '노인 교통 안전 지원 사업' },
    ip_address: '192.168.1.100',
    user_agent: 'Mozilla/5.0',
    created_at: '2026-03-14T13:00:00Z',
  },
  {
    id: 'AL-007',
    user_id: 'USR-001',
    action: 'VIEW',
    entity_type: 'audit_logs',
    entity_id: null,
    old_value: null,
    new_value: null,
    ip_address: '192.168.1.100',
    user_agent: 'Mozilla/5.0',
    created_at: '2026-03-14T10:30:00Z',
  },
  {
    id: 'AL-008',
    user_id: 'USR-002',
    action: 'UPDATE',
    entity_type: 'eligibility_case',
    entity_id: 'EC-2026-005',
    old_value: { reviewer: '미배정' },
    new_value: { reviewer: '김심사관' },
    ip_address: '192.168.1.101',
    user_agent: 'Mozilla/5.0',
    created_at: '2026-03-13T15:20:00Z',
  },
];

const USER_MAP: Record<string, string> = {
  'USR-001': '이담당자',
  'USR-002': '김심사관',
  'USR-003': '박관리자',
};

const ENTITY_TYPE_LABEL: Record<string, string> = {
  eligibility_case: '수급 자격 심사',
  provider_organization: '제공 기관',
  program: '프로그램',
  session: '세션',
  audit_logs: '감사 로그',
};

const ACTION_LABEL: Record<string, string> = {
  CREATE: '생성',
  UPDATE: '수정',
  DELETE: '삭제',
  LOGIN: '로그인',
  LOGOUT: '로그아웃',
  APPROVE: '승인',
  REJECT: '반려',
  VIEW: '조회',
};

function formatTimestamp(iso: string): string {
  const date = new Date(iso);
  const yyyy = date.getFullYear();
  const mm = String(date.getMonth() + 1).padStart(2, '0');
  const dd = String(date.getDate()).padStart(2, '0');
  const hh = String(date.getHours()).padStart(2, '0');
  const min = String(date.getMinutes()).padStart(2, '0');
  return `${yyyy}-${mm}-${dd} ${hh}:${min}`;
}

function formatDetails(entry: AuditLogEntry): string {
  if (entry.old_value !== null && entry.new_value !== null) {
    const oldStr = JSON.stringify(entry.old_value);
    const newStr = JSON.stringify(entry.new_value);
    return `${oldStr} → ${newStr}`;
  }
  if (entry.new_value !== null) {
    return JSON.stringify(entry.new_value);
  }
  return '-';
}

export default function AuditPage() {
  const [logs, setLogs] = useState<AuditLogEntry[]>([]);
  const [loading, setLoading] = useState(true);
  const [searchQuery, setSearchQuery] = useState('');
  const [actionFilter, setActionFilter] = useState('');
  const [entityTypeFilter, setEntityTypeFilter] = useState('');

  const fetchLogs = useCallback(async () => {
    setLoading(true);
    try {
      const params = new URLSearchParams();
      if (actionFilter) params.set('action', actionFilter);
      if (entityTypeFilter) params.set('entityType', entityTypeFilter);

      const response = await fetch(`/api/audit?${params.toString()}`);
      const result: AuditApiResponse = await response.json();

      if (result.success && result.data !== null) {
        setLogs(result.data);
      } else {
        // Fall back to mock data if API is not available
        setLogs(MOCK_LOGS);
      }
    } catch {
      // Fall back to mock data
      setLogs(MOCK_LOGS);
    } finally {
      setLoading(false);
    }
  }, [actionFilter, entityTypeFilter]);

  useEffect(() => {
    fetchLogs();
  }, [fetchLogs]);

  const filteredLogs = logs.filter((log) => {
    if (searchQuery === '') return true;
    const query = searchQuery.toLowerCase();
    const userName = (log.user_id ? USER_MAP[log.user_id] : '') ?? '';
    const entityLabel = (log.entity_type ? ENTITY_TYPE_LABEL[log.entity_type] : '') ?? '';
    return (
      userName.toLowerCase().includes(query) ||
      entityLabel.toLowerCase().includes(query) ||
      (log.entity_id ?? '').toLowerCase().includes(query) ||
      log.action.toLowerCase().includes(query)
    );
  });

  return (
    <GovernmentAppShell>
      <div className="space-y-6">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="page-title">감사 로그</h1>
            <p className="text-sm text-gray-500 mt-1">시스템 활동 기록 · 모든 변경사항이 기록됩니다</p>
          </div>
          <div className="flex gap-2">
            <button className="btn-secondary">
              <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                <path strokeLinecap="round" strokeLinejoin="round" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
              </svg>
              내보내기
            </button>
          </div>
        </div>

        {/* Filters */}
        <div className="card p-4">
          <div className="flex items-center gap-3 flex-wrap">
            <div className="flex-1 min-w-64">
              <div className="relative">
                <svg className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                  <path strokeLinecap="round" strokeLinejoin="round" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                </svg>
                <input
                  type="text"
                  className="input pl-9"
                  placeholder="사용자, 대상, ID 검색..."
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                />
              </div>
            </div>
            <select
              className="input w-auto"
              value={actionFilter}
              onChange={(e) => setActionFilter(e.target.value)}
            >
              <option value="">전체 동작</option>
              <option value="CREATE">생성</option>
              <option value="UPDATE">수정</option>
              <option value="DELETE">삭제</option>
              <option value="LOGIN">로그인</option>
              <option value="APPROVE">승인</option>
              <option value="REJECT">반려</option>
            </select>
            <select
              className="input w-auto"
              value={entityTypeFilter}
              onChange={(e) => setEntityTypeFilter(e.target.value)}
            >
              <option value="">전체 대상</option>
              <option value="eligibility_case">수급 자격 심사</option>
              <option value="provider_organization">제공 기관</option>
              <option value="program">프로그램</option>
              <option value="session">세션</option>
            </select>
            <button
              className="btn-secondary text-sm"
              onClick={() => {
                setSearchQuery('');
                setActionFilter('');
                setEntityTypeFilter('');
              }}
            >
              초기화
            </button>
          </div>
        </div>

        {/* Table */}
        <div className="card overflow-hidden">
          {loading ? (
            <div className="flex items-center justify-center py-16">
              <p className="text-sm text-gray-500">로딩 중...</p>
            </div>
          ) : (
            <>
              <div className="overflow-x-auto">
                <table className="w-full">
                  <thead className="bg-slate-50 border-b border-slate-200">
                    <tr>
                      <th className="table-header">일시</th>
                      <th className="table-header">사용자</th>
                      <th className="table-header">동작</th>
                      <th className="table-header">대상</th>
                      <th className="table-header">상세</th>
                      <th className="table-header">IP</th>
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-slate-100">
                    {filteredLogs.map((log) => (
                      <tr key={log.id} className="hover:bg-slate-50 transition-colors">
                        <td className="table-cell text-xs text-gray-500 whitespace-nowrap">
                          {formatTimestamp(log.created_at)}
                        </td>
                        <td className="table-cell">
                          <p className="text-sm font-medium text-gray-900">
                            {log.user_id ? (USER_MAP[log.user_id] ?? log.user_id) : '시스템'}
                          </p>
                        </td>
                        <td className="table-cell">
                          <span className={ACTION_BADGE[log.action] ?? 'badge-gray'}>
                            {ACTION_LABEL[log.action] ?? log.action}
                          </span>
                        </td>
                        <td className="table-cell">
                          <div>
                            <p className="text-sm text-gray-900">
                              {log.entity_type ? (ENTITY_TYPE_LABEL[log.entity_type] ?? log.entity_type) : '-'}
                            </p>
                            {log.entity_id && (
                              <p className="text-xs text-gray-500">{log.entity_id}</p>
                            )}
                          </div>
                        </td>
                        <td className="table-cell">
                          <p className="text-xs text-gray-500 max-w-48 truncate" title={formatDetails(log)}>
                            {formatDetails(log)}
                          </p>
                        </td>
                        <td className="table-cell text-xs text-gray-400">{log.ip_address ?? '-'}</td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
              <div className="flex items-center justify-between px-5 py-3 border-t border-slate-100 bg-slate-50">
                <p className="text-sm text-gray-500">총 {filteredLogs.length}건 표시 중</p>
                <div className="flex items-center gap-1">
                  <button className="px-3 py-1 text-sm text-gray-500 hover:bg-gray-200 rounded">이전</button>
                  <button className="px-3 py-1 text-sm bg-indigo-600 text-white rounded">1</button>
                  <button className="px-3 py-1 text-sm text-gray-500 hover:bg-gray-200 rounded">다음</button>
                </div>
              </div>
            </>
          )}
        </div>
      </div>
    </GovernmentAppShell>
  );
}
