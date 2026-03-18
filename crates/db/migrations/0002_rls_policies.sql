-- =============================================================================
-- Row-Level Security Policies
-- Enable RLS and create policies for all domain tables
-- =============================================================================

-- Helper: current user context from session variables
CREATE OR REPLACE FUNCTION current_app_user_id() RETURNS UUID AS $$
  SELECT NULLIF(current_setting('app.current_user_id', true), '')::UUID;
$$ LANGUAGE SQL STABLE;

CREATE OR REPLACE FUNCTION current_app_role() RETURNS TEXT AS $$
  SELECT NULLIF(current_setting('app.current_role', true), '');
$$ LANGUAGE SQL STABLE;

CREATE OR REPLACE FUNCTION current_app_tenant_id() RETURNS UUID AS $$
  SELECT NULLIF(current_setting('app.current_tenant_id', true), '')::UUID;
$$ LANGUAGE SQL STABLE;

-- Enable RLS on all domain tables (skip auth tables: users, accounts, sessions, verification_tokens)
ALTER TABLE person_profiles ENABLE ROW LEVEL SECURITY;
ALTER TABLE senior_profiles ENABLE ROW LEVEL SECURITY;
ALTER TABLE family_relationships ENABLE ROW LEVEL SECURITY;
ALTER TABLE consent_records ENABLE ROW LEVEL SECURITY;
ALTER TABLE provider_organizations ENABLE ROW LEVEL SECURITY;
ALTER TABLE caregiver_applications ENABLE ROW LEVEL SECURITY;
ALTER TABLE caregiver_credentials ENABLE ROW LEVEL SECURITY;
ALTER TABLE service_regions ENABLE ROW LEVEL SECURITY;
ALTER TABLE availability_slots ENABLE ROW LEVEL SECURITY;
ALTER TABLE service_types ENABLE ROW LEVEL SECURITY;
ALTER TABLE match_requests ENABLE ROW LEVEL SECURITY;
ALTER TABLE match_recommendations ENABLE ROW LEVEL SECURITY;
ALTER TABLE care_plans ENABLE ROW LEVEL SECURITY;
ALTER TABLE visits ENABLE ROW LEVEL SECURITY;
ALTER TABLE daily_observations ENABLE ROW LEVEL SECURITY;
ALTER TABLE incidents ENABLE ROW LEVEL SECURITY;
ALTER TABLE medical_history_entries ENABLE ROW LEVEL SECURITY;
ALTER TABLE medications ENABLE ROW LEVEL SECURITY;
ALTER TABLE medication_schedules ENABLE ROW LEVEL SECURITY;
ALTER TABLE medication_events ENABLE ROW LEVEL SECURITY;
ALTER TABLE appointments ENABLE ROW LEVEL SECURITY;
ALTER TABLE institution_referrals ENABLE ROW LEVEL SECURITY;
ALTER TABLE eligibility_cases ENABLE ROW LEVEL SECURITY;
ALTER TABLE approval_steps ENABLE ROW LEVEL SECURITY;
ALTER TABLE claim_or_subsidy_records ENABLE ROW LEVEL SECURITY;
ALTER TABLE observability_signals ENABLE ROW LEVEL SECURITY;
ALTER TABLE notifications ENABLE ROW LEVEL SECURITY;
ALTER TABLE audit_logs ENABLE ROW LEVEL SECURITY;

