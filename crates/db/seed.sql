-- =============================================================================
-- Korea Senior Care Portal -- Seed Data
-- =============================================================================
-- Realistic demo data for development and testing.
-- Idempotent: safe to run multiple times via ON CONFLICT DO NOTHING.
-- Uses fixed UUIDs so foreign-key references are deterministic.
-- =============================================================================

BEGIN;

-- ========================== FIXED UUID LEGEND ==========================
-- Users
--   senior   00000000-0000-0000-0000-000000000001
--   family   00000000-0000-0000-0000-000000000002
--   caregiver 00000000-0000-0000-0000-000000000003
--   provider 00000000-0000-0000-0000-000000000004
--   government 00000000-0000-0000-0000-000000000005
--   admin    00000000-0000-0000-0000-000000000006
--   applicant 00000000-0000-0000-0000-000000000007
--
-- Person profiles
--   senior   00000000-0000-0000-0000-000000000101
--   family   00000000-0000-0000-0000-000000000102
--   caregiver 00000000-0000-0000-0000-000000000103
--   applicant 00000000-0000-0000-0000-000000000104
--
-- Senior profile  00000000-0000-0000-0000-000000000201
-- Provider org    00000000-0000-0000-0000-000000000301
-- Provider org 2  00000000-0000-0000-0000-000000000302
-- Care plan       00000000-0000-0000-0000-000000000401
-- Caregiver app   00000000-0000-0000-0000-000000000501
-- Applicant app   00000000-0000-0000-0000-000000000502
-- Eligibility     00000000-0000-0000-0000-000000000601
-- Medications     00000000-0000-0000-0000-00000000070x
-- Visits          00000000-0000-0000-0000-00000000080x
-- Referral        00000000-0000-0000-0000-000000000901
-- =====================================================================

-- =============================================================================
-- 1. USERS (7 demo accounts)
-- =============================================================================

INSERT INTO users (id, email, name, phone, role, kyc_level, locale, is_active)
VALUES
  ('00000000-0000-0000-0000-000000000001',
   'senior@demo.com', '김복순', '010-1234-5678',
   'SENIOR', 'FULL_VERIFIED', 'ko', TRUE),

  ('00000000-0000-0000-0000-000000000002',
   'family@demo.com', '김영호', '010-2345-6789',
   'FAMILY', 'IDENTITY_VERIFIED', 'ko', TRUE),

  ('00000000-0000-0000-0000-000000000003',
   'caregiver@demo.com', '박미영', '010-3456-7890',
   'CAREGIVER_APPROVED', 'FULL_VERIFIED', 'ko', TRUE),

  ('00000000-0000-0000-0000-000000000004',
   'provider@demo.com', '이정수', '010-4567-8901',
   'PROVIDER_ADMIN', 'IDENTITY_VERIFIED', 'ko', TRUE),

  ('00000000-0000-0000-0000-000000000005',
   'government@demo.com', '정민지', '010-5678-9012',
   'GOVERNMENT_REVIEWER', 'FULL_VERIFIED', 'ko', TRUE),

  ('00000000-0000-0000-0000-000000000006',
   'admin@demo.com', '관리자', '010-6789-0123',
   'PLATFORM_ADMIN', 'FULL_VERIFIED', 'ko', TRUE),

  ('00000000-0000-0000-0000-000000000007',
   'applicant@demo.com', '최수진', '010-7890-1234',
   'CAREGIVER_APPLICANT', 'PHONE_VERIFIED', 'ko', TRUE)
ON CONFLICT DO NOTHING;

-- =============================================================================
-- 2. PERSON PROFILES (senior, family, caregiver, applicant)
-- =============================================================================

INSERT INTO person_profiles
  (id, user_id, korean_name, english_name, date_of_birth, gender,
   phone, address, city, district, postal_code,
   emergency_contact_name, emergency_contact_phone)
