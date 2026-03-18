import Link from 'next/link';
import FamilyAppShell from '@/components/FamilyAppShell';

const CASE = {
  caseNo: 'ELG-2025-038291',
  seniorName: '김복순',
  currentGrade: '3등급',
  gradeValidFrom: '2025-09-01',
  gradeValidTo: '2027-08-31',
  assessmentDate: '2025-08-20',
  nextAssessmentDate: '2027-07-01',
  monthlyLimit: 1341000,
  usedThisMonth: 487000,
  status: 'active',
};

const TIMELINE = [
  {
    id: 1,
    date: '2025-08-20',
    event: '장기요양 인정 심사',
    detail: '국민건강보험공단 표준장기요양인정조사',
    result: '3등급 인정',
    status: 'done',
  },
  {
    id: 2,
    date: '2025-09-01',
    event: '장기요양 인정서 발급',
    detail: '인정서 교부 및 서비스 이용 시작',
    result: '재가급여 이용 가능',
    status: 'done',
  },
  {
    id: 3,
    date: '2025-10-05',
    event: '개인별 장기요양 이용 계획 수립',
    detail: '케어매니저 최지원 방문 상담',
    result: '케어 계획 확정',
    status: 'done',
  },
  {
    id: 4,
    date: '2027-07-01',
    event: '갱신 심사 예정',
    detail: '인정 유효기간 만료 60일 전 신청 필요',
    result: '예정',
    status: 'upcoming',
  },
];

const GRADE_TABLE = [
  { grade: '1등급', score: '95점 이상', limit: '2,306,040원', current: false },
  { grade: '2등급', score: '75~95점', limit: '2,043,960원', current: false },
  { grade: '3등급', score: '60~75점', limit: '1,341,000원', current: true },
  { grade: '4등급', score: '51~60점', limit: '1,180,680원', current: false },
  { grade: '5등급', score: '45~51점 (치매)', limit: '1,080,360원', current: false },
];

export default function EligibilityPage() {
  const usageRate = Math.round((CASE.usedThisMonth / CASE.monthlyLimit) * 100);

  return (
    <FamilyAppShell>
      <div className="max-w-3xl mx-auto px-4 py-6">
        {/* Header */}
        <div className="flex items-start justify-between mb-6">
          <div>
            <h1 className="text-2xl font-bold text-gray-900">수급 자격 현황</h1>
            <p className="text-sm text-gray-500 mt-1">노인장기요양 수급 자격 및 급여 한도</p>
          </div>
          <Link
            href="/eligibility/apply"
            className="flex-shrink-0 px-4 py-2 border border-gray-300 text-gray-700 text-sm font-medium rounded-lg hover:bg-gray-50 transition-colors"
          >
            갱신 신청
          </Link>
        </div>

        {/* Status Card */}
        <div className="bg-gradient-to-br from-blue-600 to-blue-700 rounded-xl p-6 text-white mb-5 shadow-md">
          <div className="flex items-start justify-between mb-4">
            <div>
              <p className="text-blue-100 text-sm">노인장기요양 인정등급</p>
              <h2 className="text-4xl font-extrabold mt-1">{CASE.currentGrade}</h2>
            </div>
            <span className="bg-white/20 px-3 py-1 rounded-full text-sm font-semibold">유효</span>
          </div>
          <div className="grid grid-cols-2 gap-4 mb-4">
            <div>
              <p className="text-blue-200 text-xs">인정 기간</p>
              <p className="font-semibold text-sm mt-0.5">
                {CASE.gradeValidFrom} ~ {CASE.gradeValidTo}
              </p>
            </div>
            <div>
              <p className="text-blue-200 text-xs">인정번호</p>
              <p className="font-semibold text-sm mt-0.5">{CASE.caseNo}</p>
            </div>
          </div>
          <div>
            <div className="flex justify-between text-sm mb-1">
              <span className="text-blue-100">이번 달 급여 한도 사용</span>
              <span className="font-bold">{usageRate}%</span>
            </div>
            <div className="w-full bg-white/20 rounded-full h-2">
              <div
                className="bg-white h-2 rounded-full"
                style={{ width: `${usageRate}%` }}
              />
            </div>
            <div className="flex justify-between text-xs text-blue-200 mt-1">
              <span>사용: {CASE.usedThisMonth.toLocaleString()}원</span>
              <span>한도: {CASE.monthlyLimit.toLocaleString()}원</span>
            </div>
          </div>
        </div>

        {/* Grade Table */}
        <div className="bg-white border border-gray-200 rounded-xl p-5 mb-5">
          <h2 className="font-bold text-gray-900 mb-3">등급별 급여 한도 (2026년 기준)</h2>
          <div className="overflow-x-auto">
            <table className="w-full text-sm">
              <thead>
                <tr className="text-left text-xs text-gray-400 border-b border-gray-100">
                  <th className="pb-2 font-medium">등급</th>
                  <th className="pb-2 font-medium">인정 점수</th>
                  <th className="pb-2 font-medium text-right">월 한도액</th>
                </tr>
              </thead>
              <tbody>
                {GRADE_TABLE.map((row) => (
                  <tr
                    key={row.grade}
                    className={`border-b border-gray-50 last:border-0 ${row.current ? 'bg-blue-50' : ''}`}
                  >
                    <td className={`py-2.5 font-semibold ${row.current ? 'text-blue-700' : 'text-gray-800'}`}>
                      {row.grade} {row.current && <span className="text-xs ml-1 bg-blue-600 text-white px-1.5 py-0.5 rounded-full">현재</span>}
                    </td>
                    <td className="py-2.5 text-gray-600">{row.score}</td>
                    <td className={`py-2.5 text-right font-medium ${row.current ? 'text-blue-700' : 'text-gray-800'}`}>
                      {row.limit}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>

        {/* Timeline */}
        <div className="bg-white border border-gray-200 rounded-xl p-5">
          <h2 className="font-bold text-gray-900 mb-4">인정 이력</h2>
          <div className="space-y-4">
            {TIMELINE.map((item, idx) => (
              <div key={item.id} className="flex gap-4">
                <div className="flex flex-col items-center">
                  <div className={`w-8 h-8 rounded-full flex items-center justify-center text-sm font-bold flex-shrink-0 ${
                    item.status === 'done' ? 'bg-blue-600 text-white' : 'bg-gray-200 text-gray-500'
                  }`}>
                    {item.status === 'done' ? '✓' : '○'}
                  </div>
                  {idx < TIMELINE.length - 1 && (
                    <div className={`w-0.5 flex-1 mt-1 ${item.status === 'done' ? 'bg-blue-300' : 'bg-gray-200'}`} />
                  )}
                </div>
                <div className="flex-1 pb-4">
                  <div className="flex items-center gap-2">
                    <span className="text-xs text-gray-400">{item.date}</span>
                  </div>
                  <p className="font-semibold text-gray-900 text-sm mt-0.5">{item.event}</p>
                  <p className="text-xs text-gray-500 mt-0.5">{item.detail}</p>
                  <span className={`inline-block mt-1.5 text-xs px-2 py-0.5 rounded-full font-medium ${
                    item.status === 'done'
                      ? 'bg-green-50 text-green-700 border border-green-200'
                      : 'bg-gray-100 text-gray-500 border border-gray-200'
                  }`}>
                    {item.result}
                  </span>
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </FamilyAppShell>
  );
}