-- Platform admin bypasses RLS
CREATE POLICY admin_all ON person_profiles FOR ALL USING (current_app_role() = 'PLATFORM_ADMIN');
CREATE POLICY admin_all ON senior_profiles FOR ALL USING (current_app_role() = 'PLATFORM_ADMIN');
CREATE POLICY admin_all ON family_relationships FOR ALL USING (current_app_role() = 'PLATFORM_ADMIN');
CREATE POLICY admin_all ON consent_records FOR ALL USING (current_app_role() = 'PLATFORM_ADMIN');
CREATE POLICY admin_all ON provider_organizations FOR ALL USING (current_app_role() = 'PLATFORM_ADMIN');
CREATE POLICY admin_all ON caregiver_applications FOR ALL USING (current_app_role() = 'PLATFORM_ADMIN');
CREATE POLICY admin_all ON caregiver_credentials FOR ALL USING (current_app_role() = 'PLATFORM_ADMIN');
CREATE POLICY admin_all ON service_regions FOR ALL USING (current_app_role() = 'PLATFORM_ADMIN');
CREATE POLICY admin_all ON availability_slots FOR ALL USING (current_app_role() = 'PLATFORM_ADMIN');
CREATE POLICY admin_all ON service_types FOR ALL USING (current_app_role() = 'PLATFORM_ADMIN');
CREATE POLICY admin_all ON match_requests FOR ALL USING (current_app_role() = 'PLATFORM_ADMIN');
CREATE POLICY admin_all ON match_recommendations FOR ALL USING (current_app_role() = 'PLATFORM_ADMIN');
CREATE POLICY admin_all ON care_plans FOR ALL USING (current_app_role() = 'PLATFORM_ADMIN');
CREATE POLICY admin_all ON visits FOR ALL USING (current_app_role() = 'PLATFORM_ADMIN');
CREATE POLICY admin_all ON daily_observations FOR ALL USING (current_app_role() = 'PLATFORM_ADMIN');
CREATE POLICY admin_all ON incidents FOR ALL USING (current_app_role() = 'PLATFORM_ADMIN');
CREATE POLICY admin_all ON medical_history_entries FOR ALL USING (current_app_role() = 'PLATFORM_ADMIN');
CREATE POLICY admin_all ON medications FOR ALL USING (current_app_role() = 'PLATFORM_ADMIN');
CREATE POLICY admin_all ON medication_schedules FOR ALL USING (current_app_role() = 'PLATFORM_ADMIN');
CREATE POLICY admin_all ON medication_events FOR ALL USING (current_app_role() = 'PLATFORM_ADMIN');
CREATE POLICY admin_all ON appointments FOR ALL USING (current_app_role() = 'PLATFORM_ADMIN');
CREATE POLICY admin_all ON institution_referrals FOR ALL USING (current_app_role() = 'PLATFORM_ADMIN');
CREATE POLICY admin_all ON eligibility_cases FOR ALL USING (current_app_role() = 'PLATFORM_ADMIN');
CREATE POLICY admin_all ON approval_steps FOR ALL USING (current_app_role() = 'PLATFORM_ADMIN');
CREATE POLICY admin_all ON claim_or_subsidy_records FOR ALL USING (current_app_role() = 'PLATFORM_ADMIN');
CREATE POLICY admin_all ON observability_signals FOR ALL USING (current_app_role() = 'PLATFORM_ADMIN');
CREATE POLICY admin_all ON notifications FOR ALL USING (current_app_role() = 'PLATFORM_ADMIN');
CREATE POLICY admin_all ON audit_logs FOR ALL USING (current_app_role() = 'PLATFORM_ADMIN');

-- Person profiles: users can see their own
CREATE POLICY own_profile ON person_profiles FOR ALL
  USING (user_id = current_app_user_id());

-- Senior profiles: linked via person_profiles
CREATE POLICY own_senior ON senior_profiles FOR ALL
  USING (person_id IN (SELECT id FROM person_profiles WHERE user_id = current_app_user_id()));

-- Notifications: users see their own
CREATE POLICY own_notifications ON notifications FOR ALL
  USING (user_id = current_app_user_id());

-- Caregiver applications: own applications
CREATE POLICY own_application ON caregiver_applications FOR ALL
  USING (user_id = current_app_user_id());

-- Government reviewer: read all eligibility cases
CREATE POLICY gov_eligibility ON eligibility_cases FOR SELECT
  USING (current_app_role() = 'GOVERNMENT_REVIEWER');

-- Government reviewer: read all audit logs
CREATE POLICY gov_audit ON audit_logs FOR SELECT
  USING (current_app_role() = 'GOVERNMENT_REVIEWER');

-- Government reviewer: read all observability signals
CREATE POLICY gov_observability ON observability_signals FOR SELECT
  USING (current_app_role() = 'GOVERNMENT_REVIEWER');

-- Government reviewer: read all providers
CREATE POLICY gov_providers ON provider_organizations FOR SELECT
  USING (current_app_role() = 'GOVERNMENT_REVIEWER');