VALUES
  -- Senior: 김복순, 78세 여성
  ('00000000-0000-0000-0000-000000000101',
   '00000000-0000-0000-0000-000000000001',
   '김복순', 'Kim Bok-sun',
   '1948-03-15'::TIMESTAMPTZ, 'FEMALE',
   '010-1234-5678',
   '서울시 강남구 역삼동 123-45 행복아파트 301호',
   '서울특별시', '강남구', '06241',
   '김영호', '010-2345-6789'),

  -- Family: 김영호, 52세 남성 (아들)
  ('00000000-0000-0000-0000-000000000102',
   '00000000-0000-0000-0000-000000000002',
   '김영호', 'Kim Young-ho',
   '1974-07-22'::TIMESTAMPTZ, 'MALE',
   '010-2345-6789',
   '서울시 서초구 반포동 56-78 래미안아파트 1502호',
   '서울특별시', '서초구', '06591',
   '이수연', '010-8888-9999'),

  -- Caregiver: 박미영, 35세 여성
  ('00000000-0000-0000-0000-000000000103',
   '00000000-0000-0000-0000-000000000003',
   '박미영', 'Park Mi-young',
   '1991-11-03'::TIMESTAMPTZ, 'FEMALE',
   '010-3456-7890',
   '서울시 강남구 대치동 890-12 우성아파트 205호',
   '서울특별시', '강남구', '06282',
   '박정호', '010-7777-8888'),

  -- Applicant: 최수진, 28세 여성
  ('00000000-0000-0000-0000-000000000104',
   '00000000-0000-0000-0000-000000000007',
   '최수진', 'Choi Su-jin',
   '1998-05-10'::TIMESTAMPTZ, 'FEMALE',
   '010-7890-1234',
   '서울시 송파구 잠실동 34-56 리센츠아파트 801호',
   '서울특별시', '송파구', '05510',
   '최민수', '010-6666-7777')
ON CONFLICT DO NOTHING;

-- =============================================================================
-- 3. SENIOR PROFILE (care level 3, LTCI certified)
-- =============================================================================

INSERT INTO senior_profiles
  (id, person_id, care_level, has_ltci_certification, ltci_number,
   primary_diagnosis, mobility_level, cognitive_level,
   lives_alone, preferred_language)
VALUES
  ('00000000-0000-0000-0000-000000000201',
   '00000000-0000-0000-0000-000000000101',
   'level_3'::care_level_enum, TRUE, 'LTCI-2024-GN-004782',
   '고혈압, 제2형 당뇨병',
   'walker_assisted',
   'mild_impairment',
   TRUE, 'ko')
ON CONFLICT DO NOTHING;

-- =============================================================================
-- 4. FAMILY RELATIONSHIP (son -> senior, primary contact)
-- =============================================================================

INSERT INTO family_relationships
  (id, senior_person_id, family_person_id,
   relationship_type, is_primary_contact, can_make_decisions)
VALUES
  ('00000000-0000-0000-0000-0000000000f1',
   '00000000-0000-0000-0000-000000000101',
   '00000000-0000-0000-0000-000000000102',
   'CHILD', TRUE, TRUE)
ON CONFLICT DO NOTHING;

-- =============================================================================
-- 5. PROVIDER ORGANIZATIONS
-- =============================================================================

INSERT INTO provider_organizations
  (id, name, type, registration_number,
   address, city, district, postal_code,
   phone, email, website, is_active, description)
VALUES
  -- Primary home care agency
  ('00000000-0000-0000-0000-000000000301',
   '행복재가센터', 'HOME_CARE_AGENCY', 'HC-2020-GN-00142',
   '서울시 강남구 논현동 234-5 행복빌딩 3층',
   '서울특별시', '강남구', '06039',
   '02-555-1234', 'info@happyhomecare.kr', 'https://happyhomecare.kr',
   TRUE,
   '강남구 전문 재가요양센터. 10년 이상 경력의 요양보호사가 어르신의 일상생활을 돕습니다.'),

  -- Referral target: rehabilitation center
  ('00000000-0000-0000-0000-000000000302',
   '강남세브란스 재활센터', 'REHABILITATION_CENTER', 'RC-2018-GN-00089',
   '서울시 강남구 도곡동 146-92',
   '서울특별시', '강남구', '06273',
   '02-2019-3456', 'rehab@gangnam-severance.kr', 'https://gs-rehab.kr',
   TRUE,
   '강남세브란스병원 산하 재활전문센터. 노인 재활 및 물리치료 전문.')
ON CONFLICT DO NOTHING;

-- =============================================================================
-- 6. CAREGIVER APPLICATIONS (1 approved + 1 pending)
-- =============================================================================

INSERT INTO caregiver_applications
  (id, user_id, provider_id, status,
   experience_years, bio, specializations,
   has_dementia_experience, has_overnight_availability,
   languages_spoken, submitted_at, reviewed_at, reviewed_by)
