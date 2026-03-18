import { Pool } from 'pg'

// ---------------------------------------------------------------------------
// Database connection
// ---------------------------------------------------------------------------

const pool = new Pool({ connectionString: process.env.DATABASE_URL })

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

function daysAgo(n: number): Date {
  const d = new Date()
  d.setDate(d.getDate() - n)
  return d
}

function daysFromNow(n: number): Date {
  const d = new Date()
  d.setDate(d.getDate() + n)
  return d
}

function todayAt(hour: number, minute = 0): Date {
  const d = new Date()
  d.setHours(hour, minute, 0, 0)
  return d
}

function uuid(): string {
  return crypto.randomUUID()
}

// ---------------------------------------------------------------------------
// Cleanup — single TRUNCATE CASCADE covers all dependencies
// ---------------------------------------------------------------------------

async function cleanup(client: import('pg').PoolClient): Promise<void> {
  await client.query(`
    TRUNCATE TABLE
      audit_logs,
      notifications,
      observability_signals,
      claim_or_subsidy_records,
      approval_steps,
      eligibility_cases,
      institution_referrals,
      appointments,
      medical_history_entries,
      medication_events,
      medication_schedules,
      medications,
      daily_observations,
      incidents,
      visits,
      care_plans,
      match_recommendations,
      match_requests,
      service_types,
      availability_slots,
      caregiver_credentials,
      caregiver_applications,
      service_regions,
      provider_organizations,
      consent_records,
      family_relationships,
      senior_profiles,
      person_profiles,
      sessions,
      accounts,
      verification_tokens,
      users
    CASCADE
  `)
}

// ---------------------------------------------------------------------------
// Main seed — wrapped in a single transaction for atomicity
// ---------------------------------------------------------------------------

