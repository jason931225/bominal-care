-- =============================================================================
-- Migration 0010: Government Materialized Views (de-identified, aggregated)
-- =============================================================================

-- Beneficiary statistics (min 5 per group for de-identification)
CREATE MATERIALIZED VIEW IF NOT EXISTS gov_beneficiary_stats AS
SELECT
    sp.care_level,
    pp.city,
    pp.district,
    COUNT(*) AS beneficiary_count,
    AVG(EXTRACT(YEAR FROM AGE(pp.date_of_birth))) AS avg_age
FROM senior_profiles sp
JOIN person_profiles pp ON pp.id = sp.person_id
GROUP BY sp.care_level, pp.city, pp.district
HAVING COUNT(*) >= 5;

-- Visit statistics
CREATE MATERIALIZED VIEW IF NOT EXISTS gov_visit_stats AS
SELECT
    DATE_TRUNC('week', v.scheduled_start) AS week,
    cp.provider_id,
    v.status,
    COUNT(*) AS visit_count,
    AVG(EXTRACT(EPOCH FROM (v.actual_end - v.actual_start)) / 3600) AS avg_duration_hours
FROM visits v
JOIN care_plans cp ON cp.id = v.care_plan_id
WHERE v.deleted_at IS NULL
GROUP BY DATE_TRUNC('week', v.scheduled_start), cp.provider_id, v.status;

-- Provider compliance
CREATE MATERIALIZED VIEW IF NOT EXISTS gov_provider_compliance AS
SELECT
    po.id AS provider_id,
    po.name AS provider_name,
    po.type AS provider_type,
    po.is_active,
    (SELECT COUNT(*) FROM caregiver_applications ca WHERE ca.provider_id = po.id AND ca.status IN ('APPROVED_PRIVATE_PAY', 'APPROVED_UNDER_PROVIDER')) AS active_caregivers,
    (SELECT COUNT(*) FROM care_plans cp2 WHERE cp2.provider_id = po.id AND cp2.status = 'ACTIVE') AS active_care_plans,
    (SELECT COUNT(*) FROM caregiver_credentials cc
     JOIN caregiver_applications ca2 ON ca2.id = cc.application_id
     WHERE ca2.provider_id = po.id AND cc.expires_at < NOW() AND cc.status = 'VERIFIED') AS expired_credentials
FROM provider_organizations po
WHERE po.is_active = TRUE;

CREATE UNIQUE INDEX IF NOT EXISTS idx_gov_beneficiary_stats ON gov_beneficiary_stats(care_level, city, district);
CREATE UNIQUE INDEX IF NOT EXISTS idx_gov_visit_stats ON gov_visit_stats(week, provider_id, status);
CREATE UNIQUE INDEX IF NOT EXISTS idx_gov_provider_compliance ON gov_provider_compliance(provider_id);