VALUES
  -- Approved caregiver: 박미영
  ('00000000-0000-0000-0000-000000000501',
   '00000000-0000-0000-0000-000000000003',
   '00000000-0000-0000-0000-000000000301',
   'APPROVED_UNDER_PROVIDER',
   8,
   '어르신을 가족처럼 모시겠습니다. 서울시 요양보호사 자격증 보유, 치매 전문 교육 이수.',
   '노인돌봄, 치매케어, 재활보조',
   TRUE, FALSE,
   'ko',
   NOW() - INTERVAL '90 days',
   NOW() - INTERVAL '85 days',
   '00000000-0000-0000-0000-000000000005'),

  -- Pending applicant: 최수진
  ('00000000-0000-0000-0000-000000000502',
   '00000000-0000-0000-0000-000000000007',
   NULL,
   'SUBMITTED',
   1,
   '간호학과 졸업 후 요양보호사 자격을 취득했습니다. 성실하게 일하겠습니다.',
   '기본간호, 생활지원',
   FALSE, TRUE,
   'ko,en',
   NOW() - INTERVAL '5 days',
   NULL, NULL)
ON CONFLICT DO NOTHING;

-- =============================================================================
-- 7. CONSENT RECORDS (medical + both share)
-- =============================================================================

INSERT INTO consent_records
  (id, subject_person_id, purpose, granted_by,
   is_active, granted_at, expires_at)
VALUES
  ('00000000-0000-0000-0000-0000000000c1',
   '00000000-0000-0000-0000-000000000101',
   'MEDICAL_SHARE',
   '00000000-0000-0000-0000-000000000002',
   TRUE,
   NOW() - INTERVAL '60 days',
   NOW() + INTERVAL '305 days'),

  ('00000000-0000-0000-0000-0000000000c2',
   '00000000-0000-0000-0000-000000000101',
   'BOTH_SHARE',
   '00000000-0000-0000-0000-000000000002',
   TRUE,
   NOW() - INTERVAL '30 days',
   NOW() + INTERVAL '335 days')
ON CONFLICT DO NOTHING;

-- =============================================================================
-- 8. CARE PLAN (active)
-- =============================================================================

INSERT INTO care_plans
  (id, senior_id, provider_id, status,
   title, description, start_date, end_date,
   goals, created_by)
VALUES
  ('00000000-0000-0000-0000-000000000401',
   '00000000-0000-0000-0000-000000000201',
   '00000000-0000-0000-0000-000000000301',
   'ACTIVE',
   '김복순 어르신 재가요양 케어플랜 (2026년 1분기)',
   '주 3회 방문요양 서비스. 일상생활 지원, 투약 관리, 가벼운 운동 보조 포함.',
   NOW() - INTERVAL '30 days',
   NOW() + INTERVAL '60 days',
   '[
     {"goal": "혈압 안정적 유지 (130/80 이하)", "status": "in_progress", "target_date": null},
     {"goal": "보행기 사용 실내 이동 자립", "status": "in_progress", "target_date": null},
     {"goal": "투약 순응률 90% 이상 달성", "status": "in_progress", "target_date": null},
     {"goal": "낙상 예방 운동 주 3회 실시", "status": "not_started", "target_date": null}
   ]'::JSONB,
   '00000000-0000-0000-0000-000000000004')
ON CONFLICT DO NOTHING;

-- =============================================================================
-- 9. VISITS (2 completed, 1 in-progress, 2 scheduled)
-- =============================================================================

INSERT INTO visits
  (id, care_plan_id, caregiver_id, status,
   scheduled_start, scheduled_end,
   actual_start, actual_end,
   tasks, notes)