async function main(): Promise<void> {
  const client = await pool.connect()

  try {
    await client.query('BEGIN')

    console.log('Cleaning existing data...')
    await cleanup(client)
    console.log('Seeding demo data...')

    // -----------------------------------------------------------------------
    // 1. Users
    // -----------------------------------------------------------------------

    const seniorUserId = uuid()
    const familyUserId = uuid()
    const caregiverApplicantUserId = uuid()
    const caregiverApprovedUserId = uuid()
    const providerAdminUserId = uuid()
    const govReviewerUserId = uuid()
    const platformAdminUserId = uuid()

    await client.query(
      `INSERT INTO users
         (id, email, name, phone, role, kyc_level, locale, is_active, email_verified, created_at, updated_at)
       VALUES
         ($1,  $2,  $3,  $4,  $5,  $6,  $7,  $8,  $9,  NOW(), NOW()),
         ($10, $11, $12, $13, $14, $15, $16, $17, $18, NOW(), NOW()),
         ($19, $20, $21, $22, $23, $24, $25, $26, $27, NOW(), NOW()),
         ($28, $29, $30, $31, $32, $33, $34, $35, $36, NOW(), NOW()),
         ($37, $38, $39, $40, $41, $42, $43, $44, $45, NOW(), NOW()),
         ($46, $47, $48, $49, $50, $51, $52, $53, $54, NOW(), NOW()),
         ($55, $56, $57, $58, $59, $60, $61, $62, $63, NOW(), NOW())`,
      [
        // 김영자 — senior
        seniorUserId, 'youngjja.kim@example.kr', '김영자', '010-1234-5678',
        'SENIOR', 'FULL_VERIFIED', 'ko', true, daysAgo(180),
        // 김철수 — family
        familyUserId, 'chulsu.kim@example.kr', '김철수', '010-2345-6789',
        'FAMILY', 'IDENTITY_VERIFIED', 'ko', true, daysAgo(90),
        // 박미영 — caregiver applicant
        caregiverApplicantUserId, 'miyoung.park@example.kr', '박미영', '010-3456-7890',
        'CAREGIVER_APPLICANT', 'PHONE_VERIFIED', 'ko', true, daysAgo(30),
        // 이지은 — approved caregiver
        caregiverApprovedUserId, 'jieun.lee@example.kr', '이지은', '010-4567-8901',
        'CAREGIVER_APPROVED', 'FULL_VERIFIED', 'ko', true, daysAgo(365),
        // 장서연 — provider admin
        providerAdminUserId, 'seoyeon.jang@happycare.kr', '장서연', '010-5678-9012',
        'PROVIDER_ADMIN', 'FULL_VERIFIED', 'ko', true, daysAgo(200),
        // 정민호 — government reviewer
        govReviewerUserId, 'minho.jung@gov.kr', '정민호', '010-6789-0123',
        'GOVERNMENT_REVIEWER', 'FULL_VERIFIED', 'ko', true, daysAgo(500),
        // 시스템 관리자 — platform admin
        platformAdminUserId, 'admin@bominal-senior.kr', '시스템 관리자', '010-7890-1234',
        'PLATFORM_ADMIN', 'FULL_VERIFIED', 'ko', true, daysAgo(730),
      ]
    )

    // -----------------------------------------------------------------------
    // 2. PersonProfiles
    // -----------------------------------------------------------------------

    const seniorPersonId = uuid()
    const familyPersonId = uuid()
    const caregiverApplicantPersonId = uuid()
    const caregiverApprovedPersonId = uuid()
    const providerAdminPersonId = uuid()
    const govReviewerPersonId = uuid()
    const platformAdminPersonId = uuid()

    await client.query(
      `INSERT INTO person_profiles
         (id, user_id, korean_name, english_name, date_of_birth, gender,
          phone, address, city, district, postal_code,
          emergency_contact_name, emergency_contact_phone,
          created_at, updated_at)
       VALUES
         ($1,  $2,  $3,  $4,  $5,  $6,  $7,  $8,  $9,  $10, $11, $12, $13, NOW(), NOW()),
         ($14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, NOW(), NOW()),
         ($27, $28, $29, $30, $31, $32, $33, $34, $35, $36, $37, $38, $39, NOW(), NOW()),
         ($40, $41, $42, $43, $44, $45, $46, $47, $48, $49, $50, $51, $52, NOW(), NOW()),
         ($53, $54, $55, $56, $57, $58, $59, $60, $61, $62, $63, $64, $65, NOW(), NOW()),
         ($66, $67, $68, $69, $70, $71, $72, $73, $74, $75, $76, $77, $78, NOW(), NOW()),
         ($79, $80, $81, $82, $83, $84, $85, $86, $87, $88, $89, $90, $91, NOW(), NOW())`,
      [
        // 김영자
        seniorPersonId, seniorUserId, '김영자', 'Kim Young-ja',
        new Date('1945-03-12'), 'FEMALE',
        '010-1234-5678', '서울특별시 강남구 역삼동 123-45',
        '서울특별시', '강남구', '06234',
        '김철수', '010-2345-6789',
        // 김철수
        familyPersonId, familyUserId, '김철수', 'Kim Chul-su',
        new Date('1970-07-20'), 'MALE',
        '010-2345-6789', '서울특별시 서초구 서초동 456-78',
        '서울특별시', '서초구', '06713',
        '이미래', '010-9999-8888',
        // 박미영
        caregiverApplicantPersonId, caregiverApplicantUserId, '박미영', 'Park Mi-young',
        new Date('1988-11-05'), 'FEMALE',
        '010-3456-7890', '서울특별시 송파구 잠실동 789-12',
        '서울특별시', '송파구', '05510',
        '박정수', '010-1111-2222',
        // 이지은
        caregiverApprovedPersonId, caregiverApprovedUserId, '이지은', 'Lee Ji-eun',
        new Date('1985-04-22'), 'FEMALE',
        '010-4567-8901', '서울특별시 강남구 대치동 321-09',
        '서울특별시', '강남구', '06249',
        '이상호', '010-3333-4444',
        // 장서연
        providerAdminPersonId, providerAdminUserId, '장서연', 'Jang Seo-yeon',
        new Date('1982-09-15'), 'FEMALE',
        '010-5678-9012', '서울특별시 마포구 합정동 654-32',
        '서울특별시', '마포구', '04066',
        '장민준', '010-5555-6666',
        // 정민호
        govReviewerPersonId, govReviewerUserId, '정민호', 'Jung Min-ho',
        new Date('1978-02-28'), 'MALE',
        '010-6789-0123', '서울특별시 종로구 청운동 111-22',
        '서울특별시', '종로구', '03044',
        '정수연', '010-7777-8888',
        // 시스템 관리자
        platformAdminPersonId, platformAdminUserId, '시스템 관리자', 'System Admin',
        new Date('1990-01-01'), 'PREFER_NOT_TO_SAY',
        '010-7890-1234', '서울특별시 중구 을지로 100',
        '서울특별시', '중구', '04538',
        '시스템', '02-1234-5678',
      ]
    )

    // -----------------------------------------------------------------------
    // 3. SeniorProfile
    // -----------------------------------------------------------------------

    const seniorProfileId = uuid()

    await client.query(
      `INSERT INTO senior_profiles
         (id, person_id, care_level, has_ltci_certification, ltci_number,
          primary_diagnosis, mobility_level, cognitive_level,
          lives_alone, preferred_language, created_at, updated_at)
       VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, NOW(), NOW())`,
      [
        seniorProfileId, seniorPersonId,
        3, true, 'LTCI-2020-00123456',
        '고혈압, 경도 인지장애',
        '보조기구 필요',
        '경도 인지저하',
        false, 'ko',
      ]
    )

    // -----------------------------------------------------------------------
    // 4. FamilyRelationship
    // -----------------------------------------------------------------------

    await client.query(
      `INSERT INTO family_relationships
         (id, senior_person_id, family_person_id, relationship_type,
          is_primary_contact, can_make_decisions, created_at, updated_at)
       VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())`,
      [uuid(), seniorPersonId, familyPersonId, 'CHILD', true, true]
    )

    // -----------------------------------------------------------------------
    // 5. ConsentRecord
    // -----------------------------------------------------------------------

    await client.query(
      `INSERT INTO consent_records
         (id, subject_person_id, purpose, granted_by, is_active,
          granted_at, expires_at, created_at, updated_at)
       VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW())`,
      [
        uuid(), seniorPersonId, 'MEDICAL_SHARE', familyUserId, true,
        daysAgo(180), daysFromNow(185),
      ]
    )

    // -----------------------------------------------------------------------
    // 6. ProviderOrganizations
    // -----------------------------------------------------------------------

    const homeCareAgencyId = uuid()
    const nursingHospitalId = uuid()
    const clinicId = uuid()

    await client.query(
      `INSERT INTO provider_organizations
         (id, name, type, registration_number,
          address, city, district, postal_code,
          phone, email, website,
          license_number, license_expires_at,
          is_active, description, latitude, longitude,
          created_at, updated_at)
       VALUES
         ($1,  $2,  $3,  $4,  $5,  $6,  $7,  $8,  $9,  $10, $11, $12, $13, $14, $15, $16, $17, NOW(), NOW()),
         ($18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34, NOW(), NOW()),
         ($35, $36, $37, $38, $39, $40, $41, $42, $43, $44, $45, $46, $47, $48, $49, $50, $51, NOW(), NOW())`,
      [
        // 행복한 돌봄 서비스
        homeCareAgencyId,
        '행복한 돌봄 서비스', 'HOME_CARE_AGENCY', 'REG-HOME-2018-00001',
        '서울특별시 강남구 테헤란로 152', '서울특별시', '강남구', '06236',
        '02-555-1000', 'info@happycare.kr', 'https://www.happycare.kr',
        'HC-SEOUL-2018-0042', daysFromNow(365),
        true, '강남구 일대 방문 요양 전문 서비스 기관입니다.', 37.5065, 127.0536,
        // 서울 요양병원
        nursingHospitalId,
        '서울 요양병원', 'NURSING_HOSPITAL', 'REG-HOSP-2010-00088',
        '서울특별시 서초구 반포대로 235', '서울특별시', '서초구', '06560',
        '02-555-2000', 'contact@seoulcare-hospital.kr', 'https://www.seoulcare-hospital.kr',
        'NH-SEOUL-2010-0009', daysFromNow(730),
        true, '서초구 소재 노인 전문 요양병원입니다.', 37.5042, 127.0003,
        // 강남 의원
        clinicId,
        '강남 의원', 'CLINIC', 'REG-CLIN-2015-00321',
        '서울특별시 강남구 도산대로 101', '서울특별시', '강남구', '06022',
        '02-555-3000', 'gangnam@clinic.kr', 'https://www.gangnamclinic.kr',
        'CL-SEOUL-2015-0077', daysFromNow(548),
        true, '강남구 내과 전문 의원입니다.', 37.5231, 127.0388,
      ]
    )

    // -----------------------------------------------------------------------
    // 7. CaregiverApplications
    // -----------------------------------------------------------------------

    const applicantApplicationId = uuid()
    const approvedApplicationId = uuid()

    await client.query(
      `INSERT INTO caregiver_applications
         (id, user_id, provider_id, status, experience_years, bio, specializations,
          has_dementia_experience, has_overnight_availability, smoking_status,
          pet_friendly, languages_spoken, submitted_at, reviewed_at, reviewed_by,
          created_at, updated_at)
       VALUES
         ($1,  $2,  $3,  $4,  $5,  $6,  $7,  $8,  $9,  $10, $11, $12, $13, NULL, NULL, NOW(), NOW()),
         ($14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27,   $28,  NOW(), NOW())`,
      [
        // 박미영 — SUBMITTED
        applicantApplicationId,
        caregiverApplicantUserId, homeCareAgencyId, 'SUBMITTED',
        2,
        '어르신들을 정성껏 돌봐드리고 싶습니다. 요양보호사 자격증을 취득한 후 요양원에서 2년 근무한 경험이 있습니다.',
        '기본 신체활동 지원, 가사 지원',
        false, false, false, true, 'ko', daysAgo(10),
        // 이지은 — APPROVED_UNDER_PROVIDER
        approvedApplicationId,
        caregiverApprovedUserId, homeCareAgencyId, 'APPROVED_UNDER_PROVIDER',
        7,
        '10년 가까이 어르신 돌봄 현장에 있었습니다. 치매 어르신과의 소통에 특별한 관심을 갖고 있습니다.',
        '치매 케어, 신체활동 지원, 정서 지원, 가사 지원',
        true, true, false, true, 'ko', daysAgo(365), daysAgo(350), providerAdminUserId,
      ]
    )

    // -----------------------------------------------------------------------
    // 8. CaregiverCredential (이지은 — VERIFIED)
    // -----------------------------------------------------------------------

    await client.query(
      `INSERT INTO caregiver_credentials
         (id, application_id, type, status, issuer,
          issued_at, expires_at, document_url,
          verified_at, verified_by, created_at, updated_at)
       VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, NOW(), NOW())`,
      [
        uuid(), approvedApplicationId,
        'CAREGIVER_CERTIFICATE', 'VERIFIED',
        '한국보건복지인력개발원',
        new Date('2017-06-15'), new Date('2027-06-14'),
        'https://storage.bominal-senior.kr/credentials/jieun-cert-2017.pdf',
        daysAgo(350), platformAdminUserId,
      ]
    )

    // -----------------------------------------------------------------------
    // 9. ServiceRegion — 강남구 for home care agency
    // -----------------------------------------------------------------------

    await client.query(
      `INSERT INTO service_regions
         (id, provider_id, city, district, is_active, created_at, updated_at)
       VALUES ($1, $2, $3, $4, $5, NOW(), NOW())`,
      [uuid(), homeCareAgencyId, '서울특별시', '강남구', true]
    )

    // -----------------------------------------------------------------------
    // 10. AvailabilitySlots — Mon–Fri 09:00–17:00 for 이지은
    // -----------------------------------------------------------------------

    const weekdays = ['MONDAY', 'TUESDAY', 'WEDNESDAY', 'THURSDAY', 'FRIDAY']

    for (const day of weekdays) {
      await client.query(
        `INSERT INTO availability_slots
           (id, application_id, day_of_week, start_time, end_time, is_active, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())`,
        [uuid(), approvedApplicationId, day, '09:00', '17:00', true]
      )
    }

    // -----------------------------------------------------------------------
    // 11. ServiceTypes — PERSONAL_CARE, COMPANION for 이지은
    // -----------------------------------------------------------------------

    await client.query(
      `INSERT INTO service_types
         (id, application_id, category, name, description, is_active, created_at, updated_at)
       VALUES
         ($1, $2, $3, $4, $5, $6, NOW(), NOW()),
         ($7, $8, $9, $10, $11, $12, NOW(), NOW())`,
      [
        uuid(), approvedApplicationId, 'PERSONAL_CARE',
        '신체활동 지원', '목욕, 세면, 식사 보조 등 일상 신체활동 지원 서비스', true,
        uuid(), approvedApplicationId, 'COMPANION',
        '정서 지원 동행', '말벗, 산책 동행, 여가 활동 지원 서비스', true,
      ]
    )

    // -----------------------------------------------------------------------
    // 12. MatchRequest — FULFILLED
    // -----------------------------------------------------------------------

    const matchRequestId = uuid()

    await client.query(
      `INSERT INTO match_requests
         (id, senior_id, requested_by, status, service_category,
          region_city, region_district, start_date,
          schedule_notes, language_preference, gender_preference,
          requires_dementia_experience, requires_overnight_care,
          additional_notes, created_at, updated_at)
       VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, NOW(), NOW())`,
      [
        matchRequestId, seniorProfileId, familyUserId, 'FULFILLED', 'PERSONAL_CARE',
        '서울특별시', '강남구', daysAgo(30),
        '평일 오전 9시~오후 5시, 주 5일 방문 요청', 'ko', 'FEMALE',
        false, false,
        '고혈압 약 복용 중이므로 복약 지원 가능한 분 선호합니다.',
      ]
    )

    // -----------------------------------------------------------------------
    // 13. MatchRecommendation — 이지은, score 92.5
    // -----------------------------------------------------------------------

    await client.query(
      `INSERT INTO match_recommendations
         (id, match_request_id, caregiver_application_id,
          score, score_breakdown, rank, is_selected, selected_at, created_at)
       VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW())`,
      [
        uuid(), matchRequestId, approvedApplicationId,
        92.5,
        JSON.stringify({
          regionMatch: 25,
          availabilityMatch: 20,
          experienceScore: 22.5,
          specialization: 15,
          ratingScore: 10,
        }),
        1, true, daysAgo(25),
      ]
    )

    // -----------------------------------------------------------------------
    // 14. CarePlan — ACTIVE
    // -----------------------------------------------------------------------

    const carePlanId = uuid()

    await client.query(
      `INSERT INTO care_plans
         (id, senior_id, provider_id, status, title, description,
          start_date, end_date, goals, created_by, created_at, updated_at)
       VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, NOW(), NOW())`,
      [
        carePlanId, seniorProfileId, homeCareAgencyId, 'ACTIVE',
        '일상 돌봄 계획',
        '김영자 어르신의 일상 신체활동 지원 및 정서 돌봄 계획입니다.',
        daysAgo(25), daysFromNow(335),
        JSON.stringify([
          { goal: '혈압 안정 유지 및 규칙적인 복약 준수', priority: 'HIGH' },
          { goal: '일상 신체활동 보조를 통한 낙상 예방', priority: 'HIGH' },
          { goal: '정서적 안정 및 사회적 교류 증진', priority: 'MEDIUM' },
        ]),
        providerAdminUserId,
      ]
    )

    // -----------------------------------------------------------------------
    // 15. Visits — 1 completed (3 days ago), 1 scheduled (tomorrow)
    // -----------------------------------------------------------------------

    const completedVisitBase = daysAgo(3)
    const scheduledVisitBase = daysFromNow(1)

    await client.query(
      `INSERT INTO visits
         (id, care_plan_id, caregiver_id, status,
          scheduled_start, scheduled_end, actual_start, actual_end,
          check_in_latitude, check_in_longitude,
          check_out_latitude, check_out_longitude,
          tasks, notes, created_at, updated_at)
       VALUES
         ($1,  $2,  $3,  $4,  $5,  $6,  $7,  $8,  $9,  $10, $11, $12, $13, $14, NOW(), NOW()),
         ($15, $16, $17, $18, $19, $20, NULL, NULL, NULL, NULL, NULL, NULL, $21, $22, NOW(), NOW())`,
      [
        // completed visit
        uuid(), carePlanId, approvedApplicationId, 'COMPLETED',
        new Date(new Date(completedVisitBase).setHours(9, 0, 0, 0)),
        new Date(new Date(completedVisitBase).setHours(17, 0, 0, 0)),
        new Date(new Date(completedVisitBase).setHours(9, 5, 0, 0)),
        new Date(new Date(completedVisitBase).setHours(17, 2, 0, 0)),
        37.4981, 127.0276, 37.4981, 127.0276,
        JSON.stringify([
          { task: '아침 세면 및 구강 위생 보조', completed: true },
          { task: '아침 식사 준비 및 복약 지원', completed: true },
          { task: '오전 실내 산책 동행 20분', completed: true },
          { task: '점심 식사 준비 및 복약 지원', completed: true },
          { task: '오후 여가 활동 (TV 시청 동행)', completed: true },
        ]),
        '어르신 컨디션 양호. 식사량 평소의 80% 수준. 오전 산책 잘 마침.',
        // scheduled visit
        uuid(), carePlanId, approvedApplicationId, 'SCHEDULED',
        new Date(new Date(scheduledVisitBase).setHours(9, 0, 0, 0)),
        new Date(new Date(scheduledVisitBase).setHours(17, 0, 0, 0)),
        JSON.stringify([
          { task: '아침 세면 및 구강 위생 보조', completed: false },
          { task: '아침 식사 준비 및 복약 지원', completed: false },
          { task: '실내 운동 보조 15분', completed: false },
          { task: '점심 식사 준비 및 복약 지원', completed: false },
          { task: '저녁 식사 준비 및 복약 지원', completed: false },
        ]),
        '내일 방문 예정. 정기 혈압 측정 포함.',
      ]
    )

    // -----------------------------------------------------------------------
    // 16. DailyObservation — MOOD, today
    // -----------------------------------------------------------------------

    await client.query(
      `INSERT INTO daily_observations
         (id, care_plan_id, observed_by, category, date, value, notes, created_at, updated_at)
       VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW())`,
      [
        uuid(), carePlanId, caregiverApprovedUserId, 'MOOD',
        todayAt(14),
        '보통',
        '오후에 다소 기운 없어 보이셨으나 말씀은 잘 나누심. 창문 너머 경치 감상하며 긍정적인 모습.',
      ]
    )

    // -----------------------------------------------------------------------
    // 17. Medication — Amlodipine (고혈압)
    // -----------------------------------------------------------------------

    const medicationId = uuid()

    await client.query(
      `INSERT INTO medications
         (id, person_id, name, dosage, form, frequency,
          prescribed_by, prescribed_at, start_date,
          is_active, side_effects, notes,
          created_by, created_at, updated_at)
       VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, NOW(), NOW())`,
      [
        medicationId, seniorPersonId,
        '암로디핀 (Amlodipine)', '5mg', '정제', 'ONCE_DAILY',
        '강남 의원 심장내과 박의사', daysAgo(180), daysAgo(180),
        true,
        '두통, 안면 홍조, 발목 부종 가능성',
        '고혈압 치료를 위한 칼슘채널차단제. 매일 아침 식후 복용.',
        familyUserId,
      ]
    )

    // -----------------------------------------------------------------------
    // 18. MedicationSchedule — 9:00 AM daily
    // -----------------------------------------------------------------------

    await client.query(
      `INSERT INTO medication_schedules
         (id, medication_id, time_of_day, is_active, created_at, updated_at)
       VALUES ($1, $2, $3, $4, NOW(), NOW())`,
      [uuid(), medicationId, '09:00', true]
    )

    // -----------------------------------------------------------------------
    // 19. MedicationEvents — taken today, missed yesterday, scheduled tomorrow
    // -----------------------------------------------------------------------

    await client.query(
      `INSERT INTO medication_events
         (id, medication_id, scheduled_for, status, taken_at, notes, recorded_by, created_at, updated_at)
       VALUES
         ($1,  $2,  $3,  $4,  $5,  $6,  $7,  NOW(), NOW()),
         ($8,  $9,  $10, $11, NULL, $12, $13, NOW(), NOW()),
         ($14, $15, $16, $17, NULL, NULL, NULL, NOW(), NOW())`,
      [
        // taken today at 09:15
        uuid(), medicationId, todayAt(9), 'TAKEN', todayAt(9, 15),
        '정상 복약', caregiverApprovedUserId,
        // missed yesterday
        uuid(), medicationId,
        new Date(new Date(daysAgo(1)).setHours(9, 0, 0, 0)), 'MISSED',
        '방문 요양사 부재로 복약 확인 누락', caregiverApprovedUserId,
        // scheduled tomorrow
        uuid(), medicationId,
        new Date(new Date(daysFromNow(1)).setHours(9, 0, 0, 0)), 'SCHEDULED',
      ]
    )

    // -----------------------------------------------------------------------
    // 20. MedicalHistoryEntry — Hypertension
    // -----------------------------------------------------------------------

    await client.query(
      `INSERT INTO medical_history_entries
         (id, person_id, condition, diagnosed_at, treated_by,
          status, notes, created_by, created_at, updated_at)
       VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW(), NOW())`,
      [
        uuid(), seniorPersonId,
        '고혈압 (Hypertension)',
        new Date('2020-03-10'),
        '강남 의원 심장내과',
        'active',
        '2020년 3월 강남 의원 심장내과 진단. 암로디핀 5mg 처방 중. 정기적인 혈압 모니터링 필요.',
        familyUserId,
      ]
    )

    // -----------------------------------------------------------------------
    // 21. Appointment — upcoming cardiology
    // -----------------------------------------------------------------------

    await client.query(
      `INSERT INTO appointments
         (id, person_id, institution_name, institution_type,
          appointment_date, status, purpose, notes, address,
          created_by, created_at, updated_at)
       VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, NOW(), NOW())`,
      [
        uuid(), seniorPersonId,
        '강남 의원', 'CLINIC',
        daysFromNow(14), 'SCHEDULED',
        '심장내과 정기 진료 및 혈압 체크',
        '최근 혈압 기록지 지참 필요. 복약 이상반응 여부 확인 예정.',
        '서울특별시 강남구 도산대로 101 강남 의원',
        familyUserId,
      ]
    )

    // -----------------------------------------------------------------------
    // 22. InstitutionReferral — clinic → nursing hospital
    // -----------------------------------------------------------------------

    await client.query(
      `INSERT INTO institution_referrals
         (id, from_provider_id, to_provider_id, senior_person_id,
          status, reason, notes, referred_at, accepted_at,
          created_at, updated_at)
       VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, NOW(), NOW())`,
      [
        uuid(), clinicId, nursingHospitalId, seniorPersonId,
        'ACCEPTED',
        '경도 인지장애 심화 평가 및 단기 집중 관리 목적 입원 의뢰',
        '기존 고혈압 관리 중이며 최근 인지기능 저하 소견. 요양병원 전문 평가 권고.',
        daysAgo(20), daysAgo(18),
      ]
    )

    // -----------------------------------------------------------------------
    // 23. EligibilityCase — APPROVED (노인장기요양보험)
    // -----------------------------------------------------------------------

    const eligibilityCaseId = uuid()

    await client.query(
      `INSERT INTO eligibility_cases
         (id, senior_id, status, program_name,
          application_date, determination_date, notes,
          created_by, created_at, updated_at)
       VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW(), NOW())`,
      [
        eligibilityCaseId, seniorProfileId, 'APPROVED', '노인장기요양보험',
        daysAgo(120), daysAgo(90),
        '장기요양등급 3등급 인정. 방문요양 월 78시간 급여 수급 가능.',
        govReviewerUserId,
      ]
    )

    // -----------------------------------------------------------------------
    // 24. ApprovalStep — completed screening
    // -----------------------------------------------------------------------

    await client.query(
      `INSERT INTO approval_steps
         (id, case_id, step_name, step_order, status,
          assigned_to, completed_at, notes, created_at, updated_at)
       VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW(), NOW())`,
      [
        uuid(), eligibilityCaseId,
        '방문 조사', 1, 'completed',
        govReviewerUserId, daysAgo(100),
        '국민건강보험공단 조사원 방문 완료. 신체기능, 인지기능, 행동변화, 간호처치 영역 평가 실시. 요양 필요 확인.',
      ]
    )

    // -----------------------------------------------------------------------
    // 25. ClaimOrSubsidyRecord — 1 paid claim
    // -----------------------------------------------------------------------

    await client.query(
      `INSERT INTO claim_or_subsidy_records
         (id, case_id, claim_number, status, amount, currency,
          service_date, submitted_at, processed_at, notes,
          created_at, updated_at)
       VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, NOW(), NOW())`,
      [
        uuid(), eligibilityCaseId,
        'CLM-2026-00001', 'PAID', 780000, 'KRW',
        daysAgo(60), daysAgo(55), daysAgo(45),
        '2월 방문요양 78시간 급여 청구. 행복한 돌봄 서비스 제공.',
      ]
    )

    // -----------------------------------------------------------------------
    // 26. ObservabilitySignal — MEDICATION_MISSED
    // -----------------------------------------------------------------------

    await client.query(
      `INSERT INTO observability_signals
         (id, event_type, severity, subject_person_id, actor_user_id,
          entity_type, entity_id, message, metadata, created_at)
       VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, NOW())`,
      [
        uuid(),
        'MEDICATION_MISSED', 'WARNING',
        seniorPersonId, caregiverApprovedUserId,
        'MedicationEvent', medicationId,
        '김영자 어르신 고혈압 약 복약 누락 감지 (어제 09:00)',
        JSON.stringify({
          medicationName: '암로디핀 (Amlodipine)',
          scheduledTime: '09:00',
          missedDate: daysAgo(1).toISOString().split('T')[0],
        }),
      ]
    )

    // -----------------------------------------------------------------------
    // 27. Notifications
    // -----------------------------------------------------------------------

    await client.query(
      `INSERT INTO notifications
         (id, user_id, type, title, message, link, is_read, created_at)
       VALUES
         ($1, $2, $3, $4, $5, $6, $7, NOW()),
         ($8, $9, $10, $11, $12, $13, $14, NOW())`,
      [
        // medication reminder for 김철수 (family)
        uuid(), familyUserId, 'REMINDER',
        '복약 알림',
        '어머님(김영자) 오늘 오전 9시 고혈압 약 복약을 확인해 주세요.',
        '/family/medications', false,
        // visit reminder for 이지은 (caregiver)
        uuid(), caregiverApprovedUserId, 'REMINDER',
        '방문 일정 알림',
        '내일 오전 9시 김영자 어르신 댁 방문 일정이 있습니다. 준비 사항을 확인해 주세요.',
        '/caregiver/visits', false,
      ]
    )

    // -----------------------------------------------------------------------
    // 28. AuditLog — login event
    // -----------------------------------------------------------------------

    await client.query(
      `INSERT INTO audit_logs
         (id, user_id, action, entity_type, entity_id,
          ip_address, user_agent, new_value, created_at)
       VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW())`,
      [
        uuid(), familyUserId, 'LOGIN', 'User', familyUserId,
        '211.49.15.200',
        'Mozilla/5.0 (iPhone; CPU iPhone OS 18_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.0 Mobile/15E148 Safari/604.1',
        JSON.stringify({ loginAt: new Date().toISOString(), method: 'email' }),
      ]
    )

    await client.query('COMMIT')

    console.log('Seed complete.')
    console.log('  Users created:               7')
    console.log('  PersonProfiles:              7')
    console.log('  SeniorProfile:               1')
    console.log('  FamilyRelationship:          1')
    console.log('  ConsentRecord:               1')
    console.log('  ProviderOrganizations:       3')
    console.log('  CaregiverApplications:       2')
    console.log('  CaregiverCredential:         1')
    console.log('  ServiceRegion:               1')
    console.log('  AvailabilitySlots:           5 (Mon-Fri)')
    console.log('  ServiceTypes:                2')
    console.log('  MatchRequest:                1')
    console.log('  MatchRecommendation:         1')
    console.log('  CarePlan:                    1')
    console.log('  Visits:                      2')
    console.log('  DailyObservation:            1')
    console.log('  Medication:                  1')
    console.log('  MedicationSchedule:          1')
    console.log('  MedicationEvents:            3')
    console.log('  MedicalHistoryEntry:         1')
    console.log('  Appointment:                 1')
    console.log('  InstitutionReferral:         1')
    console.log('  EligibilityCase:             1')
    console.log('  ApprovalStep:                1')
    console.log('  ClaimOrSubsidyRecord:        1')
    console.log('  ObservabilitySignal:         1')
    console.log('  Notifications:               2')
    console.log('  AuditLog:                    1')
  } catch (err) {
    await client.query('ROLLBACK')
    throw err
  } finally {
    client.release()
  }
}

main()
  .catch((err) => {
    console.error('Seed failed:', err)
    process.exit(1)
  })
  .finally(() => {
    void pool.end()
  })
