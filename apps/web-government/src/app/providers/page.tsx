// 기관 디렉토리 — Provider Registry
// Shows registered provider organizations with filters and stats

import { redirect } from 'next/navigation';
import GovernmentAppShell from '@/components/GovernmentAppShell';
import { auth } from '@bominal-senior/auth';
import { pool } from '@bominal-senior/db';

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

interface Provider {
  id: string;
  name: string;
  type: string;
  district: string;
  address: string;
  phone: string;
  director: string;
  capacity: number;
  current: number;
  rating: number;
  certExpiry: string;
  status: string;
  services: string[];
}

const STATUS_BADGE: Record<string, string> = {
  '정상': 'badge-green',
  '주의': 'badge-yellow',
  '위반': 'badge-red',
  '폐업': 'badge-gray',
};

// ---------------------------------------------------------------------------
// Helpers — map DB row to Provider UI shape
// ---------------------------------------------------------------------------

function mapRowToProvider(row: Record<string, unknown>): Provider {
  const certExpiry = row.cert_expiry ? new Date(row.cert_expiry as string) : null;
  const certExpiryStr = certExpiry
    ? `${certExpiry.getFullYear()}-${String(certExpiry.getMonth() + 1).padStart(2, '0')}-${String(certExpiry.getDate()).padStart(2, '0')}`
    : '';

  // Parse services from a DB array or JSON field
  let services: string[] = [];
  if (Array.isArray(row.services)) {
    services = row.services as string[];
  } else if (typeof row.services === 'string') {
    try {
      services = JSON.parse(row.services as string);
    } catch {
      services = [(row.services as string)];
    }
  }

  // Map DB status to Korean labels
  const statusMap: Record<string, string> = {
    ACTIVE: '정상',
    WARNING: '주의',
    VIOLATION: '위반',
    CLOSED: '폐업',
  };
  const rawStatus = row.status as string ?? '';
  const status = statusMap[rawStatus] ?? rawStatus;

  return {
    id: row.id as string,
    name: (row.name as string) ?? '',
    type: (row.type as string) ?? (row.organization_type as string) ?? '',
    district: (row.district as string) ?? '',
    address: (row.address as string) ?? '',
    phone: (row.phone as string) ?? '',
    director: (row.director as string) ?? (row.representative_name as string) ?? '',
    capacity: (row.capacity as number) ?? 0,
    current: (row.current_count as number) ?? (row.current as number) ?? 0,
    rating: (row.rating as number) ?? 0,
    certExpiry: certExpiryStr,
    status,
    services,
  };
}

// ---------------------------------------------------------------------------
// Page (async server component)
// ---------------------------------------------------------------------------