VALUES
  -- Visit 1: completed 5 days ago
  ('00000000-0000-0000-0000-000000000801',
   '00000000-0000-0000-0000-000000000401',
   '00000000-0000-0000-0000-000000000501',
   'COMPLETED',
   (NOW() - INTERVAL '5 days')::DATE + TIME '09:00',
   (NOW() - INTERVAL '5 days')::DATE + TIME '12:00',
   (NOW() - INTERVAL '5 days')::DATE + TIME '09:05',
   (NOW() - INTERVAL '5 days')::DATE + TIME '11:55',
   '[
     {"task": "혈압 측정", "completed": true},
     {"task": "아침 투약 확인", "completed": true},
     {"task": "실내 보행 운동 (15분)", "completed": true},
     {"task": "점심 식사 준비", "completed": true}
   ]'::JSONB,
   '혈압 128/82로 안정적. 보행 운동 시 약간의 무릎 통증 호소하셨으나 완료하심.'),

  -- Visit 2: completed 3 days ago
  ('00000000-0000-0000-0000-000000000802',
   '00000000-0000-0000-0000-000000000401',
   '00000000-0000-0000-0000-000000000501',
   'COMPLETED',
   (NOW() - INTERVAL '3 days')::DATE + TIME '09:00',
   (NOW() - INTERVAL '3 days')::DATE + TIME '12:00',
   (NOW() - INTERVAL '3 days')::DATE + TIME '08:58',
   (NOW() - INTERVAL '3 days')::DATE + TIME '12:10',
   '[
     {"task": "혈압 측정", "completed": true},
     {"task": "아침 투약 확인", "completed": true},
     {"task": "실내 보행 운동 (15분)", "completed": true},
     {"task": "점심 식사 준비", "completed": true},
     {"task": "주변 산책 (20분)", "completed": false}
   ]'::JSONB,
   '혈압 135/85 약간 높음. 산책은 비가 와서 취소. 다음 방문 시 혈압 재확인 필요.'),

  -- Visit 3: in-progress today
  ('00000000-0000-0000-0000-000000000803',
   '00000000-0000-0000-0000-000000000401',
   '00000000-0000-0000-0000-000000000501',
   'IN_PROGRESS',
   NOW()::DATE + TIME '09:00',
   NOW()::DATE + TIME '12:00',
   NOW()::DATE + TIME '09:02',
   NULL,
   '[
     {"task": "혈압 측정", "completed": true},
     {"task": "아침 투약 확인", "completed": false},
     {"task": "실내 보행 운동 (15분)", "completed": false},
     {"task": "점심 식사 준비", "completed": false}
   ]'::JSONB,
   NULL),

  -- Visit 4: scheduled in 2 days
  ('00000000-0000-0000-0000-000000000804',
   '00000000-0000-0000-0000-000000000401',
   '00000000-0000-0000-0000-000000000501',
   'SCHEDULED',
   (NOW() + INTERVAL '2 days')::DATE + TIME '09:00',
   (NOW() + INTERVAL '2 days')::DATE + TIME '12:00',
   NULL, NULL,
   '[
     {"task": "혈압 측정", "completed": false},
     {"task": "아침 투약 확인", "completed": false},
     {"task": "실내 보행 운동 (15분)", "completed": false},
     {"task": "점심 식사 준비", "completed": false}
   ]'::JSONB,
   NULL),

  -- Visit 5: scheduled in 4 days
  ('00000000-0000-0000-0000-000000000805',
   '00000000-0000-0000-0000-000000000401',
   '00000000-0000-0000-0000-000000000501',
   'SCHEDULED',
   (NOW() + INTERVAL '4 days')::DATE + TIME '09:00',
   (NOW() + INTERVAL '4 days')::DATE + TIME '12:00',
   NULL, NULL,
   '[
     {"task": "혈압 측정", "completed": false},
     {"task": "아침 투약 확인", "completed": false},
     {"task": "실내 보행 운동 (15분)", "completed": false},
     {"task": "점심 식사 준비", "completed": false},
     {"task": "주변 산책 (20분)", "completed": false}
   ]'::JSONB,
   NULL)
ON CONFLICT DO NOTHING;

-- =============================================================================
-- 10. MEDICATIONS (4 realistic Korean medications)
-- =============================================================================

INSERT INTO medications
  (id, person_id, name, dosage, form, frequency,
   prescribed_by, prescribed_at, start_date, end_date,
   is_active, side_effects, notes)
VALUES
  -- 혈압약: 아모디핀 (Amlodipine)
  ('00000000-0000-0000-0000-000000000701',
   '00000000-0000-0000-0000-000000000101',
   '노바스크 (아모디핀)', '5mg', '정제', 'ONCE_DAILY',
   '강남세브란스병원 김의사',
   NOW() - INTERVAL '365 days',
   NOW() - INTERVAL '365 days', NULL,
   TRUE,
   '발목 부종, 어지러움',
   '아침 식후 30분 복용. 자몽 주스와 함께 복용 금지.'),

  -- 당뇨약: 메트포르민 (Metformin)
  ('00000000-0000-0000-0000-000000000702',
   '00000000-0000-0000-0000-000000000101',
   '글루코파지 (메트포르민)', '500mg', '정제', 'TWICE_DAILY',
   '강남세브란스병원 김의사',
   NOW() - INTERVAL '300 days',
   NOW() - INTERVAL '300 days', NULL,
   TRUE,
   '소화불량, 설사',
   '아침, 저녁 식사 직후 복용. 공복 시 복용 금지.'),

  -- 골다공증약: 알렌드로네이트 (Alendronate)
  ('00000000-0000-0000-0000-000000000703',
   '00000000-0000-0000-0000-000000000101',
   '포사맥스 (알렌드로네이트)', '70mg', '정제', 'WEEKLY',
   '강남세브란스병원 박의사',
   NOW() - INTERVAL '180 days',
   NOW() - INTERVAL '180 days', NULL,
   TRUE,
   '속쓰림, 식도 자극',
   '매주 월요일 기상 직후 공복 복용. 복용 후 30분간 눕지 말 것.'),

  -- 위장약: 란소프라졸 (Lansoprazole)
  ('00000000-0000-0000-0000-000000000704',
   '00000000-0000-0000-0000-000000000101',
   '란스톤 (란소프라졸)', '15mg', '캡슐', 'ONCE_DAILY',
   '강남세브란스병원 김의사',
   NOW() - INTERVAL '90 days',
   NOW() - INTERVAL '90 days', NULL,
   TRUE,
   '두통, 복통',
   '아침 식전 30분 복용. 위산 과다 및 역류성 식도염 관리.')
