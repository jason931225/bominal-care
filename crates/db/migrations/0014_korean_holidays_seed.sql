-- =============================================================================
-- Migration 0014: Korean Public Holidays Seed — 2026 and 2027
-- =============================================================================
-- Inserts official Korean public holidays (공휴일) for 2026 and 2027,
-- including substitute holidays (대체공휴일) where a holiday falls on a
-- Sunday or Saturday (per the expanded 대체공휴일 rules in force from 2023).
--
-- Uses ON CONFLICT DO NOTHING so this migration is safe to re-run
-- (idempotent). The korean_holidays table was created in 0013_lifestyle.sql.
-- =============================================================================

-- -------------------------------------------------------------------------
-- 2026 Public Holidays
-- -------------------------------------------------------------------------
INSERT INTO korean_holidays (holiday_date, name_ko, name_en, is_lunar) VALUES
    ('2026-01-01', '신정',       'New Year''s Day',              FALSE),

    -- Lunar New Year (설날) block — Feb 16–18
    ('2026-02-16', '설날 연휴',   'Lunar New Year Eve',           TRUE),
    ('2026-02-17', '설날',        'Lunar New Year',               TRUE),
    ('2026-02-18', '설날 연휴',   'Lunar New Year Day 3',         TRUE),
    -- Feb 18 is Wednesday; no overlap with Sunday → no substitute needed

    ('2026-03-01', '삼일절',      'Independence Movement Day',    FALSE),
    -- Mar 1 is Sunday → substitute: Mar 2 (Monday)
    ('2026-03-02', '삼일절 대체공휴일', 'Independence Movement Day (substitute)', FALSE),

    ('2026-05-05', '어린이날',    'Children''s Day',              FALSE),

    ('2026-05-24', '부처님오신날', 'Buddha''s Birthday',          TRUE),
    -- May 24 is Sunday → substitute: May 25 (Monday)
    ('2026-05-25', '부처님오신날 대체공휴일', 'Buddha''s Birthday (substitute)', TRUE),

    ('2026-06-06', '현충일',      'Memorial Day',                 FALSE),
    -- Jun 6 is Saturday → substitute: Jun 8 (Monday; Jun 7 is Sunday)
    ('2026-06-08', '현충일 대체공휴일', 'Memorial Day (substitute)', FALSE),

    ('2026-08-15', '광복절',      'Liberation Day',               FALSE),
    -- Aug 15 is Saturday → substitute: Aug 17 (Monday; Aug 16 is Sunday)
    ('2026-08-17', '광복절 대체공휴일', 'Liberation Day (substitute)', FALSE),

    -- Chuseok (추석) block — Sep 24–26
    ('2026-09-24', '추석 연휴',   'Chuseok Eve',                  TRUE),
    ('2026-09-25', '추석',        'Chuseok',                      TRUE),
    ('2026-09-26', '추석 연휴',   'Chuseok Day 3',                TRUE),
    -- Sep 26 is Saturday → substitute: Sep 28 (Monday; Sep 27 is Sunday)
    ('2026-09-28', '추석 대체공휴일', 'Chuseok (substitute)',      TRUE),

    ('2026-10-03', '개천절',      'National Foundation Day',      FALSE),
    -- Oct 3 is Saturday → substitute: Oct 5 (Monday; Oct 4 is Sunday)
    ('2026-10-05', '개천절 대체공휴일', 'National Foundation Day (substitute)', FALSE),

    ('2026-10-09', '한글날',      'Hangul Day',                   FALSE),
    -- Oct 9 is Friday → no substitute needed

    ('2026-12-25', '크리스마스',  'Christmas',                    FALSE)
    -- Dec 25 is Friday → no substitute needed

ON CONFLICT (holiday_date) DO NOTHING;

-- -------------------------------------------------------------------------
-- 2027 Public Holidays
-- -------------------------------------------------------------------------
INSERT INTO korean_holidays (holiday_date, name_ko, name_en, is_lunar) VALUES
    ('2027-01-01', '신정',       'New Year''s Day',              FALSE),
    -- Jan 1 is Friday → no substitute needed

    -- Lunar New Year (설날) block — Feb 5–7
    ('2027-02-05', '설날 연휴',   'Lunar New Year Eve',           TRUE),
    ('2027-02-06', '설날',        'Lunar New Year',               TRUE),
    ('2027-02-07', '설날 연휴',   'Lunar New Year Day 3',         TRUE),
    -- Feb 6 is Saturday, Feb 7 is Sunday → substitute: Feb 8 (Monday)
    ('2027-02-08', '설날 대체공휴일', 'Lunar New Year (substitute)', TRUE),

    ('2027-03-01', '삼일절',      'Independence Movement Day',    FALSE),
    -- Mar 1 is Monday → no substitute needed

    ('2027-05-05', '어린이날',    'Children''s Day',              FALSE),
    -- May 5 is Wednesday → no substitute needed

    ('2027-05-13', '부처님오신날', 'Buddha''s Birthday',          TRUE),
    -- May 13 is Thursday → no substitute needed

    ('2027-06-06', '현충일',      'Memorial Day',                 FALSE),
    -- Jun 6 is Sunday → substitute: Jun 7 (Monday)
    ('2027-06-07', '현충일 대체공휴일', 'Memorial Day (substitute)', FALSE),

    ('2027-08-15', '광복절',      'Liberation Day',               FALSE),
    -- Aug 15 is Sunday → substitute: Aug 16 (Monday)
    ('2027-08-16', '광복절 대체공휴일', 'Liberation Day (substitute)', FALSE),

    ('2027-10-03', '개천절',      'National Foundation Day',      FALSE),
    -- Oct 3 is Sunday → substitute: Oct 4 (Monday)
    ('2027-10-04', '개천절 대체공휴일', 'National Foundation Day (substitute)', FALSE),

    ('2027-10-09', '한글날',      'Hangul Day',                   FALSE),
    -- Oct 9 is Saturday → substitute: Oct 11 (Monday; Oct 10 is Sunday)
    ('2027-10-11', '한글날 대체공휴일', 'Hangul Day (substitute)',  FALSE),

    -- Chuseok (추석) block — Oct 14–16
    ('2027-10-14', '추석 연휴',   'Chuseok Eve',                  TRUE),
    ('2027-10-15', '추석',        'Chuseok',                      TRUE),
    ('2027-10-16', '추석 연휴',   'Chuseok Day 3',                TRUE),
    -- Oct 16 is Saturday → substitute: Oct 18 (Monday; Oct 17 is Sunday)
    ('2027-10-18', '추석 대체공휴일', 'Chuseok (substitute)',      TRUE),

    ('2027-12-25', '크리스마스',  'Christmas',                    FALSE)
    -- Dec 25 is Saturday → substitute: Dec 27 (Monday; Dec 26 is Sunday)
    ,('2027-12-27', '크리스마스 대체공휴일', 'Christmas (substitute)', FALSE)

ON CONFLICT (holiday_date) DO NOTHING;