export default async function ProvidersPage() {
  const session = await auth();
  if (!session?.user?.id) {
    redirect('/auth/signin');
  }

  let providers: Provider[] = [];

  try {
    const result = await pool.query(
      'SELECT * FROM provider_organizations ORDER BY name',
    );
    providers = result.rows.map(mapRowToProvider);
  } catch (error) {
    console.error('[ProvidersPage] Failed to fetch providers:', error);
  }

  const total = providers.length;
  const normalCount = providers.filter((p) => p.status === '정상').length;
  const warningCount = providers.filter((p) => p.status === '주의').length;
  const violationCount = providers.filter((p) => p.status === '위반').length;

  return (
    <GovernmentAppShell>
      <div className="space-y-6">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="page-title">기관 디렉토리</h1>
            <p className="text-sm text-gray-500 mt-1">등록 제공 기관 · 총 {total}개소</p>
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

        {/* Stats */}
        <div className="grid grid-cols-4 gap-4">
          <div className="stat-card">
            <p className="text-sm font-medium text-gray-500">전체 기관</p>
            <p className="text-3xl font-bold text-gray-900 mt-1">{total}</p>
          </div>
          <div className="stat-card border-l-4 border-l-green-500">
            <p className="text-sm font-medium text-gray-500">정상 운영</p>
            <p className="text-3xl font-bold text-green-600 mt-1">{normalCount}</p>
          </div>
          <div className="stat-card border-l-4 border-l-yellow-400">
            <p className="text-sm font-medium text-gray-500">주의</p>
            <p className="text-3xl font-bold text-yellow-600 mt-1">{warningCount}</p>
          </div>
          <div className="stat-card border-l-4 border-l-red-500">
            <p className="text-sm font-medium text-gray-500">위반</p>
            <p className="text-3xl font-bold text-red-600 mt-1">{violationCount}</p>
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
                <input type="text" className="input pl-9" placeholder="기관명, 담당자 검색..." />
              </div>
            </div>
            <select className="input w-auto">
              <option>전체 동</option>
              <option>역삼동</option>
              <option>삼성동</option>
              <option>대치동</option>
              <option>개포동</option>
              <option>논현동</option>
              <option>수서동</option>
            </select>
            <select className="input w-auto">
              <option>전체 유형</option>
              <option>방문요양</option>
              <option>재가복지</option>
              <option>시설</option>
            </select>
            <select className="input w-auto">
              <option>전체 상태</option>
              <option>정상</option>
              <option>주의</option>
              <option>위반</option>
            </select>
            <button className="btn-secondary text-sm">초기화</button>
          </div>
        </div>

        {/* Table */}
        <div className="card overflow-hidden">
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead className="bg-slate-50 border-b border-slate-200">
                <tr>
                  <th className="table-header">기관명</th>
                  <th className="table-header">유형</th>
                  <th className="table-header">위치</th>
                  <th className="table-header">대표 서비스</th>
                  <th className="table-header">원장</th>
                  <th className="table-header">정원/현원</th>
                  <th className="table-header">평점</th>
                  <th className="table-header">지정 만료</th>
                  <th className="table-header">상태</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-slate-100">
                {providers.length === 0 ? (
                  <tr>
                    <td colSpan={9} className="text-center py-12 text-gray-400">
                      등록된 기관이 없습니다
                    </td>
                  </tr>
                ) : (
                  providers.map((provider) => (
                    <tr key={provider.id} className="hover:bg-slate-50 transition-colors">
                      <td className="table-cell">
                        <div>
                          <p className="font-medium text-gray-900">{provider.name}</p>
                          <p className="text-xs text-gray-500">{provider.phone}</p>
                        </div>
                      </td>
                      <td className="table-cell">
                        <span className="badge-blue">{provider.type}</span>
                      </td>
                      <td className="table-cell">
                        <p className="text-sm">{provider.district}</p>
                        <p className="text-xs text-gray-500 truncate max-w-36">{provider.address}</p>
                      </td>
                      <td className="table-cell">
                        <div className="flex flex-wrap gap-1">
                          {provider.services.map((s) => (
                            <span key={s} className="badge-gray text-xs">{s}</span>
                          ))}
                        </div>
                      </td>
                      <td className="table-cell">{provider.director}</td>
                      <td className="table-cell">
                        <div>
                          <div className="flex items-center gap-1 text-sm">
                            <span className="font-medium">{provider.current}</span>
                            <span className="text-gray-400">/</span>
                            <span className="text-gray-500">{provider.capacity}</span>
                          </div>
                          {provider.capacity > 0 && (
                            <div className="w-24 bg-gray-100 rounded-full h-1.5 mt-1">
                              <div
                                className="bg-indigo-500 h-1.5 rounded-full"
                                style={{ width: `${(provider.current / provider.capacity) * 100}%` }}
                              />
                            </div>
                          )}
                        </div>
                      </td>
                      <td className="table-cell">
                        <div className="flex items-center gap-1">
                          <svg className="w-3 h-3 text-yellow-500" fill="currentColor" viewBox="0 0 20 20">
                            <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z" />
                          </svg>
                          <span className="text-sm font-medium">{provider.rating}</span>
                        </div>
                      </td>
                      <td className="table-cell text-gray-500 text-xs">{provider.certExpiry}</td>
                      <td className="table-cell">
                        <span className={STATUS_BADGE[provider.status] ?? 'badge-gray'}>{provider.status}</span>
                      </td>
                    </tr>
                  ))
                )}
              </tbody>
            </table>
          </div>
          <div className="flex items-center justify-between px-5 py-3 border-t border-slate-100 bg-slate-50">
            <p className="text-sm text-gray-500">총 {total}개 기관 표시 중</p>
            <div className="flex items-center gap-1">
              <button className="px-3 py-1 text-sm text-gray-500 hover:bg-gray-200 rounded">이전</button>
              <button className="px-3 py-1 text-sm bg-indigo-600 text-white rounded">1</button>
              <button className="px-3 py-1 text-sm text-gray-500 hover:bg-gray-200 rounded">다음</button>
            </div>
          </div>
        </div>
      </div>
    </GovernmentAppShell>
  );
}