ON CONFLICT DO NOTHING;

-- =============================================================================
-- 11. MEDICATION SCHEDULES
-- =============================================================================

INSERT INTO medication_schedules
  (id, medication_id, time_of_day, day_of_week, is_active)
VALUES
  -- 혈압약: 매일 아침
  ('00000000-0000-0000-0000-000000000a01',
   '00000000-0000-0000-0000-000000000701',
   '08:00', NULL, TRUE),

  -- 당뇨약: 매일 아침 + 저녁
  ('00000000-0000-0000-0000-000000000a02',
   '00000000-0000-0000-0000-000000000702',
   '08:00', NULL, TRUE),
  ('00000000-0000-0000-0000-000000000a03',
   '00000000-0000-0000-0000-000000000702',
   '18:30', NULL, TRUE),

  -- 골다공증약: 매주 월요일 아침
  ('00000000-0000-0000-0000-000000000a04',
   '00000000-0000-0000-0000-000000000703',
   '07:00', 'MONDAY', TRUE),

  -- 위장약: 매일 아침 식전
  ('00000000-0000-0000-0000-000000000a05',
   '00000000-0000-0000-0000-000000000704',
   '07:30', NULL, TRUE)
ON CONFLICT DO NOTHING;

-- =============================================================================
-- 12. MEDICATION EVENTS (past 3 days, mix of TAKEN and MISSED)
-- =============================================================================

INSERT INTO medication_events
  (id, medication_id, scheduled_for, status, taken_at, notes)
