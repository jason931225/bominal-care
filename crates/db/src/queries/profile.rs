// =============================================================================
// Profile queries — PersonProfile and SeniorProfile CRUD
// Aligned to match 0001_initial_schema.sql column names
// =============================================================================

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use bominal_types::{Gender, KycLevel, UserRole};

// -----------------------------------------------------------------------------
// Input types
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePersonProfileData {
    pub user_id: Uuid,
    pub korean_name: Option<String>,
    pub english_name: Option<String>,
    pub date_of_birth: Option<DateTime<Utc>>,
    pub gender: Option<Gender>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub postal_code: Option<String>,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdatePersonProfileData {
    pub korean_name: Option<String>,
    pub english_name: Option<String>,
    pub date_of_birth: Option<DateTime<Utc>>,
    pub gender: Option<Gender>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub postal_code: Option<String>,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSeniorProfileData {
    pub person_id: Uuid,
    pub care_level: Option<bominal_types::CareLevelEnum>,
    pub has_ltci_certification: Option<bool>,
    pub ltci_number: Option<String>,
    pub primary_diagnosis: Option<String>,
    pub mobility_level: Option<String>,
    pub cognitive_level: Option<String>,
    pub lives_alone: Option<bool>,
    pub preferred_language: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateSeniorProfileData {
    pub care_level: Option<bominal_types::CareLevelEnum>,
    pub has_ltci_certification: Option<bool>,
    pub ltci_number: Option<String>,
    pub primary_diagnosis: Option<String>,
    pub mobility_level: Option<String>,
    pub cognitive_level: Option<String>,
    pub lives_alone: Option<bool>,
    pub preferred_language: Option<String>,
}

// -----------------------------------------------------------------------------
// Row types for joined queries
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSummary {
    pub id: Uuid,
    pub email: Option<String>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub role: UserRole,
    pub kyc_level: KycLevel,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonProfileWithUser {
    pub id: Uuid,
    pub user_id: Uuid,
    pub korean_name: Option<String>,
    pub english_name: Option<String>,
    pub date_of_birth: Option<DateTime<Utc>>,
    pub gender: Option<Gender>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub postal_code: Option<String>,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user: UserSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSummaryShort {
    pub id: Uuid,
    pub email: Option<String>,
    pub name: Option<String>,
    pub role: UserRole,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonProfileSummary {
    pub id: Uuid,
    pub user_id: Uuid,
    pub korean_name: Option<String>,
    pub english_name: Option<String>,
    pub date_of_birth: Option<DateTime<Utc>>,
    pub gender: Option<Gender>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub postal_code: Option<String>,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user: UserSummaryShort,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeniorProfileWithPerson {
    pub id: Uuid,
    pub person_id: Uuid,
    pub care_level: Option<bominal_types::CareLevelEnum>,
    pub copayment_tier: bominal_types::CopaymentTier,
    pub has_ltci_certification: bool,
    pub ltci_number: Option<String>,
    pub primary_diagnosis: Option<String>,
    pub mobility_level: Option<String>,
    pub cognitive_level: Option<String>,
    pub lives_alone: bool,
    pub preferred_language: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub person_profile: PersonProfileSummary,
}

// -----------------------------------------------------------------------------
// Flat row types used by sqlx::query_as for joined queries
// -----------------------------------------------------------------------------

#[derive(Debug, sqlx::FromRow)]
struct PersonProfileUserRow {
    // person_profiles.*
    id: Uuid,
    user_id: Uuid,
    korean_name: Option<String>,
    english_name: Option<String>,
    date_of_birth: Option<DateTime<Utc>>,
    gender: Option<Gender>,
    phone: Option<String>,
    address: Option<String>,
    city: Option<String>,
    district: Option<String>,
    postal_code: Option<String>,
    emergency_contact_name: Option<String>,
    emergency_contact_phone: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    // users (aliased)
    u_id: Uuid,
    u_email: Option<String>,
    u_name: Option<String>,
    u_phone: Option<String>,
    u_role: UserRole,
    u_kyc_level: KycLevel,
    u_is_active: bool,
}

impl PersonProfileUserRow {
    fn into_profile_with_user(self) -> PersonProfileWithUser {
        PersonProfileWithUser {
            id: self.id,
            user_id: self.user_id,
            korean_name: self.korean_name,
            english_name: self.english_name,
            date_of_birth: self.date_of_birth,
            gender: self.gender,
            phone: self.phone,
            address: self.address,
            city: self.city,
            district: self.district,
            postal_code: self.postal_code,
            emergency_contact_name: self.emergency_contact_name,
            emergency_contact_phone: self.emergency_contact_phone,
            created_at: self.created_at,
            updated_at: self.updated_at,
            user: UserSummary {
                id: self.u_id,
                email: self.u_email,
                name: self.u_name,
                phone: self.u_phone,
                role: self.u_role,
                kyc_level: self.u_kyc_level,
                is_active: self.u_is_active,
            },
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct SeniorProfileJoinRow {
    // senior_profiles.*
    id: Uuid,
    person_id: Uuid,
    care_level: Option<bominal_types::CareLevelEnum>,
    copayment_tier: bominal_types::CopaymentTier,
    has_ltci_certification: bool,
    ltci_number: Option<String>,
    primary_diagnosis: Option<String>,
    mobility_level: Option<String>,
    cognitive_level: Option<String>,
    lives_alone: bool,
    preferred_language: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    // person_profiles (aliased)
    pp_id: Uuid,
    pp_user_id: Uuid,
    pp_korean_name: Option<String>,
    pp_english_name: Option<String>,
    pp_date_of_birth: Option<DateTime<Utc>>,
    pp_gender: Option<Gender>,
    pp_phone: Option<String>,
    pp_address: Option<String>,
    pp_city: Option<String>,
    pp_district: Option<String>,
    pp_postal_code: Option<String>,
    pp_emergency_contact_name: Option<String>,
    pp_emergency_contact_phone: Option<String>,
    pp_created_at: DateTime<Utc>,
    pp_updated_at: DateTime<Utc>,
    // users (aliased)
    u_id: Uuid,
    u_email: Option<String>,
    u_name: Option<String>,
    u_role: UserRole,
}

impl SeniorProfileJoinRow {
    fn into_senior_with_person(self) -> SeniorProfileWithPerson {
        SeniorProfileWithPerson {
            id: self.id,
            person_id: self.person_id,
            care_level: self.care_level,
            copayment_tier: self.copayment_tier,
            has_ltci_certification: self.has_ltci_certification,
            ltci_number: self.ltci_number,
            primary_diagnosis: self.primary_diagnosis,
            mobility_level: self.mobility_level,
            cognitive_level: self.cognitive_level,
            lives_alone: self.lives_alone,
            preferred_language: self.preferred_language,
            created_at: self.created_at,
            updated_at: self.updated_at,
            person_profile: PersonProfileSummary {
                id: self.pp_id,
                user_id: self.pp_user_id,
                korean_name: self.pp_korean_name,
                english_name: self.pp_english_name,
                date_of_birth: self.pp_date_of_birth,
                gender: self.pp_gender,
                phone: self.pp_phone,
                address: self.pp_address,
                city: self.pp_city,
                district: self.pp_district,
                postal_code: self.pp_postal_code,
                emergency_contact_name: self.pp_emergency_contact_name,
                emergency_contact_phone: self.pp_emergency_contact_phone,
                created_at: self.pp_created_at,
                updated_at: self.pp_updated_at,
                user: UserSummaryShort {
                    id: self.u_id,
                    email: self.u_email,
                    name: self.u_name,
                    role: self.u_role,
                },
            },
        }
    }
}

// -----------------------------------------------------------------------------
// PersonProfile row (for INSERT / UPDATE RETURNING *)
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PersonProfileRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub korean_name: Option<String>,
    pub english_name: Option<String>,
    pub date_of_birth: Option<DateTime<Utc>>,
    pub gender: Option<Gender>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub postal_code: Option<String>,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

// -----------------------------------------------------------------------------
// SeniorProfile row (for INSERT / UPDATE RETURNING *)
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SeniorProfileRow {
    pub id: Uuid,
    pub person_id: Uuid,
    pub care_level: Option<bominal_types::CareLevelEnum>,
    pub copayment_tier: bominal_types::CopaymentTier,
    pub has_ltci_certification: bool,
    pub ltci_number: Option<String>,
    pub primary_diagnosis: Option<String>,
    pub mobility_level: Option<String>,
    pub cognitive_level: Option<String>,
    pub lives_alone: bool,
    pub preferred_language: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// -----------------------------------------------------------------------------
// Service functions
// -----------------------------------------------------------------------------

pub async fn create_person_profile(
    pool: &PgPool,
    data: &CreatePersonProfileData,
) -> Result<PersonProfileRow, sqlx::Error> {
    sqlx::query_as::<_, PersonProfileRow>(
        "INSERT INTO person_profiles (
           user_id, korean_name, english_name, date_of_birth, gender,
           phone, address, city, district, postal_code,
           emergency_contact_name, emergency_contact_phone
         ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12)
         RETURNING *",
    )
    .bind(data.user_id)
    .bind(&data.korean_name)
    .bind(&data.english_name)
    .bind(data.date_of_birth)
    .bind(data.gender)
    .bind(&data.phone)
    .bind(&data.address)
    .bind(&data.city)
    .bind(&data.district)
    .bind(&data.postal_code)
    .bind(&data.emergency_contact_name)
    .bind(&data.emergency_contact_phone)
    .fetch_one(pool)
    .await
}

pub async fn update_person_profile(
    pool: &PgPool,
    id: Uuid,
    data: &UpdatePersonProfileData,
) -> Result<PersonProfileRow, sqlx::Error> {
    sqlx::query_as::<_, PersonProfileRow>(
        "UPDATE person_profiles SET
           korean_name = COALESCE($1, korean_name),
           english_name = COALESCE($2, english_name),
           date_of_birth = COALESCE($3, date_of_birth),
           gender = COALESCE($4, gender),
           phone = COALESCE($5, phone),
           address = COALESCE($6, address),
           city = COALESCE($7, city),
           district = COALESCE($8, district),
           postal_code = COALESCE($9, postal_code),
           emergency_contact_name = COALESCE($10, emergency_contact_name),
           emergency_contact_phone = COALESCE($11, emergency_contact_phone),
           updated_at = NOW()
         WHERE id = $12
         RETURNING *",
    )
    .bind(&data.korean_name)
    .bind(&data.english_name)
    .bind(data.date_of_birth)
    .bind(data.gender)
    .bind(&data.phone)
    .bind(&data.address)
    .bind(&data.city)
    .bind(&data.district)
    .bind(&data.postal_code)
    .bind(&data.emergency_contact_name)
    .bind(&data.emergency_contact_phone)
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn get_person_profile(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<PersonProfileWithUser>, sqlx::Error> {
    let row = sqlx::query_as::<_, PersonProfileUserRow>(
        "SELECT
           pp.id, pp.user_id, pp.korean_name, pp.english_name,
           pp.date_of_birth, pp.gender, pp.phone, pp.address,
           pp.city, pp.district, pp.postal_code,
           pp.emergency_contact_name, pp.emergency_contact_phone,
           pp.created_at, pp.updated_at,
           u.id        AS u_id,
           u.email     AS u_email,
           u.name      AS u_name,
           u.phone     AS u_phone,
           u.role      AS u_role,
           u.kyc_level AS u_kyc_level,
           u.is_active AS u_is_active
         FROM person_profiles pp
         JOIN users u ON u.id = pp.user_id
         WHERE pp.id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(PersonProfileUserRow::into_profile_with_user))
}

pub async fn get_person_profile_by_user_id(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Option<PersonProfileWithUser>, sqlx::Error> {
    let row = sqlx::query_as::<_, PersonProfileUserRow>(
        "SELECT
           pp.id, pp.user_id, pp.korean_name, pp.english_name,
           pp.date_of_birth, pp.gender, pp.phone, pp.address,
           pp.city, pp.district, pp.postal_code,
           pp.emergency_contact_name, pp.emergency_contact_phone,
           pp.created_at, pp.updated_at,
           u.id        AS u_id,
           u.email     AS u_email,
           u.name      AS u_name,
           u.phone     AS u_phone,
           u.role      AS u_role,
           u.kyc_level AS u_kyc_level,
           u.is_active AS u_is_active
         FROM person_profiles pp
         JOIN users u ON u.id = pp.user_id
         WHERE pp.user_id = $1",
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(PersonProfileUserRow::into_profile_with_user))
}

pub async fn create_senior_profile(
    pool: &PgPool,
    data: &CreateSeniorProfileData,
) -> Result<SeniorProfileRow, sqlx::Error> {
    sqlx::query_as::<_, SeniorProfileRow>(
        "INSERT INTO senior_profiles (
           person_id, care_level, has_ltci_certification, ltci_number,
           primary_diagnosis, mobility_level, cognitive_level,
           lives_alone, preferred_language
         ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,COALESCE($9,'ko'))
         RETURNING *",
    )
    .bind(data.person_id)
    .bind(data.care_level)
    .bind(data.has_ltci_certification.unwrap_or(false))
    .bind(&data.ltci_number)
    .bind(&data.primary_diagnosis)
    .bind(&data.mobility_level)
    .bind(&data.cognitive_level)
    .bind(data.lives_alone.unwrap_or(false))
    .bind(&data.preferred_language)
    .fetch_one(pool)
    .await
}

pub async fn update_senior_profile(
    pool: &PgPool,
    id: Uuid,
    data: &UpdateSeniorProfileData,
) -> Result<SeniorProfileRow, sqlx::Error> {
    sqlx::query_as::<_, SeniorProfileRow>(
        "UPDATE senior_profiles SET
           care_level = COALESCE($1, care_level),
           has_ltci_certification = COALESCE($2, has_ltci_certification),
           ltci_number = COALESCE($3, ltci_number),
           primary_diagnosis = COALESCE($4, primary_diagnosis),
           mobility_level = COALESCE($5, mobility_level),
           cognitive_level = COALESCE($6, cognitive_level),
           lives_alone = COALESCE($7, lives_alone),
           preferred_language = COALESCE($8, preferred_language),
           updated_at = NOW()
         WHERE id = $9
         RETURNING *",
    )
    .bind(data.care_level)
    .bind(data.has_ltci_certification)
    .bind(&data.ltci_number)
    .bind(&data.primary_diagnosis)
    .bind(&data.mobility_level)
    .bind(&data.cognitive_level)
    .bind(data.lives_alone)
    .bind(&data.preferred_language)
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn get_senior_profile(
    pool: &PgPool,
    person_id: Uuid,
) -> Result<Option<SeniorProfileWithPerson>, sqlx::Error> {
    let row = sqlx::query_as::<_, SeniorProfileJoinRow>(
        "SELECT
           sp.id, sp.person_id, sp.care_level, sp.copayment_tier, sp.has_ltci_certification,
           sp.ltci_number, sp.primary_diagnosis, sp.mobility_level,
           sp.cognitive_level, sp.lives_alone, sp.preferred_language,
           sp.created_at, sp.updated_at,
           pp.id                       AS pp_id,
           pp.user_id                  AS pp_user_id,
           pp.korean_name              AS pp_korean_name,
           pp.english_name             AS pp_english_name,
           pp.date_of_birth            AS pp_date_of_birth,
           pp.gender                   AS pp_gender,
           pp.phone                    AS pp_phone,
           pp.address                  AS pp_address,
           pp.city                     AS pp_city,
           pp.district                 AS pp_district,
           pp.postal_code              AS pp_postal_code,
           pp.emergency_contact_name   AS pp_emergency_contact_name,
           pp.emergency_contact_phone  AS pp_emergency_contact_phone,
           pp.created_at               AS pp_created_at,
           pp.updated_at               AS pp_updated_at,
           u.id                        AS u_id,
           u.email                     AS u_email,
           u.name                      AS u_name,
           u.role                      AS u_role
         FROM senior_profiles sp
         JOIN person_profiles pp ON pp.id = sp.person_id
         JOIN users u ON u.id = pp.user_id
         WHERE sp.person_id = $1",
    )
    .bind(person_id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(SeniorProfileJoinRow::into_senior_with_person))
}