VALUES
  -- === 3 days ago ===
  -- 혈압약 아침: TAKEN
  ('00000000-0000-0000-0000-000000000b01',
   '00000000-0000-0000-0000-000000000701',
   (NOW() - INTERVAL '3 days')::DATE + TIME '08:00',
   'TAKEN',
   (NOW() - INTERVAL '3 days')::DATE + TIME '08:12',
   NULL),
  -- 당뇨약 아침: TAKEN
  ('00000000-0000-0000-0000-000000000b02',
   '00000000-0000-0000-0000-000000000702',
   (NOW() - INTERVAL '3 days')::DATE + TIME '08:00',
   'TAKEN',
   (NOW() - INTERVAL '3 days')::DATE + TIME '08:15',
   NULL),
  -- 당뇨약 저녁: MISSED
  ('00000000-0000-0000-0000-000000000b03',
   '00000000-0000-0000-0000-000000000702',
   (NOW() - INTERVAL '3 days')::DATE + TIME '18:30',
   'MISSED',
   NULL,
   '저녁 식사를 거르셔서 복용하지 않음'),
  -- 위장약 아침: TAKEN
  ('00000000-0000-0000-0000-000000000b04',
   '00000000-0000-0000-0000-000000000704',
   (NOW() - INTERVAL '3 days')::DATE + TIME '07:30',
   'TAKEN',
   (NOW() - INTERVAL '3 days')::DATE + TIME '07:35',
   NULL),

  -- === 2 days ago ===
  -- 혈압약 아침: TAKEN
  ('00000000-0000-0000-0000-000000000b05',
   '00000000-0000-0000-0000-000000000701',
   (NOW() - INTERVAL '2 days')::DATE + TIME '08:00',
   'TAKEN',
   (NOW() - INTERVAL '2 days')::DATE + TIME '08:05',
   NULL),
  -- 당뇨약 아침: TAKEN
  ('00000000-0000-0000-0000-000000000b06',
   '00000000-0000-0000-0000-000000000702',
   (NOW() - INTERVAL '2 days')::DATE + TIME '08:00',
   'TAKEN',
   (NOW() - INTERVAL '2 days')::DATE + TIME '08:08',
   NULL),
  -- 당뇨약 저녁: TAKEN
  ('00000000-0000-0000-0000-000000000b07',
   '00000000-0000-0000-0000-000000000702',
   (NOW() - INTERVAL '2 days')::DATE + TIME '18:30',
   'TAKEN',
   (NOW() - INTERVAL '2 days')::DATE + TIME '18:45',
   NULL),
  -- 위장약 아침: TAKEN
  ('00000000-0000-0000-0000-000000000b08',
   '00000000-0000-0000-0000-000000000704',
   (NOW() - INTERVAL '2 days')::DATE + TIME '07:30',
   'TAKEN',
   (NOW() - INTERVAL '2 days')::DATE + TIME '07:32',
   NULL),

  -- === 1 day ago (yesterday) ===
  -- 혈압약 아침: MISSED
  ('00000000-0000-0000-0000-000000000b09',
   '00000000-0000-0000-0000-000000000701',
   (NOW() - INTERVAL '1 day')::DATE + TIME '08:00',
   'MISSED',
   NULL,
   '보호사 방문 전 취침 중이셔서 놓침'),
  -- 당뇨약 아침: TAKEN
  ('00000000-0000-0000-0000-000000000b10',
   '00000000-0000-0000-0000-000000000702',
   (NOW() - INTERVAL '1 day')::DATE + TIME '08:00',
   'TAKEN',
   (NOW() - INTERVAL '1 day')::DATE + TIME '09:20',
   '늦게 일어나셔서 지연 복용'),
  -- 당뇨약 저녁: TAKEN
  ('00000000-0000-0000-0000-000000000b11',
   '00000000-0000-0000-0000-000000000702',
   (NOW() - INTERVAL '1 day')::DATE + TIME '18:30',
   'TAKEN',
   (NOW() - INTERVAL '1 day')::DATE + TIME '18:40',
   NULL),
  -- 위장약 아침: MISSED
  ('00000000-0000-0000-0000-000000000b12',
   '00000000-0000-0000-0000-000000000704',
   (NOW() - INTERVAL '1 day')::DATE + TIME '07:30',
   'MISSED',
   NULL,
   '혈압약과 함께 놓침')
ON CONFLICT DO NOTHING;

-- =============================================================================
-- 13. APPOINTMENTS (1 completed, 1 scheduled, 1 confirmed)
-- =============================================================================

INSERT INTO appointments
  (id, person_id, institution_name, institution_type,
   appointment_date, status, purpose, notes, address)
VALUES
  -- Past: completed checkup
  ('00000000-0000-0000-0000-000000000d01',
   '00000000-0000-0000-0000-000000000101',
   '강남세브란스병원', 'CLINIC',
   (NOW() - INTERVAL '14 days')::DATE + TIME '10:00',
   'COMPLETED',
   '정기 혈압/혈당 검진',
   '혈압 132/84, 공복혈당 148mg/dL. 당뇨약 용량 유지, 3개월 후 재검.',
   '서울시 강남구 도곡동 146-92'),

  -- Upcoming: scheduled rehabilitation
  ('00000000-0000-0000-0000-000000000d02',
   '00000000-0000-0000-0000-000000000101',
   '강남세브란스 재활센터', 'REHABILITATION_CENTER',
   (NOW() + INTERVAL '5 days')::DATE + TIME '14:00',
   'SCHEDULED',
   '무릎 관절 물리치료',
   NULL,
   '서울시 강남구 도곡동 146-92'),

  -- Upcoming: confirmed dental
  ('00000000-0000-0000-0000-000000000d03',
   '00000000-0000-0000-0000-000000000101',
   '미소치과의원', 'CLINIC',
   (NOW() + INTERVAL '10 days')::DATE + TIME '11:00',
   'CONFIRMED',
   '틀니 점검 및 잇몸 검진',
   '보호자(김영호) 동행 예정',
   '서울시 강남구 역삼동 45-67')
ON CONFLICT DO NOTHING;

-- =============================================================================
-- 14. MEDICAL HISTORY ENTRIES (3 conditions)
-- =============================================================================

INSERT INTO medical_history_entries
  (id, person_id, condition, diagnosed_at, treated_by, status, notes)
VALUES
  ('00000000-0000-0000-0000-000000000e01',
   '00000000-0000-0000-0000-000000000101',
   '고혈압 (본태성)',
   '2010-06-15'::TIMESTAMPTZ,
   '강남세브란스병원 내과 김철수 교수',
   'active',
   '아모디핀 5mg 복용 중. 혈압 130/80 목표. 저염식 권장.'),

  ('00000000-0000-0000-0000-000000000e02',
   '00000000-0000-0000-0000-000000000101',
   '제2형 당뇨병',
   '2015-03-20'::TIMESTAMPTZ,
   '강남세브란스병원 내분비내과 이영희 교수',
   'active',
   '메트포르민 500mg 1일 2회 복용 중. HbA1c 7.2%. 식이 조절 및 운동 병행.'),

  ('00000000-0000-0000-0000-000000000e03',
   '00000000-0000-0000-0000-000000000101',
   '골다공증',
   '2020-11-10'::TIMESTAMPTZ,
   '강남세브란스병원 정형외과 박지훈 교수',
   'active',
   '포사맥스 70mg 주 1회 복용 중. T-score -2.8. 칼슘/비타민D 보충제 추가 권장. 낙상 주의.')
ON CONFLICT DO NOTHING;

-- =============================================================================
-- 15. INSTITUTION REFERRAL (home care -> rehab center, accepted)
-- =============================================================================

INSERT INTO institution_referrals
  (id, from_provider_id, to_provider_id, senior_person_id,
   status, reason, notes,
   referred_at, accepted_at)
VALUES
  ('00000000-0000-0000-0000-000000000901',
   '00000000-0000-0000-0000-000000000301',
   '00000000-0000-0000-0000-000000000302',
   '00000000-0000-0000-0000-000000000101',
   'ACCEPTED',
   '무릎 관절 기능 저하로 전문 재활 치료 필요',
   '보행기 사용 중이나 실외 보행 시 불안정. 물리치료를 통한 하지 근력 강화 목표.',
   NOW() - INTERVAL '7 days',
   NOW() - INTERVAL '5 days')
ON CONFLICT DO NOTHING;

-- =============================================================================
-- 16. ELIGIBILITY CASE (approved for LTCI)
-- =============================================================================

INSERT INTO eligibility_cases
  (id, senior_id, status, program_name,
   application_date, determination_date, notes, created_by)
VALUES
  ('00000000-0000-0000-0000-000000000601',
   '00000000-0000-0000-0000-000000000201',
   'APPROVED',
   '노인장기요양보험 재가급여',
   NOW() - INTERVAL '120 days',
   NOW() - INTERVAL '100 days',
   '장기요양 3등급 판정. 재가급여 월 한도액 1,417,200원 적용. 유효기간 2027-03-17.',
   '00000000-0000-0000-0000-000000000005')
ON CONFLICT DO NOTHING;

-- =============================================================================
-- 17. OBSERVABILITY SIGNALS (5 mixed events)
-- =============================================================================

INSERT INTO observability_signals
  (id, event_type, severity, subject_person_id,
   actor_user_id, entity_type, entity_id, message, metadata)
VALUES
  -- 1. Visit completed normally
  ('00000000-0000-0000-0000-000000001001',
   'VISIT_COMPLETED', 'INFO',
   '00000000-0000-0000-0000-000000000101',
   '00000000-0000-0000-0000-000000000003',
   'visit', '00000000-0000-0000-0000-000000000801',
   '방문요양 서비스 정상 완료',
   '{"duration_minutes": 170, "tasks_completed": 4, "tasks_total": 4}'::JSONB),

  -- 2. Medication missed (warning)
  ('00000000-0000-0000-0000-000000001002',
   'MEDICATION_MISSED', 'WARNING',
   '00000000-0000-0000-0000-000000000101',
   NULL,
   'medication_event', '00000000-0000-0000-0000-000000000b03',
   '저녁 당뇨약 (글루코파지) 복용 누락',
   '{"medication_name": "글루코파지 (메트포르민)", "scheduled_time": "18:30", "reason": "저녁 식사 거름"}'::JSONB),

  -- 3. Medication missed (alert - consecutive)
  ('00000000-0000-0000-0000-000000001003',
   'MEDICATION_MISSED', 'ALERT',
   '00000000-0000-0000-0000-000000000101',
   NULL,
   'medication_event', '00000000-0000-0000-0000-000000000b09',
   '아침 혈압약 (노바스크) 복용 누락 - 연속 미복용 주의',
   '{"medication_name": "노바스크 (아모디핀)", "scheduled_time": "08:00", "consecutive_misses": 1}'::JSONB),

  -- 4. Eligibility approved
  ('00000000-0000-0000-0000-000000001004',
   'ELIGIBILITY_STATUS_CHANGED', 'INFO',
   '00000000-0000-0000-0000-000000000101',
   '00000000-0000-0000-0000-000000000005',
   'eligibility_case', '00000000-0000-0000-0000-000000000601',
   '노인장기요양보험 재가급여 승인 완료',
   '{"program": "노인장기요양보험 재가급여", "old_status": "UNDER_REVIEW", "new_status": "APPROVED"}'::JSONB),

  -- 5. Referral updated
  ('00000000-0000-0000-0000-000000001005',
   'REFERRAL_UPDATED', 'INFO',
   '00000000-0000-0000-0000-000000000101',
   '00000000-0000-0000-0000-000000000004',
   'institution_referral', '00000000-0000-0000-0000-000000000901',
   '강남세브란스 재활센터 의뢰 수락됨',
   '{"from_provider": "행복재가센터", "to_provider": "강남세브란스 재활센터", "old_status": "CREATED", "new_status": "ACCEPTED"}'::JSONB)
ON CONFLICT DO NOTHING;

-- =============================================================================
-- 18. NOTIFICATIONS (5 for senior user)
-- =============================================================================

INSERT INTO notifications
  (id, user_id, type, title, message, link, is_read)
VALUES
  ('00000000-0000-0000-0000-000000001101',
   '00000000-0000-0000-0000-000000000001',
   'REMINDER', '오늘 투약 알림',
   '아침 약 복용 시간입니다. 노바스크, 글루코파지, 란스톤을 식사 후 복용해 주세요.',
   '/medications', FALSE),

  ('00000000-0000-0000-0000-000000001102',
   '00000000-0000-0000-0000-000000000001',
   'INFO', '방문 일정 안내',
   '오늘 오전 9시 박미영 요양보호사가 방문 예정입니다.',
   '/visits', TRUE),

  ('00000000-0000-0000-0000-000000001103',
   '00000000-0000-0000-0000-000000000001',
   'ACTION_REQUIRED', '진료 예약 확인 필요',
   '3월 23일 강남세브란스 재활센터 물리치료 예약을 확인해 주세요.',
   '/appointments', FALSE),

  ('00000000-0000-0000-0000-000000001104',
   '00000000-0000-0000-0000-000000000001',
   'WARNING', '투약 누락 알림',
   '어제 아침 혈압약(노바스크)을 복용하지 않으셨습니다. 내일 아침에 잊지 말고 복용해 주세요.',
   '/medications', FALSE),

  ('00000000-0000-0000-0000-000000001105',
   '00000000-0000-0000-0000-000000000001',
   'INFO', '장기요양 급여 승인',
   '노인장기요양보험 재가급여가 승인되었습니다. 월 한도액 1,417,200원이 적용됩니다.',
   '/eligibility', TRUE)
ON CONFLICT DO NOTHING;

-- =============================================================================
-- 19. AUDIT LOGS (3 entries)
-- =============================================================================

INSERT INTO audit_logs
  (id, user_id, action, entity_type, entity_id,
   old_value, new_value)
VALUES
  -- Family member granted consent
  ('00000000-0000-0000-0000-000000001201',
   '00000000-0000-0000-0000-000000000002',
   'CONSENT_GRANT', 'consent_records', '00000000-0000-0000-0000-0000000000c1',
   NULL,
   '{"purpose": "MEDICAL_SHARE", "subject": "김복순"}'::JSONB),

  -- Government reviewer approved eligibility
  ('00000000-0000-0000-0000-000000001202',
   '00000000-0000-0000-0000-000000000005',
   'STATUS_CHANGE', 'eligibility_cases', '00000000-0000-0000-0000-000000000601',
   '{"status": "UNDER_REVIEW"}'::JSONB,
   '{"status": "APPROVED", "program": "노인장기요양보험 재가급여"}'::JSONB),

  -- Provider admin created care plan
  ('00000000-0000-0000-0000-000000001203',
   '00000000-0000-0000-0000-000000000004',
   'CREATE', 'care_plans', '00000000-0000-0000-0000-000000000401',
   NULL,
   '{"title": "김복순 어르신 재가요양 케어플랜 (2026년 1분기)", "status": "ACTIVE"}'::JSONB)
ON CONFLICT DO NOTHING;

COMMIT;
